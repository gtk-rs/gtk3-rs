// Copyright 2013-2015, The Rust-GNOME Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

#![allow(non_camel_case_types)]

extern crate libc;

use libc::{c_void, c_int, c_uint, c_float, c_double, c_char, c_uchar, c_long, c_ulong, size_t};

pub type GQuark = u32;

pub type GType = size_t;

pub type Gboolean = c_int;
pub const GFALSE:  c_int = 0;
pub const GTRUE:   c_int = 1;

pub type gconstpointer = *const c_void;
pub type gpointer = *mut c_void;

pub type GSourceFunc = extern "C" fn(user_data: gpointer) -> Gboolean;
pub type GCallback = extern "C" fn();
pub type GClosureNotify = extern "C" fn(data: gpointer, closure: gpointer);

#[repr(C)]
pub struct C_GValue {
    type_: GType,
    data: [size_t; 2],
}

#[repr(C)]
pub struct C_GList {
    pub data: *mut c_void,
    pub next: *mut C_GList,
    pub prev: *mut C_GList
}

#[repr(C)]
pub struct C_GSList {
    pub data: *mut c_void,
    pub next: *mut C_GSList
}

#[repr(C)]
pub struct C_GError {
    pub domain : GQuark,
    pub code   : i32,
    pub message: *mut c_char
}

#[repr(C)]
pub struct C_GPermission;

#[repr(C)]
pub struct C_GObject;

#[repr(C)]
pub struct C_GMainLoop;

#[repr(C)]
pub struct C_GMainContext;

#[repr(C)]
pub struct C_GSource;

#[repr(C)]
pub struct C_GPid;

#[repr(C)]
pub struct C_GPollFD;

/// Represents a day between January 1, Year 1 and a few thousand years in the future. None of its members should be accessed directly.
/// 
/// If the GDate is obtained from g_date_new(), it will be safe to mutate but invalid and thus not safe for calendrical computations.
/// 
/// If it's declared on the stack, it will contain garbage so must be initialized with g_date_clear(). g_date_clear() makes the date
/// invalid but sane. An invalid date doesn't represent a day, it's "empty." A date becomes valid after you set it to a Julian day or
/// you set a day, month, and year.
#[repr(C)]
pub struct GDate;
/*pub struct GDate {
    /// the Julian representation of the date
    pub julian_days : u32,
    /// this bit is set if julian_days is valid
    pub julian: bool,
    /// this is set if day , month and year are valid
    pub dmy: bool,
    /// the day of the day-month-year representation of the date, as a number between 1 and 31
    pub day: u8,
    /// the day of the day-month-year representation of the date, as a number between 1 and 12
    pub month: u8,
    /// the day of the day-month-year representation of the date
    pub year: u8
}*/

//=========================================================================
// GType constants
//=========================================================================

pub const G_TYPE_FUNDAMENTAL_SHIFT: u8 = 2;
pub const G_TYPE_INVALID: GType = 0 << G_TYPE_FUNDAMENTAL_SHIFT;
pub const G_TYPE_NONE: GType = 1 << G_TYPE_FUNDAMENTAL_SHIFT;
pub const G_TYPE_INTERFACE: GType = 2 << G_TYPE_FUNDAMENTAL_SHIFT;
pub const G_TYPE_CHAR: GType = 3 << G_TYPE_FUNDAMENTAL_SHIFT;
pub const G_TYPE_UCHAR: GType = 4 << G_TYPE_FUNDAMENTAL_SHIFT;
pub const G_TYPE_BOOLEAN: GType = 5 << G_TYPE_FUNDAMENTAL_SHIFT;
pub const G_TYPE_INT: GType = 6 << G_TYPE_FUNDAMENTAL_SHIFT;
pub const G_TYPE_UINT: GType = 7 << G_TYPE_FUNDAMENTAL_SHIFT;
pub const G_TYPE_LONG: GType = 8 << G_TYPE_FUNDAMENTAL_SHIFT;
pub const G_TYPE_ULONG: GType = 9 << G_TYPE_FUNDAMENTAL_SHIFT;
pub const G_TYPE_INT64: GType = 10 << G_TYPE_FUNDAMENTAL_SHIFT;
pub const G_TYPE_UINT64: GType = 11 << G_TYPE_FUNDAMENTAL_SHIFT;
pub const G_TYPE_ENUM: GType = 12 << G_TYPE_FUNDAMENTAL_SHIFT;
pub const G_TYPE_FLAGS: GType = 13 << G_TYPE_FUNDAMENTAL_SHIFT;
pub const G_TYPE_FLOAT: GType = 14 << G_TYPE_FUNDAMENTAL_SHIFT;
pub const G_TYPE_DOUBLE: GType = 15 << G_TYPE_FUNDAMENTAL_SHIFT;
pub const G_TYPE_STRING: GType = 16 << G_TYPE_FUNDAMENTAL_SHIFT;
pub const G_TYPE_POINTER: GType = 17 << G_TYPE_FUNDAMENTAL_SHIFT;
pub const G_TYPE_BOXED: GType = 18 << G_TYPE_FUNDAMENTAL_SHIFT;
pub const G_TYPE_PARAM: GType = 19 << G_TYPE_FUNDAMENTAL_SHIFT;
pub const G_TYPE_OBJECT: GType = 20 << G_TYPE_FUNDAMENTAL_SHIFT;
pub const G_TYPE_VARIANT: GType = 21 << G_TYPE_FUNDAMENTAL_SHIFT;

extern "C" {

    pub fn g_free                          (ptr: gpointer);
    //=========================================================================
    // GSList
    //=========================================================================
    pub fn g_slist_free                    (list: *mut C_GSList);
    pub fn g_slist_append                  (list: *mut C_GSList, data: *mut c_void) -> *mut C_GSList;
    pub fn g_slist_prepend                 (list: *mut C_GSList, data: *mut c_void) -> *mut C_GSList;
    pub fn g_slist_insert                  (list: *mut C_GSList, data: *mut c_void, position: c_int) -> *mut C_GSList;
    pub fn g_slist_concat                  (list: *mut C_GSList, list2: *mut C_GSList) -> *mut C_GSList;
    pub fn g_slist_nth_data                (list: *mut C_GSList, n: c_uint) -> *mut c_void;
    pub fn g_slist_length                  (list: *mut C_GSList) -> c_uint;
    pub fn g_slist_last                    (list: *mut C_GSList) -> *mut C_GSList;
    pub fn g_slist_copy                    (list: *mut C_GSList) -> *mut C_GSList;
    pub fn g_slist_reverse                 (list: *mut C_GSList) -> *mut C_GSList;
    // pub fn g_slist_free_full               (list: *C_GSList, GDestroyNotify    free_func);
    // pub fn g_slist_free_1                  (list: *C_GSList);
    // pub fn g_slist_insert_sorted           (list: *C_GSList, data: *c_void, GCompareFunc      func) -> *C_GSList;
    // pub fn g_slist_insert_sorted_with_data (list: *C_GSList, data: *c_void, GCompareDataFunc  func, gpointer          user_data) -> *C_GSList;
    // pub fn g_slist_insert_before           (list: *C_GSList, GSList           *sibling, gpointer          data) -> *C_GSList;
    pub fn g_slist_remove                  (list: *mut C_GSList, data: *mut c_void) -> *mut C_GSList;
    pub fn g_slist_remove_all              (list: *mut C_GSList, data: *mut c_void) -> *mut C_GSList;
    pub fn g_slist_remove_link             (list: *mut C_GSList, link_: C_GSList) -> *mut C_GSList;
    pub fn g_slist_delete_link             (list: *mut C_GSList, link_: C_GSList) -> *mut C_GSList;
    pub fn g_slist_find                    (list: *mut C_GSList, data: *mut c_void) -> *mut C_GSList;
    // pub fn g_slist_find_custom             (list: *C_GSList, data: *c_void, GCompareFunc      func) -> *C_GSList;
    pub fn g_slist_position                (list: *mut C_GSList, link_: C_GSList) -> c_int;
    // pub fn g_slist_index                   (list: *C_GSList, data: *c_void) -> c_int;

    //=========================================================================
    // GList
    //=========================================================================
    pub fn g_list_free                    (list: *mut C_GList);
    pub fn g_list_append                  (list: *mut C_GList, data: *mut c_void) -> *mut C_GList;
    pub fn g_list_prepend                 (list: *mut C_GList, data: *mut c_void) -> *mut C_GList;
    pub fn g_list_insert                  (list: *mut C_GList, data: *mut c_void, position: c_int) -> *mut C_GList;
    pub fn g_list_concat                  (list: *mut C_GList, list2: *mut C_GList) -> *mut C_GList;
    pub fn g_list_nth_data                (list: *mut C_GList, n: c_uint) -> *mut c_void;
    pub fn g_list_length                  (list: *mut C_GList) -> c_uint;
    pub fn g_list_last                    (list: *mut C_GList) -> *mut C_GList;
    pub fn g_list_first                    (list: *mut C_GList) -> *mut C_GList;
    pub fn g_list_copy                    (list: *mut C_GList) -> *mut C_GList;
    pub fn g_list_reverse                 (list: *mut C_GList) -> *mut C_GList;
    // pub fn g_slist_free_full               (list: *C_GSList, GDestroyNotify    free_func);
    // pub fn g_slist_free_1                  (list: *C_GSList);
    // pub fn g_slist_insert_sorted           (list: *C_GSList, data: *c_void, GCompareFunc      func) -> *C_GSList;
    // pub fn g_slist_insert_sorted_with_data (list: *C_GSList, data: *c_void, GCompareDataFunc  func, gpointer          user_data) -> *C_GSList;
    // pub fn g_slist_insert_before           (list: *C_GSList, GSList           *sibling, gpointer          data) -> *C_GSList;
    pub fn g_list_remove                  (list: *mut C_GList, data: *mut c_void) -> *mut C_GList;
    pub fn g_list_remove_all              (list: *mut C_GList, data: *mut c_void) -> *mut C_GList;
    pub fn g_list_remove_link             (list: *mut C_GList, link_: C_GList) -> *mut C_GList;
    pub fn g_list_delete_link             (list: *mut C_GList, link_: C_GList) -> *mut C_GList;
    pub fn g_list_find                    (list: *mut C_GList, data: *mut c_void) -> *mut C_GList;
    // pub fn g_slist_find_custom             (list: *C_GSList, data: *c_void, GCompareFunc      func) -> *C_GSList;
    pub fn g_list_position                (list: *mut C_GList, link_: C_GList) -> c_int;
    // pub fn g_slist_index                   (list: *C_GSList, data: *c_void) -> c_int;



    //=========================================================================
    // GError
    //=========================================================================
    //pub fn g_error_new                    (domain: GQuark, code: c_int, format: *c_char, ...) -> *C_GError;
    pub fn g_error_new_literal            (domain: GQuark, code: c_int, message: *const c_char) -> *mut C_GError;
    //pub fn g_error_new_valist             (domain: GQuark, code: c_int, fomat: *c_char, args: va_list) -> *C_GError;
    pub fn g_error_free                   (error: *mut C_GError) -> ();
    pub fn g_error_copy                   (error: *mut C_GError) -> *mut C_GError;
    pub fn g_error_matches                (error: *mut C_GError, domain: GQuark, code: c_int) -> Gboolean;
    //pub fn g_set_error                    (error: **C_GError, domain: GQuark, code: c_int, format: *c_char, ...) -> ();
    pub fn g_set_error_literal            (error: *mut *mut C_GError, domain: GQuark, code: c_int, message: *const c_char) -> ();
    pub fn g_propagate_error              (dest: *mut *mut C_GError, src: *mut C_GError) -> ();
    pub fn g_clear_error                  (err: *mut *mut C_GError) -> ();
    //pub fn g_prefix_error                 (err: **C_GError, format: *c_char, ...) -> ();
    //pub fn g_propagate_prefixed_error     (dest: **C_GError, src: *C_GError, format: *c_char, ...) -> ();

    //=========================================================================
    // GPermission                                                       NOT OK
    //=========================================================================
    pub fn g_permission_get_allowed     (permission: *mut C_GPermission) -> Gboolean;
    pub fn g_permission_get_can_acquire (permission: *mut C_GPermission) -> Gboolean;
    pub fn g_permission_get_can_release (permission: *mut C_GPermission) -> Gboolean;
    //pub fn g_permission_acquire         (permission: *mut C_GPermission, cancellable: *mut C_GCancellable,
    //    error: *mut *mut C_GError) -> Gboolean;
    //pub fn g_permission_acquire_async   (permission: *mut C_GPermission, cancellable: *mut C_GCancellable,
    //    callback: GAsyncReadyCallback, user_data: gpointer);
    //pub fn g_permission_acquire_finish  (permission: *mut C_GPermission, result: *mut C_GAsyncResult,
    //    error: *mut *mut C_GError) -> Gboolean;
    //pub fn g_permission_release         (permission: *mut C_GPermission, cancellable: *mut C_GCancellable,
    //    error: *mut *mut C_GError) -> Gboolean;
    //pub fn g_permission_release_async   (permission: *mut C_GPermission, cancellable: *mut C_GCancellable,
    //    callback: GAsyncReadyCallback, user_data: gpointer);
    //pub fn g_permission_release_finish  (permission: *mut C_GPermission, cancellable: *mut C_GCancellable,
    //    error: *mut *mut C_GError) -> Gboolean;
    pub fn g_permission_impl_update     (permission: *mut C_GPermission, allowed: Gboolean, can_acquire: Gboolean, can_release: Gboolean);

    //pub type GAsyncReadyCallback = Option<extern "C" fn(source_object: *mut C_GObject, res: *mut C_GAsyncResult, user_data: gpointer)>;

    //=========================================================================
    // GObject
    //=========================================================================
    pub fn g_object_ref(object: *mut c_void) -> *mut c_void;
    pub fn g_object_ref_sink(object: *mut c_void) -> *mut c_void;
    pub fn g_object_unref(object: *mut c_void);

    pub fn glue_signal_connect(g_object: *mut C_GObject,
                               signal: *const c_char,
                               func: Option<extern "C" fn()>,
                               user_data: *const c_void);

    pub fn g_type_check_instance_is_a(type_instance: gconstpointer,
                                      iface_type: GType) -> Gboolean;

    //=========================================================================
    // GValue
    //=========================================================================
    pub fn create_gvalue                       () -> *mut C_GValue;
    pub fn get_gtype                           (_type: GType) -> GType;
    pub fn g_value_init                        (value: *mut C_GValue, _type: GType);
    pub fn g_value_reset                       (value: *mut C_GValue);
    pub fn g_value_unset                       (value: *mut C_GValue);
    pub fn g_strdup_value_contents             (value: *mut C_GValue) -> *mut c_char;
    pub fn g_value_set_boolean                 (value: *mut C_GValue, b: Gboolean);
    pub fn g_value_get_boolean                 (value: *const C_GValue) -> Gboolean;
    pub fn g_value_set_schar                   (value: *mut C_GValue, b: c_char);
    pub fn g_value_get_schar                   (value: *const C_GValue) -> c_char;
    pub fn g_value_set_uchar                   (value: *mut C_GValue, b: c_uchar);
    pub fn g_value_get_uchar                   (value: *const C_GValue) -> c_uchar;
    pub fn g_value_set_int                     (value: *mut C_GValue, b: c_int);
    pub fn g_value_get_int                     (value: *const C_GValue) -> c_int;
    pub fn g_value_set_uint                    (value: *mut C_GValue, b: c_uint);
    pub fn g_value_get_uint                    (value: *const C_GValue) -> c_uint;
    pub fn g_value_set_long                    (value: *mut C_GValue, b: c_long);
    pub fn g_value_get_long                    (value: *const C_GValue) -> c_long;
    pub fn g_value_set_ulong                   (value: *mut C_GValue, b: c_ulong);
    pub fn g_value_get_ulong                   (value: *const C_GValue) -> c_ulong;
    pub fn g_value_set_int64                   (value: *mut C_GValue, b: i64);
    pub fn g_value_get_int64                   (value: *const C_GValue) -> i64;
    pub fn g_value_set_uint64                  (value: *mut C_GValue, b: u64);
    pub fn g_value_get_uint64                  (value: *const C_GValue) -> u64;
    pub fn g_value_set_float                   (value: *mut C_GValue, b: c_float);
    pub fn g_value_get_float                   (value: *const C_GValue) -> c_float;
    pub fn g_value_set_double                  (value: *mut C_GValue, b: c_double);
    pub fn g_value_get_double                  (value: *const C_GValue) -> c_double;
    pub fn g_value_set_enum                    (value: *mut C_GValue, b: GType);
    pub fn g_value_get_enum                    (value: *const C_GValue) -> GType;
    pub fn g_value_set_flags                   (value: *mut C_GValue, b: GType);
    pub fn g_value_get_flags                   (value: *const C_GValue) -> GType;
    pub fn g_value_set_string                  (value: *mut C_GValue, b: *const c_char);
    pub fn g_value_set_static_string           (value: *mut C_GValue, b: *const c_char);
    pub fn g_value_get_string                  (value: *const C_GValue) -> *const c_char;
    pub fn g_value_dup_string                  (value: *mut C_GValue) -> *mut c_char;
    pub fn g_value_set_boxed                   (value: *mut C_GValue, b: *const c_void);
    pub fn g_value_set_static_boxed            (value: *mut C_GValue, b: *const c_void);
    pub fn g_value_get_boxed                   (value: *const C_GValue) -> *const c_void;
    pub fn g_value_set_pointer                 (value: *mut C_GValue, b: *const c_void);
    pub fn g_value_get_pointer                 (value: *const C_GValue) -> *const c_void;
    pub fn g_value_set_object                  (value: *mut C_GValue, b: *const c_void);
    pub fn g_value_take_object                 (value: *mut C_GValue, b: *const c_void);
    pub fn g_value_get_object                  (value: *const C_GValue) -> *const c_void;
    pub fn g_value_set_gtype                   (value: *mut C_GValue, b: GType);
    pub fn g_value_get_gtype                   (value: *const C_GValue) -> GType;
    pub fn g_value_type_compatible             (src_type: GType, dest_type: GType) -> Gboolean;
    pub fn g_value_type_transformable          (src_type: GType, dest_type: GType) -> Gboolean;

    //=========================================================================
    // GMainLoop
    //=========================================================================
    pub fn g_main_loop_new                     (context: *mut C_GMainContext, is_running: Gboolean) -> *mut C_GMainLoop;
    pub fn g_main_loop_ref                     (loop_: *mut C_GMainLoop) -> *mut C_GMainLoop;
    pub fn g_main_loop_unref                   (loop_: *mut C_GMainLoop);
    pub fn g_main_loop_run                     (loop_: *mut C_GMainLoop);
    pub fn g_main_loop_quit                    (loop_: *mut C_GMainLoop);
    pub fn g_main_loop_is_running              (loop_: *mut C_GMainLoop) -> Gboolean;
    pub fn g_main_loop_get_context             (loop_: *mut C_GMainLoop) -> *mut C_GMainContext;

    //=========================================================================
    // GMainContext
    //=========================================================================
    pub fn g_main_context_new                  () -> *mut C_GMainContext;
    pub fn g_main_context_ref                  (context: *mut C_GMainContext) -> *mut C_GMainContext;
    pub fn g_main_context_unref                (context: *mut C_GMainContext);
    pub fn g_main_context_default              () -> *mut C_GMainContext;
    pub fn g_main_context_iteration            (context: *mut C_GMainContext, may_block: Gboolean) -> Gboolean;
    pub fn g_main_context_pending              (context: *mut C_GMainContext) -> Gboolean;
    pub fn g_main_context_find_source_by_id    (context: *mut C_GMainContext, source_id: c_uint) -> *mut C_GSource;
    pub fn g_main_context_find_source_by_user_data(context: *mut C_GMainContext, user_data: gpointer) -> *mut C_GSource;
    //pub fn g_main_context_find_source_by_funcs_user_data(context: *mut C_GMainContext, funcs: GSourceFuncs, user_data: gpointer) -> *mut C_GSource;
    pub fn g_main_context_wakeup               (context: *mut C_GMainContext);
    pub fn g_main_context_acquire              (context: *mut C_GMainContext) -> Gboolean;
    pub fn g_main_context_release              (context: *mut C_GMainContext);
    pub fn g_main_context_is_owner             (context: *mut C_GMainContext) -> Gboolean;
    //pub fn g_main_context_wait                 (context: *mut C_GMainContext, cond: *mut C_GCond, mutex: *mut C_GMutex) -> Gboolean;
    pub fn g_main_context_prepare              (context: *mut C_GMainContext, priority: *mut c_int) -> Gboolean;
    //pub fn g_main_context_query                (context: *mut C_GMainContext, max_priority: c_int, timeout_: *mut c_int, fds: *mut C_GPollFD,
    //    n_fds: c_int) -> c_int;
    //pub fn g_main_context_check                (context: *mut C_GMainContext, max_priority: c_int, fds: *mut C_GPollFD,
    //    n_fds: c_int) -> c_int;
    pub fn g_main_context_dispatch             (context: *mut C_GMainContext);
    //pub fn g_main_context_set_poll_func        ();
    //pub fn g_main_context_get_poll_func        ();
    pub fn g_main_context_add_poll             (context: *mut C_GMainContext, fd: *mut C_GPollFD, priority: c_int);
    pub fn g_main_context_remove_poll          (context: *mut C_GMainContext, fd: *mut C_GPollFD);
    pub fn g_main_depth                        () -> c_int;

    pub fn g_main_current_source               () -> *mut C_GSource;
    //pub fn g_main_context_invoke               ();
    //pub fn g_main_context_invoke_full          ();
    pub fn g_main_context_get_thread_default   () -> *mut C_GMainContext;
    pub fn g_main_context_ref_thread_default   () -> *mut C_GMainContext;
    pub fn g_main_context_push_thread_default  (context: *mut C_GMainContext);
    pub fn g_main_context_pop_thread_default   (context: *mut C_GMainContext);

    //=========================================================================
    // GSource
    //=========================================================================
    pub fn g_timeout_source_new                () -> *mut C_GSource;
    pub fn g_timeout_source_new_seconds        (interval: c_uint) -> *mut C_GSource;
    //pub fn g_timeout_add                       (interval: c_uint, function: GSourceFunc, data: gpointer) -> c_uint;
    pub fn g_timeout_add                       (interval: c_uint, function: gpointer, data: gpointer) -> c_uint;
    //pub fn g_timeout_add_full                  ();
    //pub fn g_timeout_add_seconds               (interval: c_uint, function: GSourceFunc, data: gpointer) -> c_uint;
    pub fn g_timeout_add_seconds               (interval: c_uint, function: gpointer, data: gpointer) -> c_uint;
    //pub fn g_timeout_add_seconds_full          ();
    pub fn g_idle_source_new                   () -> *mut C_GSource;
    //pub fn g_idle_add                          ();
    //pub fn g_idle_add_full                     ();
    pub fn g_idle_remove_by_data               (data: gpointer) -> Gboolean;
    pub fn g_child_watch_source_new            (pid: C_GPid) -> *mut C_GSource;
    //pub fn g_child_watch_add                   ();
    //pub fn g_child_watch_add_full              ();
    pub fn g_poll                              (fds: *mut C_GPollFD, nfds: c_uint, timeout: c_int) -> c_int;
    //pub fn g_source_new                        ();
    pub fn g_source_ref                        (source: *mut C_GSource) -> *mut C_GSource;
    pub fn g_source_unref                      (source: *mut C_GSource);
    //pub fn g_source_set_funcs                  ();
    pub fn g_source_attach                     (source: *mut C_GSource, context: *mut C_GMainContext);
    pub fn g_source_destroy                    (source: *mut C_GSource);
    pub fn g_source_is_destroyed               (source: *mut C_GSource) -> Gboolean;
    pub fn g_source_set_priority               (source: *mut C_GSource, priority: c_int);
    pub fn g_source_get_priority               (source: *mut C_GSource) -> c_int;
    pub fn g_source_set_can_recurse            (source: *mut C_GSource, can_recurse: Gboolean);
    pub fn g_source_get_can_recurse            (source: *mut C_GSource) -> Gboolean;
    pub fn g_source_get_id                     (source: *mut C_GSource) -> c_uint;
    pub fn g_source_get_name                   (source: *mut C_GSource) -> *const c_char;
    pub fn g_source_set_name                   (source: *mut C_GSource, name: *const c_char);
    pub fn g_source_set_name_by_id             (tag: c_uint, name: *const c_char);
    pub fn g_source_get_context                (source: *mut C_GSource) -> *mut C_GMainContext;
    //pub fn g_source_set_callback               ();
    //pub fn g_source_set_callback_indirect      ();
    pub fn g_source_set_ready_time             (source: *mut C_GSource, ready_time: i64);
    pub fn g_source_get_ready_time             (source: *mut C_GSource) -> i64;
    //pub fn g_source_add_unix_fd                ();
    //pub fn g_source_remove_unix_fd             ();
    //pub fn g_source_modify_unix_fd             ();
    //pub fn g_source_query_unix_fd              ();
    pub fn g_source_add_poll                   (source: *mut C_GSource, fd: *mut C_GPollFD);
    pub fn g_source_remove_poll                (source: *mut C_GSource, fd: *mut C_GPollFD);
    pub fn g_source_add_child_source           (source: *mut C_GSource, child_source: *mut C_GSource);
    pub fn g_source_remove_child_source        (source: *mut C_GSource, child_source: *mut C_GSource);
    pub fn g_source_get_time                   (source: *mut C_GSource) -> i64;
    pub fn g_source_remove                     (tag: c_uint) -> Gboolean;
    //pub fn g_source_remove_by_funcs_user_data  ();
    pub fn g_source_remove_by_user_data        (user_data: gpointer) -> Gboolean;

    //=========================================================================
    // GSignal
    //=========================================================================
    pub fn g_signal_connect_data(instance: gpointer, detailed_signal: *const c_char,
                                 c_handler: GCallback, data: gpointer,
                                 destroy_data: GClosureNotify, connect_flags: c_int) -> c_ulong;

    //=========================================================================
    // GDate functions
    //=========================================================================
    pub fn g_get_current_time             (result: *mut c_void);
    pub fn g_usleep                       (microseconds: c_ulong);
    pub fn g_get_monotonic_time           () -> i64;
    pub fn g_get_real_time                () -> i64;
    pub fn g_date_get_days_in_month       (month: c_int, year: u16) -> u8;
    pub fn g_date_is_leap_year            (year: u16) -> Gboolean;
    pub fn g_date_get_monday_weeks_in_year(year: u16) -> u8;
    pub fn g_date_get_sunday_weeks_in_year(year: u16) -> u8;
    pub fn g_date_valid_day               (day: c_int) -> Gboolean;
    pub fn g_date_valid_month             (month: c_int) -> Gboolean;
    pub fn g_date_valid_year              (year: u16) -> Gboolean;
    pub fn g_date_valid_dmy               (day: c_int, month: c_int, year: u16) -> Gboolean;
    pub fn g_date_valid_julian            (julian: u32) -> Gboolean;
    pub fn g_date_valid_weekday           (year: c_int) -> Gboolean;

    //=========================================================================
    // GDate
    //=========================================================================
    pub fn g_date_new             () -> *mut GDate;
    pub fn g_date_new_dmy         (day: c_int, month: c_int, year: u16) -> *mut GDate;
    pub fn g_date_new_julian      (julian_day: u32) -> *mut GDate;
    pub fn g_date_clear           (date: *mut GDate, n_dates: c_uint);
    pub fn g_date_free            (date: *mut GDate);
    pub fn g_date_set_day         (date: *mut GDate, day: c_int);
    pub fn g_date_set_month       (date: *mut GDate, month: c_int);
    pub fn g_date_set_year        (date: *mut GDate, year: u16);
    pub fn g_date_set_dmy         (date: *mut GDate, day: c_int, month: c_int, year: u16);
    pub fn g_date_set_julian      (date: *mut GDate, julian: u32);
    pub fn g_date_set_time_t      (date: *mut GDate, timet: i64);
    pub fn g_date_set_time_val    (date: *mut GDate, timeval: *mut c_void);
    pub fn g_date_set_parse       (date: *mut GDate, str_: *const c_char);
    pub fn g_date_add_days        (date: *mut GDate, days: c_uint);
    pub fn g_date_subtract_days   (date: *mut GDate, days: c_uint);
    pub fn g_date_add_months      (date: *mut GDate, months: c_uint);
    pub fn g_date_subtract_months (date: *mut GDate, months: c_uint);
    pub fn g_date_add_years       (date: *mut GDate, years: c_uint);
    pub fn g_date_subtract_years  (date: *mut GDate, years: c_uint);
    pub fn g_date_between         (date1: *const GDate, date2: *const GDate) -> c_int;
    pub fn g_date_compare         (lhs: *const GDate, rhs: *const GDate) -> c_int;
    pub fn g_date_clamp           (date: *mut GDate, min_date: *const GDate, max_date: *const GDate);
    pub fn g_date_order           (date1: *mut GDate, date2: *mut GDate);
    pub fn g_date_get_day         (date: *const GDate) -> u8;
    pub fn g_date_get_month       (date: *const GDate) -> c_int;
    pub fn g_date_get_year        (date: *const GDate) -> u16;
    pub fn g_date_get_julian      (date: *const GDate) -> u32;
    pub fn g_date_get_weekday     (date: *const GDate) -> c_int;
    pub fn g_date_get_day_of_year (date: *const GDate) -> c_uint;
    pub fn g_date_is_first_of_month(date: *const GDate) -> Gboolean;
    pub fn g_date_is_last_of_month(date: *const GDate) -> Gboolean;
    pub fn g_date_get_monday_week_of_year(date: *const GDate) -> c_uint;
    pub fn g_date_get_sunday_week_of_year(date: *const GDate) -> c_uint;
    pub fn g_date_get_iso8601_week_of_year(date: *const GDate) -> c_uint;
    pub fn g_date_strftime        (s: *mut c_char, slen: u32, format: *const c_char, date: *const GDate) -> u32;
    //pub fn g_date_to_struct_tm    (date: *const GDate, tm: *mut struct tm);
    pub fn g_date_valid           (date: *const GDate) -> Gboolean;

    //=========================================================================
    // GTimeVal
    //=========================================================================
    pub fn g_time_val_add         (time_: *mut c_void, microseconds: c_ulong);
    pub fn g_time_val_from_iso8601(iso_date: *const c_char, time_: *mut c_void);
    pub fn g_time_val_to_iso8601  (time_: *mut c_void) -> *mut c_char;
}
