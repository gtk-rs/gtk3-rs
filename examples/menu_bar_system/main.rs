use gtk::prelude::*;
use gtk::AboutDialog;
use gtk::{gio, glib};

fn build_system_menu(application: &gtk::Application) {
    let menu = gio::Menu::new();
    let menu_bar = gio::Menu::new();
    let more_menu = gio::Menu::new();
    let switch_menu = gio::Menu::new();
    let settings_menu = gio::Menu::new();
    let submenu = gio::Menu::new();

    // The first argument is the label of the menu item whereas the second is the action name. It'll
    // makes more sense when you'll be reading the "add_actions" function.
    menu.append(Some("Quit"), Some("app.quit"));

    switch_menu.append(Some("Switch"), Some("app.switch"));
    menu_bar.append_submenu(Some("_Switch"), &switch_menu);

    settings_menu.append(Some("Sub another"), Some("app.sub_another"));
    submenu.append(Some("Sub sub another"), Some("app.sub_sub_another"));
    submenu.append(Some("Sub sub another2"), Some("app.sub_sub_another2"));
    settings_menu.append_submenu(Some("Sub menu"), &submenu);
    menu_bar.append_submenu(Some("_Another"), &settings_menu);

    more_menu.append(Some("About"), Some("app.about"));
    menu_bar.append_submenu(Some("?"), &more_menu);

    application.set_app_menu(Some(&menu));
    application.set_menubar(Some(&menu_bar));
}

/// This function creates "actions" which connect on the declared actions from the menu items.
fn add_actions(
    application: &gtk::Application,
    switch: &gtk::Switch,
    label: &gtk::Label,
    window: &gtk::ApplicationWindow,
) {
    // Thanks to this method, we can say that this item is actually a checkbox.
    let switch_action = gio::SimpleAction::new_stateful("switch", None, &false.to_variant());
    switch_action.connect_activate(glib::clone!(@weak switch => move |g, _| {
        let mut is_active = false;
        if let Some(g) = g.get_state() {
            is_active = g.get().expect("couldn't get bool");
            // We update the state of the toggle.
            switch.set_active(!is_active);
        }
        // We need to change the toggle state ourselves. `gio` dark magic.
        g.change_state(&(!is_active).to_variant());
    }));

    // The same goes the around way: if we update the switch state, we need to update the menu
    // item's state.
    switch.connect_property_active_notify(glib::clone!(@weak switch_action => move |s| {
        switch_action.change_state(&s.get_active().to_variant());
    }));

    let sub_another = gio::SimpleAction::new("sub_another", None);
    sub_another.connect_activate(glib::clone!(@weak label => move |_, _| {
        label.set_text("sub another menu item clicked");
    }));
    let sub_sub_another = gio::SimpleAction::new("sub_sub_another", None);
    sub_sub_another.connect_activate(glib::clone!(@weak label => move |_, _| {
        label.set_text("sub sub another menu item clicked");
    }));
    let sub_sub_another2 = gio::SimpleAction::new("sub_sub_another2", None);
    sub_sub_another2.connect_activate(glib::clone!(@weak label => move |_, _| {
        label.set_text("sub sub another2 menu item clicked");
    }));

    let quit = gio::SimpleAction::new("quit", None);
    quit.connect_activate(glib::clone!(@weak window => move |_, _| {
        window.close();
    }));

    let about = gio::SimpleAction::new("about", None);
    about.connect_activate(glib::clone!(@weak window => move |_, _| {
        let p = AboutDialog::new();
        p.set_website_label(Some("gtk-rs"));
        p.set_website(Some("http://gtk-rs.org"));
        p.set_authors(&["Gtk-rs developers"]);
        p.set_title("About!");
        p.set_transient_for(Some(&window));
        p.show_all();
    }));

    // We need to add all the actions to the application so they can be taken into account.
    application.add_action(&about);
    application.add_action(&quit);
    application.add_action(&sub_another);
    application.add_action(&sub_sub_another);
    application.add_action(&sub_sub_another2);
    application.add_action(&switch_action);
}

fn add_accelerators(application: &gtk::Application) {
    application.set_accels_for_action("app.about", &["F1"]);
    // `Primary` is a platform-agnostic accelerator modifier.
    // On Windows and Linux, `Primary` maps to the `Ctrl` key,
    // and on macOS it maps to the `command` key.
    application.set_accels_for_action("app.quit", &["<Primary>Q"]);
}

fn build_ui(application: &gtk::Application) {
    let window = gtk::ApplicationWindow::new(application);

    window.set_title("System menu bar");
    window.set_border_width(10);
    window.set_position(gtk::WindowPosition::Center);
    window.set_default_size(350, 70);

    let v_box = gtk::Box::new(gtk::Orientation::Vertical, 10);
    let label = gtk::Label::new(Some("Nothing happened yet"));
    let switch = gtk::Switch::new();

    v_box.pack_start(&label, false, false, 0);
    v_box.pack_start(&switch, true, true, 0);
    window.add(&v_box);

    build_system_menu(application);

    add_actions(application, &switch, &label, &window);

    window.show_all();
}

fn main() {
    let application = gtk::Application::new(
        Some("com.github.gtk-rs.examples.menu_bar_system"),
        Default::default(),
    )
    .expect("Initialization failed...");

    application.connect_startup(|app| {
        add_accelerators(app);
    });
    application.connect_activate(build_ui);

    application.run();
}
