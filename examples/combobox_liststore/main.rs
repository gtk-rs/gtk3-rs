use glib::clone;
use gtk::{
    prelude::*, CellRendererText, CellRendererToggle, ComboBox, Entry, ListStore, TreeModel,
};
use std::{cell::RefCell, rc::Rc};

fn build_ui(application: &gtk::Application) {
    let window = gtk::ApplicationWindow::new(application);
    window.set_title("First GTK+ Program");
    window.set_border_width(10);
    window.set_position(gtk::WindowPosition::Center);
    window.set_default_size(350, 70);

    let combobox = gtk::ComboBox::new();
    let renderer = CellRendererToggle::new();
    combobox.pack_start(&renderer, true);
    combobox.add_attribute(&renderer, "active", 0);

    let renderer = CellRendererText::new();
    combobox.pack_start(&renderer, true);
    combobox.add_attribute(&renderer, "text", 1);
    combobox.set_entry_text_column(1);

    let model = gtk::ListStore::new(&[glib::Type::BOOL, glib::Type::STRING]);
    let data: [(u32, &dyn ToValue); 2] = [(0, &true), (1, &"data1")];
    model.insert_with_values(Some(0), &data);
    let data: [(u32, &dyn ToValue); 2] = [(0, &false), (1, &"date2")];
    model.insert_with_values(Some(1), &data);
    let data: [(u32, &dyn ToValue); 2] = [(0, &false), (1, &"date3")];
    model.insert_with_values(Some(2), &data);

    combobox.set_model(Some(&model));
    window.add(&combobox);
    let entry = gtk::Entry::new();
    entry.set_editable(false);
    combobox.add(&entry);

    combobox.connect_changed(clone!(
        #[weak]
        entry,
        move |cb| {
            let model = cb.property::<TreeModel>("model");
            let store = model.dynamic_cast_ref::<ListStore>().unwrap();
            if let Some(iter) = cb.active_iter() {
                let value = store.value(&iter, 0).get::<bool>().unwrap();
                store.set(&iter, &[(0, &!value)]);
            }
            update_entry(&entry, cb);
            cb.set_active_iter(None);
        }
    ));
    combobox.connect_map(move |cb| update_entry(&entry, cb));
    window.show_all();
}

fn for_each<F: Fn(&gtk::ComboBox, &gtk::TreeIter, &ListStore)>(cb: &gtk::ComboBox, f: F) {
    let model = cb.property::<TreeModel>("model");
    if let Some(iter) = model.iter_first() {
        let store = model.dynamic_cast_ref::<ListStore>().unwrap();

        loop {
            f(cb, &iter, store);
            #[cfg(debug_assertions)]
            {
                if !store.iter_is_valid(&iter) {
                    unreachable!("iter is not valid: Don't remove iter in for_each.")
                }
            }

            if !model.iter_next(&iter) {
                break;
            }
        }
    }
}

fn update_entry(entry: &Entry, cb: &ComboBox) {
    entry.set_text("");
    let vec_string: Vec<String> = Default::default();
    let vec_string = Rc::new(RefCell::new(vec_string));
    for_each(
        cb,
        clone!(
            #[weak]
            vec_string,
            move |_, iter, store| {
                if store.value(iter, 0).get::<bool>().unwrap() {
                    let value = store.value(iter, 1).get::<String>().unwrap();
                    vec_string.borrow_mut().push(value);
                }
            }
        ),
    );
    entry.set_text(vec_string.borrow().join(", ").as_str());
}

fn main() {
    let application = gtk::Application::new(
        Some("com.github.gtk-rs.examples.combobox_liststore"),
        Default::default(),
    );

    application.connect_activate(build_ui);

    application.run();
}
