//! # Actions Sample
//!
//! This sample demonstrates how to use `gio::actions` macro.

use gtk::prelude::*;
use gtk::{gio, glib};

#[derive(glib::Downgrade)]
pub struct ExampleApplication(gtk::Application);

impl ExampleApplication {
    pub fn new() -> Self {
        let app = Self(
            gtk::ApplicationBuilder::new()
                .application_id("com.github.gtk-rs.examples.actions")
                .build(),
        );
        app.0.connect_startup(glib::clone!(@weak app => move |_app|
            app.startup()
        ));
        app.0.connect_activate(glib::clone!(@weak app => move |_app|
            app.activate()
        ));
        app
    }

    fn startup(&self) {
        // This line creates actions (`gio::SimpleAction`),
        // binds them to handlers, defined in the next `impl` block
        // and adds them to a `self.0`.
        self.register_actions(&self.0);
        self.add_accelerators();
        self.build_system_menu();
    }

    fn add_accelerators(&self) {
        self.0.set_accels_for_action("app.about", &["F1"]);
        self.0.set_accels_for_action("app.quit", &["<Primary>Q"]);
    }

    fn activate(&self) {
        if let Some(window) = self.0.active_window() {
            window.show_all();
        } else {
            let window = ExampleWindow::new(&self.0);
            window.0.show_all();
        }
    }

    fn build_system_menu(&self) {
        let menu = gio::Menu::new();
        menu.append(Some("Quit"), Some("app.quit"));
        self.0.set_app_menu(Some(&menu));

        let menu_bar = gio::Menu::new();
        menu_bar.append_submenu(Some("_Actions"), &{
            let menu = gio::Menu::new();
            menu.append(Some("Example Action"), Some("win.example_action"));
            menu.append(Some("Full screen"), Some("win.fullscreen"));
            menu.append(Some("Quit"), Some("app.quit"));
            menu
        });
        menu_bar.append_submenu(Some("_Group #1"), &{
            let menu = gio::Menu::new();
            menu.append(Some("Example Action"), Some("group1.example_action"));
            menu
        });
        menu_bar.append_submenu(Some("_Group #2"), &{
            let menu = gio::Menu::new();
            menu.append(Some("Example Action"), Some("group2.example_action"));
            menu
        });
        menu_bar.append_submenu(Some("?"), &{
            let menu = gio::Menu::new();
            menu.append(Some("About"), Some("app.about"));
            menu
        });
        self.0.set_menubar(Some(&menu_bar));
    }
}

impl std::default::Default for ExampleApplication {
    fn default() -> Self {
        Self::new()
    }
}

#[gio::actions]
impl ExampleApplication {
    fn about(&self) {
        let window = self.0.active_window();
        let dialog = gtk::AboutDialogBuilder::new()
            .website_label("gtk-rs")
            .website("http://gtk-rs.org")
            .authors(vec!["Gtk-rs developers".into()])
            .title("About!")
            .build();
        dialog.set_transient_for(window.as_ref());
        dialog.show_all();
    }

    fn quit(&self) {
        // Close all windows
        for window in self.0.windows() {
            window.close();
        }
    }
}

#[derive(glib::Downgrade)]
pub struct ExampleWindow(gtk::ApplicationWindow);

impl ExampleWindow {
    pub fn new(app: &gtk::Application) -> Self {
        let window = Self(
            gtk::ApplicationWindowBuilder::new()
                .application(app)
                .title("System menu bar")
                .window_position(gtk::WindowPosition::Center)
                .default_width(350)
                .default_height(70)
                .build(),
        );

        let grid = gtk::GridBuilder::new()
            .margin(10)
            .row_spacing(10)
            .column_spacing(10)
            .build();
        window.0.add(&grid);

        let label = gtk::LabelBuilder::new()
            .label("Nothing happened yet")
            .halign(gtk::Align::Start)
            .build();
        grid.attach(&label, 0, 0, 2, 1);

        let example = gtk::ButtonBuilder::new()
            .label("Example Action")
            .action_name("win.example_action")
            .build();
        grid.attach(&example, 0, 1, 1, 1);

        let label = gtk::LabelBuilder::new()
            .label("Toggle full screen mode")
            .halign(gtk::Align::Start)
            .build();
        grid.attach(&label, 0, 2, 1, 1);

        let switch = gtk::Switch::new();
        switch.set_action_name(Some("win.fullscreen"));
        grid.attach(&switch, 1, 2, 1, 1);

        let colors = RadioGroupBuilder::new("win.pick_color")
            .add("Red", "#FF0000")
            .add("Green", "#00FF00")
            .add("Blue", "#0000FF")
            .build();
        grid.attach(&colors, 0, 3, 1, 1);

        // This line creates actions (`gio::SimpleAction`),
        // binds them to handlers, defined in the next `impl` block
        // and adds them to a `window.0`.
        window.register_actions(&window.0);

        // Get "volume" action from the window. It exists after an invocation
        // of `register_action` method, which created all actions and added them
        // to our window.
        let volume_action = window.0.lookup_action("volume").unwrap();

        let volume = gtk::ScaleBuilder::new()
            .orientation(gtk::Orientation::Horizontal)
            .adjustment(&gtk::Adjustment::new(0.0, -2.0, 12.0, 0.1, 0.0, 0.0))
            .build();
        volume.connect_change_value(
            glib::clone!(@weak window, @weak volume_action => @default-return Inhibit(false), move |_scale, _scroll_type, value| {
                volume_action.activate(Some(&value.to_variant()));
                Inhibit(false)
            }),
        );
        grid.attach(&volume, 0, 4, 1, 1);

        let volume_label = gtk::LabelBuilder::new()
            .label("Volume: 0.0")
            .halign(gtk::Align::End)
            .build();
        grid.attach(&volume_label, 1, 4, 1, 1);

        volume_action.connect_property_state_notify(
            glib::clone!(@weak volume_label => move |action| {
                let value = action.state().and_then(|variant| variant.get::<f64>()).unwrap();
                volume_label.set_label(&format!("Volume: {:.2}", value));
            }),
        );

        // Additionally, we can create own action groups and add them into our window.
        // All actions in this group have a prefix "group1".
        let action_group1 = gio::SimpleActionGroup::new();
        window.register_actions_group1(&action_group1);
        window.0.insert_action_group("group1", Some(&action_group1));

        // And all actions in this group have a prefix "group2".
        let action_group2 = gio::SimpleActionGroup::new();
        window.register_actions_group2(&action_group2);
        window.0.insert_action_group("group2", Some(&action_group2));

        window
    }

    fn show_message(&self, message: &str) {
        let dialog = gtk::MessageDialogBuilder::new()
            .transient_for(&self.0)
            .modal(true)
            .message_type(gtk::MessageType::Info)
            .text(message)
            .buttons(gtk::ButtonsType::Ok)
            .build();

        dialog.show_all();
        dialog.run();
        dialog.close();
    }
}

#[gio::actions]
impl ExampleWindow {
    fn example_action(&self) {
        self.show_message("Example Action is activated!");
    }

    #[action(stateful)]
    fn fullscreen(&self, was_active: bool) -> Option<bool> {
        let is_active = !was_active; // New state
        if is_active {
            self.0.fullscreen();
        } else {
            self.0.unfullscreen();
        }
        // Update state of the action by returning new value
        Some(is_active)
    }

    // This action has both a state and a parameter.
    #[action(stateful)]
    fn pick_color(&self, previous_state: String, color_parameter: String) -> Option<String> {
        if previous_state == color_parameter {
            // We do not want to change the state to the same value. So, returning `None`.
            None
        } else {
            // Color is different, so let's update the state.
            Some(color_parameter)
        }
    }

    #[action(change_state)]
    fn volume(&self, value: f64) -> Option<f64> {
        if value >= 0.0 && value <= 10.0 {
            // accept
            Some(value)
        } else {
            // reject value
            None
        }
    }
}

// This `impl` block also contains actions, but its register method is
// renamed by a following annotation to `register_actions_group1` not to
// collide with a block above.
#[gio::actions(register_fn = "register_actions_group1")]
impl ExampleWindow {
    // We can rename an action, so its detailed name becomes `group1.example_action`.
    #[action(name = "example_action")]
    fn group1_example_action(&self) {
        self.show_message("Group #1 Example Action is activated!");
    }
}

#[gio::actions(register_fn = "register_actions_group2")]
impl ExampleWindow {
    // We can rename an action, so its detailed name becomes `group2.example_action`.
    #[action(name = "example_action")]
    fn group2_example_action(&self) {
        self.show_message("Group #2 Example Action is activated!");
    }
}

struct RadioGroupBuilder {
    grid: gtk::Grid,
    count: i32,
    last: Option<gtk::RadioButton>,
    action: String,
}

impl RadioGroupBuilder {
    pub fn new(action: &str) -> Self {
        Self {
            grid: gtk::GridBuilder::new().row_spacing(10).build(),
            count: 0,
            last: None,
            action: action.into(),
        }
    }

    pub fn add(mut self, label: &str, parameter: &str) -> Self {
        let radio = gtk::RadioButtonBuilder::new()
            .label(label)
            .action_name(&self.action)
            .action_target(&parameter.to_variant())
            .build();
        radio.join_group(self.last.as_ref());
        self.grid.attach(&radio, 0, self.count, 1, 1);
        self.last = Some(radio);
        self.count += 1;
        self
    }

    pub fn build(self) -> gtk::Widget {
        self.grid.upcast()
    }
}

fn main() {
    let application = ExampleApplication::new();
    application.0.run();
}
