extern crate cairo;
extern crate glib;
extern crate gtk;

use std::cell::RefCell;
use std::mem;
use std::rc::Rc;
use std::sync::mpsc;
use std::thread;

use cairo::prelude::*;
use cairo::{Context, Format, ImageSurface};
use glib::Continue;
use gtk::traits::*;
use gtk::signal::Inhibit;
use gtk::{DrawingArea, Window, WindowType};

// make moving clones into closures more convenient
macro_rules! clone {
    ($($n:ident),+; || $body:block) => (
        {
            $( let $n = $n.clone(); )+
            move || { $body }
        }
    );
    ($($n:ident),+; |$($p:ident),+| $body:block) => (
        {
            $( let $n = $n.clone(); )+
            move |$($p),+| { $body }
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
// Each worker thread waits for a buffer (Box<[u8]>) to render into, wraps
// it into ImageSurface, sleeps for a while, does the drawing, then sends the
// underlying buffer back and waits for the next one.
//
// The GUI thread holds an ImageSurface per image part at all times, these
// surfaces are painted on a DrawingArea in its 'draw' signal handler. This
// thread periodically checks if any worker has sent a freshly rendered buffer.
// Upon receipt it's wrapped in ImageSurface and swapped with the previously
// held surface, whose buffer is sent to the worker thread again. Then the
// appropriate part of the DrawingArea is invalidated prompting a redraw.
//
// The two buffers per thread are allocated and initialized once and sent back
// and forth repeatedly.

fn main() {
    gtk::init().unwrap_or_else(|_| panic!("Failed to initialize GTK."));
    let window = Window::new(WindowType::Toplevel).unwrap();
    let area = DrawingArea::new().unwrap();
    window.add(&area);

    window.connect_delete_event(|_, _| {
        gtk::main_quit();
        Inhibit(false)
    });

    let format = Format::Rgb24;
    let width = 200;
    let height = 200;
    area.set_size_request(width * 2, height * 2);

    let (initial_buf, stride) = draw_initial(format, width, height);
    let (ready_tx, ready_rx) = mpsc::channel();

    let mut images = Vec::new();
    let mut origins = Vec::new();
    let mut workers = Vec::new();

    for thread_num in 0..4 {
        let (tx, rx) = mpsc::channel();
        // allocate two buffers and copy the initial pattern into them
        let buf0 = initial_buf.to_vec().into_boxed_slice();
        let buf1 = initial_buf.to_vec().into_boxed_slice();
        // wrap the first one in a surface and set it up to be sent to the
        // worker when the surface is destroyed
        images.push(ImageSurface::create_for_data(buf0, clone!(tx; |b| { let _ = tx.send(b); }),
            format, width, height, stride));
        // send the second one immediately
        let _ = tx.send(buf1);
        origins.push(match thread_num {
            0 => (0, 0),
            1 => (width, 0),
            2 => (0, height),
            _ => (width, height),
        });
        workers.push(tx);

        let x = (width - origins[thread_num].0) as f64;
        let y = (height - origins[thread_num].1) as f64;
        let delay = (100 << thread_num) - 5;

        // spawn the worker thread
        thread::spawn(clone!(ready_tx; || {
            let mut n = 0;
            for buf in rx.iter() {
                n = (n + 1) % 0x10000;
                // create the surface and send the buffer back when it's destroyed
                let image = ImageSurface::create_for_data(buf,
                    clone!(ready_tx; |b| { let _ = ready_tx.send((thread_num, b)); }),
                    format, width, height, stride);
                let cr = Context::new(&image);
                // draw an arc with a weirdly calculated radius
                draw_slow(&cr, delay, x, y, 1.2_f64.powi(((n as i32) << thread_num) % 32));
                image.flush();
            }
        }));
    }

    // our signal and timeout handler closures have to be 'static,
    // so they can't just borrow these
    let cell = Rc::new(RefCell::new((images, origins, workers)));

    area.connect_draw(clone!(cell; |_x, cr| {
        let (ref images, ref origins, _) = *cell.borrow();
        for (image, origin) in images.iter().zip(origins.iter()) {
            draw_image_if_dirty(&cr, image, *origin, (width, height));
            // if we don't reset the source, the context may hold on to
            // the surface indefinitely, the buffer will be stuck there
            // and the worker thread will starve
            cr.set_source_rgb(0., 0., 0.);
        }
        Inhibit(false)
    }));

    glib::timeout_add(100, move || {
        while let Ok((thread_num, buf)) = ready_rx.try_recv() {
            let &mut (ref mut images, ref origins, ref workers) = &mut *cell.borrow_mut();
            let tx = workers[thread_num].clone();
            let mut image = ImageSurface::create_for_data(buf, move |b| { let _ = tx.send(b); },
                format, width, height, stride);
            mem::swap(&mut images[thread_num], &mut image);
            area.queue_draw_area(origins[thread_num].0, origins[thread_num].1, width, height);
        }
        Continue(true)
    });

    window.show_all();
    gtk::main();
}

fn draw_initial(format: Format, width: i32, height: i32) -> (Box<[u8]>, i32) {
    let image = ImageSurface::create(format, width, height);
    let cr = Context::new(&image);
    cr.set_source_rgb(0., 1., 0.);
    cr.paint();
    image.flush();
    let mut buf = vec![0; image.len()].into_boxed_slice();
    image.get_data(&mut buf);
    (buf, image.get_stride())
}

fn draw_slow(cr: &Context, delay: u32, x: f64, y: f64, radius: f64) {
    thread::sleep_ms(delay);
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
