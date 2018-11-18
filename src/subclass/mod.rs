// Copyright (C) 2016-2018 Sebastian Dröge <sebastian@centricular.com>
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! Module containing infrastructure for subclassing `GObject`s

#[macro_use]
#[doc(hidden)]
pub mod guard;

pub mod simple;
#[macro_use]
pub mod types;

#[macro_use]
pub mod object;
pub mod properties;

pub mod prelude {
    //! Prelude that re-exports all important traits from this crate
    pub use super::object::{ObjectClassSubclassExt, ObjectImpl};
    pub use super::types::{ClassStruct, InstanceStruct, IsSubclassable, ObjectSubclass};
}

pub use super::subclass::types::register_type;
pub use super::subclass::types::TypeData;
