// Take a look at the license at the top of the repository in the LICENSE file.

use crate::prelude::*;
use crate::StateFlags;
use crate::StyleContext;
use pango::FontDescription;

pub trait StyleContextExtManual: 'static {
    #[doc(alias = "get_font")]
    fn font(&self, state: StateFlags) -> FontDescription;
}

impl<O: IsA<StyleContext>> StyleContextExtManual for O {
    fn font(&self, state: StateFlags) -> FontDescription {
        self.style_property_for_state("font", state)
            .get()
            .expect("font property is not pango::FontDescription")
    }
}
