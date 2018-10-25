extern crate cairo;
extern crate gio;
extern crate gtk;

use std::cell::RefCell;
use std::env::args;

use std::mem;
use std::rc::Rc;
use std::sync::mpsc;
use std::thread;
use std::time::Duration;

use cairo::prelude::*;
use cairo::{Context, Format, ImageSurface};
use gio::prelude::*;
use gtk::prelude::*;
use gtk::{ApplicationWindow, DrawingArea};

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

// This example runs four worker threads rendering parts of the image
// independently at different paces in a sort of double buffered way.
//
// +---+---+
// | 0 | 1 |
// +---+---+
// | 2 | 3 |
// +---+---+
//
// Each worker thread waits for an image (cairo::ImageSurface) to render into, sleeps for a while,
// does the drawing, then sends the image back and waits for the next one.
//
// The GUI thread holds an ImageSurface per image part at all times, these
// surfaces are painted on a DrawingArea in its 'draw' signal handler. This
// thread periodically checks if any worker has sent a freshly rendered image.
// Upon receipt it's wrapped in ImageSurface and swapped with the previously
// held surface, whose image is sent to the worker thread again. Then the
// appropriate part of the DrawingArea is invalidated prompting a redraw.
//
// The two images per thread are allocated and initialized once and sent back
// and forth repeatedly.

fn build_ui(application: &gtk::Application) {
    let window = ApplicationWindow::new(application);
    let area = DrawingArea::new();
    window.add(&area);

    window.connect_delete_event(|win, _| {
        win.destroy();
        Inhibit(false)
    });

    let format = Format::Rgb24;
    let width = 200;
    let height = 200;
    area.set_size_request(width * 2, height * 2);

    let initial_image = draw_initial(format, width, height);
    let (ready_tx, ready_rx) = mpsc::channel();

    let mut images = Vec::new();
    let mut origins = Vec::new();
    let mut workers = Vec::new();

    for thread_num in 0..4 {
        let (tx, rx) = mpsc::channel();
        // copy the initial image in two images for the workers
        let image0 = duplicate_image(&initial_image);
        let image1 = duplicate_image(&initial_image);
        // store the first one in our vec
        images.push(image0);
        // send the second one immediately
        let _ = tx.send(image1);
        origins.push(match thread_num {
            0 => (0, 0),
            1 => (width, 0),
            2 => (0, height),
            _ => (width, height),
        });
        workers.push(tx);

        let x = (width - origins[thread_num].0) as f64;
        let y = (height - origins[thread_num].1) as f64;
        let delay = Duration::from_millis((100 << thread_num) - 5);

        // spawn the worker thread
        thread::spawn(clone!(ready_tx => move || {
            let mut n = 0;
            for image in rx.iter() {
                n = (n + 1) % 0x10000;
                let cr = Context::new(&image);
                // draw an arc with a weirdly calculated radius
                draw_slow(&cr, delay, x, y, 1.2_f64.powi(((n as i32) << thread_num) % 32));
                image.flush();
                let _ = ready_tx.send((thread_num, image));
            }
        }));
    }

    // our signal and timeout handler closures have to be 'static,
    // so they can't just borrow these
    let cell = Rc::new(RefCell::new((images, origins, workers)));

    area.connect_draw(clone!(cell => move |_, cr| {
        let (ref images, ref origins, _) = *cell.borrow();
        for (image, origin) in images.iter().zip(origins.iter()) {
            draw_image_if_dirty(&cr, image, *origin, (width, height));
        }
        Inhibit(false)
    }));

    gtk::timeout_add(100, move || {
        while let Ok((thread_num, mut image)) = ready_rx.try_recv() {
            let &mut (ref mut images, ref origins, ref workers) = &mut *cell.borrow_mut();
            let tx = &workers[thread_num];
            mem::swap(&mut images[thread_num], &mut image);
            let _ = tx.send(image);
            area.queue_draw_area(origins[thread_num].0, origins[thread_num].1, width, height);
        }
        Continue(true)
    });

    window.show_all();
}

fn main() {
    let application = gtk::Application::new("com.github.gtk-rs.examples.cairo_threads",
                                            gio::ApplicationFlags::empty())
                                       .expect("Initialization failed...");

    application.connect_activate(|app| {
        build_ui(app);
    });

    application.run(&args().collect::<Vec<_>>());
}

fn draw_initial(format: Format, width: i32, height: i32) -> ImageSurface {
    let image = ImageSurface::create(format, width, height).expect("Can't create surface");
    {
        let cr = Context::new(&image);
        cr.set_source_rgb(0., 1., 0.);
        cr.paint();
        // Destroying the context releases its reference to `image`.
    }
    // We have a unique reference to `image` again and return it
    image
}

fn draw_slow(cr: &Context, delay: Duration, x: f64, y: f64, radius: f64) {
    thread::sleep(delay);
    cr.set_source_rgb(0., 0., 0.);
    cr.paint();
    cr.set_source_rgb(1., 1., 1.);
    cr.arc(x, y, radius, 0.0, 3.1416 * 2.);
    cr.stroke();
}

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
}

fn duplicate_image(image: &ImageSurface) -> ImageSurface {
    let image_dup = ImageSurface::create(image.get_format(), image.get_width(), image.get_height()).expect("Can't create surface");
    {
        let cr = Context::new(&image_dup);
        cr.set_source_surface(image, 0.0, 0.0);
        cr.paint();
    }

    image_dup
}
