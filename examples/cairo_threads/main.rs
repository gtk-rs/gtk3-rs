mod drawing;
pub mod image;

use std::cell::RefCell;
use std::rc::Rc;
use std::sync::mpsc;
use std::thread;
use std::time::Duration;

use gtk::cairo::Context;
use gtk::glib;
use gtk::prelude::*;
use gtk::{ApplicationWindow, DrawingArea};

use drawing::{draw_image_if_dirty, draw_initial, draw_slow};

const WIDTH: i32 = 200;
const HEIGHT: i32 = 200;

fn main() {
    let application = gtk::Application::new(
        Some("com.github.gtk-rs.examples.cairo_threads"),
        Default::default(),
    )
    .expect("Initialization failed...");

    application.connect_activate(build_ui);
    application.run();
}

fn build_ui(application: &gtk::Application) {
    let window = ApplicationWindow::new(application);
    let area = DrawingArea::new();
    window.add(&area);

    area.set_size_request(WIDTH * 2, HEIGHT * 2);

    // Create the initial, green image
    let initial_image = draw_initial(WIDTH, HEIGHT);

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
        thread::spawn(glib::clone!(@strong ready_tx => move || {
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
    area.connect_draw(
        glib::clone!(@weak workspace => @default-return Inhibit(false), move |_, cr| {
            let (ref images, ref origins, _) = *workspace;

            for (image, origin) in images.iter().zip(origins.iter()) {
                image.borrow_mut().with_surface(|surface| {
                    draw_image_if_dirty(&cr, surface, *origin, (WIDTH, HEIGHT));
                });
            }

            Inhibit(false)
        }),
    );

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
