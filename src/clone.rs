use std::rc::{self, Rc};
use std::sync::{self, Arc};

/// Trait for generalizing downgrading a strong reference to a weak reference.
pub trait Downgrade
where
    Self: Sized,
{
    /// Weak reference type.
    type Weak;

    /// Downgrade to a weak reference.
    fn downgrade(&self) -> Self::Weak;
}

/// Trait for generalizing upgrading a weak reference to a strong reference.
pub trait Upgrade
where
    Self: Sized,
{
    /// Strong reference type.
    type Strong;

    /// Try upgrading a weak reference to a strong reference.
    fn upgrade(&self) -> Option<Self::Strong>;
}

impl<T: Downgrade + crate::ObjectType> Upgrade for crate::WeakRef<T> {
    type Strong = T;

    fn upgrade(&self) -> Option<Self::Strong> {
        self.upgrade()
    }
}

impl<T> Downgrade for Arc<T> {
    type Weak = sync::Weak<T>;

    fn downgrade(&self) -> Self::Weak {
        Arc::downgrade(self)
    }
}

impl<T> Upgrade for sync::Weak<T> {
    type Strong = Arc<T>;

    fn upgrade(&self) -> Option<Self::Strong> {
        self.upgrade()
    }
}

impl<T> Downgrade for Rc<T> {
    type Weak = rc::Weak<T>;

    fn downgrade(&self) -> Self::Weak {
        Rc::downgrade(self)
    }
}

impl<T> Upgrade for rc::Weak<T> {
    type Strong = Rc<T>;

    fn upgrade(&self) -> Option<Self::Strong> {
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
    (_ , $return_value:expr) => {};
    (self, $return_value:expr) => {};
    (@weak self, $return_value:expr) => {};
    ($variable:ident , $return_value:expr) => {};
    (@weak $variable:ident , $return_value:expr) => {
        let $variable = match $crate::clone::Upgrade::upgrade(&$variable) {
            Some(val) => val,
            None => return ($return_value)(),
        };
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! to_return_value {
    () => {
        ()
    };
    ($value:expr) => {
        $value
    };
}

/// Macro for passing variables  as strong or weak references into a closure.
///
/// This macro can be useful in combination with closures, e.g. signal handlers, to reduce the
/// boilerplate required for passing strong or weak references into the closure. It will
/// automatically create the new reference and pass it with the same name into the closure.
///
/// If upgrading the weak reference to a strong reference inside the closure is failing, the
/// closure is immediately returning an optional default return value. If none is provided, `()` is
/// returned.
///
///  ### Passing a strong reference
/// ```rust
/// use glib::clone;
/// use std::rc::Rc;
///
/// let v = Rc::new(1);
/// let closure = clone!(v => move |x| {
///     println!("v: {}, x: {}", v, x);
/// });
///
/// closure(2);
/// ```
///
///  ### Passing a strong and weak reference
/// ```rust
/// use glib::clone;
/// use std::rc::Rc;
///
/// let v = Rc::new(1);
/// let u = Rc::new(2);
/// let closure = clone!(v, @weak u => move |x| {
///     println!("v: {}, u: {}, x: {}", v, u, x);
/// });
///
/// closure(3);
/// ```
///
///  ### Providing a default return value if upgrading a weak reference fails
/// ```rust
/// use glib::clone;
/// use std::rc::Rc;
///
/// let v = Rc::new(1);
/// let closure = clone!(@weak v => @default-return false, move |x| {
///     println!("v: {}, x: {}", v, x);
///     true
/// });
///
/// // Drop value so that the weak reference can't be upgraded.
/// drop(v);
///
/// assert_eq!(closure(2), false);
/// ```
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
