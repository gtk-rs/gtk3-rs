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

use gtk::{gio, glib::clone, ResponseType};

use std::env::args;

use row_data::RowData;

mod model {
    use super::*;
    use gio::subclass::ObjectSubclass;
    use row_data::RowData;
    mod imp {
        use super::*;
        use gio::subclass::ListModelImpl;
        use glib::subclass;
        use glib::subclass::prelude::*;
        use std::cell::RefCell;
        #[derive(Debug)]
        pub struct Model(pub RefCell<Vec<RowData>>);

        // Basic declaration of our type for the GObject type system
        impl ObjectSubclass for Model {
            const NAME: &'static str = "Model";
            type Type = super::Model;
            type ParentType = glib::Object;
            type Interfaces = (gio::ListModel,);
            type Instance = subclass::simple::InstanceStruct<Self>;
            type Class = subclass::simple::ClassStruct<Self>;

            glib::object_subclass!();

            // Called once at the very beginning of instantiation
            fn new() -> Self {
                Self(RefCell::new(Vec::new()))
            }
        }

        impl ObjectImpl for Model {}

        impl ListModelImpl for Model {
            fn get_item_type(&self, _list_model: &Self::Type) -> glib::Type {
                RowData::static_type()
            }
            fn get_n_items(&self, _list_model: &Self::Type) -> u32 {
                self.0.borrow().len() as u32
            }
            fn get_item(&self, _list_model: &Self::Type, position: u32) -> Option<glib::Object> {
                self.0
                    .borrow()
                    .get(position as usize)
                    .map(|o| o.clone().upcast::<glib::Object>())
            }
        }
    }

    // Public part of the Model type.
    glib::wrapper! {
        pub struct Model(ObjectSubclass<imp::Model>) @implements gio::ListModel;
    }

    // Constructor for new instances. This simply calls glib::Object::new()
    impl Model {
        #[allow(clippy::new_without_default)]
        pub fn new() -> Model {
            glib::Object::new(&[]).expect("Failed to create Model")
        }

        pub fn append(&self, obj: &RowData) {
            let self_ = imp::Model::from_instance(self);
            let index = {
                // Borrow the data only once and ensure the borrow guard is dropped
                // before we emit the items_changed signal because the view
                // could call get_item / get_n_item from the signal handler to update its state
                let mut data = self_.0.borrow_mut();
                data.push(obj.clone());
                data.len() - 1
            };
            // Emits a signal that 1 item was added, 0 removed at the position index
            self.items_changed(index as u32, 0, 1);
        }

        pub fn remove(&self, index: u32) {
            let self_ = imp::Model::from_instance(self);
            self_.0.borrow_mut().remove(index as usize);
            // Emits a signal that 1 item was removed, 0 added at the position index
            self.items_changed(index, 1, 0);
        }
    }
}

fn build_ui(application: &gtk::Application) {
    let window = gtk::ApplicationWindow::new(application);

    window.set_title("ListBox Model Sample");
    window.set_border_width(10);
    window.set_position(gtk::WindowPosition::Center);
    window.set_default_size(320, 480);

    let vbox = gtk::Box::new(gtk::Orientation::Vertical, 5);

    // Create our list store and specify that the type stored in the
    // list should be the RowData GObject we define at the bottom
    let model = model::Model::new();

    // And then create the UI part, the listbox and bind the list store
    // model to it. Whenever the UI needs to show a new row, e.g. because
    // it was notified that the model changed, it will call the callback
    // with the corresponding item from the model and will ask for a new
    // gtk::ListBoxRow that should be displayed.
    //
    // The gtk::ListBoxRow can contain any possible widgets.
    let listbox = gtk::ListBox::new();
    listbox.bind_model(Some(&model),
        clone!(@weak window => @default-panic, move |item| {
            let box_ = gtk::ListBoxRow::new();
            let item = item.downcast_ref::<RowData>().expect("Row data is of wrong type");

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

            let spin_button = gtk::SpinButton::with_range(0.0, 100.0, 1.0);
            item.bind_property("count", &spin_button, "value")
                .flags(glib::BindingFlags::DEFAULT | glib::BindingFlags::SYNC_CREATE | glib::BindingFlags::BIDIRECTIONAL)
                .build();
        hbox.pack_start(&spin_button, false, false, 0);

        // When the edit button is clicked, a new modal dialog is created for editing
        // the corresponding row
        let edit_button = gtk::Button::with_label("Edit");
        edit_button.connect_clicked(clone!(@weak window, @strong item => move |_| {
            let dialog = gtk::Dialog::with_buttons(Some("Edit Item"), Some(&window), gtk::DialogFlags::MODAL,
                &[("Close", ResponseType::Close)]);
            dialog.set_default_response(ResponseType::Close);
            dialog.connect_response(|dialog, _| dialog.close());

            let content_area = dialog.get_content_area();

            // Similarly to the label and spin button inside the listbox, the text entry
            // and spin button in the edit dialog are connected via property bindings to
            // the item. Any changes will be immediately reflected inside the item and
            // by the listbox
            let entry = gtk::Entry::new();
            item.bind_property("name", &entry, "text")
                .flags(glib::BindingFlags::DEFAULT | glib::BindingFlags::SYNC_CREATE | glib::BindingFlags::BIDIRECTIONAL)
                .build();

            // Activating the entry (enter) will send response `ResponseType::Close` to the dialog
            entry.connect_activate(clone!(@weak dialog => move |_| {
                dialog.response(ResponseType::Close);
            }));
            content_area.add(&entry);

            let spin_button = gtk::SpinButton::with_range(0.0, 100.0, 1.0);
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
        box_.connect_activate(clone!(@weak edit_button => move |_| {
            edit_button.emit_clicked();
        }));

        box_.show_all();

        box_.upcast::<gtk::Widget>()
    }));

    let scrolled_window = gtk::ScrolledWindow::new(gtk::NONE_ADJUSTMENT, gtk::NONE_ADJUSTMENT);
    scrolled_window.add(&listbox);

    let hbox = gtk::Box::new(gtk::Orientation::Horizontal, 5);

    // The add button opens a new dialog which is basically the same as the edit
    // dialog, except that we don't have a corresponding item yet at that point
    // and only create it once the Ok button in the dialog is clicked, and only
    // then add it to the model. Once added to the model, it will immediately
    // appear in the listbox UI
    let add_button = gtk::Button::with_label("Add");
    add_button.connect_clicked(clone!(@weak window, @weak model => move |_| {
            let dialog = gtk::Dialog::with_buttons(Some("Add Item"), Some(&window), gtk::DialogFlags::MODAL,
                &[("Ok", ResponseType::Ok), ("Cancel", ResponseType::Cancel)]);
            dialog.set_default_response(ResponseType::Ok);

            let content_area = dialog.get_content_area();

            let entry = gtk::Entry::new();
            entry.connect_activate(clone!(@weak dialog => move |_| {
                dialog.response(ResponseType::Ok);
            }));
            content_area.add(&entry);

            let spin_button = gtk::SpinButton::with_range(0.0, 100.0, 1.0);
            content_area.add(&spin_button);

            dialog.connect_response(clone!(@weak model, @weak entry, @weak spin_button => move |dialog, resp| {
                let text = entry.get_text();
                if !text.is_empty() && resp == ResponseType::Ok {
                    model.append(&RowData::new(&text, spin_button.get_value() as u32));
                }
                dialog.close();
            }));

            dialog.show_all();
    }));

    hbox.add(&add_button);

    // Via the delete button we delete the item from the model that
    // is at the index of the selected row. Also deleting from the
    // model is immediately reflected in the listbox.
    let delete_button = gtk::Button::with_label("Delete");
    delete_button.connect_clicked(clone!(@weak model, @weak listbox => move |_| {
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
        model.append(&RowData::new(&format!("Name {}", i), i * 10));
    }

    window.show_all();
}

fn main() {
    let application = gtk::Application::new(
        Some("com.github.gtk-rs.examples.listbox-model"),
        Default::default(),
    )
    .expect("Initialization failed...");

    application.connect_activate(|app| {
        build_ui(app);
    });

    application.run(&args().collect::<Vec<_>>());
}

// Our GObject subclass for carrying a name and count for the ListBox model
//
// Both name and count are stored in a RefCell to allow for interior mutability
// and are exposed via normal GObject properties. This allows us to use property
// bindings below to bind the values with what widgets display in the UI
mod row_data {
    use super::*;

    use glib::subclass;
    use glib::subclass::prelude::*;

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

        // Basic declaration of our type for the GObject type system
        impl ObjectSubclass for RowData {
            const NAME: &'static str = "RowData";
            type Type = super::RowData;
            type ParentType = glib::Object;
            type Interfaces = ();
            type Instance = subclass::simple::InstanceStruct<Self>;
            type Class = subclass::simple::ClassStruct<Self>;

            glib::object_subclass!();

            // Called once at the very beginning of instantiation of each instance and
            // creates the data structure that contains all our state
            fn new() -> Self {
                Self {
                    name: RefCell::new(None),
                    count: RefCell::new(0),
                }
            }
        }

        // The ObjectImpl trait provides the setters/getters for GObject properties.
        // Here we need to provide the values that are internally stored back to the
        // caller, or store whatever new value the caller is providing.
        //
        // This maps between the GObject properties and our internal storage of the
        // corresponding values of the properties.
        impl ObjectImpl for RowData {
            fn properties() -> &'static [glib::ParamSpec] {
                use once_cell::sync::Lazy;
                static PROPERTIES: Lazy<Vec<glib::ParamSpec>> = Lazy::new(|| {
                    vec![
                        glib::ParamSpec::string(
                            "name",
                            "Name",
                            "Name",
                            None, // Default value
                            glib::ParamFlags::READWRITE,
                        ),
                        glib::ParamSpec::uint(
                            "count",
                            "Count",
                            "Count",
                            0,
                            100,
                            0, // Allowed range and default value
                            glib::ParamFlags::READWRITE,
                        ),
                    ]
                });

                PROPERTIES.as_ref()
            }

            fn set_property(
                &self,
                _obj: &Self::Type,
                _id: usize,
                value: &glib::Value,
                pspec: &glib::ParamSpec,
            ) {
                match pspec.get_name() {
                    "name" => {
                        let name = value
                            .get()
                            .expect("type conformity checked by `Object::set_property`");
                        self.name.replace(name);
                    }
                    "count" => {
                        let count = value
                            .get_some()
                            .expect("type conformity checked by `Object::set_property`");
                        self.count.replace(count);
                    }
                    _ => unimplemented!(),
                }
            }

            fn get_property(
                &self,
                _obj: &Self::Type,
                _id: usize,
                pspec: &glib::ParamSpec,
            ) -> glib::Value {
                match pspec.get_name() {
                    "name" => self.name.borrow().to_value(),
                    "count" => self.count.borrow().to_value(),
                    _ => unimplemented!(),
                }
            }
        }
    }

    // Public part of the RowData type. This behaves like a normal gtk-rs-style GObject
    // binding
    glib::wrapper! {
        pub struct RowData(ObjectSubclass<imp::RowData>);
    }

    // Constructor for new instances. This simply calls glib::Object::new() with
    // initial values for our two properties and then returns the new instance
    impl RowData {
        pub fn new(name: &str, count: u32) -> RowData {
            glib::Object::new(&[("name", &name), ("count", &count)])
                .expect("Failed to create row data")
        }
    }
}
