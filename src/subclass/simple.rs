// Copyright 2017-2018, The Gtk-rs Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

//! This module contains simple instance and class structs to be used for
//! `GObject` subclasses that don't require any additional data in these
//! structs and don't provide any new virtual methods.

use super::prelude::*;
use wrapper::Wrapper;

use std::ops;

/// A simple instance struct that does not store any additional data.
#[repr(C)]
pub struct InstanceStruct<T: ObjectSubclass> {
    parent: <T::ParentType as Wrapper>::GlibType,
}

unsafe impl<T: ObjectSubclass> super::types::InstanceStruct for InstanceStruct<T> {
    type Type = T;
}

/// A simple class struct that does not store any additional data
/// or virtual methods.
#[repr(C)]
pub struct ClassStruct<T: ObjectSubclass> {
    parent_class: <T::ParentType as Wrapper>::GlibClassType,
}

unsafe impl<T: ObjectSubclass> super::types::ClassStruct for ClassStruct<T> {
    type Type = T;
}

impl<T: ObjectSubclass> ops::Deref for ClassStruct<T> {
    type Target = <<T as ObjectSubclass>::ParentType as Wrapper>::RustClassType;

    fn deref(&self) -> &Self::Target {
        unsafe { &*(self as *const _ as *const Self::Target) }
    }
}

impl<T: ObjectSubclass> ops::DerefMut for ClassStruct<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut *(self as *mut _ as *mut Self::Target) }
    }
}
