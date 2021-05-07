// Take a look at the license at the top of the repository in the LICENSE file.

mod action_group;
mod action_map;
mod application;
mod input_stream;
mod io_stream;
mod list_model;
mod output_stream;
mod seekable;

pub use self::application::ArgumentList;

pub mod prelude {
    #[doc(hidden)]
    pub use glib::subclass::prelude::*;

    pub use super::action_group::{ActionGroupImpl, ActionGroupImplExt};
    pub use super::action_map::{ActionMapImpl, ActionMapImplExt};
    pub use super::application::{ApplicationImpl, ApplicationImplExt};
    pub use super::input_stream::{InputStreamImpl, InputStreamImplExt};
    pub use super::io_stream::{IOStreamImpl, IOStreamImplExt};
    pub use super::list_model::{ListModelImpl, ListModelImplExt};
    pub use super::output_stream::{OutputStreamImpl, OutputStreamImplExt};
    pub use super::seekable::{SeekableImpl, SeekableImplExt};
}
