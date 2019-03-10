// Copyright 2018, The Gtk-rs Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

use std::cell::RefCell;
use std::ops;

/// Like `Send` but only if we have the unique reference to the object
///
/// Note that implementing this trait has to be done especially careful.
/// It must only be implemented on types where the uniqueness of a reference
/// can be determined, i.e. the reference count field is accessible and it
/// must only have references itself to other types that are `Send`.
/// `SendUnique` is *not* enough for the other types unless uniqueness of
/// all of them can be guaranteed, which is e.g. not the case if there's a
/// getter for them.
pub unsafe trait SendUnique: 'static {
    fn is_unique(&self) -> bool;
}

/// Allows sending reference counted objects that don't implement `Send` to other threads
/// as long as only a single reference to the object exists.
#[derive(Debug)]
pub struct SendUniqueCell<T: SendUnique> {
    obj: T,
    // Thread id and refcount
    thread: RefCell<Option<(usize, usize)>>,
}

unsafe impl<T: SendUnique> Send for SendUniqueCell<T> {}

#[derive(Debug)]
pub struct BorrowError;

impl<T: SendUnique> SendUniqueCell<T> {
    /// Create a new `SendUniqueCell` out of `obj`
    ///
    /// Fails if `obj` is not unique at this time
    pub fn new(obj: T) -> Result<Self, T> {
        if !obj.is_unique() {
            return Err(obj);
        }

        Ok(SendUniqueCell {
            obj,
            thread: RefCell::new(None),
        })
    }

    /// Borrow the contained object or panic if borrowing
    /// is not possible at this time
    pub fn borrow(&self) -> Ref<T> {
        #[allow(clippy::match_wild_err_arm)]
        match self.try_borrow() {
            Err(_) => panic!("Can't borrow"),
            Ok(r) => r,
        }
    }

    /// Try borrowing the contained object
    ///
    /// Borrowing is possible as long as only a single reference
    /// to the object exists, or it is borrowed from the same
    /// thread currently
    pub fn try_borrow(&self) -> Result<Ref<T>, BorrowError> {
        let mut thread = self.thread.borrow_mut();

        // If the object is unique, we can borrow it from
        // any thread we want and just have to keep track
        // how often we borrowed it
        if self.obj.is_unique() {
            if *thread == None {
                *thread = Some((::get_thread_id(), 1));
            } else {
                thread.as_mut().unwrap().1 += 1;
            }

            return Ok(Ref(self));
        }

        // If we don't even know from which thread it is borrowed, this
        // means it somehow got borrowed from outside the SendUniqueCell
        if *thread == None {
            return Err(BorrowError);
        }

        // If the object is not unique, we can only borrow it
        // from the thread that currently has it borrowed
        if thread.as_ref().unwrap().0 != ::get_thread_id() {
            return Err(BorrowError);
        }

        thread.as_mut().unwrap().1 += 1;

        Ok(Ref(self))
    }

    /// Extract the contained object or panic if it is not possible
    /// at this time
    pub fn into_inner(self) -> T {
        #[allow(clippy::match_wild_err_arm)]
        match self.try_into_inner() {
            Err(_) => panic!("Can't convert into inner type"),
            Ok(obj) => obj,
        }
    }

    /// Try extracing the contained object
    ///
    /// Borrowing is possible as long as only a single reference
    /// to the object exists, or it is borrowed from the same
    /// thread currently
    pub fn try_into_inner(self) -> Result<T, Self> {
        if self.try_borrow().is_err() {
            Err(self)
        } else {
            Ok(self.obj)
        }
    }
}

pub struct Ref<'a, T: SendUnique>(&'a SendUniqueCell<T>);

impl<'a, T: SendUnique> AsRef<T> for Ref<'a, T> {
    fn as_ref(&self) -> &T {
        &self.0.obj
    }
}

impl<'a, T: SendUnique> ops::Deref for Ref<'a, T> {
    type Target = T;

    fn deref(&self) -> &T {
        &self.0.obj
    }
}

impl<'a, T: SendUnique> Drop for Ref<'a, T> {
    fn drop(&mut self) {
        let is_unique = self.0.obj.is_unique();
        let mut thread = self.0.thread.borrow_mut();

        if is_unique && thread.as_ref().unwrap().1 == 1 {
            *thread = None;
        } else {
            thread.as_mut().unwrap().1 -= 1;
        }
    }
}
