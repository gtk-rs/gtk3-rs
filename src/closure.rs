// Copyright 2013-2017, The Gtk-rs Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

// TODO: support marshaller.

use std::mem;
use std::ptr;
use std::slice;

use libc::{c_uint, c_void};

use ffi as glib_ffi;
use gobject_ffi;
use translate::{ToGlibPtr, ToGlibPtrMut, FromGlibPtrFull, Uninitialized, Stash, mut_override, from_glib_none};
use types::Type;
use Value;
use ToValue;

glib_wrapper! {
    pub struct Closure(Shared<gobject_ffi::GClosure>);

    match fn {
        ref => |ptr| {
            gobject_ffi::g_closure_ref(ptr);
            gobject_ffi::g_closure_sink(ptr);
        },
        unref => |ptr| gobject_ffi::g_closure_unref(ptr),
        get_type => || gobject_ffi::g_closure_get_type(),
    }
}

impl Closure {
    pub fn new<F: Fn(&[Value]) -> Option<Value> + Send + Sync + 'static>(callback: F) -> Self {
        unsafe extern "C" fn marshal<F>(_closure: *mut gobject_ffi::GClosure, return_value: *mut gobject_ffi::GValue,
            n_param_values: c_uint, param_values: *mut gobject_ffi::GValue, _invocation_hint: *mut c_void,
            marshal_data: *mut c_void)
            where F: Fn(&[Value]) -> Option<Value> + Send + Sync + 'static
        {
            let values = slice::from_raw_parts(param_values as *const _, n_param_values as usize);
            let callback: Box<F> = Box::from_raw(marshal_data as *mut _);
            let result = callback(values);
            if !return_value.is_null() {
                match result {
                    Some(result) => *return_value = result.into_raw(),
                    None => {
                        let result = Value::uninitialized();
                        *return_value = result.into_raw();
                    },
                }
            }
            mem::forget(callback);
        }

        unsafe extern "C" fn finalize<F>(notify_data: *mut c_void, _closure: *mut gobject_ffi::GClosure)
            where F: Fn(&[Value]) -> Option<Value> + Send + Sync + 'static
        {
            let _callback: Box<F> = Box::from_raw(notify_data as *mut _);
            // callback is dropped here.
        }

        unsafe {
            let size = 4 + 4 + 3 * mem::size_of::<*mut c_void>() as u32;
            let closure = gobject_ffi::g_closure_new_simple(size, ptr::null_mut());
            assert_ne!(closure, ptr::null_mut());
            let callback = Box::new(callback);
            let ptr: *mut F = Box::into_raw(callback);
            let ptr: *mut c_void = ptr as *mut _;
            gobject_ffi::g_closure_set_meta_marshal(closure, ptr, Some(marshal::<F>));
            gobject_ffi::g_closure_add_finalize_notifier(closure, ptr, Some(finalize::<F>));
            from_glib_none(closure)
        }
    }

    pub fn invoke(&self, values: &[&ToValue]) -> Option<Value> {
        let mut result = unsafe { Value::uninitialized() };

        let v_args: Vec<Value>;
        let mut s_args: [Value; 10] = unsafe { mem::zeroed() };
        let values = if values.len() <= 10 {
            for (i, arg) in values.iter().enumerate() {
                s_args[i] = arg.to_value();
            }
            &s_args[0..values.len()]
        } else {
            v_args = values.iter()
                .map(|v| v.to_value())
                .collect();
            v_args.as_slice()
        };

        unsafe {
            gobject_ffi::g_closure_invoke(self.to_glib_none().0 as *mut _, result.to_glib_none_mut().0,
                values.len() as u32, mut_override(values.as_ptr()) as *mut gobject_ffi::GValue, ptr::null_mut());
        }
        if result.type_() == Type::Invalid {
            None
        } else {
            Some(result)
        }
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;
    use std::sync::atomic::{AtomicUsize, Ordering};

    use super::Closure;
    use ToValue;
    use Value;

    fn closure_fn(values: &[Value]) -> Option<Value> {
        assert_eq!(values.len(), 2);
        let string: Option<String> = values[0].get();
        assert_eq!(string, Some("test".to_string()));
        let int: Option<i32> = values[1].get();
        assert_eq!(int, Some(42));
        Some(24.to_value())
    }

    #[test]
    fn test_closure() {
        let call_count = Arc::new(AtomicUsize::new(0));

        let count = call_count.clone();
        let closure = Closure::new(move |values| {
            count.fetch_add(1, Ordering::Relaxed);
            assert_eq!(values.len(), 2);
            let string: Option<String> = values[0].get();
            assert_eq!(string, Some("test".to_string()));
            let int: Option<i32> = values[1].get();
            assert_eq!(int, Some(42));
            None
        });
        let result = closure.invoke(&[&"test".to_string(), &42]);
        assert!(result.is_none());
        assert_eq!(call_count.load(Ordering::Relaxed), 1);

        let result = closure.invoke(&[&"test".to_string(), &42]);
        assert!(result.is_none());
        assert_eq!(call_count.load(Ordering::Relaxed), 2);

        let closure = Closure::new(closure_fn);
        let result = closure.invoke(&[&"test".to_string(), &42]);
        let int: Option<i32> = result.and_then(|result| result.get());
        assert_eq!(int, Some(24));
    }
}
