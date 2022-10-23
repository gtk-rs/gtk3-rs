use gtk::glib;
use gtk::subclass::prelude::*;
use gtk::{prelude::*, CompositeTemplate};

/// The private struct, which can hold widgets and other data.
#[derive(Debug, Default, CompositeTemplate)]
#[template(file = "composite_template.ui")]
pub struct ExampleApplicationWindow {
    // The #[template_child] attribute tells the CompositeTemplate macro
    // that a field is meant to be a child within the template.
    #[template_child]
    pub headerbar: TemplateChild<gtk::HeaderBar>,
    #[template_child]
    pub label: TemplateChild<gtk::Label>,
    // You can specify the optional `id` argument if the id is not the same
    // as the identifier
    #[template_child(id = "subtitle_label")]
    pub subtitle: TemplateChild<gtk::Label>,
}

#[glib::object_subclass]
impl ObjectSubclass for ExampleApplicationWindow {
    const NAME: &'static str = "ExampleApplicationWindow";
    type Type = super::ExampleApplicationWindow;
    type ParentType = gtk::ApplicationWindow;

    // Within class_init() you must set the template.
    // The CompositeTemplate derive macro provides a convenience function
    // bind_template() to set the template and bind all children at once.
    fn class_init(klass: &mut Self::Class) {
        Self::bind_template(klass);
    }

    // You must call `Widget`'s `init_template()` within `instance_init()`.
    fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
        obj.init_template();
    }
}

impl ObjectImpl for ExampleApplicationWindow {
    fn constructed(&self) {
        self.obj().init_label();
        self.parent_constructed();
    }
}

impl WidgetImpl for ExampleApplicationWindow {}
impl ContainerImpl for ExampleApplicationWindow {}
impl BinImpl for ExampleApplicationWindow {}
impl WindowImpl for ExampleApplicationWindow {}
impl ApplicationWindowImpl for ExampleApplicationWindow {}
