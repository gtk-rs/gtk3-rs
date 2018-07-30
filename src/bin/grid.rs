#![cfg_attr(not(feature = "gtk_3_10"), allow(unused_variables, unused_mut))]

extern crate gio;
extern crate gtk;

#[cfg(feature = "gtk_3_10")]
mod example {
    use gio;
    use gtk;
    use gio::prelude::*;
    use gtk::prelude::*;
    use gtk::{
        ApplicationWindow, Builder, Button, Grid,
    };

    use std::env::args;

    // upgrade weak reference or return
    #[macro_export]
    macro_rules! upgrade_weak {
        ($x:ident, $r:expr) => {{
            match $x.upgrade() {
                Some(o) => o,
                None => return $r,
            }
        }};
        ($x:ident) => {
            upgrade_weak!($x, ())
        };
    }

    pub fn build_ui(application: &gtk::Application) {
        let glade_src = include_str!("grid.glade");
        let builder = Builder::new_from_string(glade_src);

        let window: ApplicationWindow = builder.get_object("window").expect("Couldn't get window");
        window.set_application(application);
        let grid: Grid = builder.get_object("grid").expect("Couldn't get grid");
        let button6: Button = builder.get_object("button6").expect("Couldn't get button6");
        let weak_grid = grid.downgrade();
        button6.connect_clicked(move |button| {
            let grid = upgrade_weak!(weak_grid);
            let height = grid.get_cell_height(button);
            let new_height = if height == 2 { 1 } else { 2 };
            grid.set_cell_height(button, new_height);
        });
        let button7: Button = builder.get_object("button7").expect("Couldn't get button7");
        let weak_grid = grid.downgrade();
        button7.connect_clicked(move |button| {
            let grid = upgrade_weak!(weak_grid);
            let left_attach = grid.get_cell_left_attach(button);
            let new_left_attach = if left_attach == 2 { 0 } else { left_attach + 1 };
            grid.set_cell_left_attach(button, new_left_attach);
        });

        window.connect_delete_event(move |win, _| {
            win.destroy();
            Inhibit(false)
        });

        window.show_all();
    }

    pub fn main() {
        let application = gtk::Application::new("com.github.grid",
                                                gio::ApplicationFlags::empty())
                                           .expect("Initialization failed...");

        application.connect_startup(move |app| {
            build_ui(app);
        });
        application.connect_activate(|_| {});

        application.run(&args().collect::<Vec<_>>());
    }
}

#[cfg(feature = "gtk_3_10")]
fn main() {
    example::main()
}

#[cfg(not(feature = "gtk_3_10"))]
fn main() {
    println!("This example requires GTK 3.10 or later");
    println!("Did you forget to build with `--features gtk_3_10`?");
}
