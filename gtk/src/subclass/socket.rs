// Copyright 2020, The Gtk-rs Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

use glib;
use glib::subclass::prelude::*;
use glib::translate::*;
use glib::Cast;

use super::container::ContainerImpl;
use crate::Container;
use crate::Inhibit;
use crate::Socket;

pub trait SocketImpl: SocketImplExt + ContainerImpl {
    fn plug_added(&self, socket: &Self::Type) {
        self.parent_plug_added(socket)
    }

    fn plug_removed(&self, socket: &Self::Type) -> Inhibit {
        self.parent_plug_removed(socket)
    }
}

pub trait SocketImplExt: ObjectSubclass {
    fn parent_plug_added(&self, socket: &Self::Type);
    fn parent_plug_removed(&self, socket: &Self::Type) -> Inhibit;
}

impl<T: SocketImpl> SocketImplExt for T {
    fn parent_plug_added(&self, socket: &Self::Type) {
        unsafe {
            let data = T::type_data();
            let parent_class = data.as_ref().get_parent_class() as *mut ffi::GtkSocketClass;
            if let Some(f) = (*parent_class).plug_added {
                f(socket.unsafe_cast_ref::<Socket>().to_glib_none().0)
            }
        }
    }

    fn parent_plug_removed(&self, socket: &Self::Type) -> Inhibit {
        unsafe {
            let data = T::type_data();
            let parent_class = data.as_ref().get_parent_class() as *mut ffi::GtkSocketClass;
            if let Some(f) = (*parent_class).plug_removed {
                Inhibit(from_glib(f(socket
                    .unsafe_cast_ref::<Socket>()
                    .to_glib_none()
                    .0)))
            } else {
                Inhibit(false)
            }
        }
    }
}

unsafe impl<T: SocketImpl> IsSubclassable<T> for Socket {
    fn override_vfuncs(class: &mut glib::Class<Self>) {
        <Container as IsSubclassable<T>>::override_vfuncs(class);

        let klass = class.as_mut();
        klass.plug_added = Some(socket_plug_added::<T>);
        klass.plug_removed = Some(socket_plug_removed::<T>);
    }
}

unsafe extern "C" fn socket_plug_added<T: SocketImpl>(ptr: *mut ffi::GtkSocket) {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.get_impl();
    let wrap: Borrowed<Socket> = from_glib_borrow(ptr);

    imp.plug_added(wrap.unsafe_cast_ref())
}

unsafe extern "C" fn socket_plug_removed<T: SocketImpl>(
    ptr: *mut ffi::GtkSocket,
) -> glib_sys::gboolean {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.get_impl();
    let wrap: Borrowed<Socket> = from_glib_borrow(ptr);

    imp.plug_removed(wrap.unsafe_cast_ref()).to_glib()
}
