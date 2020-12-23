// Take a look at the license at the top of the repository in the LICENSE file.

use glib::subclass::prelude::*;

use super::container::ContainerImpl;
use crate::Bin;
use crate::Container;

glib::is_subclassable!(Bin, Container, container_ @default_override_vfuncs;);

impl<T: BinImpl> BinImplExt for T {}
