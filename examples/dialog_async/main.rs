use gtk::glib;
use gtk::prelude::*;

fn main() {
    let application = gtk::ApplicationBuilder::new()
        .application_id("com.github.gtk-rs.examples.dialog_async")
        .build();

    application.connect_activate(build_ui);

    application.run();
}

fn build_ui(application: &gtk::Application) {
    let button = gtk::ButtonBuilder::new()
        .label("Open Dialog")
        .halign(gtk::Align::Center)
        .valign(gtk::Align::Center)
        .visible(true)
        .build();

    let window = gtk::ApplicationWindowBuilder::new()
        .application(application)
        .title("Dialog Async")
        .default_width(350)
        .default_height(70)
        .child(&button)
        .visible(true)
        .build();

    button.connect_clicked(glib::clone!(@weak window => move |_| {
            glib::MainContext::default().spawn_local(dialog(window.clone()));
        }
    ));
}

async fn dialog<W: IsA<gtk::Window>>(window: W) {
    let question_dialog = gtk::MessageDialogBuilder::new()
        .transient_for(&window)
        .modal(true)
        .buttons(gtk::ButtonsType::OkCancel)
        .text("What is your answer?")
        .build();

    let answer = question_dialog.run_future().await;
    question_dialog.close();
    question_dialog.hide();

    let info_dialog = gtk::MessageDialogBuilder::new()
        .transient_for(&window)
        .modal(true)
        .buttons(gtk::ButtonsType::Close)
        .text("You answered")
        .secondary_text(&format!("Your answer: {:?}", answer))
        .build();

    info_dialog.run_future().await;
    info_dialog.close();
}
