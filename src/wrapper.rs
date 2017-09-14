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
///     /// Documentation
///     pub struct $name($kind<$foreign>);
///
///     match fn {
///         $fn_name => /* a closure-like expression */,
///         ...
///     }
/// }
/// ```
///
/// This creates a wrapper named `$name` around the foreign type `$foreign`
/// of `$kind` (one of `Boxed`, `Shared`, `Object`) using expressions from the `match fn`
/// block to implement type-specific low-level operations (the expression
/// will be evaluated in `unsafe` context).
///
/// ### Boxed
///
/// Boxed records with single ownership.
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
/// `get_type`: `||` (optional) returns the `Type`, if any
///
/// ### Shared
///
/// Records with reference counted shared ownership.
///
/// ```ignore
/// glib_wrapper! {
///     /// Object holding timing information for a single frame.
///     pub struct FrameTimings(Shared<ffi::GdkFrameTimings>);
///
///     match fn {
///         ref => |ptr| ffi::gdk_frame_timings_ref(ptr),
///         unref => |ptr| ffi::gdk_frame_timings_unref(ptr),
///     }
/// }
/// ```
///
/// `ref`: `|*mut $foreign|` increases the refcount.
///
/// `unref`: `|*mut $foreign|` decreases the refcount.
///
/// `get_type`: `||` (optional) returns the `Type`, if any
///
/// ### Object
///
/// Objects -- classes and interfaces.  Note that the class name must
/// be specified after the $foreign type.
///
/// ```ignore
/// glib_wrapper! {
///     /// Object representing an input device.
///     pub struct Device(Object<ffi::GdkDevice, ffi::GdkDeviceClass>);
///
///     match fn {
///         get_type => || ffi::gdk_device_get_type(),
///     }
/// }
/// ```
///
/// ```ignore
/// glib_wrapper! {
///     /// A container with just one child.
///     pub struct Bin(Object<ffi::GtkBin, ffi::GtkBinClass>): Container, Widget, Buildable;
///
///     match fn {
///         get_type => || ffi::gtk_bin_get_type(),
///     }
/// }
/// ```
///
/// Implementing types from other crates requires specifying their FFI
/// counterparts as well:
///
/// ```ignore
/// glib_wrapper! {
///     pub struct Application(Object<ffi::GtkApplication, ffi::GtkApplicationClass>): [
///         gio::Application => gio_ffi::GApplication,
///         gio::ActionGroup => gio_ffi::GActionGroup,
///         gio::ActionMap => gio_ffi::GActionMap,
///     ];
///
///     match fn {
///         get_type => || ffi::gtk_application_get_type(),
///     }
/// }
/// ```
///
/// #### Non-derivable classes
///
/// By convention, GObject implements "final" classes, i.e. those who cannot
/// be subclassed, by exposing a public Instance struct, but no corresponding
/// Class struct.  In this case, don't specify a class name at all:
///
/// ```ignore
/// glib_wrapper! {
///     pub struct Clipboard(Object<ffi::GtkClipboard>);
///     ...
/// }
/// ```
///
/// `get_type: || -> GType` returns the type identifier of the class or interface.
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
        pub struct $name:ident(Object<$ffi_name:path>);

        match fn {
            get_type => || $get_type_expr:expr,
        }
    ) => {
        glib_object_wrapper!([$($attr)*] $name, $ffi_name, $crate::wrapper::Void, @get_type $get_type_expr, []);
    };

    // Object, class struct, no parents
    (
        $(#[$attr:meta])*
        pub struct $name:ident(Object<$ffi_name:path, $ffi_class_name:path>);

        match fn {
            get_type => || $get_type_expr:expr,
        }
    ) => {
        glib_object_wrapper!([$($attr)*] $name, $ffi_name, $ffi_class_name, @get_type $get_type_expr, []);
    };

    // Object, no class struct, parents in other crates
    (
        $(#[$attr:meta])*
        pub struct $name:ident(Object<$ffi_name:path>): [$($implements:tt)+];

        match fn {
            get_type => || $get_type_expr:expr,
        }
    ) => {
        glib_object_wrapper!([$($attr)*] $name, $ffi_name, $crate::wrapper::Void, @get_type $get_type_expr,
            @implements $($implements)+);
    };

    // Object, class struct, parents in other crates
    (
        $(#[$attr:meta])*
        pub struct $name:ident(Object<$ffi_name:path, $ffi_class_name:path>): [$($implements:tt)+];

        match fn {
            get_type => || $get_type_expr:expr,
        }
    ) => {
        glib_object_wrapper!([$($attr)*] $name, $ffi_name, $ffi_class_name, @get_type $get_type_expr,
            @implements $($implements)+);
    };

    // Object, no class struct, parents
    (
        $(#[$attr:meta])*
        pub struct $name:ident(Object<$ffi_name:path>): $($implements:path),+;

        match fn {
            get_type => || $get_type_expr:expr,
        }
    ) => {
        glib_object_wrapper!([$($attr)*] $name, $ffi_name, $crate::wrapper::Void, @get_type $get_type_expr,
            [$($implements),+]);
    };

    // Object, class struct, parents
    (
        $(#[$attr:meta])*
        pub struct $name:ident(Object<$ffi_name:path, $ffi_class_name:path>): $($implements:path),+;

        match fn {
            get_type => || $get_type_expr:expr,
        }
    ) => {
        glib_object_wrapper!([$($attr)*] $name, $ffi_name, $ffi_class_name, @get_type $get_type_expr,
            [$($implements),+]);
    };
}

/// Represents a pair of structures (instance, class) as exposed by descendants of GObject
pub trait Wrapper {
    /// type of the Instance structure
    type GlibType: 'static;
    /// type of the Class structure
    type GlibClassType: 'static;
}

pub trait UnsafeFrom<T> {
    unsafe fn from(t: T) -> Self;
}

// So we can refer to the empty type by a path
pub type Void = ();
