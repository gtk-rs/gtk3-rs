use glib::subclass::prelude::*;
use gtk::glib;

// FileSize is a simple object that will just contain the read file size.
// Initially the optional size field will be initialized to None.
#[derive(Default)]
pub struct FileSize {
    pub size: std::cell::RefCell<Option<i64>>,
}

#[glib::object_subclass]
impl ObjectSubclass for FileSize {
    const NAME: &'static str = "FileSize";
    type ParentType = glib::Object;
    type Type = super::FileSize;
}

impl ObjectImpl for FileSize {}
