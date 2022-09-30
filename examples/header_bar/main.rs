mod title_bar;

use gtk::prelude::*;
use gtk::{ApplicationWindow, CheckButton, Label, Stack, WindowPosition};
use title_bar::TitleBar;

fn build_ui(application: &gtk::Application) {
    let window = ApplicationWindow::builder()
        .application(application)
        .title("HeaderBar example")
        .window_position(WindowPosition::Center)
        .build();

    let stack = Stack::builder().expand(true).build();
    window.add(&stack);

    let check_button = CheckButton::builder()
        .expand(true)
        .label("check button")
        .build();
    stack.add_titled(&check_button, "check_button", "check button");
    let label = Label::builder().label("Hello world").build();
    stack.add_titled(&label, "label", "label");

    let title_bar = TitleBar::new();
    window.set_titlebar(Some(title_bar.header()));
    title_bar.set_stack(&stack);

    window.show_all();
}

fn main() {
    let application = gtk::Application::new(
        Some("com.github.gtk-rs.examples.menu_bar"),
        Default::default(),
    );

    application.connect_activate(build_ui);

    application.run();
}
