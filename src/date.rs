// Copyright 2013-2015, The Gtk-rs Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

//! Date and time.

use libc::{c_long, c_ulong};
use glib_ffi;
use std;
use super::translate::ToGlibPtr;

pub use glib_ffi::GDateDay as Day;
pub use glib_ffi::GDateMonth as Month;
pub use glib_ffi::GDateWeekday as Weekday;
pub use glib_ffi::GDateYear as Year;
pub use glib_ffi::GTime as Time;

pub struct Date {
    pointer: *mut glib_ffi::GDate
}

impl Date {
    pub fn new() -> Option<Date> {
        let tmp = unsafe { glib_ffi::g_date_new() };

        if tmp.is_null() {
            Some(Date {
                pointer: tmp
            })
        } else {
            None
        }
    }

    pub fn new_dmy(day: Day, month: Month, year: Year) -> Option<Date> {
        let tmp = unsafe { glib_ffi::g_date_new_dmy(day, month, year) };

        if tmp.is_null() {
            Some(Date {
                pointer: tmp
            })
        } else {
            None
        }
    }

    pub fn new_julian(julian_day: u32) -> Option<Date> {
        let tmp = unsafe { glib_ffi::g_date_new_julian(julian_day) };

        if tmp.is_null() {
            Some(Date {
                pointer: tmp
            })
        } else {
            None
        }
    }

    pub fn clear(&mut self) {
        unsafe { glib_ffi::g_date_clear(self.pointer, 1) }
    }

    pub fn set_day(&mut self, day: Day) {
        unsafe { glib_ffi::g_date_set_day(self.pointer, day) }
    }

    pub fn set_month(&mut self, month: Month) {
        unsafe { glib_ffi::g_date_set_month(self.pointer, month) }
    }

    pub fn set_year(&mut self, year: Year) {
        unsafe { glib_ffi::g_date_set_year(self.pointer, year) }
    }

    pub fn set_dmy(&mut self, day: Day, month: Month, year: Year) {
        unsafe { glib_ffi::g_date_set_dmy(self.pointer, day, month, year) }
    }

    pub fn set_julian(&mut self, julian: u32) {
        unsafe { glib_ffi::g_date_set_julian(self.pointer, julian) }
    }

    pub fn set_time_t(&mut self, timet: i64) {
        unsafe { glib_ffi::g_date_set_time_t(self.pointer, timet as c_long) }
    }

    pub fn set_time_val(&mut self, timeval: &mut TimeVal) {
        unsafe { glib_ffi::g_date_set_time_val(self.pointer, std::mem::transmute(timeval)) }
    }

    pub fn set_parse(&mut self, str_: &str) {
        unsafe { glib_ffi::g_date_set_parse(self.pointer, str_.to_glib_none().0) }
    }

    pub fn add_days(&mut self, days: u32) {
        unsafe { glib_ffi::g_date_add_days(self.pointer, days) }
    }

    pub fn subtract_days(&mut self, days: u32) {
        unsafe { glib_ffi::g_date_subtract_days(self.pointer, days) }
    }

    pub fn add_months(&mut self, months: u32) {
        unsafe { glib_ffi::g_date_add_months(self.pointer, months) }
    }

    pub fn subtract_months(&mut self, months: u32) {
        unsafe { glib_ffi::g_date_subtract_months(self.pointer, months) }
    }

    pub fn add_years(&mut self, years: u32) {
        unsafe { glib_ffi::g_date_add_years(self.pointer, years) }
    }

    pub fn subtract_years(&mut self, years: u32) {
        unsafe { glib_ffi::g_date_subtract_years(self.pointer, years) }
    }

    pub fn days_between(&self, other: &Date) -> isize {
        unsafe { glib_ffi::g_date_days_between(self.pointer, other.pointer) as isize }
    }

    pub fn compare(&self, other: &Date) -> isize {
        unsafe { glib_ffi::g_date_compare(self.pointer, other.pointer) as isize   }
    }

    pub fn clamp(&mut self, min_date: &Date, max_date: &Date) {
        unsafe { glib_ffi::g_date_clamp(self.pointer, min_date.pointer, max_date.pointer) }
    }

    pub fn order(&mut self, other: &mut Date) {
        unsafe { glib_ffi::g_date_order(self.pointer, other.pointer) }
    }

    pub fn get_day(&self) -> Day {
        unsafe { glib_ffi::g_date_get_day(self.pointer) }
    }

    pub fn get_month(&self) -> Month {
        unsafe { std::mem::transmute(glib_ffi::g_date_get_month(self.pointer)) }
    }

    pub fn get_year(&self) -> Year {
        unsafe { glib_ffi::g_date_get_year(self.pointer) }
    }

    pub fn get_julian(&self) -> u32 {
        unsafe { glib_ffi::g_date_get_julian(self.pointer) }
    }

    pub fn get_weekday(&self) -> Weekday {
        unsafe { std::mem::transmute(glib_ffi::g_date_get_weekday(self.pointer)) }
    }

    pub fn get_day_of_year(&self) -> u32 {
        unsafe { glib_ffi::g_date_get_day_of_year(self.pointer) }
    }

    pub fn is_first_of_month(&self) -> bool {
        unsafe { super::to_bool(glib_ffi::g_date_is_first_of_month(self.pointer)) }
    }

    pub fn is_last_of_month(&self) -> bool {
        unsafe { super::to_bool(glib_ffi::g_date_is_last_of_month(self.pointer)) }
    }

    pub fn get_monday_week_of_year(&self) -> u32 {
        unsafe { glib_ffi::g_date_get_monday_week_of_year(self.pointer) }
    }

    pub fn get_sunday_week_of_year(&self) -> u32 {
        unsafe { glib_ffi::g_date_get_sunday_week_of_year(self.pointer) }
    }

    pub fn get_iso8601_week_of_year(&self) -> u32 {
        unsafe { glib_ffi::g_date_get_iso8601_week_of_year(self.pointer) }
    }

    /*
    pub fn strftime(&self, s: &mut String, format: &str) -> u32 {
        unsafe { glib_ffi::g_date_strftime(self.pointer) }
    }*/

    pub fn is_valid(&self) -> bool {
        unsafe { super::to_bool(glib_ffi::g_date_valid(self.pointer)) }
    }
}

impl Drop for Date {
    fn drop(&mut self) {
        if !self.pointer.is_null() {
            unsafe { glib_ffi::g_date_free(self.pointer); }
            self.pointer = std::ptr::null_mut();
        }
    }
}

#[repr(C)]
pub struct TimeVal {
    pub tv_sec: c_long,
    pub tv_usec: c_long,
}

impl TimeVal {
    pub fn add(&mut self, microseconds: u64) {
        unsafe { glib_ffi::g_time_val_add(std::mem::transmute(self), microseconds as c_long) }
    }

    pub fn from_iso8601(&mut self, iso_date: &str) {
        unsafe { glib_ffi::g_time_val_from_iso8601(iso_date.to_glib_none().0, std::mem::transmute(self)); }
    }

    pub fn to_iso8601(&mut self) -> Option<String> {
        unsafe { ::translate::from_glib_none(glib_ffi::g_time_val_to_iso8601(std::mem::transmute(self))) }
    }
}

pub fn get_current_time() -> TimeVal {
    let mut t = TimeVal {
        tv_sec: 0,
        tv_usec: 0
    };

    unsafe { glib_ffi::g_get_current_time(std::mem::transmute(&mut t)) };
    t
}

pub fn usleep(microseconds: u64) {
    unsafe { glib_ffi::g_usleep(microseconds as c_ulong) }
}

pub fn get_monotonic_time() -> i64 {
    unsafe { glib_ffi::g_get_monotonic_time() }
}

pub fn get_real_time() -> i64 {
    unsafe { glib_ffi::g_get_real_time() }
}

pub fn get_days_in_month(month: Month, year: Year) -> u8 {
    unsafe { glib_ffi::g_date_get_days_in_month(month, year) }
}

pub fn is_leap_year(year: Year) -> bool {
    unsafe { super::to_bool(glib_ffi::g_date_is_leap_year(year)) }
}

pub fn get_monday_weeks_in_year(year: Year) -> u8 {
    unsafe { glib_ffi::g_date_get_monday_weeks_in_year(year) }
}

pub fn get_sunday_weeks_in_year(year: Year) -> u8 {
    unsafe { glib_ffi::g_date_get_sunday_weeks_in_year(year) }
}

pub fn is_valid_day(day: Day) -> bool {
    unsafe { super::to_bool(glib_ffi::g_date_valid_day(day)) }
}

pub fn is_valid_month(month: Month) -> bool {
    unsafe { super::to_bool(glib_ffi::g_date_valid_month(month)) }
}

pub fn is_valid_year(year: Year) -> bool {
    unsafe { super::to_bool(glib_ffi::g_date_valid_year(year)) }
}

pub fn is_valid_dmy(day: Day, month: Month, year: Year) -> bool {
    unsafe { super::to_bool(glib_ffi::g_date_valid_dmy(day, month, year)) }
}

pub fn is_valid_julian(julian: u32) -> bool {
    unsafe { super::to_bool(glib_ffi::g_date_valid_julian(julian)) }
}

pub fn is_valid_weekday(day: Weekday) -> bool {
    unsafe { super::to_bool(glib_ffi::g_date_valid_weekday(day)) }
}
