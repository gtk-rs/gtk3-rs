//! # Synchronizing Widgets
//!
//! You can use signals in order to synchronize the values of widgets. In this example a spin
//1 button and a horizontal scale will get interlocked.

extern crate gio;
extern crate gtk;

use gio::prelude::*;
use gtk::Builder;
use gtk::prelude::*;

use std::env::args;

// make moving clones into closures more convenient
macro_rules! clone {
    (@param _) => ( _ );
    (@param $x:ident) => ( $x );
    ($($n:ident),+ => move || $body:expr) => (
        {
            $( let $n = $n.clone(); )+
            move || $body
        }
    );
    ($($n:ident),+ => move |$($p:tt),+| $body:expr) => (
        {
            $( let $n = $n.clone(); )+
            move |$(clone!(@param $p),)+| $body
        }
    );
}

fn build_ui(application: &gtk::Application) {
    let glade_src = include_str!("sync_widgets.glade");
    let builder = Builder::new();
    builder.add_from_string(glade_src).expect("Couldn't add from string");

    let slider: gtk::Scale = builder.get_object("slider").expect("Couldn't get slider");
    let spin_button: gtk::SpinButton = builder.get_object("spin_button")
                                              .expect("Couldn't get spin_button");
    let slider_adj = slider.get_adjustment();
    spin_button.get_adjustment().connect_value_changed(move |adj| {
        slider_adj.set_value(adj.get_value());
    });
    let spin_button_adj = spin_button.get_adjustment();
    slider.get_adjustment().connect_value_changed(move |adj| {
        spin_button_adj.set_value(adj.get_value());
    });

    let window: gtk::ApplicationWindow = builder.get_object("window").expect("Couldn't get window");
    window.set_application(application);
    window.connect_delete_event(clone!(window => move |_, _| {
        window.destroy();
        Inhibit(false)
    }));

    window.show_all();
}

fn main() {
    let application = gtk::Application::new("com.github.sync_widgets",
                                            gio::ApplicationFlags::empty())
                                       .expect("Initialization failed...");

    application.connect_startup(move |app| {
        build_ui(app);
    });
    application.connect_activate(|_| {});

    application.run(&args().collect::<Vec<_>>());
}
