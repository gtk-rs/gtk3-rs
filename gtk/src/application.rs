// Copyright 2020, The Gtk-rs Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

use crate::rt;
use crate::Application;
use gio::ApplicationExt;
use gio::ApplicationFlags;
use glib;
use glib::signal::SignalHandlerId;
use glib::translate::*;
use glib::ObjectExt;
use gtk_sys;

use std::cell::RefCell;
use std::rc::Rc;

impl Application {
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

    pub fn new(
        application_id: Option<&str>,
        flags: ApplicationFlags,
    ) -> Result<Application, glib::BoolError> {
        skip_assert_initialized!();
        let app: Application = unsafe {
            Option::from_glib_full(gtk_sys::gtk_application_new(
                application_id.to_glib_none().0,
                flags.to_glib(),
            ))
            .ok_or_else(|| glib::glib_bool_error!("Failed to create application"))?
        };
        Application::register_startup_hook(&app);
        Ok(app)
    }
}
