use futures::{channel::mpsc, StreamExt};
use gtk::glib;
use gtk::prelude::*;
use gtk::{ApplicationWindow, Label};
use std::thread;

fn main() {
    let application = gtk::Application::new(
        Some("com.github.gtk-rs.examples.communication_thread"),
        Default::default(),
    )
    .expect("Initialization failed...");
    application.connect_activate(build_ui);
    application.run();
}

fn build_ui(application: &gtk::Application) {
    let window = ApplicationWindow::new(application);
    let label = Label::new(None);
    window.add(&label);

    // Create a channel between communication thread and main event loop:
    let (sender, receiver) = mpsc::channel(1000);

    spawn_local_handler(label, receiver);
    start_communication_thread(sender);
    window.show_all();
}

/// Spawn channel receive task on the main event loop.
fn spawn_local_handler(label: gtk::Label, mut receiver: mpsc::Receiver<String>) {
    let main_context = glib::MainContext::default();
    let future = async move {
        while let Some(item) = receiver.next().await {
            label.set_text(&item);
        }
    };
    main_context.spawn_local(future);
}

/// Spawn separate thread to handle communication.
fn start_communication_thread(mut sender: mpsc::Sender<String>) {
    // Note that blocking I/O with threads can be prevented
    // by using asynchronous code, which is often a better
    // choice. For the sake of this example, we showcase the
    // way to use a thread when there is no other option.

    thread::spawn(move || {
        let mut counter = 0;
        loop {
            // Instead of a counter, your application code will
            // block here on TCP or serial communications.
            let data = format!("Counter = {}!", counter);
            println!("Thread received data: {}", data);
            match sender.try_send(data) {
                Ok(_) => {}
                Err(err) => {
                    if err.is_full() {
                        println!("Data is produced too fast for GUI");
                    } else if err.is_disconnected() {
                        println!("GUI stopped, stopping thread.");
                        break;
                    }
                }
            }
            counter += 1;
            thread::sleep(std::time::Duration::from_millis(100));
        }
    });
}
