use std::thread;
use std::time::Duration;

use gtk::cairo::{Context, ImageSurface};

use crate::image::Image;

/// Creates a new image and fill it with green
pub fn draw_initial(width: i32, height: i32) -> Image {
    let mut image = Image::new(width, height);

    image.with_surface(|surface| {
        let cr = Context::new(surface);
        cr.set_source_rgb(0., 1., 0.);
        cr.paint();
    });

    image
}

/// Sleep for a while and then draw an arc with a given radius
pub fn draw_slow(cr: &Context, delay: Duration, x: f64, y: f64, radius: f64) {
    use std::f64::consts::PI;

    thread::sleep(delay);
    cr.set_source_rgb(0., 0., 0.);
    cr.paint();
    cr.set_source_rgb(1., 1., 1.);
    cr.arc(x, y, radius, 0.0, 2. * PI);
    cr.stroke();
}

/// Render the image surface into the context at the given position
pub fn draw_image_if_dirty(
    cr: &Context,
    image: &ImageSurface,
    origin: (i32, i32),
    dimensions: (i32, i32),
) {
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
