mod model;
pub mod row_data;

use gtk::{
    glib::{self, clone},
    prelude::*,
    ResponseType,
};
use row_data::RowData;
use std::env::args;

fn build_ui(application: &gtk::Application) {
    let window = gtk::ApplicationWindow::new(application);

    window.set_title("ListBox Model example");
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

    application.connect_activate(build_ui);

    application.run();
}
