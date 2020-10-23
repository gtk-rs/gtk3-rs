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

impl<T: Downgrade> Downgrade for &T {
    type Weak = T::Weak;

    fn downgrade(&self) -> Self::Weak {
        T::downgrade(*self)
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
    ($($variable:ident).+ $(as $rename:ident)?) => (
        // In case we have:
        // clone!(v => move || {});
        compile_error!("You need to specify if this is a weak or a strong clone.");
    );
    (@strong $variable:ident) => (
        let $variable = $variable.clone();
    );
    (@weak $variable:ident) => (
        let $variable = $crate::clone::Downgrade::downgrade(&$variable);
    );
    (@weak-allow-none $variable:ident) => (
        let $variable = $crate::clone::Downgrade::downgrade(&$variable);
    );
    (@strong $($variable:ident).+ as $rename:ident) => (
        let $rename = $($variable).+.clone();
    );
    (@weak $($variable:ident).+ as $rename:ident) => (
        let $rename = $crate::clone::Downgrade::downgrade(&$($variable).+);
    );
    // The three following cases are just here so "@strong", "@weak-allow-none" and "@weak" aren't
    // detected as invalid when passing an expression (like "@default-return" => "-return" is the
    // start of an expression there).
    (@weak-allow-none $variable:expr) => (
        let $variable = $crate::clone::Downgrade::downgrade(&$variable);
    );
    (@strong $variable:expr) => (
        let $variable = $variable.clone();
    );
    (@weak $variable:expr) => (
        let $variable = $crate::clone::Downgrade::downgrade(&$variable);
    );
    (@ $keyword:ident $($variable:tt)+) => (
        // In case we have:
        // clone!(@yolo v => move || {});
        compile_error!(
            concat!(
                "Unknown keyword \"",
                stringify!($keyword),
                "\", only `weak`, `weak-allow-none` and `strong` are allowed",
            ),
        );
    );
}

#[doc(hidden)]
#[macro_export]
macro_rules! to_type_after {
    (@default-panic, @weak-allow-none $variable:ident) => {
        let $variable = $crate::clone::Upgrade::upgrade(&$variable);
    };
    (as $rename:ident @default-panic, @weak-allow-none $($variable:ident).+) => {
        let $rename = $crate::clone::Upgrade::upgrade(&$rename);
    };
    (@default-panic, @weak $variable:ident) => {
        let $variable = match $crate::clone::Upgrade::upgrade(&$variable) {
            Some(val) => val,
            None => panic!(
                "failed to upgrade `{}` (if you don't want to panic, use @default-return)",
                stringify!($variable)
            ),
        };
    };
    (as $rename:ident @default-panic, @weak $($variable:ident).+) => {
        let $rename = match $crate::clone::Upgrade::upgrade(&$rename) {
            Some(val) => val,
            None => panic!(
                "failed to upgrade `{}` (if you don't want to panic, use @default-return)",
                stringify!($rename)
            ),
        };
    };
    ($(as $rename:ident)? @default-panic, @strong $($variable:ident).+) => {};
    (@weak-allow-none $variable:ident , $return_value:expr) => {
        let $variable = $crate::clone::Upgrade::upgrade(&$variable);
    };
    (as $rename:ident @weak-allow-none $($variable:ident).+ , $return_value:expr) => {
        let $rename = $crate::clone::Upgrade::upgrade(&$rename);
    };
    (@weak $variable:ident , $return_value:expr) => {
        let $variable = match $crate::clone::Upgrade::upgrade(&$variable) {
            Some(val) => val,
            None => {
                $crate::g_debug!(
                    $crate::CLONE_MACRO_LOG_DOMAIN,
                    "Failed to upgrade {}",
                    stringify!($variable)
                );
                return ($return_value)();
            }
        };
    };
    (as $rename:ident @weak $($variable:ident).+ , $return_value:expr) => {
        let $rename = match $crate::clone::Upgrade::upgrade(&$rename) {
            Some(val) => val,
            None => {
                $crate::g_debug!(
                    $crate::CLONE_MACRO_LOG_DOMAIN,
                    "Failed to upgrade {} {}",
                    stringify!($rename),
                    "yolo",
                );
                return ($return_value)();
            }
        };
    };
    ($(as $rename:ident)? @strong $($variable:ident).+ , $return_value:expr) => {};
    ($(as $rename:ident)? @ $keyword:ident $($variable:ident).+, $return_value:expr) => {};
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
/// ### Debugging
///
/// In case something goes wrong inside the `clone!` macro, we use the [`g_debug`] macro. Meaning
/// that if you want to see these debug messages, you'll have to set the `G_MESSAGES_DEBUG`
/// environment variable when running your code (either in the code directly or when running the
/// binary) to either "all" or [`CLONE_MACRO_LOG_DOMAIN`][crate::CLONE_MACRO_LOG_DOMAIN]:
///
/// ```rust
/// use glib::CLONE_MACRO_LOG_DOMAIN;
///
/// ::std::env::set_var("G_MESSAGES_DEBUG", CLONE_MACRO_LOG_DOMAIN);
/// ::std::env::set_var("G_MESSAGES_DEBUG", "all");
/// ```
///
/// Or:
///
/// ```bash
/// $ G_MESSAGES_DEBUG=all ./binary
/// ```
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
/// ### Passing a weak reference
///
/// ```
/// use glib::clone;
/// use std::rc::Rc;
///
/// let u = Rc::new(2);
/// let closure = clone!(@weak u => move |x| {
///     println!("u: {}, x: {}", u, x);
/// });
///
/// closure(3);
/// ```
///
/// #### Allowing a nullable weak reference
///
/// In some cases, even if the weak references can't be retrieved, you might want to still have
/// your closure called. In this case, you need to use `@weak-allow-none`:
///
/// ```
/// use glib::clone;
/// use std::rc::Rc;
///
/// let closure = {
///     // This `Rc` won't be available in the closure because it's dropped at the end of the
///     // current block
///     let u = Rc::new(2);
///     clone!(@weak-allow-none u => @default-return false, move |x| {
///         // We need to use a Debug print for `u` because it'll be an `Option`.
///         println!("u: {:?}, x: {}", u, x);
///         true
///     })
/// };
///
/// assert_eq!(closure(3), true);
/// ```
///
/// ### Renaming variables
///
/// ```
/// use glib::clone;
/// use std::rc::Rc;
///
/// let v = Rc::new(1);
/// let u = Rc::new(2);
/// let closure = clone!(@strong v as y, @weak u => move |x| {
///     println!("v as y: {}, u: {}, x: {}", y, u, x);
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
/// let closure = clone!(v => move |x| println!("v: {}, x: {}", v, x));
/// # drop(v);
/// # closure(2);
/// ```
///
/// **Passing `self` as an argument**:
///
/// ```compile_fail
/// # use glib::clone;
/// # use std::rc::Rc;
/// #[derive(Debug)]
/// struct Foo;
///
/// impl Foo {
///     fn foo(&self) {
///         let closure = clone!(@strong self => move |x| {
///             println!("self: {:?}", self);
///         });
///         # closure(2);
///     }
/// }
/// ```
///
/// If you want to use `self` directly, you'll need to rename it:
///
/// ```
/// # use glib::clone;
/// # use std::rc::Rc;
/// #[derive(Debug)]
/// struct Foo;
///
/// impl Foo {
///     fn foo(&self) {
///         let closure = clone!(@strong self as this => move |x| {
///             println!("self: {:?}", this);
///         });
///         # closure(2);
///     }
/// }
/// ```
///
/// **Passing fields directly**
///
/// ```compile_fail
/// # use glib::clone;
/// # use std::rc::Rc;
/// #[derive(Debug)]
/// struct Foo {
///     v: Rc<usize>,
/// }
///
/// impl Foo {
///     fn foo(&self) {
///         let closure = clone!(@strong self.v => move |x| {
///             println!("self.v: {:?}", v);
///         });
///         # closure(2);
///     }
/// }
/// ```
///
/// You can do it by renaming it:
///
/// ```
/// # use glib::clone;
/// # use std::rc::Rc;
/// # struct Foo {
/// #     v: Rc<usize>,
/// # }
/// impl Foo {
///     fn foo(&self) {
///         let closure = clone!(@strong self.v as v => move |x| {
///             println!("self.v: {}", v);
///         });
///         # closure(2);
///     }
/// }
/// ```
#[macro_export]
macro_rules! clone {
    ( => $($_:tt)*) => (
        // In case we have:
        // clone!( => move || {});
        compile_error!("If you have nothing to clone, no need to use this macro!");
    );
    ($(move)? || $($_:tt)*) => (
        // In case we have:
        // clone!(|| {});
        compile_error!("If you have nothing to clone, no need to use this macro!");
    );
    ($(move)? | $($arg:tt $(: $typ:ty)?),* | $($_:tt)*) => (
        // In case we have:
        // clone!(|a, b| {});
        compile_error!("If you have nothing to clone, no need to use this macro!")
    );
    ($($(@weak-allow-none)? $(@weak)? $(@strong)? $($variables:ident).+ $(as $rename:ident)?,)* @default-return $($_:tt)*) => (
        // In case we have:
        // clone!(@strong v, @default-return lol => move || {println!(\"foo\");});
        compile_error!("`@default-return` should be after `=>`");
    );
    ($($(@ $strength:ident$(-$var:ident-$var2:ident)?)? self),+ => $($_:tt)* ) => (
        compile_error!("Can't use `self` as variable name. Try storing it in a temporary variable or rename it using `as`.");
    );
    ($($(@ $strength:ident$(-$var:ident-$var2:ident)?)? $up:ident.$($variables:ident).+),+ => $($_:tt)* ) => (
        compile_error!("Field accesses are not allowed as is, you must rename it!");
    );
    ($($(@ $strength:ident$(-$var:ident-$var2:ident)?)? $($variables:ident).+ $(as $rename:ident)?),+ => @default-panic, move || $body:block ) => (
        {
            $( $crate::to_type_before!($(@ $strength$(-$var-$var2)?)? $($variables).+ $(as $rename)?); )*
            move || {
                $( $crate::to_type_after!($(as $rename)? @default-panic, $(@ $strength$(-$var-$var2)?)? $($variables).+);)*
                $body
            }
        }
    );
    ($($(@ $strength:ident$(-$var:ident-$var2:ident)?)? $($variables:ident).+ $(as $rename:ident)?),+ => @default-panic, move || $body:expr ) => (
        $crate::clone!($($(@ $strength$(-$var-$var2)?)? $($variables).+ $(as $rename)?),* => @default-panic, move || { $body })
    );
    ($($(@ $strength:ident$(-$var:ident-$var2:ident)?)? $($variables:ident).+ $(as $rename:ident)?),+ => $(@default-return $return_value:expr,)? move || $body:block ) => (
        {
            $( $crate::to_type_before!($(@ $strength$(-$var-$var2)?)? $($variables).+ $(as $rename)?); )*
            move || {
                let _return_value = || $crate::to_return_value!($($return_value)?);
                $( $crate::to_type_after!($(as $rename)? $(@ $strength$(-$var-$var2)?)? $($variables).+, _return_value );)*
                $body
            }
        }
    );
    ($($(@ $strength:ident$(-$var:ident-$var2:ident)?)? $($variables:ident).+ $(as $rename:ident)?),+ => $(@default-return $return_value:expr,)? move || $body:expr ) => (
        $crate::clone!($($(@ $strength$(-$var-$var2)?)? $($variables).+ $(as $rename)?),* => $(@default-return $return_value,)? move || { $body })
    );
    ($($(@ $strength:ident$(-$var:ident-$var2:ident)?)? $($variables:ident).+ $(as $rename:ident)?),+ => @default-panic, move | $($arg:tt $(: $typ:ty)?),* | $body:block ) => (
        {
            $( $crate::to_type_before!($(@ $strength$(-$var-$var2)?)? $($variables).+ $(as $rename)?); )*
            move |$($arg $(: $typ)?),*| {
                $( $crate::to_type_after!($(as $rename)? @default-panic, $(@ $strength$(-$var-$var2)?)? $($variables).+);)*
                $body
            }
        }
    );
    ($($(@ $strength:ident$(-$var:ident-$var2:ident)?)? $($variables:ident).+ $(as $rename:ident)?),+ => @default-panic, move | $($arg:tt $(: $typ:ty)?),* | $body:expr ) => (
        $crate::clone!($($(@ $strength$(-$var-$var2)?)? $($variables).+ $(as $rename)?),* => @default-panic, move |$($arg $(: $typ)?),*| { $body })
    );
    ($($(@ $strength:ident$(-$var:ident-$var2:ident)?)? $($variables:ident).+ $(as $rename:ident)?),+ => $(@default-return $return_value:expr,)? move | $($arg:tt $(: $typ:ty)?),* | $body:block ) => (
        {
            $( $crate::to_type_before!($(@ $strength$(-$var-$var2)?)? $($variables).+ $(as $rename)?); )*
            move | $($arg $(: $typ)?),* | {
                let _return_value = || $crate::to_return_value!($($return_value)?);
                $( $crate::to_type_after!($(as $rename)? $(@ $strength$(-$var-$var2)?)? $($variables).+, _return_value);)*
                $body
            }
        }
    );
    ($($(@ $strength:ident$(-$var:ident-$var2:ident)?)? $($variables:ident).+ $(as $rename:ident)?),+ => $(@default-return $return_value:expr,)? move | $($arg:tt $(: $typ:ty)?),* | $body:expr ) => (
        $crate::clone!($($(@ $strength$(-$var-$var2)?)? $($variables).+ $(as $rename)?),+ => $(@default-return $return_value,)? move |$($arg $(: $typ)?),*| { $body })
    );
    ($($(@ $strength:ident$(-$var:ident-$var2:ident)?)? $($variables:ident).+ $(as $rename:ident)?),+ => @default-return $return_value:expr, || $($body:tt)* ) => (
        // In case we have:
        // clone!(@weak foo => @default-return false, || {});
        compile_error!("Closure needs to be \"moved\" so please add `move` before closure");
    );
    ($($(@ $strength:ident$(-$var:ident-$var2:ident)?)? $($variables:ident).+ $(as $rename:ident)?),+ => @default-return $return_value:expr, | $($arg:tt $(: $typ:ty)?),* | $($x:tt)* ) => (
        // In case we have:
        // clone!(@weak foo => @default-return false, |bla| {});
        compile_error!("Closure needs to be \"moved\" so please add `move` before closure");
    );
    ($($(@ $strength:ident$(-$var:ident-$var2:ident)?)? $($variables:ident).+ $(as $rename:ident)?),+ => || $($x:tt)* ) => (
        // In case we have:
        // clone!(@weak foo => || {});
        compile_error!("Closure needs to be \"moved\" so please add `move` before closure");
    );
    ($($(@ $strength:ident$(-$var:ident-$var2:ident)?)? $($variables:ident).+ $(as $rename:ident)?),+ => | $($arg:tt $(: $typ:ty)?),* | $($x:tt)* ) => (
        // In case we have:
        // clone!(@weak foo => |bla| {});
        compile_error!("Closure needs to be \"moved\" so please add `move` before closure");
    );
    ($($(@ $strength:ident$(-$var:ident-$var2:ident)?)? $($variables:ident).+ $(as $rename:ident)?),+ => async $($x:tt)+ ) => (
        compile_error!("async blocks are not supported by the clone! macro");
    );
    ($($(@ $strength:ident$(-$var:ident-$var2:ident)?)? $variables:expr),+ => move || $($_:tt)* ) => (
        $( $crate::to_type_before!($(@ $strength$(-$var-$var2)?)? $variables); )*
    );
    ($($(@ $strength:ident$(-$var:ident-$var2:ident)?)? $($variables:ident).+ $(as $rename:ident)?),+ => default-return $($x:tt)+ ) => (
        // In case we have:
        // clone!(@weak foo => default-return false, move || {});
        compile_error!("Missing `@` before `default-return`");
    );
    ($($(@ $strength:ident$(-$var:ident-$var2:ident)?)? $($variables:ident).+ $(as $rename:ident)?),+ => @default-return $($x:tt)+ ) => (
        // In case we have:
        // clone!(@weak foo => @default-return false move || {});
        compile_error!("Missing comma after `@default-return`'s value");
    );
    ($($(@ $strength:ident$(-$var:ident-$var2:ident)?)? $variables:expr),+ => $_:expr) => (
        // In case we have:
        // clone!(@weak foo => move {});
        compile_error!("Missing `move` and closure declaration");
    );
    ($($(@ $strength:ident$(-$var:ident-$var2:ident)?)? $variables:expr),+ => $_:block) => (
        // In case we have:
        // clone!(@weak foo => move {println!("a");});
        compile_error!("Missing `move` and closure declaration");
    );
    ($($(@ $strength:ident$(-$var:ident-$var2:ident)?)? $variables:expr),+ => move $($_:tt)* ) => (
        compile_error!(concat!("Variables need to be valid identifiers, e.g. field accesses are not allowed as is, you must rename it!", $(
            $(stringify!($strength),$("-", stringify!($var), "-", stringify!($var2),)?)?
            stringify!($variables),)+));
    );
}

#[cfg(test)]
mod tests {
    #![allow(dead_code)]
    #![allow(unused_variables)]
    use std::rc::Rc;

    #[test]
    fn test_clone_macro_self_rename() {
        #[derive(Debug)]
        struct Foo {
            v: u8,
        }

        impl Foo {
            fn foo(&self) {
                let closure = clone!(@strong self as this => move |_x| {
                    println!("v: {:?}", this);
                });
                closure(0i8); // to prevent compiler error for unknown `x` type.
                let _ = clone!(@strong self as this => move || {
                    println!("v: {:?}", this);
                });
                let closure = clone!(@strong self as this => move |_x| println!("v: {:?}", this));
                closure(0i8); // to prevent compiler error for unknown `x` type.
                let _ = clone!(@strong self as this => move || println!("v: {:?}", this));

                // Fields now!
                let closure = clone!(@strong self.v as v => move |_x| {
                    println!("v: {:?}", v);
                });
                closure(0i8); // to prevent compiler error for unknown `x` type.
                let _ = clone!(@strong self.v as v => move || println!("v: {:?}", v));

                // With @default-panic
                let closure = clone!(@strong self.v as v => @default-panic, move |_x| {
                    println!("v: {:?}", v);
                });
                closure(0i8); // to prevent compiler error for unknown `x` type.
                let _ =
                    clone!(@strong self.v as v => @default-panic, move || println!("v: {:?}", v));

                // With @default-return
                let closure = clone!(@strong self.v as _v => @default-return true, move |_x| {
                    false
                });
                closure(0i8); // to prevent compiler error for unknown `x` type.
                let _ = clone!(@strong self.v as _v => @default-return true, move || false);
            }
        }
    }

    #[test]
    fn test_clone_macro_rename() {
        let v = Rc::new(1);

        let closure = clone!(@weak v as y => @default-panic, move |_x| {
            println!("v: {}", y);
        });
        closure(0i8); // to prevent compiler error for unknown `x` type.
        let _ = clone!(@weak v as y => @default-panic, move || println!("v: {}", y));

        let closure = clone!(@strong v as y => @default-panic, move |_x| {
            println!("v: {}", y);
        });
        closure(0i8); // to prevent compiler error for unknown `x` type.
        let _ = clone!(@strong v as y => @default-panic, move || println!("v: {}", y));

        let closure = clone!(@weak v as y => move |_x| {
            println!("v: {}", y);
        });
        closure(0i8); // to prevent compiler error for unknown `x` type.
        let _ = clone!(@weak v as y => move || println!("v: {}", y));

        let closure = clone!(@strong v as y => move |_x| {
            println!("v: {}", y);
        });
        closure(0i8); // to prevent compiler error for unknown `x` type.
        let _ = clone!(@strong v as y => move || println!("v: {}", y));

        let closure = clone!(@weak v as _y => @default-return true, move |_x| {
            false
        });
        closure(0i8); // to prevent compiler error for unknown `x` type.
        let _ = clone!(@weak v as _y => @default-return true, move || false);

        let closure = clone!(@strong v as _y => @default-return true, move |_x| false);
        closure(0i8); // to prevent compiler error for unknown `x` type.
        let _ = clone!(@strong v as _y => @default-return true, move || false);
    }

    #[test]
    fn test_clone_macro_simple() {
        let v = Rc::new(1);

        let closure = clone!(@weak v => @default-panic, move |_x| {
            println!("v: {}", v);
        });
        closure(0i8); // to prevent compiler error for unknown `x` type.
        let _ = clone!(@weak v => @default-panic, move || println!("v: {}", v));

        let closure = clone!(@strong v => @default-panic, move |_x| {
            println!("v: {}", v);
        });
        closure(0i8); // to prevent compiler error for unknown `x` type.
        let _ = clone!(@strong v => @default-panic, move || println!("v: {}", v));

        let closure = clone!(@weak v => move |_x| {
            println!("v: {}", v);
        });
        closure(0i8); // to prevent compiler error for unknown `x` type.
        let _ = clone!(@weak v => move || println!("v: {}", v));

        let closure = clone!(@strong v => move |_x| {
            println!("v: {}", v);
        });
        closure(0i8); // to prevent compiler error for unknown `x` type.
        let _ = clone!(@strong v => move || println!("v: {}", v));

        let closure = clone!(@weak v => @default-return true, move |_x| {
            false
        });
        closure(0i8); // to prevent compiler error for unknown `x` type.
        let _ = clone!(@weak v => @default-return true, move || false);

        let closure = clone!(@strong v => @default-return true, move |_x| false);
        closure(0i8); // to prevent compiler error for unknown `x` type.
        let _ = clone!(@strong v => @default-return true, move || false);
    }

    #[test]
    fn test_clone_macro_double_simple() {
        let v = Rc::new(1);
        let w = Rc::new(2);

        let closure = clone!(@weak v, @weak w => @default-panic, move |_x| {
            println!("v: {}, w: {}", v, w);
        });
        closure(0i8); // to prevent compiler error for unknown `x` type.
        let _ = clone!(@weak v, @weak w => @default-panic, move || println!("v: {}, w: {}", v, w));

        let closure = clone!(@strong v, @strong w => @default-panic, move |_x| {
            println!("v: {}, w: {}", v, w);
        });
        closure(0i8); // to prevent compiler error for unknown `x` type.
        let _ =
            clone!(@strong v, @strong w => @default-panic, move || println!("v: {}, w: {}", v, w));

        let closure = clone!(@weak v, @weak w => move |_x| {
            println!("v: {}, w: {}", v, w);
        });
        closure(0i8); // to prevent compiler error for unknown `x` type.
        let _ = clone!(@weak v, @weak w => move || println!("v: {}, w: {}", v, w));

        let closure = clone!(@strong v, @strong w => move |_x| {
            println!("v: {}, w: {}", v, w);
        });
        closure(0i8); // to prevent compiler error for unknown `x` type.
        let _ = clone!(@strong v, @strong w => move || println!("v: {}, w: {}", v, w));

        let closure = clone!(@weak v, @weak w => @default-return true, move |_x| {
            false
        });
        closure(0i8); // to prevent compiler error for unknown `x` type.
        let _ = clone!(@weak v, @weak w => @default-return true, move || false);

        let closure = clone!(@strong v, @strong w => @default-return true, move |_x| false);
        closure(0i8); // to prevent compiler error for unknown `x` type.
        let _ = clone!(@strong v, @strong w => @default-return true, move || false);
    }

    #[test]
    fn test_clone_macro_double_rename() {
        let v = Rc::new(1);
        let w = Rc::new(2);

        let closure = clone!(@weak v as x, @weak w => @default-panic, move |_x| {
            println!("v: {}, w: {}", x, w);
        });
        closure(0i8); // to prevent compiler error for unknown `x` type.
        let _ =
            clone!(@weak v as x, @weak w => @default-panic, move || println!("v: {}, w: {}", x, w));

        let closure = clone!(@weak v, @weak w as x => @default-panic, move |_x| {
            println!("v: {}, w: {}", v, x);
        });
        closure(0i8); // to prevent compiler error for unknown `x` type.
        let _ =
            clone!(@weak v, @weak w as x => @default-panic, move || println!("v: {}, w: {}", v, x));

        let closure = clone!(@strong v as x, @strong w => @default-panic, move |_x| {
            println!("v: {}, w: {}", x, w);
        });
        closure(0i8); // to prevent compiler error for unknown `x` type.
        let _ = clone!(@strong v as x, @strong w => @default-panic, move || println!("v: {}, w: {}", x, w));

        let closure = clone!(@strong v, @strong w as x => @default-panic, move |_x| {
            println!("v: {}, w: {}", v, x);
        });
        closure(0i8); // to prevent compiler error for unknown `x` type.
        let _ = clone!(@strong v, @strong w as x => @default-panic, move || println!("v: {}, w: {}", v, x));

        let closure = clone!(@weak v as x, @weak w => move |_x| {
            println!("v: {}, w: {}", x, w);
        });
        closure(0i8); // to prevent compiler error for unknown `x` type.
        let _ = clone!(@weak v as x, @weak w => move || println!("v: {}, w: {}", x, w));

        let closure = clone!(@weak v, @weak w as x => move |_x| {
            println!("v: {}, w: {}", v, x);
        });
        closure(0i8); // to prevent compiler error for unknown `x` type.
        let _ = clone!(@weak v, @weak w as x => move || println!("v: {}, w: {}", v, x));

        let closure = clone!(@strong v as x, @strong w => move |_x| {
            println!("v: {}, w: {}", x, w);
        });
        closure(0i8); // to prevent compiler error for unknown `x` type.
        let _ = clone!(@strong v as x, @strong w => move || println!("v: {}, w: {}", x, w));

        let closure = clone!(@strong v, @strong w as x => move |_x| {
            println!("v: {}, w: {}", v, x);
        });
        closure(0i8); // to prevent compiler error for unknown `x` type.
        let _ = clone!(@strong v, @strong w as x => move || println!("v: {}, w: {}", v, x));

        let closure = clone!(@weak v as _x, @weak w => @default-return true, move |_| {
            false
        });
        closure(0u8); // to prevent compiler error for unknown `x` type.
        let _ = clone!(@weak v as _x, @weak w => @default-return true, move || false);

        let closure = clone!(@weak v, @weak w as _x => @default-return true, move |_| {
            false
        });
        closure(0i8); // to prevent compiler error for unknown `x` type.
        let _ = clone!(@weak v, @weak w as _x => @default-return true, move || false);

        let closure =
            clone!(@strong v as _x, @strong w => @default-return true, move |_| { false });
        closure(0i8); // to prevent compiler error for unknown `x` type.
        let _ = clone!(@strong v as _x, @strong w => @default-return true, move || false);

        let closure =
            clone!(@strong v, @strong w as _x => @default-return true, move |_| { false });
        closure(0i8); // to prevent compiler error for unknown `x` type.
        let _ = clone!(@strong v, @strong w as _x => @default-return true, move || false);
    }

    #[test]
    fn test_clone_macro_typed_args() {
        let v = Rc::new(1);
        let w = Rc::new(2);

        let closure = clone!(@weak v as x, @weak w => @default-panic, move |_x: i8| {
            println!("v: {}, w: {}", x, w);
        });

        let closure = clone!(@weak v, @weak w as x => @default-panic, move |_x: i8| {
            println!("v: {}, w: {}", v, x);
        });

        let closure = clone!(@strong v as x, @strong w => @default-panic, move |_x: i8| {
            println!("v: {}, w: {}", x, w);
        });

        let closure = clone!(@strong v, @strong w as x => @default-panic, move |_x: i8| {
            println!("v: {}, w: {}", v, x);
        });

        let closure = clone!(@weak v as x, @weak w => move |_x: i8| {
            println!("v: {}, w: {}", x, w);
        });

        let closure = clone!(@weak v, @weak w as x => move |_x: i8| {
            println!("v: {}, w: {}", v, x);
        });

        let closure = clone!(@weak v, @weak w as x => move |_: i8, _| {
            println!("v: {}, w: {}", v, x);
        });
        closure(0, 'a');
    }
}
