extern crate gtk;

// make moving clones into closures more convenient
#[cfg(feature = "gtk_3_10")]
macro_rules! clone {
    (@param _) => ( _ );
    (@param $x:ident) => ( $x );
    ($($n:ident),+ => move || $body:expr) => (
        {
            $( let $n = $n.clone(); )+
                move || $body
        }
    );
    ($($n:ident),+ => move |$($p:tt),+| $body:expr) => (
        {
            $( let $n = $n.clone(); )+
                move |$(clone!(@param $p),)+| $body
        }
    );
}

#[cfg(feature = "gtk_3_10")]
mod example {
    use gtk;
    use gtk::prelude::*;
    use gtk::{
        Builder, Button, Grid, Window,
    };

    pub fn main() {
        if gtk::init().is_err() {
            println!("Failed to initialize GTK.");
            return;
        }
        let glade_src = include_str!("grid.glade");
        let builder = Builder::new_from_string(glade_src);

        let window: Window = builder.get_object("window").unwrap();
        let grid: Grid = builder.get_object("grid").unwrap();
        let button6: Button = builder.get_object("button6").unwrap();
        button6.connect_clicked(clone!(grid, button6 => move |_| {
            let height = grid.get_cell_height(&button6);
            let new_height = if height == 2 { 1 } else { 2 };
            grid.set_cell_height(&button6, new_height);
        }));
        let button7: Button = builder.get_object("button7").unwrap();
        button7.connect_clicked(clone!(grid, button7 => move |_| {
            let left_attach = grid.get_cell_left_attach(&button7);
            let new_left_attach = if left_attach == 2 { 0 } else { left_attach + 1 };
            grid.set_cell_left_attach(&button7, new_left_attach);
        }));

        window.connect_delete_event(|_, _| {
            gtk::main_quit();
            Inhibit(false)
        });

        window.show_all();
        gtk::main();
    }
}

#[cfg(feature = "gtk_3_10")]
fn main() {
    example::main()
}

#[cfg(not(feature = "gtk_3_10"))]
fn main() {
    println!("This example only work with GTK 3.10 and later");
    println!("Did you forget to build with `--features gtk_3_10`?");
}
