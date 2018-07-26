//! # ListBox and ListModel Sample
//!
//! This sample demonstrates how to use gtk::ListBox in combination with
//! gio::ListStore as a model with a custom row type.
//!
//! It sets up a gtk::ListBox containing, per row, a label, spinbutton and
//! an edit button. The edit button allows to edit the underlying data structure
//! and changes are taking place immediately in the listbox by making use of GObject
//! property bindings.
//!
//! In addition it is possible to add new rows and delete old ones.

#[macro_use]
extern crate glib;
extern crate gio;
extern crate gtk;

extern crate glib_sys as glib_ffi;
extern crate gobject_sys as gobject_ffi;

extern crate gobject_subclass;

use gio::prelude::*;
use gtk::prelude::*;

use std::env::args;

use row_data::RowData;

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
    let window = gtk::ApplicationWindow::new(application);

    window.set_title("ListBox Model Sample");
    window.set_border_width(10);
    window.set_position(gtk::WindowPosition::Center);
    window.set_default_size(320, 480);

    window.connect_delete_event(move |win, _| {
        win.destroy();
        Inhibit(false)
    });

    let vbox = gtk::Box::new(gtk::Orientation::Vertical, 5);

    // Create our list store and specify that the type stored in the
    // list should be the RowData GObject we define at the bottom
    let model = gio::ListStore::new(RowData::static_type());

    // And then create the UI part, the listbox and bind the list store
    // model to it. Whenever the UI needs to show a new row, e.g. because
    // it was notified that the model changed, it will call the callback
    // with the corresponding item from the model and will ask for a new
    // gtk::ListBoxRow that should be displayed.
    //
    // The gtk::ListBoxRow can contain any possible widgets.
    let listbox = gtk::ListBox::new();
    listbox.bind_model(&model, clone!(window => move |item| {
        let box_ = gtk::ListBoxRow::new();
        let item = item.downcast_ref::<RowData>().unwrap();

        let hbox = gtk::Box::new(gtk::Orientation::Horizontal, 5);

        // Create the label and spin button that shows the two values
        // of the item. We bind the properties for the two values to the
        // corresponding properties of the widgets so that they are automatically
        // updated whenever the item is changing. By specifying SYNC_CREATE the
        // widget will automatically get the initial value of the item set.
        //
        // In case of the spin button the binding is bidirectional, that is any
        // change of value in the spin button will be automatically reflected in
        // the item.
        let label = gtk::Label::new(None);
        item.bind_property("name", &label, "label")
            .flags(glib::BindingFlags::DEFAULT | glib::BindingFlags::SYNC_CREATE)
            .build();
        hbox.pack_start(&label, true, true, 0);

        let spin_button = gtk::SpinButton::new_with_range(0.0, 100.0, 1.0);
        item.bind_property("count", &spin_button, "value")
            .flags(glib::BindingFlags::DEFAULT | glib::BindingFlags::SYNC_CREATE | glib::BindingFlags::BIDIRECTIONAL)
            .build();
        hbox.pack_start(&spin_button, false, false, 0);

        // When the edit button is clicked, a new modal dialog is created for editing
        // the corresponding row
        let edit_button = gtk::Button::new_with_label("Edit");
        edit_button.connect_clicked(clone!(window, item => move |_| {
            let dialog = gtk::Dialog::new_with_buttons(Some("Edit Item"), Some(&window), gtk::DialogFlags::MODAL,
                &[("Close", 0)]);
            dialog.set_default_response(0);
            dialog.connect_response(|dialog, _| dialog.destroy());

            let content_area = dialog.get_content_area();

            // Similarly to the label and spin button inside the listbox, the text entry
            // and spin button in the edit dialog are connected via property bindings to
            // the item. Any changes will be immediately reflected inside the item and
            // by the listbox
            let entry = gtk::Entry::new();
            item.bind_property("name", &entry, "text")
                .flags(glib::BindingFlags::DEFAULT | glib::BindingFlags::SYNC_CREATE | glib::BindingFlags::BIDIRECTIONAL)
                .build();

            // Activating the entry (enter) will send response 0 to the dialog, which
            // is the response code we used for the Close button. It will close the dialog
            entry.connect_activate(clone!(dialog => move |_| {
                dialog.response(0);
            }));
            content_area.add(&entry);

            let spin_button = gtk::SpinButton::new_with_range(0.0, 100.0, 1.0);
            item.bind_property("count", &spin_button, "value")
                .flags(glib::BindingFlags::DEFAULT | glib::BindingFlags::SYNC_CREATE | glib::BindingFlags::BIDIRECTIONAL)
                .build();
            content_area.add(&spin_button);

            dialog.show_all();
        }));
        hbox.pack_start(&edit_button, false, false, 0);

        box_.add(&hbox);

        // When a row is activated (select + enter) we simply emit the clicked
        // signal on the corresponding edit button to open the edit dialog
        box_.connect_activate(clone!(edit_button => move |_| {
            edit_button.emit_clicked();
        }));

        box_.show_all();

        box_
    }));

    let scrolled_window = gtk::ScrolledWindow::new(None, None);
    scrolled_window.add(&listbox);

    let hbox = gtk::Box::new(gtk::Orientation::Horizontal, 5);

    // The add button opens a new dialog which is basically the same as the edit
    // dialog, except that we don't have a corresponding item yet at that point
    // and only create it once the Ok button in the dialog is clicked, and only
    // then add it to the model. Once added to the model, it will immediately
    // appear in the listbox UI
    let add_button = gtk::Button::new_with_label("Add");
    add_button.connect_clicked(clone!(window, model => move |_| {
            let dialog = gtk::Dialog::new_with_buttons(Some("Add Item"), Some(&window), gtk::DialogFlags::MODAL,
                &[("Ok", 0), ("Cancel", 1)]);
            dialog.set_default_response(0);

            let content_area = dialog.get_content_area();

            let entry = gtk::Entry::new();
            entry.connect_activate(clone!(dialog => move |_| {
                dialog.response(0);
            }));
            content_area.add(&entry);

            let spin_button = gtk::SpinButton::new_with_range(0.0, 100.0, 1.0);
            content_area.add(&spin_button);

            dialog.connect_response(clone!(model, entry, spin_button => move |dialog, resp| {
                if let Some(text) = entry.get_text() {
                    if !text.is_empty() && resp == 0 {
                        model.append(&RowData::new(&text, spin_button.get_value() as u32));
                    }
                }
                dialog.destroy();
            }));

            dialog.show_all();
    }));

    hbox.add(&add_button);

    // Via the delete button we delete the item from the model that
    // is at the index of the selected row. Also deleting from the
    // model is immediately reflected in the listbox.
    let delete_button = gtk::Button::new_with_label("Delete");
    delete_button.connect_clicked(clone!(model, listbox => move |_| {
        let selected = listbox.get_selected_row();

        if let Some(selected) = selected {
            let idx = selected.get_index();
            model.remove(idx as u32);
        }
    }));
    hbox.add(&delete_button);

    vbox.pack_start(&hbox, false, false, 0);
    vbox.pack_start(&scrolled_window, true, true, 0);

    window.add(&vbox);

    for i in 0..10 {
        model.append(&RowData::new(&format!("Name {}", i), i*10));
    }

    window.show_all();
}

fn main() {
    let application = gtk::Application::new("com.github.gtk-rs.examples.listbox-model",
                                            gio::ApplicationFlags::empty())
        .expect("Initialization failed...");

    application.connect_startup(|app| {
        build_ui(app);
    });
    application.connect_activate(|_| {});

    application.run(&args().collect::<Vec<_>>());
}

// Our GObject subclass for carrying a name and count for the ListBox model
//
// Both name and count are stored in a RefCell to allow for interior mutability
// and are exposed via normal GObject properties. This allows us to use property
// bindings below to bind the values with what widgets display in the UI
mod row_data {
    use super::*;

    use gobject_subclass::object::*;

    use glib::translate::*;

    use std::ptr;
    use std::mem;

    // Implementation sub-module of the GObject
    mod imp {
        use super::*;
        use std::cell::RefCell;

        // The actual data structure that stores our values. This is not accessible
        // directly from the outside.
        pub struct RowData {
            name: RefCell<Option<String>>,
            count: RefCell<u32>,
        }

        // GObject property definitions for our two values
        static PROPERTIES: [Property; 2] = [
            Property::String(
                "name",
                "Name",
                "Name",
                None, // Default value
                PropertyMutability::ReadWrite,
            ),
            Property::UInt(
                "count",
                "Count",
                "Count",
                (0, 100), 0, // Allowed range and default value
                PropertyMutability::ReadWrite,
            ),
        ];

        impl RowData {
            // glib::Type registration of the RowData type. The very first time
            // this registers the type with GObject and afterwards only returns
            // the type id that was registered the first time
            pub fn get_type() -> glib::Type {
                use std::sync::{Once, ONCE_INIT};

                // unsafe code here because static mut variables are inherently
                // unsafe. Via std::sync::Once we guarantee here that the variable
                // is only ever set once, and from that point onwards is only ever
                // read, which makes its usage safe.
                static ONCE: Once = ONCE_INIT;
                static mut TYPE: glib::Type = glib::Type::Invalid;

                ONCE.call_once(|| {
                    let t = register_type(RowDataStatic);
                    unsafe {
                        TYPE = t;
                    }
                });

                unsafe { TYPE }
            }

            // Called exactly once before the first instantiation of an instance. This
            // sets up any type-specific things, in this specific case it installs the
            // properties so that GObject knows about their existence and they can be
            // used on instances of our type
            fn class_init(klass: &mut ObjectClass) {
                klass.install_properties(&PROPERTIES);
            }

            // Called once at the very beginning of instantiation of each instance and
            // creates the data structure that contains all our state
            fn init(_obj: &Object) -> Box<ObjectImpl<Object>> {
                let imp = Self {
                    name: RefCell::new(None),
                    count: RefCell::new(0),
                };
                Box::new(imp)
            }
        }

        // The ObjectImpl trait provides the setters/getters for GObject properties.
        // Here we need to provide the values that are internally stored back to the
        // caller, or store whatever new value the caller is providing.
        //
        // This maps between the GObject properties and our internal storage of the
        // corresponding values of the properties.
        impl ObjectImpl<Object> for RowData {
            fn set_property(&self, _obj: &glib::Object, id: u32, value: &glib::Value) {
                let prop = &PROPERTIES[id as usize];

                match *prop {
                    Property::String("name", ..) => {
                        let name = value.get();
                        self.name.replace(name.clone());
                    }
                    Property::UInt("count", ..) => {
                        let count = value.get().unwrap();
                        self.count.replace(count);
                    }
                    _ => unimplemented!(),
                }
            }

            fn get_property(&self, _obj: &glib::Object, id: u32) -> Result<glib::Value, ()> {
                let prop = &PROPERTIES[id as usize];

                match *prop {
                    Property::String("name", ..) => Ok(self.name.borrow().clone().to_value()),
                    Property::UInt("count", ..) => Ok(self.count.borrow().clone().to_value()),
                    _ => unimplemented!(),
                }
            }
        }

        // Static, per-type data that is used for actually registering the type
        // and providing the name of our type and how to initialize it to GObject
        //
        // It is used above in the get_type() function for passing that information
        // to GObject
        struct RowDataStatic;

        impl ImplTypeStatic<Object> for RowDataStatic {
            fn get_name(&self) -> &str {
                "RowData"
            }

            fn new(&self, obj: &Object) -> Box<ObjectImpl<Object>> {
                RowData::init(obj)
            }

            fn class_init(&self, klass: &mut ObjectClass) {
                RowData::class_init(klass);
            }
        }
    }

    // Public part of the RowData type. This behaves like a normal gtk-rs-style GObject
    // binding
    glib_wrapper! {
        pub struct RowData(Object<imp::RowData>):
            [Object => InstanceStruct<Object>];

        match fn {
            get_type => || imp::RowData::get_type().to_glib(),
        }
    }

    // Constructor for new instances. This simply calls glib::Object::new() with
    // initial values for our two properties and then returns the new instance
    impl RowData {
        pub fn new(name: &str, count: u32) -> RowData {
            use glib::object::Downcast;

            unsafe {
                glib::Object::new(
                    Self::static_type(),
                    &[("name", &name),
                      ("count", &count),
                    ])
                    .unwrap()
                    .downcast_unchecked()
            }
        }
    }
}
