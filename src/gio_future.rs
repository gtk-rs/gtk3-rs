// Copyright 2018, The Gtk-rs Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

use futures::prelude::*;
use futures::channel::oneshot;
use futures::task::{Poll, Context};
use std::pin;
use std::marker::Unpin;

use glib;
use Cancellable;
use CancellableExt;

pub struct GioFuture<F, O, T, E> {
    obj: O,
    schedule_operation: Option<F>,
    cancellable: Option<(Cancellable, oneshot::Receiver<Result<T, E>>)>,
}

impl<F, O, T: 'static, E: 'static> GioFuture<F, O, T, E>
where
    O: Clone + 'static,
    F: FnOnce(&O, oneshot::Sender<Result<T, E>>) -> Cancellable + 'static,
{
    pub fn new(obj: &O, schedule_operation: F) -> Box<Future<Output = Result<T, E>> + Unpin> {
        Box::new(GioFuture {
            obj: obj.clone(),
            schedule_operation: Some(schedule_operation),
            cancellable: None,
        })
    }
}

impl<F, O, T, E> Future for GioFuture<F, O, T, E>
where
    O: Clone + 'static,
    F: FnOnce(&O, oneshot::Sender<Result<T, E>>) -> Cancellable + 'static,
{
    type Output = Result<T, E>;

    fn poll(mut self: pin::Pin<&mut Self>, ctx: &mut Context) -> Poll<Result<T, E>> {
        let GioFuture {
            ref obj,
            ref mut schedule_operation,
            ref mut cancellable,
            ..
        } = *self;

        if let Some(schedule_operation) = schedule_operation.take() {
            let main_context = glib::MainContext::ref_thread_default();
            assert!(main_context.is_owner(), "Spawning futures only allowed if the thread is owning the MainContext");

            // Channel for sending back the GIO async operation
            // result to our future here.
            //
            // In theory we could directly continue polling the
            // corresponding task from the GIO async operation
            // callback, however this would break at the very
            // least the g_main_current_source() API.
            let (send, recv) = oneshot::channel();

            let c = schedule_operation(obj, send);

            *cancellable = Some((c, recv));
        }

        // At this point we must have a receiver
        let res = {
            let &mut (_, ref mut receiver) = cancellable.as_mut().unwrap();
            receiver.poll_unpin(ctx)
        };

        match res {
            Poll::Pending => Poll::Pending,
            Poll::Ready(Err(_)) => panic!("Async operation sender was unexpectedly closed"),
            Poll::Ready(Ok(v)) => {
                // Get rid of the reference to the cancellable
                let _ = cancellable.take();
                Poll::Ready(v)
            }
        }
    }
}

impl<F, O, T, E> Drop for GioFuture<F, O, T, E> {
    fn drop(&mut self) {
        if let Some((cancellable, _)) = self.cancellable.take() {
            cancellable.cancel();
        }
    }
}

impl<F, O, T, E> Unpin for GioFuture<F, O, T, E> { }
