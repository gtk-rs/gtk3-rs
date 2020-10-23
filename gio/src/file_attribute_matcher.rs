use gio_sys;
use glib::translate::*;
use glib::GString;
use std::iter::{IntoIterator, Iterator};

pub struct FileAttributematcherIter(::FileAttributeMatcher);

impl Iterator for FileAttributematcherIter {
    type Item = GString;

    fn next(&mut self) -> Option<GString> {
        unsafe {
            from_glib_none(gio_sys::g_file_attribute_matcher_enumerate_next(
                self.0.to_glib_none().0,
            ))
        }
    }
}

impl IntoIterator for ::FileAttributeMatcher {
    type Item = GString;
    type IntoIter = FileAttributematcherIter;

    fn into_iter(self) -> Self::IntoIter {
        FileAttributematcherIter(self)
    }
}
