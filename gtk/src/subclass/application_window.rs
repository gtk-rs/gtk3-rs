// Take a look at the license at the top of the repository in the LICENSE file.

use glib::subclass::prelude::*;

use super::window::WindowImpl;
use crate::ApplicationWindow;
use crate::Window;

glib::is_subclassable!(ApplicationWindow, Window, application_window_ @default_override_vfuncs;);

impl<T: ApplicationWindowImpl> ApplicationWindowImplExt for T {}
