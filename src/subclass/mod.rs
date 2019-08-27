// Copyright 2019, The Gtk-rs Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

pub mod application;
pub mod input_stream;
pub mod output_stream;

pub mod prelude {
    pub use super::application::{ApplicationImpl, ArgumentList};
    pub use super::input_stream::InputStreamImpl;
    pub use super::output_stream::OutputStreamImpl;
    pub use glib::subclass::prelude::*;
}
