// Take a look at the license at the top of the repository in the LICENSE file.

use crate::StateFlags;
use crate::StyleContext;
use glib::prelude::*;
use pango::FontDescription;

pub trait StyleContextExtManual: 'static {
    fn font(&self, state: StateFlags) -> FontDescription;
}

impl<O: IsA<StyleContext>> StyleContextExtManual for O {
    fn font(&self, state: StateFlags) -> FontDescription {
        <Self as crate::prelude::StyleContextExt>::property(self, "font", state)
            .get()
            .expect("font property is not pango::FontDescription")
    }
}
