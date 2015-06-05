// Copyright 2013-2015, The Rust-GNOME Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

//! callback timeout functions

pub mod timeout {
    use ffi;

    /// Sets a function to be called at regular intervals, with the default priority, G_PRIORITY_DEFAULT.
    /// The function is called repeatedly until it returns FALSE, at which point the timeout is
    /// automatically destroyed and the function will not be called again. The first call to the
    /// function will be at the end of the first interval .
    /// 
    /// Note that timeout functions may be delayed, due to the processing of other event sources. Thus
    /// they should not be relied on for precise timing. After each call to the timeout function, the
    /// time of the next timeout is recalculated based on the current time and the given interval (it
    /// does not try to 'catch up' time lost in delays).
    /// 
    /// If you want to have a timer in the "seconds" range and do not care about the exact time of the
    /// first call of the timer, use the g_timeout_add_seconds() function; this function allows for more
    /// optimizations and more efficient system power usage.
    /// 
    /// This internally creates a main loop source using g_timeout_source_new() and attaches it to the
    /// global GMainContext using g_source_attach(), so the callback will be invoked in whichever thread
    /// is running that main context. You can do these steps manually if you need greater control or to
    /// use a custom main context.
    /// 
    /// The interval given is in terms of monotonic time, not wall clock time. See g_get_monotonic_time().
    pub fn add<T>(interval: u32, func: fn(&mut T) -> i32, data: &T) -> u32 {
        let tmp = data as *const T;
        let tmp_f = func as ffi::gpointer;

        unsafe { ffi::g_timeout_add(interval, tmp_f, tmp as ffi::gpointer) }
    }

    /// Sets a function to be called at regular intervals with the default priority, G_PRIORITY_DEFAULT.
    /// The function is called repeatedly until it returns FALSE, at which point the timeout is automatically
    /// destroyed and the function will not be called again.
    /// 
    /// This internally creates a main loop source using g_timeout_source_new_seconds() and attaches it to
    /// the main loop context using g_source_attach(). You can do these steps manually if you need greater
    /// control. Also see g_timeout_add_seconds_full().
    /// 
    /// Note that the first call of the timer may not be precise for timeouts of one second. If you need
    /// finer precision and have such a timeout, you may want to use g_timeout_add() instead.
    /// 
    /// The interval given is in terms of monotonic time, not wall clock time. See g_get_monotonic_time().
    pub fn add_seconds<T>(interval: u32, func: fn(&mut T) -> i32, data: &T) -> u32 {
        let tmp = data as *const T;
        let tmp_f = func as ffi::gpointer;

        unsafe { ffi::g_timeout_add_seconds(interval, tmp_f, tmp as ffi::gpointer) }
    }
}
