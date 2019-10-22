use std::rc::{self, Rc};
use std::sync::{self, Arc};

pub trait Downgrade {
    type Target;

    fn downgrade(&self) -> Self::Target;
}

impl<T> Downgrade for Arc<T> {
    type Target = sync::Weak<T>;

    fn downgrade(&self) -> Self::Target {
        Arc::downgrade(self)
    }
}

impl<T> Downgrade for Rc<T> {
    type Target = rc::Weak<T>;

    fn downgrade(&self) -> Self::Target {
        Rc::downgrade(self)
    }
}

pub trait Upgrade {
    type Target;

    fn upgrade(&self) -> Option<Self::Target>;
}

impl<T> Upgrade for sync::Weak<T> {
    type Target = Arc<T>;

    fn upgrade(&self) -> Option<Self::Target> {
        self.upgrade()
    }
}

impl<T> Upgrade for rc::Weak<T> {
    type Target = Rc<T>;

    fn upgrade(&self) -> Option<Self::Target> {
        self.upgrade()
    }
}

#[macro_export]
macro_rules! clone {
    ($($n:ident),+ => move |$($p:pat),*| $body:expr) => (
        {
            $( let $n = $crate::clone::Downgrade::downgrade(&$n); )+
            move |$($p,)*| {
                $(let $n = match $crate::clone::Upgrade::upgrade(&$n) {
                    Some(val) => val,
                    None => panic!("cannot upgrade weak reference `{}`", stringify!($n)),
                };)+
                $body
            }
        }
    );
}
