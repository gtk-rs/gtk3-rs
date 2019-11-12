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

#[doc(hidden)]
#[macro_export]
macro_rules! to_type_before {
    (_) => ();
    (self) => (
        compile_error!("Can't use `self` as variable name for the `clone!` macro. Try storing it in a temporary variable.");
    );
    (@weak self) => (
        compile_error!("Can't use `self` as variable name for the `clone!` macro. Try storing it in a temporary variable.");
    );
    ($variable:ident) => (
        let $variable = $variable.clone();
    );
    (@weak $variable:ident) => (
        let $variable = $crate::clone::Downgrade::downgrade(&$variable);
    );
}

#[doc(hidden)]
#[macro_export]
macro_rules! to_type_after {
    (_ , $return_value:expr) => ();
    (self, $return_value:expr) => ();
    (@weak self, $return_value:expr) => ();
    ($variable:ident , $return_value:expr) => ();
    (@weak $variable:ident , $return_value:expr) => (
        let $variable = match $crate::clone::Upgrade::upgrade(&$variable) {
            Some(val) => val,
            None => return ($return_value)(),
        };
    );
}

#[doc(hidden)]
#[macro_export]
macro_rules! to_return_value {
    () => (());
    ($value:expr) => ( $value );
}

#[macro_export]
macro_rules! clone {
    ($($(@ $weak:ident)? $variables:ident),+ => $(@default-return $return_value:expr,)? move || $body:block ) => (
        {
            $( $crate::to_type_before!($(@ $weak)? $variables); )*
            move || {
                let return_value = || $crate::to_return_value!($($return_value)?);
                $( $crate::to_type_after!($(@ $weak)? $variables, return_value );)*
                $body
            }
        }
    );
    ($($(@ $weak:ident)? $variables:ident),+ => $(@default-return $return_value:expr ,)? move | $($pattern:pat),* | $body:block ) => (
        {
            $( $crate::to_type_before!($(@ $weak)? $variables); )*
            move |$($pattern),*| {
                let return_value = || $crate::to_return_value!($($return_value)?);
                $( $crate::to_type_after!($(@ $weak)? $variables, return_value );)*
                $body
            }
        }
    );
}
