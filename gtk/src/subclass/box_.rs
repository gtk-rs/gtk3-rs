// Take a look at the license at the top of the repository in the LICENSE file.

use glib::subclass::prelude::*;

use super::container::ContainerImpl;
use crate::Box;
use crate::Container;

glib::is_subclassable!(Box, Container, box_ @default_override_vfuncs;);

impl<T: BoxImpl> BoxImplExt for T {}
