// Take a look at the license at the top of the repository in the LICENSE file.

mod clone;
mod downgrade_derive;
mod gboxed_derive;
mod genum_derive;
mod gflags_attribute;
mod object_subclass_attribute;
mod utils;

use proc_macro::TokenStream;
use proc_macro_error::proc_macro_error;
use syn::{parse_macro_input, DeriveInput, LitStr};

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
/// binary) to either "all" or [`CLONE_MACRO_LOG_DOMAIN`]:
///
/// [`g_debug`]: https://gtk-rs.org/docs/glib/macro.g_debug.html
/// [`CLONE_MACRO_LOG_DOMAIN`]: https://gtk-rs.org/docs/glib/constant.CLONE_MACRO_LOG_DOMAIN.html
///
/// ```rust,ignore
/// use glib::CLONE_MACRO_LOG_DOMAIN;
///
/// std::env::set_var("G_MESSAGES_DEBUG", CLONE_MACRO_LOG_DOMAIN);
/// std::env::set_var("G_MESSAGES_DEBUG", "all");
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
/// # use glib_macros::clone;
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
/// # use glib_macros::clone;
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
/// # use glib_macros::clone;
/// use std::rc::Rc;
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
/// use glib_macros::clone;
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
/// use glib_macros::clone;
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
/// # use glib_macros::clone;
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
/// # use glib_macros::clone;
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
/// # use glib_macros::clone;
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
/// # use glib_macros::clone;
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
/// # use glib_macros::clone;
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
/// # use glib_macros::clone;
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
#[proc_macro]
pub fn clone(item: TokenStream) -> TokenStream {
    clone::clone_inner(item)
}

#[proc_macro_derive(GEnum, attributes(genum))]
#[proc_macro_error]
pub fn genum_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let gen = genum_derive::impl_genum(&input);
    gen.into()
}

/// Derive macro for defining a [`BoxedType`]`::get_type` function and
/// the [`glib::Value`] traits.
///
/// # Example
///
/// ```
/// use glib::prelude::*;
/// use glib::subclass::prelude::*;
///
/// #[derive(Clone, Debug, PartialEq, Eq, glib::GBoxed)]
/// #[gboxed(type_name = "MyBoxed")]
/// struct MyBoxed(String);
/// ```
///
/// [`BoxedType`]: subclass/boxed/trait.BoxedType.html
/// [`glib::Value`]: value/struct.Value.html
#[proc_macro_derive(GBoxed, attributes(gboxed))]
#[proc_macro_error]
pub fn gboxed_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let gen = gboxed_derive::impl_gboxed(&input);
    gen.into()
}

/// Attribute macro for defining flags using the `bitflags` crate.
/// This macro will also define a `GFlags::get_type` function and
/// the [`glib::Value`] traits.
///
/// The expected `GType` name has to be passed as macro attribute.
/// The name and nick of each flag can also be optionally defined.
/// Default name is the flag identifier in CamelCase and default nick
/// is the identifier in kebab-case.
/// Combined flags should not be registered with the `GType` system
/// and so needs to be tagged with the `#[gflags(skip)]` attribute.
///
/// # Example
///
/// ```
/// use glib::prelude::*;
/// use glib::subclass::prelude::*;
///
/// #[glib::gflags("MyFlags")]
/// enum MyFlags {
///     #[gflags(name = "Flag A", nick = "nick-a")]
///     A = 0b00000001,
///     #[gflags(name = "Flag B")]
///     B = 0b00000010,
///     #[gflags(skip)]
///     AB = Self::A.bits() | Self::B.bits(),
///     C = 0b00000100,
/// }
/// ```
///
/// [`glib::Value`]: value/struct.Value.html
#[proc_macro_attribute]
#[proc_macro_error]
pub fn gflags(attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as DeriveInput);
    let gtype_name = parse_macro_input!(attr as LitStr);
    let gen = gflags_attribute::impl_gflags(&input, &gtype_name);
    gen.into()
}

/// Macro for boilerplate of [`ObjectSubclass`] implementations.
///
/// This adds implementations for the `type_data()` and `get_type()` methods,
/// which should probably never be defined differently.
///
/// It provides default values for the `Instance`, `Class`, and `Interfaces`
/// type parameters. If these are present, the macro will use the provided value
/// instead of the default.
///
/// Usually the defaults for `Instance` and `Class` will work. `Interfaces` is
/// necessary for types that implement interfaces.
///
/// ```ignore
/// type Instance = glib::subclass::simple::InstanceStruct<Self>;
/// type Class = glib::subclass::simple::ClassStruct<Self>;
/// type Interfaces = ();
/// ```
///
/// If no `new()` or `with_class()` method is provide, the macro adds a `new()`
/// implementation calling `Default::default()`. So the type needs to implement
/// `Default`, or this should be overridden.
///
/// ```ignore
/// fn new() -> Self {
///     Default::default()
/// }
/// ```
///
/// [`ObjectSubclass`]: subclass/types/trait.ObjectSubclass.html
#[proc_macro_attribute]
#[proc_macro_error]
pub fn object_subclass(_attr: TokenStream, item: TokenStream) -> TokenStream {
    use proc_macro_error::abort_call_site;
    match syn::parse::<syn::ItemImpl>(item) {
        Ok(input) => object_subclass_attribute::impl_object_subclass(&input).into(),
        Err(_) => abort_call_site!(object_subclass_attribute::WRONG_PLACE_MSG),
    }
}

/// Macro for deriving implementations of [`glib::clone::Downgrade`] and
/// [`glib::clone::Upgrade`] traits and a weak type.
///
/// # Examples
///
/// ## New Type Idiom
///
/// ```rust,ignore
/// #[derive(glib::Downgrade)]
/// pub struct FancyLabel(gtk::Label);
///
/// impl FancyLabel {
///     pub fn new(label: &str) -> Self {
///         Self(gtk::LabelBuilder::new().label(label).build())
///     }
///
///     pub fn flip(&self) {
///         self.0.set_angle(180.0 - self.0.get_angle());
///     }
/// }
///
/// let fancy_label = FancyLabel::new("Look at me!");
/// let button = gtk::ButtonBuilder::new().label("Click me!").build();
/// button.connect_clicked(clone!(@weak fancy_label => move || fancy_label.flip()));
/// ```
///
/// ## Generic New Type
///
/// ```rust,ignore
/// #[derive(glib::Downgrade)]
/// pub struct TypedEntry<T>(gtk::Entry, std::marker::PhantomData<T>);
///
/// impl<T: ToString + FromStr> for TypedEntry<T> {
///     // ...
/// }
/// ```
///
/// ## Structures and Enums
///
/// ```rust,ignore
/// #[derive(Clone, glib::Downgrade)]
/// pub struct ControlButtons {
///     pub up: gtk::Button,
///     pub down: gtk::Button,
///     pub left: gtk::Button,
///     pub right: gtk::Button,
/// }
///
/// #[derive(Clone, glib::Downgrade)]
/// pub enum DirectionButton {
///     Left(gtk::Button),
///     Right(gtk::Button),
///     Up(gtk::Button),
///     Down(gtk::Button),
/// }
/// ```
///
/// [`glib::clone::Downgrade`]: clone/trait.Downgrade.html
/// [`glib::clone::Upgrade`]: clone/trait.Upgrade.html
#[proc_macro_derive(Downgrade)]
pub fn downgrade(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    downgrade_derive::impl_downgrade(input)
}
