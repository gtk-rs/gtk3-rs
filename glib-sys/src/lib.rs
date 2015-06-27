// Copyright 2013-2015, The Rust-GNOME Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

#![allow(non_camel_case_types)]

extern crate libc;

use libc::{c_void, c_int, c_uint, c_float, c_double, c_char, c_uchar, c_long, c_ulong, size_t};

pub type GQuark = u32;

pub type gsize = size_t;
pub type GType = gsize;

pub type gboolean = c_int;
pub const GFALSE:  c_int = 0;
pub const GTRUE:   c_int = 1;

pub type gconstpointer = *const c_void;
pub type gpointer = *mut c_void;

pub type GSourceFunc = extern "C" fn(user_data: gpointer) -> gboolean;
pub type GCallback = extern "C" fn();
pub type GClosureNotify = extern "C" fn(data: gpointer, closure: gpointer);
pub type GDestroyNotify = extern "C" fn(data: gpointer);
pub type GHashFunc = unsafe extern "C" fn(v: gconstpointer) -> c_uint;
pub type GEqualFunc = unsafe extern "C" fn(v1: gconstpointer, v2: gconstpointer) -> gboolean;

#[repr(C)]
pub struct GAppInfo;

#[repr(C)]
pub struct GValue {
    type_: GType,
    data: [size_t; 2],
}

#[repr(C)]
pub struct GList {
    pub data: *mut c_void,
    pub next: *mut GList,
    pub prev: *mut GList
}

#[repr(C)]
pub struct GSList {
    pub data: *mut c_void,
    pub next: *mut GSList
}

#[repr(C)]
pub struct GError {
    pub domain : GQuark,
    pub code   : i32,
    pub message: *mut c_char
}

#[repr(C)]
pub struct GPermission;

#[repr(C)]
pub struct GObject;

#[repr(C)]
pub struct GMainLoop;

#[repr(C)]
pub struct GMainContext;

#[repr(C)]
pub struct GSource;

#[repr(C)]
pub struct GPid;

#[repr(C)]
pub struct GPollFD;

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

#[repr(C)]
pub struct GHashTable;

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
    pub fn g_slist_free                    (list: *mut GSList);
    pub fn g_slist_append                  (list: *mut GSList, data: *mut c_void) -> *mut GSList;
    pub fn g_slist_prepend                 (list: *mut GSList, data: *mut c_void) -> *mut GSList;
    pub fn g_slist_insert                  (list: *mut GSList, data: *mut c_void, position: c_int) -> *mut GSList;
    pub fn g_slist_concat                  (list: *mut GSList, list2: *mut GSList) -> *mut GSList;
    pub fn g_slist_nth_data                (list: *mut GSList, n: c_uint) -> *mut c_void;
    pub fn g_slist_length                  (list: *mut GSList) -> c_uint;
    pub fn g_slist_last                    (list: *mut GSList) -> *mut GSList;
    pub fn g_slist_copy                    (list: *mut GSList) -> *mut GSList;
    pub fn g_slist_reverse                 (list: *mut GSList) -> *mut GSList;
    // pub fn g_slist_free_full               (list: *GSList, GDestroyNotify    free_func);
    // pub fn g_slist_free_1                  (list: *GSList);
    // pub fn g_slist_insert_sorted           (list: *GSList, data: *c_void, GCompareFunc      func) -> *GSList;
    // pub fn g_slist_insert_sorted_with_data (list: *GSList, data: *c_void, GCompareDataFunc  func, gpointer          user_data) -> *GSList;
    // pub fn g_slist_insert_before           (list: *GSList, GSList           *sibling, gpointer          data) -> *GSList;
    pub fn g_slist_remove                  (list: *mut GSList, data: *mut c_void) -> *mut GSList;
    pub fn g_slist_remove_all              (list: *mut GSList, data: *mut c_void) -> *mut GSList;
    pub fn g_slist_remove_link             (list: *mut GSList, link_: GSList) -> *mut GSList;
    pub fn g_slist_delete_link             (list: *mut GSList, link_: GSList) -> *mut GSList;
    pub fn g_slist_find                    (list: *mut GSList, data: *mut c_void) -> *mut GSList;
    // pub fn g_slist_find_custom             (list: *GSList, data: *c_void, GCompareFunc      func) -> *GSList;
    pub fn g_slist_position                (list: *mut GSList, link_: GSList) -> c_int;
    // pub fn g_slist_index                   (list: *GSList, data: *c_void) -> c_int;

    //=========================================================================
    // GList
    //=========================================================================
    pub fn g_list_free                    (list: *mut GList);
    pub fn g_list_append                  (list: *mut GList, data: *mut c_void) -> *mut GList;
    pub fn g_list_prepend                 (list: *mut GList, data: *mut c_void) -> *mut GList;
    pub fn g_list_insert                  (list: *mut GList, data: *mut c_void, position: c_int) -> *mut GList;
    pub fn g_list_concat                  (list: *mut GList, list2: *mut GList) -> *mut GList;
    pub fn g_list_nth_data                (list: *mut GList, n: c_uint) -> *mut c_void;
    pub fn g_list_length                  (list: *mut GList) -> c_uint;
    pub fn g_list_last                    (list: *mut GList) -> *mut GList;
    pub fn g_list_first                    (list: *mut GList) -> *mut GList;
    pub fn g_list_copy                    (list: *mut GList) -> *mut GList;
    pub fn g_list_reverse                 (list: *mut GList) -> *mut GList;
    // pub fn g_slist_free_full               (list: *GSList, GDestroyNotify    free_func);
    // pub fn g_slist_free_1                  (list: *GSList);
    // pub fn g_slist_insert_sorted           (list: *GSList, data: *c_void, GCompareFunc      func) -> *GSList;
    // pub fn g_slist_insert_sorted_with_data (list: *GSList, data: *c_void, GCompareDataFunc  func, gpointer          user_data) -> *GSList;
    // pub fn g_slist_insert_before           (list: *GSList, GSList           *sibling, gpointer          data) -> *GSList;
    pub fn g_list_remove                  (list: *mut GList, data: *mut c_void) -> *mut GList;
    pub fn g_list_remove_all              (list: *mut GList, data: *mut c_void) -> *mut GList;
    pub fn g_list_remove_link             (list: *mut GList, link_: GList) -> *mut GList;
    pub fn g_list_delete_link             (list: *mut GList, link_: GList) -> *mut GList;
    pub fn g_list_find                    (list: *mut GList, data: *mut c_void) -> *mut GList;
    // pub fn g_slist_find_custom             (list: *GSList, data: *c_void, GCompareFunc      func) -> *GSList;
    pub fn g_list_position                (list: *mut GList, link_: GList) -> c_int;
    // pub fn g_slist_index                   (list: *GSList, data: *c_void) -> c_int;

    //=========================================================================
    // GAppInfo
    //=========================================================================
    pub fn g_app_info_get_type            () -> GType;

    //=========================================================================
    // GError
    //=========================================================================
    //pub fn g_error_new                    (domain: GQuark, code: c_int, format: *c_char, ...) -> *GError;
    pub fn g_error_new_literal            (domain: GQuark, code: c_int, message: *const c_char) -> *mut GError;
    //pub fn g_error_new_valist             (domain: GQuark, code: c_int, fomat: *c_char, args: va_list) -> *GError;
    pub fn g_error_free                   (error: *mut GError) -> ();
    pub fn g_error_copy                   (error: *mut GError) -> *mut GError;
    pub fn g_error_matches                (error: *mut GError, domain: GQuark, code: c_int) -> gboolean;
    //pub fn g_set_error                    (error: **GError, domain: GQuark, code: c_int, format: *c_char, ...) -> ();
    pub fn g_set_error_literal            (error: *mut *mut GError, domain: GQuark, code: c_int, message: *const c_char) -> ();
    pub fn g_propagate_error              (dest: *mut *mut GError, src: *mut GError) -> ();
    pub fn g_clear_error                  (err: *mut *mut GError) -> ();
    //pub fn g_prefix_error                 (err: **GError, format: *c_char, ...) -> ();
    //pub fn g_propagate_prefixed_error     (dest: **GError, src: *GError, format: *c_char, ...) -> ();

    //=========================================================================
    // GPermission                                                       NOT OK
    //=========================================================================
    pub fn g_permission_get_allowed     (permission: *mut GPermission) -> gboolean;
    pub fn g_permission_get_can_acquire (permission: *mut GPermission) -> gboolean;
    pub fn g_permission_get_can_release (permission: *mut GPermission) -> gboolean;
    //pub fn g_permission_acquire         (permission: *mut GPermission, cancellable: *mut GCancellable,
    //    error: *mut *mut GError) -> gboolean;
    //pub fn g_permission_acquire_async   (permission: *mut GPermission, cancellable: *mut GCancellable,
    //    callback: GAsyncReadyCallback, user_data: gpointer);
    //pub fn g_permission_acquire_finish  (permission: *mut GPermission, result: *mut GAsyncResult,
    //    error: *mut *mut GError) -> gboolean;
    //pub fn g_permission_release         (permission: *mut GPermission, cancellable: *mut GCancellable,
    //    error: *mut *mut GError) -> gboolean;
    //pub fn g_permission_release_async   (permission: *mut GPermission, cancellable: *mut GCancellable,
    //    callback: GAsyncReadyCallback, user_data: gpointer);
    //pub fn g_permission_release_finish  (permission: *mut GPermission, cancellable: *mut GCancellable,
    //    error: *mut *mut GError) -> gboolean;
    pub fn g_permission_impl_update     (permission: *mut GPermission, allowed: gboolean, can_acquire: gboolean, can_release: gboolean);

    //pub type GAsyncReadyCallback = Option<extern "C" fn(source_object: *mut GObject, res: *mut GAsyncResult, user_data: gpointer)>;

    //=========================================================================
    // GObject
    //=========================================================================
    pub fn g_object_ref(object: *mut c_void) -> *mut c_void;
    pub fn g_object_ref_sink(object: *mut c_void) -> *mut c_void;
    pub fn g_object_unref(object: *mut c_void);

    pub fn glue_signal_connect(g_object: *mut GObject,
                               signal: *const c_char,
                               func: Option<extern "C" fn()>,
                               user_data: *const c_void);

    pub fn g_type_check_instance_is_a(type_instance: gconstpointer,
                                      iface_type: GType) -> gboolean;

    //=========================================================================
    // GValue
    //=========================================================================
    pub fn create_gvalue                       () -> *mut GValue;
    pub fn get_gtype                           (_type: GType) -> GType;
    pub fn g_value_init                        (value: *mut GValue, _type: GType);
    pub fn g_value_reset                       (value: *mut GValue);
    pub fn g_value_unset                       (value: *mut GValue);
    pub fn g_strdup_value_contents             (value: *mut GValue) -> *mut c_char;
    pub fn g_value_set_boolean                 (value: *mut GValue, b: gboolean);
    pub fn g_value_get_boolean                 (value: *const GValue) -> gboolean;
    pub fn g_value_set_schar                   (value: *mut GValue, b: c_char);
    pub fn g_value_get_schar                   (value: *const GValue) -> c_char;
    pub fn g_value_set_uchar                   (value: *mut GValue, b: c_uchar);
    pub fn g_value_get_uchar                   (value: *const GValue) -> c_uchar;
    pub fn g_value_set_int                     (value: *mut GValue, b: c_int);
    pub fn g_value_get_int                     (value: *const GValue) -> c_int;
    pub fn g_value_set_uint                    (value: *mut GValue, b: c_uint);
    pub fn g_value_get_uint                    (value: *const GValue) -> c_uint;
    pub fn g_value_set_long                    (value: *mut GValue, b: c_long);
    pub fn g_value_get_long                    (value: *const GValue) -> c_long;
    pub fn g_value_set_ulong                   (value: *mut GValue, b: c_ulong);
    pub fn g_value_get_ulong                   (value: *const GValue) -> c_ulong;
    pub fn g_value_set_int64                   (value: *mut GValue, b: i64);
    pub fn g_value_get_int64                   (value: *const GValue) -> i64;
    pub fn g_value_set_uint64                  (value: *mut GValue, b: u64);
    pub fn g_value_get_uint64                  (value: *const GValue) -> u64;
    pub fn g_value_set_float                   (value: *mut GValue, b: c_float);
    pub fn g_value_get_float                   (value: *const GValue) -> c_float;
    pub fn g_value_set_double                  (value: *mut GValue, b: c_double);
    pub fn g_value_get_double                  (value: *const GValue) -> c_double;
    pub fn g_value_set_enum                    (value: *mut GValue, b: GType);
    pub fn g_value_get_enum                    (value: *const GValue) -> GType;
    pub fn g_value_set_flags                   (value: *mut GValue, b: GType);
    pub fn g_value_get_flags                   (value: *const GValue) -> GType;
    pub fn g_value_set_string                  (value: *mut GValue, b: *const c_char);
    pub fn g_value_set_static_string           (value: *mut GValue, b: *const c_char);
    pub fn g_value_get_string                  (value: *const GValue) -> *const c_char;
    pub fn g_value_dup_string                  (value: *mut GValue) -> *mut c_char;
    pub fn g_value_set_boxed                   (value: *mut GValue, b: *const c_void);
    pub fn g_value_set_static_boxed            (value: *mut GValue, b: *const c_void);
    pub fn g_value_get_boxed                   (value: *const GValue) -> *const c_void;
    pub fn g_value_set_pointer                 (value: *mut GValue, b: *const c_void);
    pub fn g_value_get_pointer                 (value: *const GValue) -> *const c_void;
    pub fn g_value_set_object                  (value: *mut GValue, b: *const c_void);
    pub fn g_value_take_object                 (value: *mut GValue, b: *const c_void);
    pub fn g_value_get_object                  (value: *const GValue) -> *const c_void;
    pub fn g_value_set_gtype                   (value: *mut GValue, b: GType);
    pub fn g_value_get_gtype                   (value: *const GValue) -> GType;
    pub fn g_value_type_compatible             (src_type: GType, dest_type: GType) -> gboolean;
    pub fn g_value_type_transformable          (src_type: GType, dest_type: GType) -> gboolean;

    //=========================================================================
    // GMainLoop
    //=========================================================================
    pub fn g_main_loop_new                     (context: *mut GMainContext, is_running: gboolean) -> *mut GMainLoop;
    pub fn g_main_loop_ref                     (loop_: *mut GMainLoop) -> *mut GMainLoop;
    pub fn g_main_loop_unref                   (loop_: *mut GMainLoop);
    pub fn g_main_loop_run                     (loop_: *mut GMainLoop);
    pub fn g_main_loop_quit                    (loop_: *mut GMainLoop);
    pub fn g_main_loop_is_running              (loop_: *mut GMainLoop) -> gboolean;
    pub fn g_main_loop_get_context             (loop_: *mut GMainLoop) -> *mut GMainContext;

    //=========================================================================
    // GMainContext
    //=========================================================================
    pub fn g_main_context_new                  () -> *mut GMainContext;
    pub fn g_main_context_ref                  (context: *mut GMainContext) -> *mut GMainContext;
    pub fn g_main_context_unref                (context: *mut GMainContext);
    pub fn g_main_context_default              () -> *mut GMainContext;
    pub fn g_main_context_iteration            (context: *mut GMainContext, may_block: gboolean) -> gboolean;
    pub fn g_main_context_pending              (context: *mut GMainContext) -> gboolean;
    pub fn g_main_context_find_source_by_id    (context: *mut GMainContext, source_id: c_uint) -> *mut GSource;
    pub fn g_main_context_find_source_by_user_data(context: *mut GMainContext, user_data: gpointer) -> *mut GSource;
    //pub fn g_main_context_find_source_by_funcs_user_data(context: *mut GMainContext, funcs: GSourceFuncs, user_data: gpointer) -> *mut GSource;
    pub fn g_main_context_wakeup               (context: *mut GMainContext);
    pub fn g_main_context_acquire              (context: *mut GMainContext) -> gboolean;
    pub fn g_main_context_release              (context: *mut GMainContext);
    pub fn g_main_context_is_owner             (context: *mut GMainContext) -> gboolean;
    //pub fn g_main_context_wait                 (context: *mut GMainContext, cond: *mut GCond, mutex: *mut GMutex) -> gboolean;
    pub fn g_main_context_prepare              (context: *mut GMainContext, priority: *mut c_int) -> gboolean;
    //pub fn g_main_context_query                (context: *mut GMainContext, max_priority: c_int, timeout_: *mut c_int, fds: *mut GPollFD,
    //    n_fds: c_int) -> c_int;
    //pub fn g_main_context_check                (context: *mut GMainContext, max_priority: c_int, fds: *mut GPollFD,
    //    n_fds: c_int) -> c_int;
    pub fn g_main_context_dispatch             (context: *mut GMainContext);
    //pub fn g_main_context_set_poll_func        ();
    //pub fn g_main_context_get_poll_func        ();
    pub fn g_main_context_add_poll             (context: *mut GMainContext, fd: *mut GPollFD, priority: c_int);
    pub fn g_main_context_remove_poll          (context: *mut GMainContext, fd: *mut GPollFD);
    pub fn g_main_depth                        () -> c_int;

    pub fn g_main_current_source               () -> *mut GSource;
    //pub fn g_main_context_invoke               ();
    //pub fn g_main_context_invoke_full          ();
    pub fn g_main_context_get_thread_default   () -> *mut GMainContext;
    pub fn g_main_context_ref_thread_default   () -> *mut GMainContext;
    pub fn g_main_context_push_thread_default  (context: *mut GMainContext);
    pub fn g_main_context_pop_thread_default   (context: *mut GMainContext);

    //=========================================================================
    // GSource
    //=========================================================================
    pub fn g_timeout_source_new                () -> *mut GSource;
    pub fn g_timeout_source_new_seconds        (interval: c_uint) -> *mut GSource;
    //pub fn g_timeout_add                       (interval: c_uint, function: GSourceFunc, data: gpointer) -> c_uint;
    pub fn g_timeout_add_full                  (priority: c_int, interval: c_uint, function: GSourceFunc, data: gpointer, notify: GDestroyNotify) -> c_uint;
    //pub fn g_timeout_add_seconds               (interval: c_uint, function: GSourceFunc, data: gpointer) -> c_uint;
    pub fn g_timeout_add_seconds_full          (priority: c_int, interval: c_uint, function: GSourceFunc, data: gpointer, notify: GDestroyNotify) -> c_uint;
    pub fn g_idle_source_new                   () -> *mut GSource;
    // pub fn g_idle_add                          (function: GSourceFunc, data: gpointer) -> c_uint;
    pub fn g_idle_add_full                     (priority: c_int, function: GSourceFunc, data: gpointer, notify: GDestroyNotify) -> c_uint;
    pub fn g_idle_remove_by_data               (data: gpointer) -> gboolean;
    pub fn g_child_watch_source_new            (pid: GPid) -> *mut GSource;
    //pub fn g_child_watch_add                   ();
    //pub fn g_child_watch_add_full              ();
    pub fn g_poll                              (fds: *mut GPollFD, nfds: c_uint, timeout: c_int) -> c_int;
    //pub fn g_source_new                        ();
    pub fn g_source_ref                        (source: *mut GSource) -> *mut GSource;
    pub fn g_source_unref                      (source: *mut GSource);
    //pub fn g_source_set_funcs                  ();
    pub fn g_source_attach                     (source: *mut GSource, context: *mut GMainContext);
    pub fn g_source_destroy                    (source: *mut GSource);
    pub fn g_source_is_destroyed               (source: *mut GSource) -> gboolean;
    pub fn g_source_set_priority               (source: *mut GSource, priority: c_int);
    pub fn g_source_get_priority               (source: *mut GSource) -> c_int;
    pub fn g_source_set_can_recurse            (source: *mut GSource, can_recurse: gboolean);
    pub fn g_source_get_can_recurse            (source: *mut GSource) -> gboolean;
    pub fn g_source_get_id                     (source: *mut GSource) -> c_uint;
    pub fn g_source_get_name                   (source: *mut GSource) -> *const c_char;
    pub fn g_source_set_name                   (source: *mut GSource, name: *const c_char);
    pub fn g_source_set_name_by_id             (tag: c_uint, name: *const c_char);
    pub fn g_source_get_context                (source: *mut GSource) -> *mut GMainContext;
    //pub fn g_source_set_callback               ();
    //pub fn g_source_set_callback_indirect      ();
    pub fn g_source_set_ready_time             (source: *mut GSource, ready_time: i64);
    pub fn g_source_get_ready_time             (source: *mut GSource) -> i64;
    //pub fn g_source_add_unix_fd                ();
    //pub fn g_source_remove_unix_fd             ();
    //pub fn g_source_modify_unix_fd             ();
    //pub fn g_source_query_unix_fd              ();
    pub fn g_source_add_poll                   (source: *mut GSource, fd: *mut GPollFD);
    pub fn g_source_remove_poll                (source: *mut GSource, fd: *mut GPollFD);
    pub fn g_source_add_child_source           (source: *mut GSource, child_source: *mut GSource);
    pub fn g_source_remove_child_source        (source: *mut GSource, child_source: *mut GSource);
    pub fn g_source_get_time                   (source: *mut GSource) -> i64;
    pub fn g_source_remove                     (tag: c_uint) -> gboolean;
    //pub fn g_source_remove_by_funcs_user_data  ();
    pub fn g_source_remove_by_user_data        (user_data: gpointer) -> gboolean;

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
    pub fn g_date_is_leap_year            (year: u16) -> gboolean;
    pub fn g_date_get_monday_weeks_in_year(year: u16) -> u8;
    pub fn g_date_get_sunday_weeks_in_year(year: u16) -> u8;
    pub fn g_date_valid_day               (day: c_int) -> gboolean;
    pub fn g_date_valid_month             (month: c_int) -> gboolean;
    pub fn g_date_valid_year              (year: u16) -> gboolean;
    pub fn g_date_valid_dmy               (day: c_int, month: c_int, year: u16) -> gboolean;
    pub fn g_date_valid_julian            (julian: u32) -> gboolean;
    pub fn g_date_valid_weekday           (year: c_int) -> gboolean;

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
    pub fn g_date_days_between    (date1: *const GDate, date2: *const GDate) -> c_int;
    pub fn g_date_compare         (lhs: *const GDate, rhs: *const GDate) -> c_int;
    pub fn g_date_clamp           (date: *mut GDate, min_date: *const GDate, max_date: *const GDate);
    pub fn g_date_order           (date1: *mut GDate, date2: *mut GDate);
    pub fn g_date_get_day         (date: *const GDate) -> u8;
    pub fn g_date_get_month       (date: *const GDate) -> c_int;
    pub fn g_date_get_year        (date: *const GDate) -> u16;
    pub fn g_date_get_julian      (date: *const GDate) -> u32;
    pub fn g_date_get_weekday     (date: *const GDate) -> c_int;
    pub fn g_date_get_day_of_year (date: *const GDate) -> c_uint;
    pub fn g_date_is_first_of_month(date: *const GDate) -> gboolean;
    pub fn g_date_is_last_of_month(date: *const GDate) -> gboolean;
    pub fn g_date_get_monday_week_of_year(date: *const GDate) -> c_uint;
    pub fn g_date_get_sunday_week_of_year(date: *const GDate) -> c_uint;
    pub fn g_date_get_iso8601_week_of_year(date: *const GDate) -> c_uint;
    pub fn g_date_strftime        (s: *mut c_char, slen: u32, format: *const c_char, date: *const GDate) -> u32;
    //pub fn g_date_to_struct_tm    (date: *const GDate, tm: *mut struct tm);
    pub fn g_date_valid           (date: *const GDate) -> gboolean;

    //=========================================================================
    // GTimeVal
    //=========================================================================
    pub fn g_time_val_add         (time_: *mut c_void, microseconds: c_ulong);
    pub fn g_time_val_from_iso8601(iso_date: *const c_char, time_: *mut c_void);
    pub fn g_time_val_to_iso8601  (time_: *mut c_void) -> *mut c_char;

    //=========================================================================
    // GHashTable
    //=========================================================================
    pub fn g_hash_table_new         (hash_func: GHashFunc, key_equal_func: GEqualFunc) -> *mut GHashTable;
//  pub fn g_hash_table_new_full    (hash_func: GHashFunc, key_equal_func: GEqualFunc, .. ) -> *mut GHashTable;
    pub fn g_hash_table_insert      (hash_table: *mut GHashTable, key: gpointer, value: gpointer) -> gboolean;
    pub fn g_hash_table_replace     (hash_table: *mut GHashTable, key: gpointer, value: gpointer) -> gboolean;
    pub fn g_hash_table_add         (hash_table: *mut GHashTable, key: gpointer) -> gboolean;
    pub fn g_hash_table_contains    (hash_table: *mut GHashTable, key: gconstpointer) -> gboolean;
    pub fn g_hash_table_size        (hash_table: *mut GHashTable) -> c_uint;
    pub fn g_hash_table_lookup      (hash_table: *mut GHashTable, key: gconstpointer) -> gpointer;
    pub fn g_hash_table_lookup_extended (hash_table: *mut GHashTable, lookup_key: gconstpointer, orig_key: gpointer, value: gpointer) -> gboolean;
//  pub fn g_hash_table_foreach     ();
//  pub fn g_hash_table_find        ();
    pub fn g_hash_table_remove      (hash_table: *mut GHashTable, key: gconstpointer) -> gboolean;
    pub fn g_hash_table_steal       (hash_table: *mut GHashTable, key: gconstpointer) -> gboolean;
//  pub fn g_hash_table_foreach_remove() -> c_uint;
//  pub fn g_hash_table_foreach_steal() -> c_uint;
    pub fn g_hash_table_remove_all  (hash_table: *mut GHashTable);
    pub fn g_hash_table_steal_all   (hash_table: *mut GHashTable);
    pub fn g_hash_table_get_keys    (hash_table: *mut GHashTable) -> *mut GList;
    pub fn g_hash_table_get_values  (hash_table: *mut GHashTable) -> *mut GList;
    pub fn g_hash_table_get_keys_as_array(hash_table: *mut GHashTable, length: *mut c_uint) -> gpointer;
    pub fn g_hash_table_destroy     (hash_table: *mut GHashTable);
    pub fn g_hash_table_ref         (hash_table: *mut GHashTable) -> *mut GHashTable;
    pub fn g_hash_table_unref       (hash_table: *mut GHashTable);
//  skipped g_hash_table_iter functions (TODO?)
    pub fn g_direct_equal           (v1: gconstpointer, v2: gconstpointer) -> gboolean;
    pub fn g_direct_hash            (v: gconstpointer) -> c_uint;
    pub fn g_int_equal              (v1: gconstpointer, v2: gconstpointer) -> gboolean;
    pub fn g_int_hash               (v: gconstpointer) -> c_uint;
    pub fn g_int64_equal            (v1: gconstpointer, v2: gconstpointer) -> gboolean;
    pub fn g_int64_hash             (v: gconstpointer) -> c_uint;
    pub fn g_double_equal           (v1: gconstpointer, v2: gconstpointer) -> gboolean;
    pub fn g_double_hash            (v: gconstpointer) -> c_uint;
    pub fn g_str_equal              (v1: gconstpointer, v2: gconstpointer) -> gboolean;
    pub fn g_str_hash               (v: gconstpointer) -> c_uint;
}
