use cairo::{Context, Format, ImageSurface};
use std::fs::File;

fn main() {
    let surface = ImageSurface::create(Format::ARgb32, 120, 120).expect("Can't create surface");
    let cr = Context::new(&surface);
    // Examples are in 1.0 x 1.0 coordinate space
    cr.scale(120.0, 120.0);

    // Drawing code goes here
    cr.set_line_width(0.1);
    cr.set_source_rgb(0.0, 0.0, 0.0);
    cr.rectangle(0.25, 0.25, 0.5, 0.5);
    cr.stroke();

    let mut file = File::create("file.png").expect("Couldn't create 'file.png'");
    match surface.write_to_png(&mut file) {
        Ok(_) => println!("file.png created"),
        Err(_) => println!("Error create file.png"),
    }
}
