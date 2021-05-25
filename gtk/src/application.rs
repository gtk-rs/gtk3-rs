// Take a look at the license at the top of the repository in the LICENSE file.

use crate::rt;
use crate::Application;
use gio::prelude::*;
use gio::ApplicationFlags;
use glib::signal::SignalHandlerId;
use glib::translate::*;

use std::cell::RefCell;
use std::rc::Rc;

impl Application {
    /// Creates a new [Application](crate::Application) instance.
    ///
    /// When using [Application](crate::Application), it is not necessary to call `gtk_init`
    /// manually. It is called as soon as the application gets registered as
    /// the primary instance.
    ///
    /// Concretely, `gtk_init` is called in the default handler for the
    /// `GApplication::::startup` signal. Therefore, [Application](crate::Application) subclasses should
    /// chain up in their `GApplication::::startup` handler before using any GTK+ API.
    ///
    /// Note that commandline arguments are not passed to `gtk_init`.
    /// All GTK+ functionality that is available via commandline arguments
    /// can also be achieved by setting suitable environment variables
    /// such as `G_DEBUG`, so this should not be a big
    /// problem. If you absolutely must support GTK+ commandline arguments,
    /// you can explicitly call `gtk_init` before creating the application
    /// instance.
    ///
    /// If non-[`None`], the application ID must be valid. See
    /// [Application::id_is_valid](crate::gio::Application::id_is_valid).
    ///
    /// If no application ID is given then some features (most notably application
    /// uniqueness) will be disabled. A null application ID is only allowed with
    /// GTK+ 3.6 or later.
    /// ## `application_id`
    /// The application ID.
    /// ## `flags`
    /// the application flags
    ///
    /// # Returns
    ///
    /// a new [Application](crate::Application) instance
    #[doc(alias = "gtk_application_new")]
    pub fn new(application_id: Option<&str>, flags: ApplicationFlags) -> Application {
        skip_assert_initialized!();
        let app: Application = unsafe {
            from_glib_full(ffi::gtk_application_new(
                application_id.to_glib_none().0,
                flags.into_glib(),
            ))
        };
        Application::register_startup_hook(&app);
        app
    }

    pub(crate) fn register_startup_hook(app: &Application) {
        skip_assert_initialized!();
        let signalid: Rc<RefCell<Option<SignalHandlerId>>> = Rc::new(RefCell::new(None));
        {
            let signalid_ = signalid.clone();

            let id = app.connect_startup(move |app| {
                app.disconnect(
                    signalid_
                        .borrow_mut()
                        .take()
                        .expect("Signal ID went missing"),
                );
                unsafe { rt::set_initialized() }
            });
            *signalid.borrow_mut() = Some(id);
        }
    }
}
