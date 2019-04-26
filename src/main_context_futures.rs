// Copyright 2018, The Gtk-rs Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

use futures;
use futures::future::{FutureObj, LocalFutureObj};
use futures::prelude::*;
use futures::task::{
    Context, LocalSpawn, Poll, RawWaker, RawWakerVTable, Spawn, SpawnError, Waker,
};
use get_thread_id;
use glib_sys;
use std::mem;
use std::ptr;
use std::sync::atomic::{AtomicUsize, Ordering};
use translate::{from_glib_full, from_glib_none, mut_override, ToGlib};

use MainContext;
use MainLoop;
use Priority;
use Source;

// We can't use an enum here because we want to store this in an atomic variable
const INIT: usize = 0;
const NOT_READY: usize = 1;
const READY: usize = 2;
const DONE: usize = 3;

#[allow(clippy::type_complexity)]
#[repr(C)]
struct TaskSource {
    source: glib_sys::GSource,
    future: Option<FutureObj<'static, ()>>,
    thread: Option<usize>,
    state: AtomicUsize,
}

static TASK_SOURCE_WAKER_VTABLE: RawWakerVTable = RawWakerVTable::new(
    TaskSource::clone_raw,
    TaskSource::wake_raw,
    TaskSource::wake_by_ref_raw,
    TaskSource::drop_raw,
);

impl TaskSource {
    unsafe fn clone_raw(waker: *const ()) -> RawWaker {
        let waker = &*(waker as *const TaskSource);
        glib_sys::g_source_ref(mut_override(&waker.source));
        RawWaker::new(waker as *const Self as *const (), &TASK_SOURCE_WAKER_VTABLE)
    }
    unsafe fn wake_raw(waker: *const ()) {
        Self::wake_by_ref_raw(waker);
        Self::drop_raw(waker);
    }

    unsafe fn wake_by_ref_raw(waker: *const ()) {
        let waker = &*(waker as *const TaskSource);
        if waker
            .state
            .compare_and_swap(NOT_READY, READY, Ordering::SeqCst)
            == NOT_READY
        {
            glib_sys::g_source_set_ready_time(mut_override(&waker.source), 0);
        }
    }

    unsafe fn drop_raw(waker: *const ()) {
        let waker = &*(waker as *const TaskSource);
        glib_sys::g_source_unref(mut_override(&waker.source));
    }

    fn as_waker(&self) -> Waker {
        unsafe { Waker::from_raw(Self::clone_raw(self as *const Self as *const ())) }
    }
}

unsafe extern "C" fn prepare(
    source: *mut glib_sys::GSource,
    timeout: *mut i32,
) -> glib_sys::gboolean {
    let source = &mut *(source as *mut TaskSource);

    *timeout = -1;

    let mut cur = source
        .state
        .compare_and_swap(INIT, NOT_READY, Ordering::SeqCst);
    if cur == INIT {
        // XXX: This is not actually correct, we should not dispatch the
        // GSource here already but we need to know its current status so
        // that if it is not ready yet something can register to the waker
        if let Poll::Ready(()) = source.poll() {
            source.state.store(DONE, Ordering::SeqCst);
            cur = DONE;
        } else {
            cur = NOT_READY;
        }
    }

    if cur == READY || cur == DONE {
        glib_sys::GTRUE
    } else {
        glib_sys::GFALSE
    }
}

unsafe extern "C" fn check(source: *mut glib_sys::GSource) -> glib_sys::gboolean {
    let source = &mut *(source as *mut TaskSource);

    let cur = source.state.load(Ordering::SeqCst);
    if cur == READY || cur == DONE {
        glib_sys::GTRUE
    } else {
        glib_sys::GFALSE
    }
}

unsafe extern "C" fn dispatch(
    source: *mut glib_sys::GSource,
    callback: glib_sys::GSourceFunc,
    _user_data: glib_sys::gpointer,
) -> glib_sys::gboolean {
    let source = &mut *(source as *mut TaskSource);
    assert!(callback.is_none());

    glib_sys::g_source_set_ready_time(mut_override(&source.source), -1);
    let mut cur = source
        .state
        .compare_and_swap(READY, NOT_READY, Ordering::SeqCst);
    if cur == READY {
        if let Poll::Ready(()) = source.poll() {
            source.state.store(DONE, Ordering::SeqCst);
            cur = DONE;
        } else {
            cur = NOT_READY;
        }
    }

    if cur == DONE {
        glib_sys::G_SOURCE_REMOVE
    } else {
        glib_sys::G_SOURCE_CONTINUE
    }
}

unsafe extern "C" fn finalize(source: *mut glib_sys::GSource) {
    let source = source as *mut TaskSource;
    let _ = (*source).future.take();
}

static SOURCE_FUNCS: glib_sys::GSourceFuncs = glib_sys::GSourceFuncs {
    check: Some(check),
    prepare: Some(prepare),
    dispatch: Some(dispatch),
    finalize: Some(finalize),
    closure_callback: None,
    closure_marshal: None,
};

unsafe impl Send for TaskSource {}
unsafe impl Sync for TaskSource {}

impl TaskSource {
    #[allow(clippy::new_ret_no_self)]
    fn new(priority: Priority, thread: Option<usize>, future: FutureObj<'static, ()>) -> Source {
        unsafe {
            let source = glib_sys::g_source_new(
                mut_override(&SOURCE_FUNCS),
                mem::size_of::<TaskSource>() as u32,
            );
            {
                let source = &mut *(source as *mut TaskSource);
                ptr::write(&mut source.future, Some(future));
                source.thread = thread;
                source.state = AtomicUsize::new(INIT);
            }

            glib_sys::g_source_set_priority(source, priority.to_glib());

            from_glib_full(source)
        }
    }

    fn poll(&mut self) -> Poll<()> {
        // Make sure that the first time we're polled that the current thread is remembered
        // and from there one we ensure that we're always polled from exactly the same thread.
        //
        // In theory a GMainContext can be first run from one thread and later from another
        // thread, but we allow spawning non-Send futures and must not ever use them from
        // any other thread.
        match &mut self.thread {
            thread @ &mut None => {
                *thread = Some(get_thread_id());
            }
            &mut Some(thread_id) => {
                assert_eq!(
                    get_thread_id(),
                    thread_id,
                    "Task polled on a different thread than before"
                );
            }
        }

        let waker = self.as_waker();
        let source = &self.source as *const _;
        if let Some(ref mut future) = self.future {
            let mut executor: MainContext =
                unsafe { from_glib_none(glib_sys::g_source_get_context(mut_override(source))) };

            assert!(
                executor.is_owner(),
                "Polling futures only allowed if the thread is owning the MainContext"
            );

            // Clone that we store in the task local data so that
            // it can be retrieved as needed
            executor.push_thread_default();

            let res = {
                let enter = futures::executor::enter().unwrap();
                let mut context = Context::from_waker(&waker);

                let res = future.poll_unpin(&mut context);

                drop(enter);

                res
            };

            executor.pop_thread_default();
            res
        } else {
            Poll::Ready(())
        }
    }
}

impl MainContext {
    /// Spawn a new infallible `Future` on the main context.
    ///
    /// This can be called from any thread and will execute the future from the thread
    /// where main context is running, e.g. via a `MainLoop`.
    pub fn spawn<F: Future<Output = ()> + Send + 'static>(&self, f: F) {
        self.spawn_with_priority(::PRIORITY_DEFAULT, f);
    }

    /// Spawn a new infallible `Future` on the main context.
    ///
    /// The given `Future` does not have to be `Send`.
    ///
    /// This can be called only from the thread where the main context is running, e.g.
    /// from any other `Future` that is executed on this main context, or after calling
    /// `push_thread_default` or `acquire` on the main context.
    pub fn spawn_local<F: Future<Output = ()> + 'static>(&self, f: F) {
        self.spawn_local_with_priority(::PRIORITY_DEFAULT, f);
    }

    /// Spawn a new infallible `Future` on the main context, with a non-default priority.
    ///
    /// This can be called from any thread and will execute the future from the thread
    /// where main context is running, e.g. via a `MainLoop`.
    pub fn spawn_with_priority<F: Future<Output = ()> + Send + 'static>(
        &self,
        priority: Priority,
        f: F,
    ) {
        let f = FutureObj::new(Box::new(f));
        let source = TaskSource::new(priority, None, f);
        source.attach(Some(&*self));
    }

    /// Spawn a new infallible `Future` on the main context, with a non-default priority.
    ///
    /// The given `Future` does not have to be `Send`.
    ///
    /// This can be called only from the thread where the main context is running, e.g.
    /// from any other `Future` that is executed on this main context, or after calling
    /// `push_thread_default` or `acquire` on the main context.
    pub fn spawn_local_with_priority<F: Future<Output = ()> + 'static>(
        &self,
        priority: Priority,
        f: F,
    ) {
        assert!(
            self.is_owner(),
            "Spawning local futures only allowed on the thread owning the MainContext"
        );
        unsafe {
            let f = LocalFutureObj::new(Box::new(f));
            // We ensure here that we only ever run the future on this very task
            // and that the futures executor is running on this task. Otherwise
            // we will panic later.
            // As such we can add the Send impl here safely
            let f = f.into_future_obj();
            let source = TaskSource::new(priority, Some(get_thread_id()), f);
            source.attach(Some(&*self));
        }
    }

    /// Runs a new, infallible `Future` on the main context and block until it finished, returning
    /// the result of the `Future`.
    ///
    /// The given `Future` does not have to be `Send` or `'static`.
    ///
    /// This must only be called if no `MainLoop` or anything else is running on this specific main
    /// context.
    #[allow(clippy::transmute_ptr_to_ptr)]
    pub fn block_on<F: Future>(&self, f: F) -> F::Output {
        let mut res = None;
        let l = MainLoop::new(Some(&*self), false);
        let l_clone = l.clone();

        unsafe {
            let f = f.then(|r| {
                res = Some(r);
                l_clone.quit();
                future::ready(())
            });

            // Super-unsafe: We transmute here to get rid of the 'static lifetime
            let f = LocalFutureObj::new(Box::new(f));
            let f: (LocalFutureObj<'static, ()>) = mem::transmute(f);

            // And ensure that we are only ever running on this very thread.
            let f = f.into_future_obj();

            let source = TaskSource::new(::PRIORITY_DEFAULT, Some(get_thread_id()), f);
            source.attach(Some(&*self));
        }

        l.run();

        res.unwrap()
    }
}

impl Spawn for MainContext {
    fn spawn_obj(&mut self, f: FutureObj<'static, ()>) -> Result<(), SpawnError> {
        let source = TaskSource::new(::PRIORITY_DEFAULT, None, f);
        source.attach(Some(&*self));
        Ok(())
    }
}

impl LocalSpawn for MainContext {
    fn spawn_local_obj(&mut self, f: LocalFutureObj<'static, ()>) -> Result<(), SpawnError> {
        let source = TaskSource::new(::PRIORITY_DEFAULT, Some(get_thread_id()), unsafe {
            f.into_future_obj()
        });
        source.attach(Some(&*self));
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use futures::channel::oneshot;
    use std::sync::mpsc;
    use std::thread;

    #[test]
    fn test_spawn() {
        let c = MainContext::new();
        let l = ::MainLoop::new(Some(&c), false);

        let (sender, receiver) = mpsc::channel();
        let (o_sender, o_receiver) = oneshot::channel();

        let l_clone = l.clone();
        c.spawn(
            o_receiver
                .and_then(move |()| {
                    sender.send(()).unwrap();
                    l_clone.quit();

                    future::ok(())
                })
                .then(|res| future::ready(res.unwrap())),
        );

        thread::spawn(move || {
            l.run();
        });

        o_sender.send(()).unwrap();

        let _ = receiver.recv().unwrap();
    }

    #[test]
    fn test_spawn_local() {
        let c = MainContext::new();
        let l = ::MainLoop::new(Some(&c), false);

        c.push_thread_default();
        let l_clone = l.clone();
        c.spawn_local(future::lazy(move |_ctx| {
            l_clone.quit();
        }));

        l.run();

        c.pop_thread_default();
    }

    #[test]
    fn test_block_on() {
        let c = MainContext::new();

        let mut v = None;
        {
            let v = &mut v;

            let future = future::lazy(|_ctx| {
                *v = Some(123);
                Ok::<i32, ()>(123)
            });

            let res = c.block_on(future);
            assert_eq!(res, Ok(123));
        }

        assert_eq!(v, Some(123));
    }
}
