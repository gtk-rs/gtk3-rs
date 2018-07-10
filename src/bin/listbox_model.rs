//! # ListBox and ListModel Sample
//!
//! This sample demonstrates how to use gtk::Listbox in combination with
//! gio::ListStore as a model with a custom row type.

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

    window.connect_delete_event(clone!(window => move |_, _| {
        window.destroy();
        Inhibit(false)
    }));

    let vbox = gtk::Box::new(gtk::Orientation::Vertical, 5);

    let model = gio::ListStore::new(RowData::static_type());
    let listbox = gtk::ListBox::new();
    listbox.bind_model(&model, clone!(window => move |item| {
        let box_ = gtk::ListBoxRow::new();
        let item = item.clone().downcast::<RowData>().unwrap();

        let hbox = gtk::Box::new(gtk::Orientation::Horizontal, 5);

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

        let edit_button = gtk::Button::new_with_label("Edit");
        edit_button.connect_clicked(clone!(window, item => move |_| {
            let dialog = gtk::Dialog::new_with_buttons(Some("Edit Item"), Some(&window), gtk::DialogFlags::MODAL,
                &[("Close", 0)]);
            dialog.set_default_response(0);
            dialog.connect_response(|dialog, _| dialog.destroy());

            let content_area = dialog.get_content_area();

            let entry = gtk::Entry::new();
            item.bind_property("name", &entry, "text")
                .flags(glib::BindingFlags::DEFAULT | glib::BindingFlags::SYNC_CREATE | glib::BindingFlags::BIDIRECTIONAL)
                .build();
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

        box_.connect_activate(clone!(edit_button => move |_| {
            edit_button.emit_clicked();
        }));

        box_.show_all();

        box_
    }));

    let scrolled_window = gtk::ScrolledWindow::new(None, None);
    scrolled_window.add(&listbox);

    let hbox = gtk::Box::new(gtk::Orientation::Horizontal, 5);
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
mod row_data {
    use super::*;

    use gobject_subclass::object::*;

    use glib::translate::*;

    use std::ptr;
    use std::mem;

    mod imp {
        use super::*;
        use std::cell::RefCell;

        pub struct RowData {
            name: RefCell<Option<String>>,
            count: RefCell<u32>,
        }

        static PROPERTIES: [Property; 2] = [
            Property::String(
                "name",
                "Name",
                "Name",
                None,
                PropertyMutability::ReadWrite,
            ),
            Property::UInt(
                "count",
                "Count",
                "Count",
                (0, 100), 0,
                PropertyMutability::ReadWrite,
            ),
        ];

        impl RowData {
            pub fn get_type() -> glib::Type {
                use std::sync::{Once, ONCE_INIT};

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

            fn class_init(klass: &mut ObjectClass) {
                klass.install_properties(&PROPERTIES);
            }

            fn init(_obj: &Object) -> Box<ObjectImpl<Object>> {
                let imp = Self {
                    name: RefCell::new(None),
                    count: RefCell::new(0),
                };
                Box::new(imp)
            }
        }

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

    glib_wrapper! {
        pub struct RowData(Object<imp::RowData>):
            [Object => InstanceStruct<Object>];

        match fn {
            get_type => || imp::RowData::get_type().to_glib(),
        }
    }

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
