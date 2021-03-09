// Take a look at the license at the top of the repository in the LICENSE file.

//! `IMPL` The `wrapper!` macro and miscellaneous wrapper traits.

/// Defines a wrapper type and implements the appropriate traits.
///
/// The basic syntax is
///
/// ```ignore
/// wrapper! {
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
/// wrapper! {
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
/// wrapper! {
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
/// wrapper! {
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
/// wrapper! {
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
/// Objects -- classes.  Note that the class name, if available, must be specified after the
/// $foreign type; see below for [non-derivable classes][#non-derivable-classes].
///
/// The basic syntax is this:
///
/// ```ignore
/// wrapper! {
///     /// Your documentation goes here
///     pub struct InstanceName(Object<ffi::InstanceStruct, ffi::ClassStruct>)
///         @extends ParentClass, GrandparentClass, ...,
///         @implements Interface1, Interface2, ...;
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
/// In the example above, "`@extends ParentClass, GrandparentClass, ...,`" is where you must
/// specify all the parent classes of the one you are wrapping. The uppermost parent class,
/// `glib::Object`, must not be specified.
///
/// For example, `ffi::GtkWindowGroup` derives directly from
/// `GObject`, so it can be simply wrapped as follows:
///
/// ```ignore
/// wrapper! {
///     pub struct WindowGroup(Object<ffi::GtkWindowGroup, ffi::GtkWindowGroupClass>);
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
/// wrapper! {
///     pub struct Button(Object<ffi::GtkButton>) @extends Bin, Container, Widget;
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
/// `Buildable` and `Actionable`.  In this case, they must be specified after all the parent classes
/// behind the `@implements` keyword:
///
/// ```ignore
/// wrapper! {
///     pub struct Button(Object<ffi::GtkButton>)
///         @extends Bin, Container, Widget, // parent classes
///         @implements Buildable, Actionable;  // interfaces
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
/// specify a FFI class name at all in the `Object<>` part:
///
/// ```ignore
/// wrapper! {
///     pub struct Clipboard(Object<ffi::GtkClipboard>);
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
/// wrapper! {
///     pub struct TreeModel(Interface<ffi::GtkTreeModel, ffi::GtkTreeModelIface>);
///     ...
/// }
/// ```
///
/// #### Interfaces with prerequisites
///
/// Interfaces can declare prerequisites, i.e. the classes from which types that implement the
/// interface have to inherit or interfaces that have to be implemented:
///
/// ```ignore
/// wrapper! {
///     pub struct TreeSortable(Interface<ffi::GtkTreeSortable, ffi::GtkTreeSortable>) @requires TreeModel;
///     ...
/// }
/// ```
///
/// [#boxed]: #boxed
/// [#shared]: #shared
/// [#object]: #object
/// [#non-derivable-classes]: #non-derivable-classes

#[macro_export]
macro_rules! wrapper {
    // Boxed

    (
        $(#[$attr:meta])*
        pub struct $name:ident(Boxed<$ffi_name:ty>);

        match fn {
            copy => |$copy_arg:ident| $copy_expr:expr,
            free => |$free_arg:ident| $free_expr:expr,
        }
    ) => {
        $crate::glib_boxed_wrapper!([$($attr)*] $name, $ffi_name, @copy $copy_arg $copy_expr,
            @free $free_arg $free_expr);
    };

    (
        $(#[$attr:meta])*
        pub struct $name:ident(Boxed<$ffi_name:ty>);

        match fn {
            copy => |$copy_arg:ident| $copy_expr:expr,
            free => |$free_arg:ident| $free_expr:expr,
            get_type => || $get_type_expr:expr,
        }
    ) => {
        $crate::glib_boxed_wrapper!([$($attr)*] $name, $ffi_name, @copy $copy_arg $copy_expr,
            @free $free_arg $free_expr, @get_type $get_type_expr);
    };

    (
        $(#[$attr:meta])*
        pub struct $name:ident(Boxed<$ffi_name:ty>);

        match fn {
            copy => |$copy_arg:ident| $copy_expr:expr,
            free => |$free_arg:ident| $free_expr:expr,
            init => |$init_arg:ident| $init_expr:expr,
            clear => |$clear_arg:ident| $clear_expr:expr,
        }
    ) => {
        $crate::glib_boxed_wrapper!([$($attr)*] $name, $ffi_name, @copy $copy_arg $copy_expr,
            @free $free_arg $free_expr, @init $init_arg $init_expr, @clear $clear_arg $clear_expr);
    };

    (
        $(#[$attr:meta])*
        pub struct $name:ident(Boxed<$ffi_name:ty>);

        match fn {
            copy => |$copy_arg:ident| $copy_expr:expr,
            free => |$free_arg:ident| $free_expr:expr,
            init => |$init_arg:ident| $init_expr:expr,
            clear => |$clear_arg:ident| $clear_expr:expr,
            get_type => || $get_type_expr:expr,
        }
    ) => {
        $crate::glib_boxed_wrapper!([$($attr)*] $name, $ffi_name, @copy $copy_arg $copy_expr,
            @free $free_arg $free_expr, @init $init_arg $init_expr, @clear $clear_arg $clear_expr,
            @get_type $get_type_expr);
    };

    // Shared

    (
        $(#[$attr:meta])*
        pub struct $name:ident(Shared<$ffi_name:ty>);

        match fn {
            ref => |$ref_arg:ident| $ref_expr:expr,
            unref => |$unref_arg:ident| $unref_expr:expr,
        }
    ) => {
        $crate::glib_shared_wrapper!([$($attr)*] $name, $ffi_name, @ref $ref_arg $ref_expr,
            @unref $unref_arg $unref_expr);
    };

    (
        $(#[$attr:meta])*
        pub struct $name:ident(Shared<$ffi_name:ty>);

        match fn {
            ref => |$ref_arg:ident| $ref_expr:expr,
            unref => |$unref_arg:ident| $unref_expr:expr,
            get_type => || $get_type_expr:expr,
        }
    ) => {
        $crate::glib_shared_wrapper!([$($attr)*] $name, $ffi_name, @ref $ref_arg $ref_expr,
            @unref $unref_arg $unref_expr, @get_type $get_type_expr);
    };

    // Object, no class struct, no parents or interfaces
    (
        $(#[$attr:meta])*
        pub struct $name:ident(Object<$ffi_name:ty>);

        match fn {
            get_type => || $get_type_expr:expr,
        }
    ) => {
        $crate::glib_object_wrapper!(@object [$($attr)*] $name, $ffi_name, std::os::raw::c_void, @get_type $get_type_expr, @extends [], @implements []);
    };

    // Object, class struct, no parents or interfaces
    (
        $(#[$attr:meta])*
        pub struct $name:ident(Object<$ffi_name:ty, $ffi_class_name:ty>);

        match fn {
            get_type => || $get_type_expr:expr,
        }
    ) => {
        $crate::glib_object_wrapper!(@object [$($attr)*] $name, $ffi_name, $ffi_class_name, @get_type $get_type_expr, @extends [], @implements []);
    };

    // Object, no class struct, parents, no interfaces
    (
        $(#[$attr:meta])*
        pub struct $name:ident(Object<$ffi_name:ty>) @extends $($extends:path),+;

        match fn {
            get_type => || $get_type_expr:expr,
        }
    ) => {
        $crate::glib_object_wrapper!(@object [$($attr)*] $name, $ffi_name, std::os::raw::c_void,
            @get_type $get_type_expr, @extends [$($extends),+], @implements []);
    };

    // Object, class struct, parents, no interfaces
    (
        $(#[$attr:meta])*
        pub struct $name:ident(Object<$ffi_name:ty, $ffi_class_name:ty>) @extends $($extends:path),+;

        match fn {
            get_type => || $get_type_expr:expr,
        }
    ) => {
        $crate::glib_object_wrapper!(@object [$($attr)*] $name, $ffi_name, $ffi_class_name,
            @get_type $get_type_expr, @extends [$($extends),+], @implements []);
    };

    // Object, no class struct, no parents, interfaces
    (
        $(#[$attr:meta])*
        pub struct $name:ident(Object<$ffi_name:ty>) @implements $($implements:path),+;

        match fn {
            get_type => || $get_type_expr:expr,
        }
    ) => {
        $crate::glib_object_wrapper!(@object [$($attr)*] $name, $ffi_name, std::os::raw::c_void,
            @get_type $get_type_expr, @extends [], @implements [$($implements),+]);
    };

    // Object, class struct, no parents, interfaces
    (
        $(#[$attr:meta])*
        pub struct $name:ident(Object<$ffi_name:ty, $ffi_class_name:ty>) @implements $($implements:path),+;

        match fn {
            get_type => || $get_type_expr:expr,
        }
    ) => {
        $crate::glib_object_wrapper!(@object [$($attr)*] $name, $ffi_name, $ffi_class_name,
            @get_type $get_type_expr, @extends [], @implements [$($implements),+]);
    };

    // Object, no class struct, parents and interfaces
    (
        $(#[$attr:meta])*
        pub struct $name:ident(Object<$ffi_name:ty>) @extends $($extends:path),+, @implements $($implements:path),+;

        match fn {
            get_type => || $get_type_expr:expr,
        }
    ) => {
        $crate::glib_object_wrapper!(@object [$($attr)*] $name, $ffi_name, std::os::raw::c_void,
            @get_type $get_type_expr, @extends [$($extends),+], @implements [$($implements),+]);
    };

    // Object, class struct, parents and interfaces
    (
        $(#[$attr:meta])*
        pub struct $name:ident(Object<$ffi_name:ty, $ffi_class_name:ty>) @extends $($extends:path),+, @implements $($implements:path),+;

        match fn {
            get_type => || $get_type_expr:expr,
        }
    ) => {
        $crate::glib_object_wrapper!(@object [$($attr)*] $name, $ffi_name, $ffi_class_name,
            @get_type $get_type_expr, @extends [$($extends),+], @implements [$($implements),+]);
    };

    // ObjectSubclass, no parents or interfaces
    (
        $(#[$attr:meta])*
        pub struct $name:ident(ObjectSubclass<$subclass:ty>);
    ) => {
        $crate::glib_object_wrapper!(@object [$($attr)*] $name, <$subclass as $crate::subclass::types::ObjectSubclass>::Instance, <$subclass as $crate::subclass::types::ObjectSubclass>::Class,
            @get_type $crate::translate::ToGlib::to_glib(&<$subclass as $crate::subclass::types::ObjectSubclassType>::get_type()),
            @extends [], @implements []);
        unsafe impl $crate::object::ObjectSubclassIs for $name {
            type Subclass = $subclass;
        }
    };

    // ObjectSubclass, no parents, interfaces
    (
        $(#[$attr:meta])*
        pub struct $name:ident(ObjectSubclass<$subclass:ty>) @implements $($implements:path),+;
    ) => {
        $crate::glib_object_wrapper!(@object [$($attr)*] $name, <$subclass as $crate::subclass::types::ObjectSubclass>::Instance, <$subclass as $crate::subclass::types::ObjectSubclass>::Class,
            @get_type $crate::translate::ToGlib::to_glib(&<$subclass as $crate::subclass::types::ObjectSubclassType>::get_type()),
            @extends [], @implements [$($implements),+]);
        unsafe impl $crate::object::ObjectSubclassIs for $name {
            type Subclass = $subclass;
        }
    };

    // ObjectSubclass, parents, no interfaces
    (
        $(#[$attr:meta])*
        pub struct $name:ident(ObjectSubclass<$subclass:ty>) @extends $($extends:path),+;
    ) => {
        $crate::glib_object_wrapper!(@object [$($attr)*] $name, <$subclass as $crate::subclass::types::ObjectSubclass>::Instance, <$subclass as $crate::subclass::types::ObjectSubclass>::Class,
            @get_type $crate::translate::ToGlib::to_glib(&<$subclass as $crate::subclass::types::ObjectSubclassType>::get_type()),
            @extends [$($extends),+], @implements []);
        unsafe impl $crate::object::ObjectSubclassIs for $name {
            type Subclass = $subclass;
        }
    };

    // ObjectSubclass, parents and interfaces
    (
        $(#[$attr:meta])*
        pub struct $name:ident(ObjectSubclass<$subclass:ty>) @extends $($extends:path),+, @implements $($implements:path),+;
    ) => {
        $crate::glib_object_wrapper!(@object [$($attr)*] $name, <$subclass as $crate::subclass::types::ObjectSubclass>::Instance, <$subclass as $crate::subclass::types::ObjectSubclass>::Class,
            @get_type $crate::translate::ToGlib::to_glib(&<$subclass as $crate::subclass::types::ObjectSubclassType>::get_type()),
            @extends [$($extends),+], @implements [$($implements),+]);
        unsafe impl $crate::object::ObjectSubclassIs for $name {
            type Subclass = $subclass;
        }
    };

    // Interface, no prerequisites, no class
    (
        $(#[$attr:meta])*
        pub struct $name:ident(Interface<$ffi_name:ty>);

        match fn {
            get_type => || $get_type_expr:expr,
        }
    ) => {
        $crate::glib_object_wrapper!(@interface [$($attr)*] $name, $ffi_name, std::os::raw::c_void, @get_type $get_type_expr, @requires []);
    };

    // Interface, prerequisites, no class
    (
        $(#[$attr:meta])*
        pub struct $name:ident(Interface<$ffi_name:ty>) @requires $($requires:path),+;

        match fn {
            get_type => || $get_type_expr:expr,
        }
    ) => {
        $crate::glib_object_wrapper!(@interface [$($attr)*] $name, $ffi_name, std::os::raw::c_void, @get_type $get_type_expr, @requires [$($requires),+]);
    };

    // Interface, no prerequisites, class
    (
        $(#[$attr:meta])*
        pub struct $name:ident(Interface<$ffi_name:ty, $ffi_class_name:ty>);

        match fn {
            get_type => || $get_type_expr:expr,
        }
    ) => {
        $crate::glib_object_wrapper!(@interface [$($attr)*] $name, $ffi_name, $ffi_class_name, @get_type $get_type_expr, @requires []);
    };

    // Interface, prerequisites, class
    (
        $(#[$attr:meta])*
        pub struct $name:ident(Interface<$ffi_name:ty, $ffi_class_name:ty>) @requires $($requires:path),+;

        match fn {
            get_type => || $get_type_expr:expr,
        }
    ) => {
        $crate::glib_object_wrapper!(@interface [$($attr)*] $name, $ffi_name, $ffi_class_name, @get_type $get_type_expr, @requires [$($requires),+]);
    };

    // ObjectInterface, no prerequisites
    (
        $(#[$attr:meta])*
        pub struct $name:ident(ObjectInterface<$iface_name:ty>);
    ) => {
        $crate::glib_object_wrapper!(@interface [$($attr)*] $name, std::os::raw::c_void, $iface_name,
            @get_type $crate::translate::ToGlib::to_glib(&<$iface_name as $crate::subclass::interface::ObjectInterfaceType>::get_type()),
            @requires []);
    };

    // ObjectInterface, prerequisites
    (
        $(#[$attr:meta])*
        pub struct $name:ident(ObjectInterface<$iface_name:ty>) @requires $($requires:path),+;
    ) => {
        $crate::glib_object_wrapper!(@interface [$($attr)*] $name, std::os::raw::c_void, $iface_name,
            @get_type $crate::translate::ToGlib::to_glib(&<$iface_name as $crate::subclass::interface::ObjectInterfaceType>::get_type()),
            @requires [$($requires),+]);
    };
}
