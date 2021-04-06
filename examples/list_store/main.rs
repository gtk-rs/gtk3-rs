use gtk::glib;
use gtk::prelude::*;

use std::rc::Rc;
use std::time::Duration;

#[derive(Debug)]
#[repr(i32)]
enum Columns {
    Fixed = 0,
    Number,
    Severity,
    Description,
    Pulse,
    Icon,
    Active,
    Sensitive,
}

fn build_ui(application: &gtk::Application) {
    let window = gtk::ApplicationWindow::new(application);

    window.set_title("List Store");
    window.set_border_width(10);
    window.set_position(gtk::WindowPosition::Center);
    window.set_default_size(280, 250);

    let vbox = gtk::Box::new(gtk::Orientation::Vertical, 8);
    window.add(&vbox);

    let label = gtk::Label::new(Some(
        "This is the bug list (note: not based on real data, it would be \
         nice to have a nice ODBC interface to bugzilla or so, though).",
    ));
    vbox.add(&label);

    let sw = gtk::ScrolledWindow::new(None::<&gtk::Adjustment>, None::<&gtk::Adjustment>);
    sw.set_shadow_type(gtk::ShadowType::EtchedIn);
    sw.set_policy(gtk::PolicyType::Never, gtk::PolicyType::Automatic);
    vbox.add(&sw);

    let model = Rc::new(create_model());
    let treeview = gtk::TreeView::with_model(&*model);
    treeview.set_vexpand(true);
    treeview.set_search_column(Columns::Description as i32);

    sw.add(&treeview);

    add_columns(&model, &treeview);

    window.show_all();

    glib::timeout_add_local(
        Duration::from_millis(80),
        glib::clone!(@weak model => @default-return glib::Continue(false), move || {
            spinner_timeout(&model);
            glib::Continue(false)
        }),
    );
}

struct Data {
    fixed: bool,
    number: u32,
    severity: String,
    description: String,
}

fn create_model() -> gtk::ListStore {
    let col_types: [glib::Type; 8] = [
        glib::Type::BOOL,
        glib::Type::U32,
        glib::Type::STRING,
        glib::Type::STRING,
        glib::Type::U32,
        glib::Type::STRING,
        glib::Type::BOOL,
        glib::Type::BOOL,
    ];

    let data: [Data; 14] = [
        Data {
            fixed: false,
            number: 60482,
            severity: "Normal".to_string(),
            description: "scrollable notebooks and hidden tabs".to_string(),
        },
        Data {
            fixed: false,
            number: 60620,
            severity: "Critical".to_string(),
            description: "gdk_surface_clear_area (gdksurface-win32.c) is not thread-safe"
                .to_string(),
        },
        Data {
            fixed: false,
            number: 50214,
            severity: "Major".to_string(),
            description: "Xft support does not clean up correctly".to_string(),
        },
        Data {
            fixed: true,
            number: 52877,
            severity: "Major".to_string(),
            description: "GtkFileSelection needs a refresh method. ".to_string(),
        },
        Data {
            fixed: false,
            number: 56070,
            severity: "Normal".to_string(),
            description: "Can't click button after setting in sensitive".to_string(),
        },
        Data {
            fixed: true,
            number: 56355,
            severity: "Normal".to_string(),
            description: "GtkLabel - Not all changes propagate correctly".to_string(),
        },
        Data {
            fixed: false,
            number: 50055,
            severity: "Normal".to_string(),
            description: "Rework width/height computations for TreeView".to_string(),
        },
        Data {
            fixed: false,
            number: 58278,
            severity: "Normal".to_string(),
            description: "gtk_dialog_set_response_sensitive () doesn't work".to_string(),
        },
        Data {
            fixed: false,
            number: 55767,
            severity: "Normal".to_string(),
            description: "Getters for all setters".to_string(),
        },
        Data {
            fixed: false,
            number: 56925,
            severity: "Normal".to_string(),
            description: "Gtkcalender size".to_string(),
        },
        Data {
            fixed: false,
            number: 56221,
            severity: "Normal".to_string(),
            description: "Selectable label needs right-click copy menu".to_string(),
        },
        Data {
            fixed: true,
            number: 50939,
            severity: "Normal".to_string(),
            description: "Add shift clicking to GtkTextView".to_string(),
        },
        Data {
            fixed: false,
            number: 6112,
            severity: "Normal".to_string(),
            description: "netscape-like collapsable toolbars".to_string(),
        },
        Data {
            fixed: false,
            number: 1,
            severity: "Normal".to_string(),
            description: "First bug :=)".to_string(),
        },
    ];

    let store = gtk::ListStore::new(&col_types);

    for (d_idx, d) in data.iter().enumerate() {
        let icon_name = if d_idx == 1 || d_idx == 3 {
            "battery-caution-charging-symbolic"
        } else {
            ""
        };

        let sensitive = d_idx != 3;

        let values: [(u32, &dyn ToValue); 8] = [
            (0, &d.fixed),
            (1, &d.number),
            (2, &d.severity),
            (3, &d.description),
            (4, &0u32),
            (5, &icon_name),
            (6, &false),
            (7, &sensitive),
        ];
        store.set(&store.append(), &values);
    }

    store
}

fn fixed_toggled<W: IsA<gtk::CellRendererToggle>>(
    model: &gtk::ListStore,
    _w: &W,
    path: gtk::TreePath,
) {
    let iter = model.get_iter(&path).unwrap();
    let mut fixed = model
        .get_value(&iter, Columns::Fixed as i32)
        .get_some::<bool>()
        .unwrap_or_else(|err| {
            panic!(
                "ListStore value for {:?} at path {}: {}",
                Columns::Fixed,
                path,
                err
            )
        });
    fixed = !fixed;
    model.set_value(&iter, Columns::Fixed as u32, &fixed.to_value());
}

fn add_columns(model: &Rc<gtk::ListStore>, treeview: &gtk::TreeView) {
    // Column for fixed toggles
    {
        let renderer = gtk::CellRendererToggle::new();
        let model_clone = model.clone();
        renderer.connect_toggled(move |w, path| fixed_toggled(&model_clone, w, path));
        let column = gtk::TreeViewColumn::new();
        column.pack_start(&renderer, true);
        column.set_title("Fixed?");
        column.add_attribute(&renderer, "active", Columns::Fixed as i32);
        column.set_sizing(gtk::TreeViewColumnSizing::Fixed);
        column.set_fixed_width(50);
        treeview.append_column(&column);
    }

    // Column for bug numbers
    {
        let renderer = gtk::CellRendererText::new();
        let column = gtk::TreeViewColumn::new();
        column.pack_start(&renderer, true);
        column.set_title("Bug number");
        column.add_attribute(&renderer, "text", Columns::Number as i32);
        column.set_sort_column_id(Columns::Number as i32);
        treeview.append_column(&column);
    }

    // Column for severities
    {
        let renderer = gtk::CellRendererText::new();
        let column = gtk::TreeViewColumn::new();
        column.pack_start(&renderer, true);
        column.set_title("Severity");
        column.add_attribute(&renderer, "text", Columns::Severity as i32);
        column.set_sort_column_id(Columns::Severity as i32);
        treeview.append_column(&column);
    }

    // Column for description
    {
        let renderer = gtk::CellRendererText::new();
        let column = gtk::TreeViewColumn::new();
        column.pack_start(&renderer, true);
        column.set_title("Description");
        column.add_attribute(&renderer, "text", Columns::Description as i32);
        column.set_sort_column_id(Columns::Description as i32);
        treeview.append_column(&column);
    }

    // Column for spinner
    {
        let renderer = gtk::CellRendererSpinner::new();
        let column = gtk::TreeViewColumn::new();
        column.pack_start(&renderer, true);
        column.set_title("Spinning");
        column.add_attribute(&renderer, "pulse", Columns::Pulse as i32);
        column.add_attribute(&renderer, "active", Columns::Active as i32);
        treeview.append_column(&column);
    }

    // Column for symbolic icon
    {
        let renderer = gtk::CellRendererPixbuf::new();
        let column = gtk::TreeViewColumn::new();
        column.pack_start(&renderer, true);
        column.set_title("Symbolic icon");
        column.add_attribute(&renderer, "icon-name", Columns::Icon as i32);
        column.add_attribute(&renderer, "sensitive", Columns::Sensitive as i32);
        column.set_sort_column_id(Columns::Icon as i32);
        treeview.append_column(&column);
    }
}

fn spinner_timeout(model: &gtk::ListStore) -> Continue {
    let iter = model.get_iter_first().unwrap();
    let pulse = model
        .get_value(&iter, Columns::Pulse as i32)
        .get_some::<u32>()
        .unwrap_or_else(|err| {
            panic!(
                "ListStore value for {:?} at first entry: {}",
                Columns::Pulse,
                err
            )
        })
        .wrapping_add(1);

    model.set_value(&iter, Columns::Pulse as i32 as u32, &pulse.to_value());
    model.set_value(&iter, Columns::Active as i32 as u32, &true.to_value());

    Continue(true)
}

fn main() {
    let application = gtk::Application::new(
        Some("com.github.gtk-rs.examples.list-store"),
        Default::default(),
    )
    .expect("Initialization failed...");

    application.connect_startup(|app| {
        build_ui(app);
    });

    application.connect_activate(|_| {});

    application.run();
}
