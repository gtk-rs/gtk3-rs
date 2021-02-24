// Take a look at the license at the top of the repository in the LICENSE file.

// TODO: support marshaller.

use std::mem;
use std::ptr;
use std::slice;

use libc::{c_uint, c_void};

use crate::translate::{from_glib_none, mut_override, ToGlibPtr, ToGlibPtrMut, Uninitialized};
use crate::ToValue;
use crate::Value;

wrapper! {
    #[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Hash)]
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
        unsafe { Closure::new_unsafe(callback) }
    }

    pub fn new_local<F: Fn(&[Value]) -> Option<Value> + 'static>(callback: F) -> Self {
        let callback = crate::ThreadGuard::new(callback);

        unsafe { Closure::new_unsafe(move |values| (callback.get_ref())(values)) }
    }

    pub unsafe fn new_unsafe<F: Fn(&[Value]) -> Option<Value>>(callback: F) -> Self {
        unsafe extern "C" fn marshal<F>(
            _closure: *mut gobject_ffi::GClosure,
            return_value: *mut gobject_ffi::GValue,
            n_param_values: c_uint,
            param_values: *const gobject_ffi::GValue,
            _invocation_hint: *mut c_void,
            marshal_data: *mut c_void,
        ) where
            F: Fn(&[Value]) -> Option<Value>,
        {
            let values = slice::from_raw_parts(param_values as *const _, n_param_values as usize);
            let callback: &F = &*(marshal_data as *mut _);
            let result = callback(values);
            if !return_value.is_null() {
                match result {
                    Some(result) => *return_value = result.into_raw(),
                    None => {
                        let result = Value::uninitialized();
                        *return_value = result.into_raw();
                    }
                }
            }
        }

        unsafe extern "C" fn finalize<F>(
            notify_data: *mut c_void,
            _closure: *mut gobject_ffi::GClosure,
        ) where
            F: Fn(&[Value]) -> Option<Value>,
        {
            let _callback: Box<F> = Box::from_raw(notify_data as *mut _);
            // callback is dropped here.
        }

        // Due to bitfields we have to do our own calculations here for the size of the GClosure:
        // - 4: 32 bits in guint bitfields at the beginning
        // - padding due to alignment needed for the following pointer
        // - 3 * size_of<*mut c_void>: 3 pointers
        // We don't store any custom data ourselves in the GClosure
        let size = u32::max(4, mem::align_of::<*mut c_void>() as u32)
            + 3 * mem::size_of::<*mut c_void>() as u32;
        let closure = gobject_ffi::g_closure_new_simple(size, ptr::null_mut());
        assert_ne!(closure, ptr::null_mut());
        let callback = Box::new(callback);
        let ptr: *mut F = Box::into_raw(callback);
        let ptr: *mut c_void = ptr as *mut _;
        gobject_ffi::g_closure_set_meta_marshal(closure, ptr, Some(marshal::<F>));
        gobject_ffi::g_closure_add_finalize_notifier(closure, ptr, Some(finalize::<F>));
        from_glib_none(closure)
    }

    pub fn invoke(&self, values: &[&dyn ToValue]) -> Option<Value> {
        let values = values
            .iter()
            .copied()
            .map(ToValue::to_value)
            .collect::<smallvec::SmallVec<[_; 10]>>();

        self.invoke_with_values(&values)
    }

    pub fn invoke_with_values(&self, values: &[Value]) -> Option<Value> {
        let result = unsafe {
            let mut result = Value::uninitialized();
            gobject_ffi::g_closure_invoke(
                self.to_glib_none().0 as *mut _,
                result.to_glib_none_mut().0,
                values.len() as u32,
                mut_override(values.as_ptr()) as *mut gobject_ffi::GValue,
                ptr::null_mut(),
            );

            result
        };

        Some(result).filter(|r| r.type_().is_valid())
    }
}

unsafe impl Send for Closure {}
unsafe impl Sync for Closure {}

#[cfg(test)]
mod tests {
    use std::sync::atomic::{AtomicUsize, Ordering};
    use std::sync::Arc;

    use super::Closure;
    use crate::{ToValue, Value};

    #[allow(clippy::unnecessary_wraps)]
    fn closure_fn(values: &[Value]) -> Option<Value> {
        assert_eq!(values.len(), 2);
        let string_arg = values[0].get::<String>();
        assert_eq!(string_arg, Ok(Some("test".to_string())));
        let int_arg = values[1].get_some::<i32>();
        assert_eq!(int_arg, Ok(42));
        Some(24.to_value())
    }

    #[test]
    fn test_closure() {
        let call_count = Arc::new(AtomicUsize::new(0));

        let count = call_count.clone();
        let closure = Closure::new(move |values| {
            count.fetch_add(1, Ordering::Relaxed);
            assert_eq!(values.len(), 2);
            let string_arg = values[0].get::<String>();
            assert_eq!(string_arg, Ok(Some("test".to_string())));
            let int_arg = values[1].get_some::<i32>();
            assert_eq!(int_arg, Ok(42));
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
        let int_res = result.map(|result| result.get_some::<i32>());
        assert_eq!(int_res, Some(Ok(24)));
    }
}
