mod file_size;

use futures_channel::oneshot;
use glib::clone;
use gtk::glib::translate::*;
use gtk::{gio, glib};

use file_size::FileSize;

fn main() {
    let main_context = glib::MainContext::default();
    let main_loop = glib::MainLoop::new(Some(&main_context), false);
    let (send_safe, recv_safe) = oneshot::channel();
    let (send_unsafe, recv_unsafe) = oneshot::channel();

    main_context.push_thread_default();
    main_context.invoke_local(move || {
        run_unsafe(send_unsafe);
    });
    main_context.invoke_local(move || {
        run_safe(send_safe);
    });

    main_context.spawn_local(clone!(@strong main_loop => async move {
        recv_safe.await.unwrap();
        recv_unsafe.await.unwrap();
        main_loop.quit();
    }));
    main_loop.run();
    main_context.pop_thread_default();
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
        let ret = file_size::ffi::my_file_size_get_file_size_finish(
            source_object as *mut file_size::ffi::FileSize,
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
            file_size::ffi::my_file_size_get_retrieved_size(
                source_object as *mut file_size::ffi::FileSize
            )
        );

        Box::from_raw(user_data as *mut oneshot::Sender<()>)
            .send(())
            .unwrap();
    }

    // The actual call to my_file_size_get_file_size_async
    unsafe {
        file_size::ffi::my_file_size_get_file_size_async(
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
