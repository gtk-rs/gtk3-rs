// Copyright 2019, The Gtk-rs Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <https://opensource.org/licenses/MIT>

mod application;
mod input_stream;
mod io_stream;
#[cfg(any(feature = "v2_44", feature = "dox"))]
mod list_model;
mod output_stream;
mod seekable;

pub use self::application::ArgumentList;
pub use self::prelude::*;

pub mod prelude {
    pub use super::application::{ApplicationImpl, ApplicationImplExt};
    pub use super::input_stream::{InputStreamImpl, InputStreamImplExt};
    pub use super::io_stream::{IOStreamImpl, IOStreamImplExt};
    #[cfg(any(feature = "v2_44", feature = "dox"))]
    pub use super::list_model::ListModelImpl;
    pub use super::output_stream::{OutputStreamImpl, OutputStreamImplExt};
    pub use super::seekable::SeekableImpl;
    pub use glib::subclass::prelude::*;
}
