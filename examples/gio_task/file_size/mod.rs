pub mod ffi;
mod imp;

use gtk::glib::subclass::prelude::*;
use gtk::prelude::*;
use gtk::{gio, glib};

glib::wrapper! {
    pub struct FileSize(ObjectSubclass<imp::FileSize>);
}

impl FileSize {
    pub fn new() -> Self {
        glib::Object::new(&[]).expect("Failed to create FileSize")
    }

    pub fn retrieved_size(&self) -> Option<i64> {
        let simple_object = imp::FileSize::from_instance(self);
        *simple_object.size.borrow()
    }

    pub fn file_size_async<Q: FnOnce(i64, &FileSize) + 'static>(
        &self,
        cancellable: Option<&gio::Cancellable>,
        callback: Q,
    ) {
        let closure = move |result: &gio::AsyncResult, source_object: Option<&glib::Object>| {
            let value = result
                .downcast_ref::<gio::Task>()
                .unwrap()
                .propagate_value()
                .unwrap()
                .get::<i64>()
                .unwrap();
            let source_object = source_object.unwrap().downcast_ref::<FileSize>().unwrap();
            callback(value, &source_object);
        };

        let task = gio::Task::new(
            Some(&self.upcast_ref::<glib::Object>()),
            cancellable,
            closure,
        );

        glib::MainContext::default().spawn_local(async move {
            let size = gio::File::new_for_path("Cargo.toml")
                .query_info_async_future("*", gio::FileQueryInfoFlags::NONE, glib::PRIORITY_DEFAULT)
                .await
                .unwrap()
                .size();

            let source_object = task
                .upcast_ref::<gio::AsyncResult>()
                .source_object()
                .unwrap();

            let source_object =
                imp::FileSize::from_instance(&source_object.downcast_ref::<FileSize>().unwrap());

            source_object.size.replace(Some(size));
            task.return_value(&size.to_value());
        });
    }
}

impl Default for FileSize {
    fn default() -> Self {
        FileSize::new()
    }
}
