// Copyright 2017, The Gtk-rs Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

use DateDay;
use DateMonth;
use DateWeekday;
use DateYear;
use Time;
use ffi;
use ffi as glib_ffi;
use gobject_ffi;
use libc;
use std::cmp;
use std::mem;
use std::ptr;
use translate::*;

glib_wrapper! {
    pub struct Date(Boxed<ffi::GDate>);

    match fn {
        copy => |ptr| gobject_ffi::g_boxed_copy(ffi::g_date_get_type(), ptr as *const _) as *mut _,
        free => |ptr| ffi::g_date_free(ptr),
        get_type => || ffi::g_date_get_type(),
    }
}

unsafe impl Send for Date { }

impl Date {
    pub fn new() -> Date {
        unsafe {
            from_glib_full(ffi::g_date_new())
        }
    }

    pub fn new_dmy(day: DateDay, month: DateMonth, year: DateYear) -> Date {
        unsafe {
            from_glib_full(ffi::g_date_new_dmy(day, month.to_glib(), year))
        }
    }

    pub fn new_julian(julian_day: u32) -> Date {
        unsafe {
            from_glib_full(ffi::g_date_new_julian(julian_day))
        }
    }

    pub fn add_days(&mut self, n_days: u32) {
        unsafe {
            ffi::g_date_add_days(self.to_glib_none_mut().0, n_days);
        }
    }

    pub fn add_months(&mut self, n_months: u32) {
        unsafe {
            ffi::g_date_add_months(self.to_glib_none_mut().0, n_months);
        }
    }

    pub fn add_years(&mut self, n_years: u32) {
        unsafe {
            ffi::g_date_add_years(self.to_glib_none_mut().0, n_years);
        }
    }

    pub fn clamp(&mut self, min_date: &Date, max_date: &Date) {
        unsafe {
            ffi::g_date_clamp(self.to_glib_none_mut().0, min_date.to_glib_none().0, max_date.to_glib_none().0);
        }
    }

    pub fn clear(&mut self, n_dates: u32) {
        unsafe {
            ffi::g_date_clear(self.to_glib_none_mut().0, n_dates);
        }
    }

    fn compare(&self, rhs: &Date) -> i32 {
        unsafe {
            ffi::g_date_compare(self.to_glib_none().0, rhs.to_glib_none().0)
        }
    }

    pub fn days_between(&self, date2: &Date) -> i32 {
        unsafe {
            ffi::g_date_days_between(self.to_glib_none().0, date2.to_glib_none().0)
        }
    }

    pub fn get_day(&self) -> DateDay {
        unsafe {
            ffi::g_date_get_day(self.to_glib_none().0)
        }
    }

    pub fn get_day_of_year(&self) -> u32 {
        unsafe {
            ffi::g_date_get_day_of_year(self.to_glib_none().0)
        }
    }

    pub fn get_iso8601_week_of_year(&self) -> u32 {
        unsafe {
            ffi::g_date_get_iso8601_week_of_year(self.to_glib_none().0)
        }
    }

    pub fn get_julian(&self) -> u32 {
        unsafe {
            ffi::g_date_get_julian(self.to_glib_none().0)
        }
    }

    pub fn get_monday_week_of_year(&self) -> u32 {
        unsafe {
            ffi::g_date_get_monday_week_of_year(self.to_glib_none().0)
        }
    }

    pub fn get_month(&self) -> DateMonth {
        unsafe {
            from_glib(ffi::g_date_get_month(self.to_glib_none().0))
        }
    }

    pub fn get_sunday_week_of_year(&self) -> u32 {
        unsafe {
            ffi::g_date_get_sunday_week_of_year(self.to_glib_none().0)
        }
    }

    pub fn get_weekday(&self) -> DateWeekday {
        unsafe {
            from_glib(ffi::g_date_get_weekday(self.to_glib_none().0))
        }
    }

    pub fn get_year(&self) -> DateYear {
        unsafe {
            ffi::g_date_get_year(self.to_glib_none().0)
        }
    }

    pub fn is_first_of_month(&self) -> bool {
        unsafe {
            from_glib(ffi::g_date_is_first_of_month(self.to_glib_none().0))
        }
    }

    pub fn is_last_of_month(&self) -> bool {
        unsafe {
            from_glib(ffi::g_date_is_last_of_month(self.to_glib_none().0))
        }
    }

    pub fn order(&mut self, date2: &mut Date) {
        unsafe {
            ffi::g_date_order(self.to_glib_none_mut().0, date2.to_glib_none_mut().0);
        }
    }

    pub fn set_day(&mut self, day: DateDay) {
        unsafe {
            ffi::g_date_set_day(self.to_glib_none_mut().0, day);
        }
    }

    pub fn set_dmy(&mut self, day: DateDay, month: DateMonth, y: DateYear) {
        unsafe {
            ffi::g_date_set_dmy(self.to_glib_none_mut().0, day, month.to_glib(), y);
        }
    }

    pub fn set_julian(&mut self, julian_date: u32) {
        unsafe {
            ffi::g_date_set_julian(self.to_glib_none_mut().0, julian_date);
        }
    }

    pub fn set_month(&mut self, month: DateMonth) {
        unsafe {
            ffi::g_date_set_month(self.to_glib_none_mut().0, month.to_glib());
        }
    }

    pub fn set_parse(&mut self, str: &str) {
        unsafe {
            ffi::g_date_set_parse(self.to_glib_none_mut().0, str.to_glib_none().0);
        }
    }

    pub fn set_time(&mut self, time_: Time) {
        unsafe {
            ffi::g_date_set_time(self.to_glib_none_mut().0, time_);
        }
    }

    pub fn set_time_t(&mut self, timet: libc::c_long) {
        unsafe {
            ffi::g_date_set_time_t(self.to_glib_none_mut().0, timet);
        }
    }

    //pub fn set_time_val(&mut self, timeval: /*Ignored*/&mut TimeVal) {
    //    unsafe { TODO: call ffi::g_date_set_time_val() }
    //}

    pub fn set_year(&mut self, year: DateYear) {
        unsafe {
            ffi::g_date_set_year(self.to_glib_none_mut().0, year);
        }
    }

    pub fn subtract_days(&mut self, n_days: u32) {
        unsafe {
            ffi::g_date_subtract_days(self.to_glib_none_mut().0, n_days);
        }
    }

    pub fn subtract_months(&mut self, n_months: u32) {
        unsafe {
            ffi::g_date_subtract_months(self.to_glib_none_mut().0, n_months);
        }
    }

    pub fn subtract_years(&mut self, n_years: u32) {
        unsafe {
            ffi::g_date_subtract_years(self.to_glib_none_mut().0, n_years);
        }
    }

    //pub fn to_struct_tm(&self, tm: /*Unimplemented*/Fundamental: Pointer) {
    //    unsafe { TODO: call ffi::g_date_to_struct_tm() }
    //}

    pub fn valid(&self) -> bool {
        unsafe {
            from_glib(ffi::g_date_valid(self.to_glib_none().0))
        }
    }

    pub fn get_days_in_month(month: DateMonth, year: DateYear) -> u8 {
        unsafe {
            ffi::g_date_get_days_in_month(month.to_glib(), year)
        }
    }

    pub fn get_monday_weeks_in_year(year: DateYear) -> u8 {
        unsafe {
            ffi::g_date_get_monday_weeks_in_year(year)
        }
    }

    pub fn get_sunday_weeks_in_year(year: DateYear) -> u8 {
        unsafe {
            ffi::g_date_get_sunday_weeks_in_year(year)
        }
    }

    pub fn is_leap_year(year: DateYear) -> bool {
        unsafe {
            from_glib(ffi::g_date_is_leap_year(year))
        }
    }

    pub fn strftime(s: &str, format: &str, date: &Date) -> usize {
        let slen = s.len() as usize;
        unsafe {
            ffi::g_date_strftime(s.to_glib_none().0, slen, format.to_glib_none().0, date.to_glib_none().0)
        }
    }

    pub fn valid_day(day: DateDay) -> bool {
        unsafe {
            from_glib(ffi::g_date_valid_day(day))
        }
    }

    pub fn valid_dmy(day: DateDay, month: DateMonth, year: DateYear) -> bool {
        unsafe {
            from_glib(ffi::g_date_valid_dmy(day, month.to_glib(), year))
        }
    }

    pub fn valid_julian(julian_date: u32) -> bool {
        unsafe {
            from_glib(ffi::g_date_valid_julian(julian_date))
        }
    }

    pub fn valid_month(month: DateMonth) -> bool {
        unsafe {
            from_glib(ffi::g_date_valid_month(month.to_glib()))
        }
    }

    pub fn valid_weekday(weekday: DateWeekday) -> bool {
        unsafe {
            from_glib(ffi::g_date_valid_weekday(weekday.to_glib()))
        }
    }

    pub fn valid_year(year: DateYear) -> bool {
        unsafe {
            from_glib(ffi::g_date_valid_year(year))
        }
    }
}

impl Default for Date {
    fn default() -> Self {
        Self::new()
    }
}

impl PartialEq for Date {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.compare(other) == 0
    }
}

impl Eq for Date {}

impl PartialOrd for Date {
    #[inline]
    fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
        self.compare(other).partial_cmp(&0)
    }
}

impl Ord for Date {
    #[inline]
    fn cmp(&self, other: &Self) -> cmp::Ordering {
        self.compare(other).cmp(&0)
    }
}
