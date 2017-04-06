extern crate gtk;

use gtk::prelude::*;

use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

fn create_sub_window(title: &str, main_window_entry: gtk::Entry, id: usize,
                     windows: Rc<RefCell<HashMap<usize, gtk::Window>>>) {
    let window = gtk::Window::new(gtk::WindowType::Toplevel);

    window.set_title(title);
    window.set_default_size(400, 200);

    let windows_clone = windows.clone();
    window.connect_delete_event(move |_, _| {
        windows_clone.borrow_mut().remove(&id);
        Inhibit(false)
    });

    let button = gtk::Button::new_with_label(&format!("Notify main window with id {}!", id));
    button.connect_clicked(move |_| {
        // When the button is clicked, let's write it on the main window's entry!
        main_window_entry.get_buffer().set_text(&format!("sub window {} clicked", id));
    });
    window.add(&button);

    window.show_all();
    // Once the new window has been created, we put it into our hashmap so we can update its
    // title when needed.
    windows.borrow_mut().insert(id, window.clone());
}

fn create_main_window() -> gtk::Window {
    let window = gtk::Window::new(gtk::WindowType::Toplevel);

    window.set_title("I'm the main window");
    window.set_default_size(400, 200);
    window.set_position(gtk::WindowPosition::Center);

    window.connect_delete_event(move |_, _| {
        gtk::main_quit();
        Inhibit(false)
    });

    window.show_all();
    window
}

fn generate_new_id(windows: Rc<RefCell<HashMap<usize, gtk::Window>>>) -> usize {
    let mut id = 0;
    // As long as the id is already there, we just continue to increment.
    while windows.borrow().get(&id).is_some() {
        id += 1;
    }
    id
}

fn main() {
    if gtk::init().is_err() {
        println!("Failed to initialize GTK.");
        return;
    }

    let windows: Rc<RefCell<HashMap<usize, gtk::Window>>> = Rc::new(RefCell::new(HashMap::new()));
    let window = create_main_window();

    // Why not changing all sub-windows' title at once?
    let windows_title_entry = gtk::Entry::new();
    windows_title_entry.set_placeholder_text("Update all sub-windows' title");
    // We clone widgets so we can use them both inside and outside the connect_changed closure.
    let windows_clone = windows.clone();
    let windows_title_entry_clone = windows_title_entry.clone();
    windows_title_entry.connect_changed(move |_| {
        // When the entry's text is updated, we update the title of every sub windows.
        let text = windows_title_entry_clone.get_buffer().get_text();
        for window in windows_clone.borrow().values() {
            window.set_title(&text);
        }
    });

    let entry = gtk::Entry::new();
    entry.set_editable(false);
    entry.set_placeholder_text("Events notification will be sent here");

    // I clone it here so we can use it both in the closure and in the layout below.
    let windows_title_entry_clone = windows_title_entry.clone();
    let main_entry = entry.clone();

    // Now let's create a button to create a looooot of new windows!
    let button = gtk::Button::new_with_label("Create new window");
    button.connect_clicked(move |_| {
        let new_id = generate_new_id(windows.clone());
        create_sub_window(&windows_title_entry_clone.get_buffer().get_text(),
                          main_entry.clone(),
                          new_id,
                          windows.clone());
    });


    // Now we add a layout so we can put all widgets into it.
    let layout = gtk::Box::new(gtk::Orientation::Vertical, 5);
    layout.add(&windows_title_entry);
    layout.add(&button);
    layout.add(&entry);
    window.add(&layout);

    window.set_focus(Some(&button));

    // Then we show everything.
    window.show_all();
    gtk::main();
}
