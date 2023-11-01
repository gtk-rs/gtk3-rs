use gtk::glib;
use gtk::prelude::*;

use std::thread;
use std::time::Duration;

fn main() {
    let application = gtk::Application::new(
        Some("com.github.gtk-rs.examples.multithreading_context"),
        Default::default(),
    );

    application.connect_activate(build_ui);

    application.run();
}

fn build_ui(application: &gtk::Application) {
    let window = gtk::ApplicationWindow::new(application);

    window.set_title("Multithreading GTK+ Program");
    window.set_border_width(10);
    window.set_position(gtk::WindowPosition::Center);
    window.set_default_size(600, 400);

    let text_view = gtk::TextView::new();
    let scroll = gtk::ScrolledWindow::new(gtk::Adjustment::NONE, gtk::Adjustment::NONE);
    scroll.set_policy(gtk::PolicyType::Automatic, gtk::PolicyType::Automatic);
    scroll.add(&text_view);

    let (tx, rx) = async_channel::bounded(10);

    thread::spawn(move || {
        for i in 1..100 {
            // do long work
            thread::sleep(Duration::from_millis(50));
            // send result to channel
            tx.send_blocking(format!("#{i} Text from another thread."))
                .expect("Couldn't send data to channel");
            // receiver will be run on the main thread
        }
    });

    let text_buffer = text_view
        .buffer()
        .expect("Couldn't get buffer from text_view");

    // Spawn a future on main context and set the text buffer text from here
    glib::MainContext::default().spawn_local(async move {
        while let Ok(text) = rx.recv().await {
            text_buffer.set_text(&text);
        }
    });

    window.add(&scroll);
    window.show_all();
}
