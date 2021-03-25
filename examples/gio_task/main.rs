use futures_channel::oneshot;
use gtk::glib::subclass::prelude::*;
use gtk::glib::translate::*;
use gtk::prelude::AsyncResultExt;
use gtk::prelude::Cast;
use gtk::prelude::FileExt;
use gtk::prelude::ToValue;
use gtk::{gio, glib};

mod imp {
    use super::*;

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
}

mod ffi {
    use super::*;
    pub type FileSize = <imp::FileSize as ObjectSubclass>::Instance;

    /// # Safety
    ///
    /// This is the ffi method to asynchronously get the file size. It accepts a callback of type
    /// GAsyncReadyCallback, that will be invoked when the async operation completes. Typically,
    /// this callback will invoke the get_file_size_finish method (implemented below) to get the
    /// Task result and perform some operation with it.
    #[no_mangle]
    pub unsafe extern "C" fn my_file_size_get_file_size_async(
        this: *mut FileSize,
        cancellable: *mut gio::ffi::GCancellable,
        callback: gio::ffi::GAsyncReadyCallback,
        user_data: glib::ffi::gpointer,
    ) {
        let cancellable = gio::Cancellable::from_glib_borrow(cancellable);
        let closure = move |result: &gio::AsyncResult, source_object: Option<&glib::Object>| {
            let result: *mut gio::ffi::GAsyncResult = result.to_glib_none().0;
            let source_object: *mut glib::object::GObject = source_object.to_glib_none().0;
            callback.unwrap()(source_object, result, user_data)
        };

        let source_object = &super::FileSize::from_glib_borrow(this);
        let task = gio::Task::new(
            Some(&source_object.upcast_ref::<glib::Object>()),
            Some(cancellable.as_ref()),
            closure,
        );

        glib::MainContext::default().spawn_local(async move {
            let size = gio::File::new_for_path("Cargo.toml")
                .query_info_async_future("*", gio::FileQueryInfoFlags::NONE, glib::PRIORITY_DEFAULT)
                .await
                .unwrap()
                .get_size();

            let source_object = task
                .upcast_ref::<gio::AsyncResult>()
                .get_source_object()
                .unwrap();

            let source_object = imp::FileSize::from_instance(
                &source_object.downcast_ref::<super::FileSize>().unwrap(),
            );

            source_object.size.replace(Some(size));
            task.return_value(&size.to_value());
        });
    }

    /// # Safety
    ///
    /// This method will typically be invoked in the callback passed to my_file_size_get_file_size_async in order
    /// to propagate the Task result.
    #[no_mangle]
    pub unsafe extern "C" fn my_file_size_get_file_size_finish(
        _this: *mut FileSize,
        result: *mut gio::ffi::GAsyncResult,
        _error: *mut *mut glib::ffi::GError,
    ) -> i64 {
        gio::AsyncResult::from_glib_borrow(result)
            .downcast_ref::<gio::Task>()
            .unwrap()
            .propagate_value()
            .unwrap()
            .get::<i64>()
            .unwrap()
            .unwrap()
    }

    /// # Safety
    ///
    /// Simple getter
    #[no_mangle]
    pub unsafe extern "C" fn my_file_size_get_retrieved_size(this: *mut FileSize) -> i64 {
        let simple_object = super::FileSize::from_glib_borrow(this);
        let simple_object =
            imp::FileSize::from_instance(&simple_object.downcast_ref::<super::FileSize>().unwrap());
        let x = *simple_object.size.borrow();
        x.unwrap_or(-1)
    }
}

glib::wrapper! {
    pub struct FileSize(ObjectSubclass<imp::FileSize>);
}

impl FileSize {
    pub fn new() -> Self {
        glib::Object::new(&[]).expect("Failed to create FileSize")
    }

    fn retrieved_size(&self) -> Option<i64> {
        let simple_object = imp::FileSize::from_instance(self);
        *simple_object.size.borrow()
    }

    pub fn get_file_size_async<Q: FnOnce(i64, &FileSize) + 'static>(
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
                .unwrap()
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
                .get_size();

            let source_object = task
                .upcast_ref::<gio::AsyncResult>()
                .get_source_object()
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

// This function mimicks what the C code using the exported async/finish API would do. It first defines a
// callback of type GAsyncResult that internally calls get_file_size_finish to retrieve the Task
// result, and simply prints it out. Then it invokes the my_file_size_get_file_size_async method, passing the
// callback to it as parameter.
fn run_unsafe(send: oneshot::Sender<()>) {
    let simple_object = FileSize::new();
    let cancellable = gio::Cancellable::new();

    // The callback to be passed to my_file_size_get_file_size_async
    unsafe extern "C" fn c_callback(
        source_object: *mut glib::object::GObject,
        result: *mut gio::ffi::GAsyncResult,
        user_data: glib::ffi::gpointer,
    ) {
        let mut error = std::ptr::null_mut();
        let ret = ffi::my_file_size_get_file_size_finish(
            source_object as *mut ffi::FileSize,
            result,
            &mut error,
        );
        if !error.is_null() {
            eprintln!("Task returned error!");
            return;
        }

        println!("Unsafe callback - Returned value from task: {}", ret);
        println!(
            "Unsafe callback - FileSize::size: {}",
            ffi::my_file_size_get_retrieved_size(source_object as *mut ffi::FileSize)
        );

        Box::from_raw(user_data as *mut oneshot::Sender<()>)
            .send(())
            .unwrap();
    }

    // The actual call to my_file_size_get_file_size_async
    unsafe {
        ffi::my_file_size_get_file_size_async(
            simple_object.to_glib_none().0,
            cancellable.to_glib_none().0,
            Some(c_callback),
            Box::into_raw(Box::new(send)) as glib::ffi::gpointer,
        );
    }
}

// This function is the "safe" counterpart of "run_unsafe", using only safe rust
// bindings to accomplish the same task
fn run_safe(send: oneshot::Sender<()>) {
    let simple_object = FileSize::new();
    let cancellable = gio::Cancellable::new();

    let closure = move |value: i64, source_object: &FileSize| {
        println!("Safe callback - Returned value from task: {}", value);
        println!(
            "Safe callback - FileSize::size: {}",
            source_object.retrieved_size().unwrap()
        );

        send.send(()).unwrap();
    };

    simple_object.get_file_size_async(Some(&cancellable), closure);
}

fn main() {
    let main_context = glib::MainContext::default();
    let main_loop = glib::MainLoop::new(Some(&main_context), false);
    let main_loop_clone = main_loop.clone();
    let (send_safe, recv_safe) = oneshot::channel();
    let (send_unsafe, recv_unsafe) = oneshot::channel();

    main_context.push_thread_default();
    main_context.invoke_local(move || {
        run_unsafe(send_unsafe);
    });
    main_context.invoke_local(move || {
        run_safe(send_safe);
    });

    main_context.spawn_local(async move {
        recv_safe.await.unwrap();
        recv_unsafe.await.unwrap();
        main_loop_clone.quit();
    });
    main_loop.run();
    main_context.pop_thread_default();
}
