// Copyright 2015-2016, The Gtk-rs Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

//! `IMPL` The `glib_wrapper!` macro and miscellaneous wrapper traits.

/// Defines a wrapper type and implements the appropriate traits.
///
/// The basic syntax is
///
/// ```ignore
/// glib_wrapper! {
///     /// Your documentation goes here
///     pub struct $name($kind<$foreign>);
///
///     match fn {
///         $fn_name => /* a closure-like expression */,
///         ...
///     }
/// }
/// ```
///
/// This creates a wrapper named `$name` around the foreign type
/// `$foreign` of `$kind` — one of [`Boxed`][#boxed],
/// [`Shared`][#shared], or [`Object`][#object].
///
/// Inside the `match fn` block there are closure-like expressions to
/// provide ways of copying/freeing, or referencing/unreferencing the
/// value that you are wrapping.  These expressions will be evaluated
/// in an `unsafe` context, since they frequently invoke `extern`
/// functions from an FFI crate.
///
/// What follows is a description of each of the possible `$kind`:
/// [`Boxed`][#boxed], [`Shared`][#shared], and [`Object`][#object];
/// note that each supports different sets of `$fn_name` inside the
/// `match fn` block.  Also, `Object` may require you to specify
/// things like the class struct to wrap, plus any interfaces that the
/// class implements.
///
/// ### Boxed
///
/// Boxed records with single ownership.
///
/// With no registered `glib_ffi::GType`:
///
/// ```ignore
/// glib_wrapper! {
///     /// Text buffer iterator
///     pub struct TextIter(Boxed<ffi::GtkTextIter>);
///
///     match fn {
///         copy => |ptr| ffi::gtk_text_iter_copy(ptr),
///         free => |ptr| ffi::gtk_text_iter_free(ptr),
///     }
/// }
/// ```
///
/// `copy`: `|*const $foreign| -> *mut $foreign` creates a copy of the value.
///
/// `free`: `|*mut $foreign|` frees the value.
///
/// With a registered `glib_ffi::GType`:
///
/// ```ignore
/// glib_wrapper! {
///     /// Text buffer iterator
///     pub struct TextIter(Boxed<ffi::GtkTextIter>);
///
///     match fn {
///         copy     => |ptr| ffi::gtk_text_iter_copy(ptr),
///         free     => |ptr| ffi::gtk_text_iter_free(ptr),
///         get_type => ||    ffi::gtk_text_iter_get_type(),
///     }
/// }
/// ```
///
/// `get_type`: `|| -> glib_ffi::GType` (optional) returns the
/// `glib_ffi::GType` that corresponds to the foreign struct.
///
/// ### Shared
///
/// Records with reference-counted, shared ownership.
///
/// With no registered `glib_ffi::GType`:
///
/// ```ignore
/// glib_wrapper! {
///     /// Object holding timing information for a single frame.
///     pub struct FrameTimings(Shared<ffi::GdkFrameTimings>);
///
///     match fn {
///         ref   => |ptr| ffi::gdk_frame_timings_ref(ptr),
///         unref => |ptr| ffi::gdk_frame_timings_unref(ptr),
///     }
/// }
/// ```
///
/// `ref`: `|*mut $foreign|` increases the refcount.
///
/// `unref`: `|*mut $foreign|` decreases the refcount.
///
/// With a registered `glib_ffi::GType`:
///
/// ```ignore
/// glib_wrapper! {
///     /// Object holding timing information for a single frame.
///     pub struct FrameTimings(Shared<ffi::GdkFrameTimings>);
///
///     match fn {
///         ref      => |ptr| ffi::gdk_frame_timings_ref(ptr),
///         unref    => |ptr| ffi::gdk_frame_timings_unref(ptr),
///         get_type => ||    ffi::gdk_frame_timings_get_type(),
///     }
/// }
/// ```
///
/// `get_type`: `|| -> glib_ffi::GType` (optional) returns the
/// `glib_ffi::GType` that corresponds to the foreign struct.
///
/// ### Object
///
/// Objects -- classes and interfaces.  Note that the class name, if
/// available, must be specified after the $foreign type; see below
/// for [non-derivable classes][#non-derivable-classes].
///
/// The basic syntax is this:
///
/// ```ignore
/// glib_wrapper! {
///     /// Your documentation goes here
///     pub struct InstanceName(Object<ffi::InstanceStruct, ffi::ClassStruct, ClassName>):
///         ParentClass, GrandparentClass, ...,
///         Interface1, Interface2, ...;
///
///     match fn {
///         get_type => || ffi::instance_get_type(),
///     }
/// }
/// ```
///
/// `get_type`: `|| -> glib_ffi::GType` returns the `glib_ffi::GType`
/// that corresponds to the foreign class.
///
/// #### All parent classes must be specified
///
/// In the example above, "`ParentClass, GrandparentClass, ...,`" is where you must specify all the
/// parent classes of the one you are wrapping.  It is not necessary to specify the uppermost
/// `GObject` or `GInitiallyUnowned` parent classes.
///
/// For example, `ffi::GtkWindowGroup` derives directly from
/// `GObject`, so it can be simply wrapped as follows:
///
/// ```ignore
/// glib_wrapper! {
///     pub struct WindowGroup(Object<ffi::GtkWindowGroup, ffi::GtkWindowGroupClass, WindowGroupClass>);
///
///     match fn {
///         get_type => || ffi::gtk_window_group_get_type(),
///     }
/// }
/// ```
///
/// In contrast, `ffi::GtkButton` has a parent, grandparent, etc. classes, which must be specified:
///
/// ```ignore
/// glib_wrapper! {
///     pub struct Button(Object<ffi::GtkButton, ButtonClass>): Bin, Container, Widget;
///         // see note on interfaces in the example below
///
///     match fn {
///         get_type => || ffi::gtk_button_get_type(),
///     }
/// }
/// ```
///
/// #### Objects which implement interfaces
///
/// The example above is incomplete, since `ffi::GtkButton` actually implements two interfaces,
/// `Buildable` and `Actionable`.  In this case, they must be specified after all the parent classes:
///
/// ```ignore
/// glib_wrapper! {
///     pub struct Button(Object<ffi::GtkButton, ButtonClass>):
///         Bin, Container, Widget, // parent classes
///         Buildable, Actionable;  // interfaces
///
///     match fn {
///         get_type => || ffi::gtk_button_get_type(),
///     }
/// }
/// ```
///
/// #### Non-derivable classes
///
/// By convention, GObject implements "final" classes, i.e. those who
/// cannot be subclassed, by *not* exposing a public Class struct.
/// This way it is not possible to override any methods, as there are
/// no `klass.method_name` fields to overwrite.  In this case, don't
/// specify a class name at all in the `Object<>` part:
///
/// ```ignore
/// glib_wrapper! {
///     pub struct Clipboard(Object<ffi::GtkClipboard, ClipboardClass>);
///     ...
/// }
/// ```
///
/// #### Interfaces
///
/// Interfaces are passed in the same way to the macro but instead of specifying
/// `Object`, `Interface` has to be specified:
///
/// ```ignore
/// glib_wrapper! {
///     pub struct TreeModel(Interface<ffi::GtkTreeModel, ffi::GtkTreeModelIface, TreeModelIface>);
///     ...
/// }
/// ```
///
/// [#boxed]: #boxed
/// [#shared]: #shared
/// [#object]: #object
/// [#non-derivable-classes]: #non-derivable-classes

#[macro_export]
macro_rules! glib_wrapper {
    // Boxed

    (
        $(#[$attr:meta])*
        pub struct $name:ident(Boxed<$ffi_name:path>);

        match fn {
            copy => |$copy_arg:ident| $copy_expr:expr,
            free => |$free_arg:ident| $free_expr:expr,
        }
    ) => {
        glib_boxed_wrapper!([$($attr)*] $name, $ffi_name, @copy $copy_arg $copy_expr,
            @free $free_arg $free_expr);
    };

    (
        $(#[$attr:meta])*
        pub struct $name:ident(Boxed<$ffi_name:path>);

        match fn {
            copy => |$copy_arg:ident| $copy_expr:expr,
            free => |$free_arg:ident| $free_expr:expr,
            get_type => || $get_type_expr:expr,
        }
    ) => {
        glib_boxed_wrapper!([$($attr)*] $name, $ffi_name, @copy $copy_arg $copy_expr,
            @free $free_arg $free_expr, @get_type $get_type_expr);
    };

    // Shared

    (
        $(#[$attr:meta])*
        pub struct $name:ident(Shared<$ffi_name:path>);

        match fn {
            ref => |$ref_arg:ident| $ref_expr:expr,
            unref => |$unref_arg:ident| $unref_expr:expr,
        }
    ) => {
        glib_shared_wrapper!([$($attr)*] $name, $ffi_name, @ref $ref_arg $ref_expr,
            @unref $unref_arg $unref_expr);
    };

    (
        $(#[$attr:meta])*
        pub struct $name:ident(Shared<$ffi_name:path>);

        match fn {
            ref => |$ref_arg:ident| $ref_expr:expr,
            unref => |$unref_arg:ident| $unref_expr:expr,
            get_type => || $get_type_expr:expr,
        }
    ) => {
        glib_shared_wrapper!([$($attr)*] $name, $ffi_name, @ref $ref_arg $ref_expr,
            @unref $unref_arg $unref_expr, @get_type $get_type_expr);
    };

    // Object, no class struct, no parents
    (
        $(#[$attr:meta])*
        pub struct $name:ident(Object<$ffi_name:path, $rust_class_name:ident>);

        match fn {
            get_type => || $get_type_expr:expr,
        }
    ) => {
        glib_object_wrapper!(@object [$($attr)*] $name, $ffi_name, $crate::wrapper::Void, $rust_class_name, @get_type $get_type_expr, @implements $crate::object::Object);
    };

    // Object, class struct, no parents
    (
        $(#[$attr:meta])*
        pub struct $name:ident(Object<$ffi_name:path, $ffi_class_name:path, $rust_class_name:ident>);

        match fn {
            get_type => || $get_type_expr:expr,
        }
    ) => {
        glib_object_wrapper!(@object [$($attr)*] $name, $ffi_name, $ffi_class_name, $rust_class_name, @get_type $get_type_expr, @implements $crate::object::Object);
    };

    // Object, no class struct, parents
    (
        $(#[$attr:meta])*
        pub struct $name:ident(Object<$ffi_name:path, $rust_class_name:ident>): $($implements:path),+;

        match fn {
            get_type => || $get_type_expr:expr,
        }
    ) => {
        glib_object_wrapper!(@object [$($attr)*] $name, $ffi_name, $crate::wrapper::Void, $rust_class_name,
            @get_type $get_type_expr, @implements $($implements),+);
    };

    // Object, class struct, parents
    (
        $(#[$attr:meta])*
        pub struct $name:ident(Object<$ffi_name:path, $ffi_class_name:path, $rust_class_name:ident>): $($implements:path),+;

        match fn {
            get_type => || $get_type_expr:expr,
        }
    ) => {
        glib_object_wrapper!(@object [$($attr)*] $name, $ffi_name, $ffi_class_name, $rust_class_name,
            @get_type $get_type_expr, @implements $($implements),+);
    };

    // Interface, class struct, no prerequisites
    (
        $(#[$attr:meta])*
        pub struct $name:ident(Interface<$ffi_name:path, $ffi_class_name:path, $rust_class_name:ident>);

        match fn {
            get_type => || $get_type_expr:expr,
        }
    ) => {
        glib_object_wrapper!(@interface [$($attr)*] $name, $ffi_name, $ffi_class_name, $rust_class_name, @get_type $get_type_expr, @implements $crate::object::Object);
    };

    // Interface, class struct, prerequisites
    (
        $(#[$attr:meta])*
        pub struct $name:ident(Interface<$ffi_name:path, $ffi_class_name:path, $rust_class_name:ident>): $($implements:path),+;

        match fn {
            get_type => || $get_type_expr:expr,
        }
    ) => {
        glib_object_wrapper!(@interface [$($attr)*] $name, $ffi_name, $ffi_class_name, $rust_class_name,
            @get_type $get_type_expr, @implements $($implements),+);
    };
}

// So we can refer to the empty type by a path
pub type Void = ();
