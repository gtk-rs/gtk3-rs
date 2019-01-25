extern crate cairo;
extern crate gio;
extern crate gtk;

use std::thread;
use std::env::args;
use std::sync::mpsc;
use std::rc::Rc;
use std::cell::RefCell;
use std::time::Duration;

use cairo::prelude::*;
use cairo::{Context, Format, ImageSurface};
use gio::prelude::*;
use gtk::prelude::*;
use gtk::{ApplicationWindow, DrawingArea};

const WIDTH: i32 = 200;
const HEIGHT: i32 = 200;

// make moving clones into closures more convenient
macro_rules! clone {
    (@param _) => ( _ );
    (@param $x:ident) => ( $x );
    ($($n:ident),+ => move || $body:expr) => (
        {
            $( let $n = $n.clone(); )+
            move || $body
        }
    );
    ($($n:ident),+ => move |$($p:tt),+| $body:expr) => (
        {
            $( let $n = $n.clone(); )+
            move |$(clone!(@param $p),)+| $body
        }
    );
}

// Our custom image type. This stores a heap allocated byte array for the pixels for each of our
// images, can be sent safely between threads and can be temporarily converted to a Cairo image
// surface for drawing operations
#[derive(Clone)]
struct Image(Option<Box<[u8]>>);

impl Image {
    // Creates a new, black image
    fn new() -> Self {
        Image(Some(vec![0; 4 * WIDTH as usize * HEIGHT as usize].into()))
    }

    // Calls the given closure with a temporary Cairo image surface. After the closure has returned
    // there must be no further references to the surface.
    fn with_surface<F: FnOnce(&ImageSurface)>(&mut self, func: F) {
        // Helper struct that allows passing the pixels to the Cairo image surface and once the
        // image surface is destroyed the pixels will be stored in the return_location.
        //
        // This allows us to give temporary ownership of the pixels to the Cairo surface and later
        // retrieve them back in a safe way while ensuring that nothing else still has access to
        // it.
        struct ImageHolder {
            image: Option<Box<[u8]>>,
            return_location: Rc<RefCell<Option<Box<[u8]>>>>,
        }

        // This stores the pixels back into the return_location as now nothing
        // references the pixels anymore
        impl Drop for ImageHolder {
            fn drop(&mut self) {
                *self.return_location.borrow_mut() = Some(self.image.take().expect("Holding no image"));
            }
        }

        // Needed for ImageSurface::create_for_data() to be able to access the pixels
        impl AsRef<[u8]> for ImageHolder {
            fn as_ref(&self) -> &[u8] {
                self.image.as_ref().expect("Holding no image").as_ref()
            }
        }

        impl AsMut<[u8]> for ImageHolder {
            fn as_mut(&mut self) -> &mut [u8] {
                self.image.as_mut().expect("Holding no image").as_mut()
            }
        }

        // Temporary move out the pixels
        let image = self.0.take().expect("Empty image");

        // A new return location that is then passed to our helper struct below
        let return_location = Rc::new(RefCell::new(None));
        {
            let holder = ImageHolder {
                image: Some(image),
                return_location: return_location.clone(),
            };

            // The surface will own the image for the scope of the block below
            let surface = ImageSurface::create_for_data(holder, Format::Rgb24, WIDTH, HEIGHT, 4 * WIDTH).expect("Can't create surface");
            func(&surface);

            // Now the surface will be destroyed and the pixels are stored in the return_location
        }

        // And here move the pixels back again
        self.0 = Some(return_location.borrow_mut().take().expect("Image not returned"));
    }
}

// This example runs four worker threads rendering parts of the image independently at different
// paces in a sort of double buffered way.
//
// +---+---+
// | 0 | 1 |
// +---+---+
// | 2 | 3 |
// +---+---+
//
// Each worker thread waits for an image to render into, sleeps for a while, does the drawing, then
// sends the image back and waits for the next one.
//
// The GUI thread holds an image per image part at all times and these images are painted on a
// DrawingArea in its 'draw' signal handler whenever needed.
//
// Additionally the GUI thread has a channel for receiving the images from the worker threads. If
// there is a new image, the old image stored by the GUI thread is replaced with the new one and
// the old image is sent back to the worker thread. Then the appropriate part of the DrawingArea is
// invalidated prompting a redraw.
//
// The two images per thread are allocated and initialized once and sent back and forth repeatedly.

fn build_ui(application: &gtk::Application) {
    let window = ApplicationWindow::new(application);
    let area = DrawingArea::new();
    window.add(&area);

    area.set_size_request(WIDTH * 2, HEIGHT * 2);

    // Create the initial, green image
    let initial_image = draw_initial();

    // This is the channel for sending results from the worker thread to the main thread
    // For every received image, queue the corresponding part of the DrawingArea for redrawing
    let (ready_tx, ready_rx) = glib::MainContext::channel(glib::PRIORITY_DEFAULT);

    let mut images = Vec::new();
    let mut origins = Vec::new();
    let mut workers = Vec::new();

    for thread_num in 0..4 {
        // This is the channel for sending image from the GUI thread to the workers so that the
        // worker can draw the new content into them
        let (tx, rx) = mpsc::channel();

        // Copy the initial image in two images for the workers
        let image0 = initial_image.clone();
        let image1 = initial_image.clone();

        // Store the first one in our vec. This is the one that would be drawn in the very
        // beginning by the DrawingArea.
        images.push(RefCell::new(image0));

        // Send the second one immediately to the worker thread for drawing the new content
        let _ = tx.send(image1);

        // Store for each worker thread its render rectangle inside the DrawingArea
        origins.push(match thread_num {
            0 => (0, 0),
            1 => (WIDTH, 0),
            2 => (0, HEIGHT),
            3 => (WIDTH, HEIGHT),
            _ => unreachable!(),
        });

        // And remember the sender side of the channel that allows the GUI thread to send back
        // images to the worker thread
        workers.push(tx);

        let x = (WIDTH - origins[thread_num].0) as f64;
        let y = (HEIGHT - origins[thread_num].1) as f64;
        let delay = Duration::from_millis((100 << thread_num) - 5);

        // Spawn the worker thread
        thread::spawn(clone!(ready_tx => move || {
            let mut n = 0;
            for mut image in rx.iter() {
                n = (n + 1) % 0x10000;

                // Draw an arc with a weirdly calculated radius
                image.with_surface(|surface| {
                    let cr = Context::new(surface);
                    draw_slow(&cr, delay, x, y, 1.2_f64.powi(((n as i32) << thread_num) % 32));
                    surface.flush();
                });

                // Send the finished image back to the GUI thread
                let _ = ready_tx.send((thread_num, image));
            }
        }));
    }

    // The connect-draw signal and the timeout handler closures have to be 'static, and both need
    // to have access to our images and related state.
    let workspace = Rc::new((images, origins, workers));

    // Whenever the drawing area has to be redrawn, render the latest images in the correct
    // locations
    area.connect_draw(clone!(workspace => move |_, cr| {
        let (ref images, ref origins, _) = *workspace;

        for (image, origin) in images.iter().zip(origins.iter()) {
            image.borrow_mut().with_surface(|surface| {
                draw_image_if_dirty(&cr, surface, *origin, (WIDTH, HEIGHT));
            });
        }

        Inhibit(false)
    }));

    ready_rx.attach(None, move |(thread_num, image)| {
        let (ref images, ref origins, ref workers) = *workspace;

        // Swap the newly received image with the old stored one and send the old one back to
        // the worker thread
        let tx = &workers[thread_num];
        let image = images[thread_num].replace(image);
        let _ = tx.send(image);

        area.queue_draw_area(origins[thread_num].0, origins[thread_num].1, WIDTH, HEIGHT);

        Continue(true)
    });

    window.show_all();
}

fn main() {
    let application = gtk::Application::new("com.github.gtk-rs.examples.cairo_threads",
                                            Default::default())
                                       .expect("Initialization failed...");

    application.connect_activate(|app| {
        build_ui(app);
    });

    application.run(&args().collect::<Vec<_>>());
}

// Creates a new image and fill it with green
fn draw_initial() -> Image {
    let mut image = Image::new();

    image.with_surface(|surface| {
        let cr = Context::new(surface);
        cr.set_source_rgb(0., 1., 0.);
        cr.paint();
    });

    image
}

// Sleep for a while and then draw an arc with a given radius
fn draw_slow(cr: &Context, delay: Duration, x: f64, y: f64, radius: f64) {
    use std::f64::consts::PI;

    thread::sleep(delay);
    cr.set_source_rgb(0., 0., 0.);
    cr.paint();
    cr.set_source_rgb(1., 1., 1.);
    cr.arc(x, y, radius, 0.0, 2. * PI);
    cr.stroke();
}

// Render the image surface into the context at the given position
fn draw_image_if_dirty(cr: &Context, image: &ImageSurface, origin: (i32, i32),
        dimensions: (i32, i32)) {
    let x = origin.0 as f64;
    let y = origin.1 as f64;
    let w = dimensions.0 as f64;
    let h = dimensions.1 as f64;
    let (clip_x1, clip_y1, clip_x2, clip_y2) = cr.clip_extents();
    if clip_x1 >= x + w || clip_y1 >= y + h || clip_x2 <= x || clip_y2 <= y {
        return;
    }
    cr.set_source_surface(image, x, y);
    cr.paint();

    // Release the reference to the surface again
    cr.set_source_rgba(0.0, 0.0, 0.0, 0.0);
}

