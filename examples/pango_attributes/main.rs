use gtk::pango;
use gtk::prelude::*;

fn build_ui(application: &gtk::Application) {
    let window = gtk::ApplicationWindow::new(application);

    window.set_title("Pango text attributes");
    window.set_border_width(10);
    window.set_position(gtk::WindowPosition::Center);
    window.set_default_size(350, 70);

    let label = gtk::Label::new(Some("Some text"));
    let attr_list = pango::AttrList::new();

    let mut attr = pango::Attribute::new_background(65535, 0, 0);
    attr.set_start_index(0);
    attr.set_end_index(2);
    attr_list.insert(attr);

    let mut attr = pango::Attribute::new_underline(pango::Underline::Single);
    attr.set_start_index(1);
    attr.set_end_index(4);
    attr_list.insert(attr);

    let mut attr = pango::Attribute::new_strikethrough(true);
    attr.set_start_index(5);
    attr_list.insert(attr);

    let mut attr = pango::Attribute::new_scale(1.2);
    attr.set_start_index(6);
    attr_list.insert(attr);

    label.set_attributes(Some(&attr_list));
    window.add(&label);

    window.show_all();
}

fn main() {
    let application = gtk::Application::new(
        Some("com.github.gtk-rs.examples.pango_attributes"),
        Default::default(),
    )
    .expect("Initialization failed...");

    application.connect_activate(build_ui);

    application.run();
}
