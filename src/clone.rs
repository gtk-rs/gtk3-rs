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
    (@strong self) => (
        compile_error!("Can't use `self` as variable name for the `clone!` macro. Try storing it in a temporary variable.");
    );
    ($variable:ident) => (
        compile_error!("You need to specify if this is a weak or a strong clone.");
    );
    (@strong $variable:ident) => (
        let $variable = $variable.clone();
    );
    (@weak $variable:ident) => (
        let $variable = $crate::clone::Downgrade::downgrade(&$variable);
    );
}

#[doc(hidden)]
#[macro_export]
macro_rules! to_type_after {
    (@weak self, $return_value:expr) => {};
    (@strong self, $return_value:expr) => {};
    (@weak $variable:ident , $return_value:expr) => {
        let $variable = match $crate::clone::Upgrade::upgrade(&$variable) {
            Some(val) => val,
            None => return ($return_value)(),
        };
    };
    (@strong $variable:ident , $return_value:expr) => {};
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

/// Macro for passing variables as strong or weak references into a closure.
///
/// This macro can be useful in combination with closures, e.g. signal handlers, to reduce the
/// boilerplate required for passing strong or weak references into the closure. It will
/// automatically create the new reference and pass it with the same name into the closure.
///
/// If upgrading the weak reference to a strong reference inside the closure is failing, the
/// closure is immediately returning an optional default return value. If none is provided, `()` is
/// returned.
///
/// ### Passing a strong reference
///
/// ```
/// use glib::clone;
/// use std::rc::Rc;
///
/// let v = Rc::new(1);
/// let closure = clone!(@strong v => move |x| {
///     println!("v: {}, x: {}", v, x);
/// });
///
/// closure(2);
/// ```
///
/// ### Passing a strong and weak reference
///
/// ```
/// use glib::clone;
/// use std::rc::Rc;
///
/// let v = Rc::new(1);
/// let u = Rc::new(2);
/// let closure = clone!(@strong v, @weak u => move |x| {
///     println!("v: {}, u: {}, x: {}", v, u, x);
/// });
///
/// closure(3);
/// ```
///
/// ### Providing a default return value if upgrading a weak reference fails
///
/// You can do it in two different ways:
///
/// Either by providing the value yourself using `@default-return`:
///
/// ```
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
///
/// Or by using `@default-panic` (if the value fails to get upgraded, it'll panic):
///
/// ```run_fail
/// # use glib::clone;
/// # use std::rc::Rc;
/// # let v = Rc::new(1);
/// let closure = clone!(@weak v => @default-panic, move |x| {
///     println!("v: {}, x: {}", v, x);
///     true
/// });
/// # drop(v);
/// # assert_eq!(closure(2), false);
/// ```
///
/// ### Errors
///
/// Here is a list of errors you might encounter:
///
/// **Missing `@weak` or `@strong`**:
///
/// ```compile_fail
/// # use glib::clone;
/// # use std::rc::Rc;
/// let v = Rc::new(1);
///
/// let closure = clone!(v => move |x| {
///     println!("v: {}, x: {}", v, x);
/// });
/// ```
///
/// **Passing `self` as an argument**:
///
/// ```compile_fail
/// # use glib::clone;
/// # use std::rc::Rc;
/// struct Foo;
///
/// impl Foo {
///     fn foo(&self) {
///         let v = Rc::new(1);
///
///         let closure = clone!(self => move |x| {
///             println!("v: {}, x: {}", v, x);
///         });
///     }
/// }
/// ```
#[macro_export]
macro_rules! clone {
    ($($(@ $strength:ident)? $variables:ident),+ => $(@default-panic,)? move || $body:block ) => (
        {
            $( $crate::to_type_before!($(@ $strength)? $variables); )*
            move || {
                let return_value = || $crate::to_return_value!(panic!("Failed to upgrade weak reference"));
                $( $crate::to_type_after!($(@ $strength)? $variables, return_value );)*
                $body
            }
        }
    );
    ($($(@ $strength:ident)? $variables:ident),+ => $(@default-return $return_value:expr,)? move || $body:block ) => (
        {
            $( $crate::to_type_before!($(@ $strength)? $variables); )*
            move || {
                let return_value = || $crate::to_return_value!($($return_value)?);
                $( $crate::to_type_after!($(@ $strength)? $variables, return_value );)*
                $body
            }
        }
    );
    ($($(@ $strength:ident)? $variables:ident),+ => $(@default-panic,)? move | $($pattern:pat),* | $body:block ) => (
        {
            $( $crate::to_type_before!($(@ $strength)? $variables); )*
            move |$($pattern),*| {
                let return_value = || $crate::to_return_value!(panic!("Failed to upgrade weak reference"));
                $( $crate::to_type_after!($(@ $strength)? $variables, return_value );)*
                $body
            }
        }
    );
    ($($(@ $strength:ident)? $variables:ident),+ => $(@default-return $return_value:expr,)? move | $($pattern:pat),* | $body:block ) => (
        {
            $( $crate::to_type_before!($(@ $strength)? $variables); )*
            move |$($pattern),*| {
                let return_value = || $crate::to_return_value!($($return_value)?);
                $( $crate::to_type_after!($(@ $strength)? $variables, return_value );)*
                $body
            }
        }
    );
}
