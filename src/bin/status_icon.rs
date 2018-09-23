//! # TreeView Sample
//!
//! This sample demonstrates how to create a StatusIcon.

extern crate gdk_pixbuf;
extern crate gio;
extern crate glib;
extern crate gtk;

use gdk_pixbuf::Pixbuf;
use gio::MemoryInputStream;
use glib::Bytes;
use gtk::{
    ButtonsType, DialogExt, DialogFlags, Menu, MenuExt, MenuExtManual, MenuItem,
    MenuItemExt, MenuShellExt, MessageDialog, MessageType, StatusIcon, StatusIconExt, WidgetExt,
};

use std::rc::Rc;
use std::cell::RefCell;

fn main() {
    gtk::init().expect("gtk initialization failed");

    let menu = Menu::new();
    let dialog_item = MenuItem::new_with_label("Show notification");
    let close_item = MenuItem::new_with_label("Close app");

    menu.append(&dialog_item);
    menu.append(&close_item);

    let is_shown = Rc::new(RefCell::new(false));

    let c_is_shown = is_shown.clone();
    dialog_item.connect_activate(move |_| {
        *c_is_shown.borrow_mut() = false;
        let parent: Option<&gtk::Window> = None; // stupid hack to avoid rustc error
        let diag = MessageDialog::new(parent,
                                      DialogFlags::MODAL,
                                      MessageType::Error,
                                      ButtonsType::Ok,
                                      "What about the Kirin Tor?");
        diag.run();
        diag.destroy();
    });
    close_item.connect_activate(|_| {
        println!("time to quit!");
        gtk::main_quit();
    });

    let memory_stream = MemoryInputStream::new_from_bytes(
                            &Bytes::from_static(include_bytes!("../../resources/menu.png")));
    let logo = Pixbuf::new_from_stream(&memory_stream, None);
    let logo = match logo {
        Ok(l) => l,
        Err(e) => {
            println!("Got an error: {:?}", e);
            return
        }
    };
    let status_icon = StatusIcon::new_from_pixbuf(&logo);

    status_icon.connect_popup_menu(move |_, _, _| {
        let new_shown = if *is_shown.borrow() == true {
            menu.popdown();
            false
        } else {
            menu.show_all();
            // Why 1? Well, any button is accepted so why not?
            menu.popup_easy(1, gtk::get_current_event_time());
            true
        };
        *is_shown.borrow_mut() = new_shown;
    });
    status_icon.connect_activate(|_| {
        let parent: Option<&gtk::Window> = None; // stupid hack to avoid rustc error
        let diag = MessageDialog::new(parent,
                                      DialogFlags::MODAL,
                                      MessageType::Info,
                                      ButtonsType::Ok,
                                      "Right click on the status icon!");
        diag.run();
        diag.destroy();
    });

    status_icon.set_visible(true);
    status_icon.set_has_tooltip(true);

    gtk::main();
}
