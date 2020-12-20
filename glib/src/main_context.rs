// Take a look at the license at the top of the repository in the LICENSE file.

use crate::source::Priority;
use crate::translate::*;
use crate::MainContext;
use crate::Source;
use crate::SourceId;
use ffi::{self, gboolean, gpointer};
use std::mem;

impl MainContext {
    #[doc(alias = "g_main_context_prepare")]
    pub fn prepare(&self) -> (bool, i32) {
        unsafe {
            let mut priority = mem::MaybeUninit::uninit();

            let res = from_glib(ffi::g_main_context_prepare(
                self.to_glib_none().0,
                priority.as_mut_ptr(),
            ));
            let priority = priority.assume_init();
            (res, priority)
        }
    }

    #[doc(alias = "g_main_context_find_source_by_id")]
    pub fn find_source_by_id(&self, source_id: &SourceId) -> Option<Source> {
        unsafe {
            from_glib_none(ffi::g_main_context_find_source_by_id(
                self.to_glib_none().0,
                source_id.to_glib(),
            ))
        }
    }

    /// Invokes `func` on the main context.
    pub fn invoke<F>(&self, func: F)
    where
        F: FnOnce() + Send + 'static,
    {
        self.invoke_with_priority(crate::PRIORITY_DEFAULT_IDLE, func);
    }

    /// Invokes `func` on the main context with the given priority.
    pub fn invoke_with_priority<F>(&self, priority: Priority, func: F)
    where
        F: FnOnce() + Send + 'static,
    {
        unsafe {
            self.invoke_unsafe(priority, func);
        }
    }

    /// Invokes `func` on the main context.
    ///
    /// Different to `invoke()`, this does not require `func` to be
    /// `Send` but can only be called from the thread that owns the main context.
    ///
    /// This function panics if called from a different thread than the one that
    /// owns the main context.
    pub fn invoke_local<F>(&self, func: F)
    where
        F: FnOnce() + 'static,
    {
        self.invoke_local_with_priority(crate::PRIORITY_DEFAULT_IDLE, func);
    }

    /// Invokes `func` on the main context with the given priority.
    ///
    /// Different to `invoke_with_priority()`, this does not require `func` to be
    /// `Send` but can only be called from the thread that owns the main context.
    ///
    /// This function panics if called from a different thread than the one that
    /// owns the main context.
    pub fn invoke_local_with_priority<F>(&self, priority: Priority, func: F)
    where
        F: FnOnce() + 'static,
    {
        unsafe {
            assert!(self.is_owner());
            self.invoke_unsafe(priority, func);
        }
    }

    #[doc(alias = "g_main_context_invoke_full")]
    unsafe fn invoke_unsafe<F>(&self, priority: Priority, func: F)
    where
        F: FnOnce() + 'static,
    {
        unsafe extern "C" fn trampoline<F: FnOnce() + 'static>(func: gpointer) -> gboolean {
            let func: &mut Option<F> = &mut *(func as *mut Option<F>);
            let func = func
                .take()
                .expect("MainContext::invoke() closure called multiple times");
            func();
            ffi::G_SOURCE_REMOVE
        }
        unsafe extern "C" fn destroy_closure<F: FnOnce() + 'static>(ptr: gpointer) {
            Box::<Option<F>>::from_raw(ptr as *mut _);
        }
        let func = Box::into_raw(Box::new(Some(func)));
        ffi::g_main_context_invoke_full(
            self.to_glib_none().0,
            priority.to_glib(),
            Some(trampoline::<F>),
            func as gpointer,
            Some(destroy_closure::<F>),
        )
    }

    /// Calls closure with context configured as the thread default one.
    ///
    /// Thread default context is changed in panic-safe manner by calling
    /// [`push_thread_default`][push_thread_default] before calling closure
    /// and [`pop_thread_default`][pop_thread_default] afterwards regardless
    /// of whether closure panicked or not.
    ///
    /// [push_thread_default]: struct.MainContext.html#method.push_thread_default
    /// [pop_thread_default]: struct.MainContext.html#method.pop_thread_default
    pub fn with_thread_default<R, F: Sized>(&self, func: F) -> R
    where
        F: FnOnce() -> R,
    {
        let _thread_default = ThreadDefaultContext::new(self);
        func()
    }
}

struct ThreadDefaultContext<'a>(&'a MainContext);

impl<'a> ThreadDefaultContext<'a> {
    fn new(ctx: &MainContext) -> ThreadDefaultContext {
        ctx.push_thread_default();
        ThreadDefaultContext(ctx)
    }
}

impl<'a> Drop for ThreadDefaultContext<'a> {
    fn drop(&mut self) {
        self.0.pop_thread_default();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::panic;
    use std::ptr;
    use std::thread;

    #[test]
    fn test_invoke() {
        let c = MainContext::new();
        let l = crate::MainLoop::new(Some(&c), false);

        let l_clone = l.clone();
        thread::spawn(move || {
            c.invoke(move || l_clone.quit());
        });

        l.run();
    }

    fn is_same_context(a: &MainContext, b: &MainContext) -> bool {
        ptr::eq(a.to_glib_none().0, b.to_glib_none().0)
    }

    #[test]
    fn test_with_thread_default() {
        let a = MainContext::new();
        let b = MainContext::new();

        assert!(!is_same_context(&a, &b));

        a.with_thread_default(|| {
            let t = MainContext::get_thread_default().unwrap();
            assert!(is_same_context(&a, &t));

            b.with_thread_default(|| {
                let t = MainContext::get_thread_default().unwrap();
                assert!(is_same_context(&b, &t));
            });

            let t = MainContext::get_thread_default().unwrap();
            assert!(is_same_context(&a, &t));
        });
    }

    #[test]
    fn test_with_thread_default_is_panic_safe() {
        let a = MainContext::new();
        let b = MainContext::new();

        assert!(!is_same_context(&a, &b));

        a.with_thread_default(|| {
            let t = MainContext::get_thread_default().unwrap();
            assert!(is_same_context(&a, &t));

            let result = panic::catch_unwind(|| {
                b.with_thread_default(|| {
                    panic!();
                });
            });
            assert!(result.is_err());

            let t = MainContext::get_thread_default().unwrap();
            assert!(is_same_context(&a, &t));
        });
    }
}
