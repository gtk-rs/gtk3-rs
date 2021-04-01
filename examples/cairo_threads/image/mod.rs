mod image_holder;

use gtk::cairo::{Format, ImageSurface};
use std::cell::RefCell;
use std::rc::Rc;

use image_holder::ImageHolder;

/// Our custom image type. This stores a heap allocated byte array for the pixels for each of our
/// images, can be sent safely between threads and can be temporarily converted to a Cairo image
/// surface for drawing operations
#[derive(Clone)]
pub struct Image {
    width: i32,
    height: i32,
    data: Option<Box<[u8]>>,
}

impl Image {
    /// Creates a new, black image
    pub fn new(width: i32, height: i32) -> Self {
        Image {
            width,
            height,
            data: Some(vec![0; 4 * width as usize * height as usize].into()),
        }
    }

    /// Calls the given closure with a temporary Cairo image surface. After the closure has returned
    /// there must be no further references to the surface.
    pub fn with_surface<F: FnOnce(&ImageSurface)>(&mut self, func: F) {
        // Temporary move out the pixels
        let image = self.data.take().expect("Empty image");

        // A new return location that is then passed to our helper struct below
        let return_location = Rc::new(RefCell::new(None));
        {
            let holder = ImageHolder::new(Some(image), return_location.clone());

            // The surface will own the image for the scope of the block below
            let surface = ImageSurface::create_for_data(
                holder,
                Format::Rgb24,
                self.width,
                self.width,
                4 * self.width,
            )
            .expect("Can't create surface");
            func(&surface);

            // Now the surface will be destroyed and the pixels are stored in the return_location
        }

        // And here move the pixels back again
        self.data = Some(
            return_location
                .borrow_mut()
                .take()
                .expect("Image not returned"),
        );
    }
}
