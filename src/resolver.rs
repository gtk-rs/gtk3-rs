// Copyright 2013-2018, The Gtk-rs Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

use Cancellable;
use Error;
use ffi;
use glib::object::IsA;
use glib::translate::*;
use std::ptr;
use glib_ffi;
use gobject_ffi;
use Resolver;
use InetAddress;
use SrvTarget;

pub trait ResolverExtManual {
    fn lookup_by_address_async<'a, P: Into<Option<&'a Cancellable>>, Q: FnOnce(Result<String, Error>) + Send + 'static>(&self, address: &InetAddress, cancellable: P, callback: Q);

    fn lookup_by_name_async<'a, P: Into<Option<&'a Cancellable>>, Q: FnOnce(Result<Vec<InetAddress>, Error>) + Send + 'static>(&self, hostname: &str, cancellable: P, callback: Q);

    #[cfg(any(feature = "v2_34", feature = "dox"))]
    fn lookup_records_async<'a, P: Into<Option<&'a Cancellable>>, Q: FnOnce(Result<Vec<glib::Variant>, Error>) + Send + 'static>(&self, rrname: &str, record_type: ResolverRecordType, cancellable: P, callback: Q);

    fn lookup_service_async<'a, P: Into<Option<&'a Cancellable>>, Q: FnOnce(Result<Vec<SrvTarget>, Error>) + Send + 'static>(&self, service: &str, protocol: &str, domain: &str, cancellable: P, callback: Q);
}

impl<O: IsA<Resolver>> ResolverExtManual for O {
    fn lookup_by_address_async<'a, P: Into<Option<&'a Cancellable>>, Q: FnOnce(Result<String, Error>) + Send + 'static>(&self, address: &InetAddress, cancellable: P, callback: Q) {
        let cancellable = cancellable.into();
        let cancellable = cancellable.to_glib_none();
        let user_data: Box<Box<Q>> = Box::new(Box::new(callback));
        unsafe extern "C" fn lookup_by_address_async_trampoline<Q: FnOnce(Result<String, Error>) + Send + 'static>(_source_object: *mut gobject_ffi::GObject, res: *mut ffi::GAsyncResult, user_data: glib_ffi::gpointer)
        {
            callback_guard!();
            let mut error = ptr::null_mut();
            let res = ffi::g_resolver_lookup_by_address_finish(_source_object as *mut _, res, &mut error);
            let result = if error.is_null() { Ok(from_glib_full(res)) } else { Err(from_glib_full(error)) };
            let callback: Box<Box<Q>> = Box::from_raw(user_data as *mut _);
            callback(result);
        }
        let callback = lookup_by_address_async_trampoline::<Q>;
        unsafe {
            ffi::g_resolver_lookup_by_address_async(self.to_glib_none().0, address.to_glib_none().0, cancellable.0, Some(callback), Box::into_raw(user_data) as *mut _);
        }
    }

    fn lookup_by_name_async<'a, P: Into<Option<&'a Cancellable>>, Q: FnOnce(Result<Vec<InetAddress>, Error>) + Send + 'static>(&self, hostname: &str, cancellable: P, callback: Q) {
        let cancellable = cancellable.into();
        let cancellable = cancellable.to_glib_none();
        let user_data: Box<Box<Q>> = Box::new(Box::new(callback));
        unsafe extern "C" fn lookup_by_name_async_trampoline<Q: FnOnce(Result<Vec<InetAddress>, Error>) + Send + 'static>(_source_object: *mut gobject_ffi::GObject, res: *mut ffi::GAsyncResult, user_data: glib_ffi::gpointer)
        {
            callback_guard!();
            let mut error = ptr::null_mut();
            let res = ffi::g_resolver_lookup_by_name_finish(_source_object as *mut _, res, &mut error);
            let result = if error.is_null() { Ok(FromGlibPtrContainer::from_glib_full(res)) } else { Err(from_glib_full(error)) };
            let callback: Box<Box<Q>> = Box::from_raw(user_data as *mut _);
            callback(result);
        }
        let callback = lookup_by_name_async_trampoline::<Q>;
        unsafe {
            ffi::g_resolver_lookup_by_name_async(self.to_glib_none().0, hostname.to_glib_none().0, cancellable.0, Some(callback), Box::into_raw(user_data) as *mut _);
        }
    }

    #[cfg(any(feature = "v2_34", feature = "dox"))]
    fn lookup_records_async<'a, P: Into<Option<&'a Cancellable>>, Q: FnOnce(Result<Vec<glib::Variant>, Error>) + Send + 'static>(&self, rrname: &str, record_type: ResolverRecordType, cancellable: P, callback: Q) {
        let cancellable = cancellable.into();
        let cancellable = cancellable.to_glib_none();
        let user_data: Box<Box<Q>> = Box::new(Box::new(callback));
        unsafe extern "C" fn lookup_records_async_trampoline<Q: FnOnce(Result<Vec<glib::Variant>, Error>) + Send + 'static>(_source_object: *mut gobject_ffi::GObject, res: *mut ffi::GAsyncResult, user_data: glib_ffi::gpointer)
        {
            callback_guard!();
            let mut error = ptr::null_mut();
            let res = ffi::g_resolver_lookup_records_finish(_source_object as *mut _, res, &mut error);
            let result = if error.is_null() { Ok(FromGlibPtrContainer::from_glib_full(res)) } else { Err(from_glib_full(error)) };
            let callback: Box<Box<Q>> = Box::from_raw(user_data as *mut _);
            callback(result);
        }
        let callback = lookup_records_async_trampoline::<Q>;
        unsafe {
            ffi::g_resolver_lookup_records_async(self.to_glib_none().0, rrname.to_glib_none().0, record_type.to_glib(), cancellable.0, Some(callback), Box::into_raw(user_data) as *mut _);
        }
    }

    fn lookup_service_async<'a, P: Into<Option<&'a Cancellable>>, Q: FnOnce(Result<Vec<SrvTarget>, Error>) + Send + 'static>(&self, service: &str, protocol: &str, domain: &str, cancellable: P, callback: Q) {
        let cancellable = cancellable.into();
        let cancellable = cancellable.to_glib_none();
        let user_data: Box<Box<Q>> = Box::new(Box::new(callback));
        unsafe extern "C" fn lookup_service_async_trampoline<Q: FnOnce(Result<Vec<SrvTarget>, Error>) + Send + 'static>(_source_object: *mut gobject_ffi::GObject, res: *mut ffi::GAsyncResult, user_data: glib_ffi::gpointer)
        {
            callback_guard!();
            let mut error = ptr::null_mut();
            let res = ffi::g_resolver_lookup_service_finish(_source_object as *mut _, res, &mut error);
            let result = if error.is_null() { Ok(FromGlibPtrContainer::from_glib_full(res)) } else { Err(from_glib_full(error)) };
            let callback: Box<Box<Q>> = Box::from_raw(user_data as *mut _);
            callback(result);
        }
        let callback = lookup_service_async_trampoline::<Q>;
        unsafe {
            ffi::g_resolver_lookup_service_async(self.to_glib_none().0, service.to_glib_none().0, protocol.to_glib_none().0, domain.to_glib_none().0, cancellable.0, Some(callback), Box::into_raw(user_data) as *mut _);
        }
    }
}
