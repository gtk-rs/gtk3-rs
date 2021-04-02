use std::cell::RefCell;
use std::rc::Rc;

/// Helper struct that allows passing the pixels to the Cairo image surface and once the
/// image surface is destroyed the pixels will be stored in the return_location.
///
/// This allows us to give temporary ownership of the pixels to the Cairo surface and later
/// retrieve them back in a safe way while ensuring that nothing else still has access to
/// it.
pub struct ImageHolder {
    image: Option<Box<[u8]>>,
    return_location: Rc<RefCell<Option<Box<[u8]>>>>,
}

impl ImageHolder {
    pub fn new(image: Option<Box<[u8]>>, return_location: Rc<RefCell<Option<Box<[u8]>>>>) -> Self {
        Self {
            image,
            return_location,
        }
    }
}

/// This stores the pixels back into the return_location as now nothing
/// references the pixels anymore
impl Drop for ImageHolder {
    fn drop(&mut self) {
        *self.return_location.borrow_mut() = Some(self.image.take().expect("Holding no image"));
    }
}

/// Needed for ImageSurface::create_for_data() to be able to access the pixels
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
