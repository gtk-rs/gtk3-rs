// Copyright 2013-2015, The Rust-GNOME Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

//! Date and Time Functions — calendrical calculations and miscellaneous time stuff

use libc::{c_int, c_uint};
use ffi;
use std;
use super::translate::ToGlibPtr;

/// Simply a replacement for time_t. It has been deprecated since it is not equivalent to time_t on 64-bit platforms with a 64-bit time_t.
/// Unrelated to GTimer.
/// 
/// Note that GTime is defined to always be a 32-bit integer, unlike time_t which may be 64-bit on some systems. Therefore, GTime will
/// overflow in the year 2038, and you cannot use the address of a GTime variable as argument to the UNIX time() function.
pub type Time = i32;

/// Integer representing a year; G_DATE_BAD_YEAR is the invalid value. The year must be 1 or higher; negative (BC) years are not allowed.
/// The year is represented with four digits.
pub type Year = u16;
/// Integer representing a day of the month; between 1 and 31. G_DATE_BAD_DAY represents an invalid day of the month.
pub type Day = u8;

/// Enumeration representing a month; values are G_DATE_JANUARY, G_DATE_FEBRUARY, etc. G_DATE_BAD_MONTH is the invalid value.
#[derive(Clone, Copy)]
#[repr(C)]
pub enum Month {
    /// invalid value
    BadMonth,
    /// january
    January,
    /// february
    February,
    /// march
    March,
    /// april
    April,
    /// may
    May,
    /// june
    June,
    /// july
    July,
    /// august
    August,
    /// september
    September,
    /// october
    October,
    /// november
    November,
    /// december
    December
}

/// Enumeration representing a day of the week; G_DATE_MONDAY, G_DATE_TUESDAY, etc. G_DATE_BAD_WEEKDAY is an invalid weekday.
#[derive(Clone, Copy)]
#[repr(C)]
pub enum Weekday {
    /// invalid value
    BadWeekday,
    /// monday
    Monday,
    /// tuesday
    Tuesday,
    /// wednesday
    Wednesday,
    /// thrusday
    Thursday,
    /// friday
    Friday,
    /// saturday
    Saturday,
    /// sunday
    Sunday
}

pub struct Date {
    pointer: *mut ffi::GDate
}

impl Date {
    pub fn new() -> Option<Date> {
        let tmp = unsafe { ffi::g_date_new() };

        if tmp.is_null() {
            Some(Date {
                pointer: tmp
            })
        } else {
            None
        }
    }

    pub fn new_dmy(day: Day, month: Month, year: Year) -> Option<Date> {
        let tmp = unsafe { ffi::g_date_new_dmy(day as c_int, month as c_int, year) };

        if tmp.is_null() {
            Some(Date {
                pointer: tmp
            })
        } else {
            None
        }
    }

    pub fn new_julian(julian_day: u32) -> Option<Date> {
        let tmp = unsafe { ffi::g_date_new_julian(julian_day) };

        if tmp.is_null() {
            Some(Date {
                pointer: tmp
            })
        } else {
            None
        }
    }

    pub fn clear(&mut self) {
        unsafe { ffi::g_date_clear(self.pointer, 1) }
    }

    pub fn set_day(&mut self, day: Day) {
        unsafe { ffi::g_date_set_day(self.pointer, day as c_int) }
    }

    pub fn set_month(&mut self, month: Month) {
        unsafe { ffi::g_date_set_month(self.pointer, month as c_int) }
    }

    pub fn set_year(&mut self, year: Year) {
        unsafe { ffi::g_date_set_year(self.pointer, year) }
    }

    pub fn set_dmy(&mut self, day: Day, month: Month, year: Year) {
        unsafe { ffi::g_date_set_dmy(self.pointer, day as c_int, month as c_int, year) }
    }

    pub fn set_julian(&mut self, julian: u32) {
        unsafe { ffi::g_date_set_julian(self.pointer, julian) }
    }

    pub fn set_time_t(&mut self, timet: i64) {
        unsafe { ffi::g_date_set_time_t(self.pointer, timet) }
    }

    pub fn set_time_val(&mut self, timeval: &mut TimeVal) {
        unsafe { ffi::g_date_set_time_val(self.pointer, std::mem::transmute(timeval)) }
    }

    pub fn set_parse(&mut self, str_: &str) {
        unsafe { ffi::g_date_set_parse(self.pointer, str_.borrow_to_glib().0) }
    }

    pub fn add_days(&mut self, days: usize) {
        unsafe { ffi::g_date_add_days(self.pointer, days as c_uint) }
    }

    pub fn subtract_days(&mut self, days: usize) {
        unsafe { ffi::g_date_subtract_days(self.pointer, days as c_uint) }
    }

    pub fn add_months(&mut self, months: usize) {
        unsafe { ffi::g_date_add_months(self.pointer, months as c_uint) }
    }

    pub fn subtract_months(&mut self, months: usize) {
        unsafe { ffi::g_date_subtract_months(self.pointer, months as c_uint) }
    }

    pub fn add_years(&mut self, years: usize) {
        unsafe { ffi::g_date_add_years(self.pointer, years as c_uint) }
    }

    pub fn subtract_years(&mut self, years: usize) {
        unsafe { ffi::g_date_subtract_years(self.pointer, years as c_uint) }
    }

    pub fn between(&self, other: &Date) -> isize {
        unsafe { ffi::g_date_between(self.pointer, other.pointer) as isize }
    }

    pub fn compare(&self, other: &Date) -> isize {
        unsafe { ffi::g_date_compare(self.pointer, other.pointer) as isize   }
    }

    pub fn clamp(&mut self, min_date: &Date, max_date: &Date) {
        unsafe { ffi::g_date_clamp(self.pointer, min_date.pointer, max_date.pointer) }
    }

    pub fn order(&mut self, other: &mut Date) {
        unsafe { ffi::g_date_order(self.pointer, other.pointer) }
    }

    pub fn get_day(&self) -> Day {
        unsafe { ffi::g_date_get_day(self.pointer) }
    }

    pub fn get_month(&self) -> Month {
        unsafe { std::mem::transmute(ffi::g_date_get_month(self.pointer)) }
    }

    pub fn get_year(&self) -> Year {
        unsafe { ffi::g_date_get_year(self.pointer) }
    }

    pub fn get_julian(&self) -> u32 {
        unsafe { ffi::g_date_get_julian(self.pointer) }
    }

    pub fn get_weekday(&self) -> Weekday {
        unsafe { std::mem::transmute(ffi::g_date_get_weekday(self.pointer)) }
    }

    pub fn get_day_of_year(&self) -> u32 {
        unsafe { ffi::g_date_get_day_of_year(self.pointer) }
    }

    pub fn is_first_of_month(&self) -> bool {
        unsafe { super::to_bool(ffi::g_date_is_first_of_month(self.pointer)) }
    }

    pub fn is_last_of_month(&self) -> bool {
        unsafe { super::to_bool(ffi::g_date_is_last_of_month(self.pointer)) }
    }

    pub fn get_monday_week_of_year(&self) -> u32 {
        unsafe { ffi::g_date_get_monday_week_of_year(self.pointer) }
    }

    pub fn get_sunday_week_of_year(&self) -> u32 {
        unsafe { ffi::g_date_get_sunday_week_of_year(self.pointer) }
    }

    pub fn get_iso8601_week_of_year(&self) -> u32 {
        unsafe { ffi::g_date_get_iso8601_week_of_year(self.pointer) }
    }

    /*pub fn strftime(&self, s: &mut String, format: &str) -> u32 {
        unsafe { ffi::g_date_strftime(self.pointer) }
    }*/

    pub fn is_valid(&self) -> bool {
        unsafe { super::to_bool(ffi::g_date_valid(self.pointer)) }
    }
}

impl Drop for Date {
    fn drop(&mut self) {
        if !self.pointer.is_null() {
            unsafe { ffi::g_date_free(self.pointer); }
            self.pointer = std::ptr::null_mut();
        }
    }
}

/// Represents a precise time, with seconds and microseconds. Similar to the struct timeval returned by the gettimeofday() UNIX
/// system call.
/// 
/// GLib is attempting to unify around the use of 64bit integers to represent microsecond-precision time. As such, this type
/// will be removed from a future version of GLib.
#[repr(C)]
pub struct TimeVal {
    /// seconds
    pub tv_sec: i32,
    /// microseconds
    pub tv_usec: i32
}

impl TimeVal {
    pub fn add(&mut self, microseconds: u64) {
        unsafe { ffi::g_time_val_add(std::mem::transmute(self), microseconds) }
    }

    pub fn from_iso8601(&mut self, iso_date: &str) {
        unsafe { ffi::g_time_val_from_iso8601(iso_date.borrow_to_glib().0, std::mem::transmute(self)) }
    }

    pub fn to_iso8601(&mut self) -> Option<String> {
        unsafe { super::translate::FromGlibPtr::borrow(ffi::g_time_val_to_iso8601(std::mem::transmute(self))) }
    }
}

pub fn get_current_time() -> TimeVal {
    let mut t = TimeVal {
        tv_sec: 0,
        tv_usec: 0
    };

    unsafe { ffi::g_get_current_time(std::mem::transmute(&mut t)) };
    t
}

pub fn usleep(microseconds: u64) {
    unsafe { ffi::g_usleep(microseconds) }
}

pub fn get_monotonic_time() -> i64 {
    unsafe { ffi::g_get_monotonic_time() }
}

pub fn get_real_time() -> i64 {
    unsafe { ffi::g_get_real_time() }
}

pub fn get_days_in_month(month: Month, year: Year) -> u8 {
    unsafe { ffi::g_date_get_days_in_month(month as c_int, year) }
}

pub fn is_leap_year(year: Year) -> bool {
    unsafe { super::to_bool(ffi::g_date_is_leap_year(year)) }
}

pub fn get_monday_weeks_in_year(year: Year) -> u8 {
    unsafe { ffi::g_date_get_monday_weeks_in_year(year) }
}

pub fn get_sunday_weeks_in_year(year: Year) -> u8 {
    unsafe { ffi::g_date_get_sunday_weeks_in_year(year) }
}

pub fn is_valid_day(day: Day) -> bool {
    unsafe { super::to_bool(ffi::g_date_valid_day(day as c_int)) }
}

pub fn is_valid_month(month: Month) -> bool {
    unsafe { super::to_bool(ffi::g_date_valid_month(month as c_int)) }
}

pub fn is_valid_year(year: Year) -> bool {
    unsafe { super::to_bool(ffi::g_date_valid_year(year)) }
}

pub fn is_valid_dmy(day: Day, month: Month, year: Year) -> bool {
    unsafe { super::to_bool(ffi::g_date_valid_dmy(day as c_int, month as c_int, year)) }
}

pub fn is_valid_julian(julian: u32) -> bool {
    unsafe { super::to_bool(ffi::g_date_valid_julian(julian)) }
}

pub fn is_valid_weekday(day: Weekday) -> bool {
    unsafe { super::to_bool(ffi::g_date_valid_weekday(day as c_int)) }
}
