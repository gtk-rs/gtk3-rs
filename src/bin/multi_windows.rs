extern crate glib;
extern crate gio;
extern crate gtk;

use gio::prelude::*;
use gtk::prelude::*;

use std::cell::RefCell;
use std::collections::HashMap;
use std::env::args;
use std::rc::Rc;

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

// upgrade weak reference or return
#[macro_export]
macro_rules! upgrade_weak {
    ($x:ident, $r:expr) => {{
        match $x.upgrade() {
            Some(o) => o,
            None => return $r,
        }
    }};
    ($x:ident) => {
        upgrade_weak!($x, ())
    };
}

fn create_sub_window(application: &gtk::Application, title: &str, main_window_entry: &gtk::Entry, id: usize,
                     windows: &Rc<RefCell<HashMap<usize, glib::WeakRef<gtk::Window>>>>) {
    let window = gtk::Window::new(gtk::WindowType::Toplevel);

    application.add_window(&window);

    window.set_title(title);
    window.set_default_size(400, 200);

    window.connect_delete_event(clone!(windows => move |_, _| {
        windows.borrow_mut().remove(&id);
        Inhibit(false)
    }));

    let button = gtk::Button::new_with_label(&format!("Notify main window with id {}!", id));
    button.connect_clicked(clone!(main_window_entry => move |_| {
        // When the button is clicked, let's write it on the main window's entry!
        main_window_entry.get_buffer().set_text(&format!("sub window {} clicked", id));
    }));
    window.add(&button);

    window.show_all();
    // Once the new window has been created, we put it into our hashmap so we can update its
    // title when needed.
    windows.borrow_mut().insert(id, window.downgrade());
}

fn create_main_window(application: &gtk::Application) -> gtk::ApplicationWindow {
    let window = gtk::ApplicationWindow::new(application);

    window.set_title("I'm the main window");
    window.set_default_size(400, 200);
    window.set_position(gtk::WindowPosition::Center);

    window.connect_delete_event(|win, _| {
        win.destroy();
        Inhibit(false)
    });

    window.show_all();
    window
}

fn generate_new_id(windows: &HashMap<usize, glib::WeakRef<gtk::Window>>) -> usize {
    let mut id = 0;
    // As long as the id is already there, we just continue to increment.
    while windows.get(&id).is_some() {
        id += 1;
    }
    id
}

fn build_ui(application: &gtk::Application) {
    let windows: Rc<RefCell<HashMap<usize, glib::WeakRef<gtk::Window>>>> = Rc::new(RefCell::new(HashMap::new()));
    let window = create_main_window(application);

    // Why not changing all sub-windows' title at once?
    let windows_title_entry = gtk::Entry::new();
    windows_title_entry.set_placeholder_text("Update all sub-windows' title");
    windows_title_entry.connect_changed(clone!(windows => move |windows_title_entry| {
        // When the entry's text is updated, we update the title of every sub windows.
        let text = windows_title_entry.get_buffer().get_text();
        for window in windows.borrow().values() {
            window.upgrade().map(|w| w.set_title(&text));
        }
    }));

    let entry = gtk::Entry::new();
    entry.set_editable(false);
    entry.set_placeholder_text("Events notification will be sent here");

    // Now let's create a button to create a looooot of new windows!
    let button = gtk::Button::new_with_label("Create new window");
    let application_weak = application.downgrade();
    button.connect_clicked(clone!(windows_title_entry, entry => move |_| {
        let application = upgrade_weak!(application_weak);
        let new_id = generate_new_id(&windows.borrow());
        create_sub_window(&application,
                          &windows_title_entry.get_buffer().get_text(),
                          &entry,
                          new_id,
                          &windows);
    }));

    // Now we add a layout so we can put all widgets into it.
    let layout = gtk::Box::new(gtk::Orientation::Vertical, 5);
    layout.add(&windows_title_entry);
    layout.add(&button);
    layout.add(&entry);
    window.add(&layout);

    window.set_focus(Some(&button));

    // Then we show everything.
    window.show_all();
}

fn main() {
    let application = gtk::Application::new("com.github.gtk-rs.examples.multi_windows",
                                            Default::default())
                                       .expect("Initialization failed...");

    application.connect_activate(|app| {
        build_ui(app);
    });

    application.run(&args().collect::<Vec<_>>());
}
