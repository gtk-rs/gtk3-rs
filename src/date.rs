// Copyright 2013-2015, The Rust-GNOME Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

//! Date and Time Functions — calendrical calculations and miscellaneous time stuff

use libc::{c_int, c_uint, c_long, c_ulong};
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
    /// Allocates a GDate and initializes it to a sane state. The new date will be cleared
    /// (as if you'd called g_date_clear()) but invalid (it won't represent an existing day).
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

    /// Like g_date_new(), but also sets the value of the date. Assuming the day-month-year
    /// triplet you pass in represents an existing day, the returned date will be valid.
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

    /// Like g_date_new(), but also sets the value of the date. Assuming the Julian day
    /// number you pass in is valid (greater than 0, less than an unreasonably large
    /// number), the returned date will be valid.
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

    /// Initializes one or more GDate structs to a sane but invalid state. The cleared
    /// dates will not represent an existing date, but will not contain garbage. Useful
    /// to init a date declared on the stack. Validity can be tested with g_date_valid().
    pub fn clear(&mut self) {
        unsafe { ffi::g_date_clear(self.pointer, 1) }
    }

    /// Sets the day of the month for a GDate. If the resulting day-month-year triplet is
    /// invalid, the date will be invalid.
    pub fn set_day(&mut self, day: Day) {
        unsafe { ffi::g_date_set_day(self.pointer, day as c_int) }
    }

    /// Sets the month of the year for a GDate. If the resulting day-month-year triplet is
    /// invalid, the date will be invalid.
    pub fn set_month(&mut self, month: Month) {
        unsafe { ffi::g_date_set_month(self.pointer, month as c_int) }
    }

    /// Sets the year for a GDate. If the resulting day-month-year triplet is invalid, the
    /// date will be invalid.
    pub fn set_year(&mut self, year: Year) {
        unsafe { ffi::g_date_set_year(self.pointer, year) }
    }

    /// Sets the value of a GDate from a day, month, and year. The day-month-year triplet
    /// must be valid; if you aren't sure it is, call g_date_valid_dmy() to check before
    /// you set it.
    pub fn set_dmy(&mut self, day: Day, month: Month, year: Year) {
        unsafe { ffi::g_date_set_dmy(self.pointer, day as c_int, month as c_int, year) }
    }

    /// Sets the value of a GDate from a Julian day number.
    pub fn set_julian(&mut self, julian: u32) {
        unsafe { ffi::g_date_set_julian(self.pointer, julian) }
    }

    /// Sets the value of a date to the date corresponding to a time specified as a time_t.
    /// The time to date conversion is done using the user's current timezone.
    /// To set the value of a date to the current day, you could write:
    /// ```
    /// Date::new().set_time_t(date, time::get_time().sec);
    /// ```
    pub fn set_time_t(&mut self, timet: i64) {
        unsafe { ffi::g_date_set_time_t(self.pointer, timet) }
    }

    /// Sets the value of a date from a GTimeVal value. Note that the tv_usec member is ignored,
    /// because GDate can't make use of the additional precision.
    /// 
    /// The time to date conversion is done using the user's current timezone.
    pub fn set_time_val(&mut self, timeval: &mut TimeVal) {
        unsafe { ffi::g_date_set_time_val(self.pointer, std::mem::transmute(timeval)) }
    }

    /// Parses a user-inputted string str , and try to figure out what date it represents,
    /// taking the current locale into account. If the string is successfully parsed, the
    /// date will be valid after the call. Otherwise, it will be invalid. You should check
    /// using g_date_valid() to see whether the parsing succeeded.
    /// 
    /// This function is not appropriate for file formats and the like; it isn't very precise,
    /// and its exact behavior varies with the locale. It's intended to be a heuristic routine
    /// that guesses what the user means by a given string (and it does work pretty well in
    /// that capacity).
    pub fn set_parse(&mut self, str_: &str) {
        unsafe { ffi::g_date_set_parse(self.pointer, str_.to_glib_none().0) }
    }

    /// Increments a date some number of days. To move forward by weeks, add weeks*7 days. The
    /// date must be valid.
    pub fn add_days(&mut self, days: usize) {
        unsafe { ffi::g_date_add_days(self.pointer, days as c_uint) }
    }

    /// Moves a date some number of days into the past. To move by weeks, just move by weeks*7
    /// days. The date must be valid.
    pub fn subtract_days(&mut self, days: usize) {
        unsafe { ffi::g_date_subtract_days(self.pointer, days as c_uint) }
    }

    /// Increments a date by some number of months. If the day of the month is greater than 28,
    /// this routine may change the day of the month (because the destination month may not have
    /// the current day in it). The date must be valid.
    pub fn add_months(&mut self, months: usize) {
        unsafe { ffi::g_date_add_months(self.pointer, months as c_uint) }
    }

    /// Moves a date some number of months into the past. If the current day of the month doesn't
    /// exist in the destination month, the day of the month may change. The date must be valid.
    pub fn subtract_months(&mut self, months: usize) {
        unsafe { ffi::g_date_subtract_months(self.pointer, months as c_uint) }
    }

    /// Increments a date by some number of years. If the date is February 29, and the destination
    /// year is not a leap year, the date will be changed to February 28. The date must be valid.
    pub fn add_years(&mut self, years: usize) {
        unsafe { ffi::g_date_add_years(self.pointer, years as c_uint) }
    }

    /// Moves a date some number of years into the past. If the current day doesn't exist in the
    /// destination year (i.e. it's February 29 and you move to a non-leap-year) then the day is
    /// changed to February 29. The date must be valid.
    pub fn subtract_years(&mut self, years: usize) {
        unsafe { ffi::g_date_subtract_years(self.pointer, years as c_uint) }
    }

    /// Computes the number of days between two dates. If date2 is prior to date1 , the returned
    /// value is negative. Both dates must be valid.
    pub fn days_between(&self, other: &Date) -> isize {
        unsafe { ffi::g_date_days_between(self.pointer, other.pointer) as isize }
    }

    /// qsort()-style comparison function for dates. Both dates must be valid.
    /// 
    /// returned value :
    /// * 0 for equal
    /// * < 0 if lhs is less than rhs
    /// * > 0 if lhs is greater than rhs
    pub fn compare(&self, other: &Date) -> isize {
        unsafe { ffi::g_date_compare(self.pointer, other.pointer) as isize   }
    }

    /// If date is prior to min_date , sets date equal to min_date . If date falls after
    /// max_date , sets date equal to max_date . Otherwise, date is unchanged. Either of min_date
    /// and max_date may be NULL. All non-NULL dates must be valid.
    pub fn clamp(&mut self, min_date: &Date, max_date: &Date) {
        unsafe { ffi::g_date_clamp(self.pointer, min_date.pointer, max_date.pointer) }
    }

    /// Checks if date1 is less than or equal to date2 , and swap the values if this is not
    /// the case.
    pub fn order(&mut self, other: &mut Date) {
        unsafe { ffi::g_date_order(self.pointer, other.pointer) }
    }

    /// Returns the day of the month. The date must be valid.
    pub fn get_day(&self) -> Day {
        unsafe { ffi::g_date_get_day(self.pointer) }
    }

    /// Returns the month of the year. The date must be valid.
    pub fn get_month(&self) -> Month {
        unsafe { std::mem::transmute(ffi::g_date_get_month(self.pointer)) }
    }

    /// Returns the year of a GDate. The date must be valid.
    pub fn get_year(&self) -> Year {
        unsafe { ffi::g_date_get_year(self.pointer) }
    }

    /// Returns the Julian day or "serial number" of the GDate. The Julian day is simply the
    /// number of days since January 1, Year 1; i.e., January 1, Year 1 is Julian day 1;
    /// January 2, Year 1 is Julian day 2, etc. The date must be valid.
    pub fn get_julian(&self) -> u32 {
        unsafe { ffi::g_date_get_julian(self.pointer) }
    }

    /// Returns the day of the week for a GDate. The date must be valid.
    pub fn get_weekday(&self) -> Weekday {
        unsafe { std::mem::transmute(ffi::g_date_get_weekday(self.pointer)) }
    }

    /// Returns the day of the year, where Jan 1 is the first day of the year. The date
    /// must be valid.
    pub fn get_day_of_year(&self) -> u32 {
        unsafe { ffi::g_date_get_day_of_year(self.pointer) }
    }

    /// Returns true if the date is on the first of a month. The date must be valid.
    pub fn is_first_of_month(&self) -> bool {
        unsafe { super::to_bool(ffi::g_date_is_first_of_month(self.pointer)) }
    }

    /// Returns true if the date is the last day of the month. The date must be valid.
    pub fn is_last_of_month(&self) -> bool {
        unsafe { super::to_bool(ffi::g_date_is_last_of_month(self.pointer)) }
    }

    /// Returns the week of the year, where weeks are understood to start on Monday. If
    /// the date is before the first Monday of the year, return ???
    /// 
    /// The date must be valid.
    pub fn get_monday_week_of_year(&self) -> u32 {
        unsafe { ffi::g_date_get_monday_week_of_year(self.pointer) }
    }

    /// Returns the week of the year during which this date falls, if weeks are understood
    /// to being on Sunday. The date must be valid. Can return 0 if the day is before the
    /// first Sunday of the year.
    pub fn get_sunday_week_of_year(&self) -> u32 {
        unsafe { ffi::g_date_get_sunday_week_of_year(self.pointer) }
    }

    /// Returns the week of the year, where weeks are interpreted according to ISO 8601.
    pub fn get_iso8601_week_of_year(&self) -> u32 {
        unsafe { ffi::g_date_get_iso8601_week_of_year(self.pointer) }
    }

    /*
    /// Generates a printed representation of the date, in a locale-specific way. Works
    /// just like the platform's C library strftime() function, but only accepts date-related
    /// formats; time-related formats give undefined results. Date must be valid. Unlike
    /// strftime() (which uses the locale encoding), works on a UTF-8 format string and
    /// stores a UTF-8 result.
    /// 
    /// This function does not provide any conversion specifiers in addition to those
    /// implemented by the platform's C library. For example, don't expect that using
    /// g_date_strftime() would make the %F provided by the C99 strftime() work on Windows
    /// where the C library only complies to C89.
    pub fn strftime(&self, s: &mut String, format: &str) -> u32 {
        unsafe { ffi::g_date_strftime(self.pointer) }
    }*/

    /// Returns TRUE if the GDate represents an existing day. The date must not contain
    /// garbage; it should have been initialized with g_date_clear() if it wasn't allocated
    /// by one of the g_date_new() variants.
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
    pub tv_sec: c_long,
    /// microseconds
    pub tv_usec: c_long,
}

impl TimeVal {
    /// Adds the given number of microseconds to self . microseconds can also be negative to
    /// decrease the value of self .
    pub fn add(&mut self, microseconds: u64) {
        unsafe { ffi::g_time_val_add(std::mem::transmute(self), microseconds as c_ulong) }
    }

    /// Converts a string containing an ISO 8601 encoded date and time to a GTimeVal and puts
    /// it into self .
    /// 
    /// iso_date must include year, month, day, hours, minutes, and seconds. It can optionally
    /// include fractions of a second and a time zone indicator. (In the absence of any time
    /// zone indication, the timestamp is assumed to be in local time.)
    pub fn from_iso8601(&mut self, iso_date: &str) {
        unsafe { ffi::g_time_val_from_iso8601(iso_date.to_glib_none().0, std::mem::transmute(self)) }
    }

    /// Converts time_ into an RFC 3339 encoded string, relative to the Coordinated Universal
    /// Time (UTC). This is one of the many formats allowed by ISO 8601.
    /// 
    /// ISO 8601 allows a large number of date/time formats, with or without punctuation and
    /// optional elements. The format returned by this function is a complete date and time,
    /// with optional punctuation included, the UTC time zone represented as "Z", and the tv_usec
    /// part included if and only if it is nonzero, i.e. either "YYYY-MM-DDTHH:MM:SSZ" or
    /// "YYYY-MM-DDTHH:MM:SS.fffffZ".
    /// 
    /// This corresponds to the Internet date/time format defined by RFC 3339, and to either of
    /// the two most-precise formats defined by the W3C Note Date and Time Formats. Both of these
    /// documents are profiles of ISO 8601.
    /// 
    /// Use g_date_time_format() or g_strdup_printf() if a different variation of ISO 8601 format
    /// is required.
    pub fn to_iso8601(&mut self) -> Option<String> {
        unsafe { ::translate::from_glib_none(ffi::g_time_val_to_iso8601(std::mem::transmute(self))) }
    }
}

/// Equivalent to the UNIX gettimeofday() function, but portable.
/// 
/// You may find g_get_real_time() to be more convenient.
pub fn get_current_time() -> TimeVal {
    let mut t = TimeVal {
        tv_sec: 0,
        tv_usec: 0
    };

    unsafe { ffi::g_get_current_time(std::mem::transmute(&mut t)) };
    t
}

/// Pauses the current thread for the given number of microseconds.
/// 
/// There are 1 million microseconds per second (represented by the
/// G_USEC_PER_SEC macro). g_usleep() may have limited precision, depending
/// on hardware and operating system; don't rely on the exact length of
/// the sleep.
pub fn usleep(microseconds: u64) {
    unsafe { ffi::g_usleep(microseconds as c_ulong) }
}

/// Queries the system monotonic time.
/// 
/// The monotonic clock will always increase and doesn't suffer discontinuities
/// when the user (or NTP) changes the system time. It may or may not continue
/// to tick during times where the machine is suspended.
/// 
/// We try to use the clock that corresponds as closely as possible to the passage
/// of time as measured by system calls such as poll() but it may not always be
/// possible to do this.
/// 
/// Returns the monotonic time, in microseconds
pub fn get_monotonic_time() -> i64 {
    unsafe { ffi::g_get_monotonic_time() }
}

/// Queries the system wall-clock time.
/// 
/// This call is functionally equivalent to g_get_current_time() except that
/// the return value is often more convenient than dealing with a GTimeVal.
/// 
/// You should only use this call if you are actually interested in the real
/// wall-clock time. g_get_monotonic_time() is probably more useful for measuring
/// intervals.
/// 
/// Returns the number of microseconds since January 1, 1970 UTC.
pub fn get_real_time() -> i64 {
    unsafe { ffi::g_get_real_time() }
}

/// Returns the number of days in a month, taking leap years into account.
pub fn get_days_in_month(month: Month, year: Year) -> u8 {
    unsafe { ffi::g_date_get_days_in_month(month as c_int, year) }
}

/// Returns TRUE if the year is a leap year.
/// 
/// For the purposes of this function, leap year is every year divisible by 4 unless
/// that year is divisible by 100. If it is divisible by 100 it would be a leap year
/// only if that year is also divisible by 400.
pub fn is_leap_year(year: Year) -> bool {
    unsafe { super::to_bool(ffi::g_date_is_leap_year(year)) }
}

/// Returns the number of weeks in the year, where weeks are taken to start on Monday.
/// Will be 52 or 53. The date must be valid. (Years always have 52 7-day periods,
/// plus 1 or 2 extra days depending on whether it's a leap year. This function is
/// basically telling you how many Mondays are in the year, i.e. there are 53 Mondays
/// if one of the extra days happens to be a Monday.)
pub fn get_monday_weeks_in_year(year: Year) -> u8 {
    unsafe { ffi::g_date_get_monday_weeks_in_year(year) }
}

/// Returns the number of weeks in the year, where weeks are taken to start on Sunday.
/// Will be 52 or 53. The date must be valid. (Years always have 52 7-day periods, plus
/// 1 or 2 extra days depending on whether it's a leap year. This function is basically
/// telling you how many Sundays are in the year, i.e. there are 53 Sundays if one of
/// the extra days happens to be a Sunday.)
pub fn get_sunday_weeks_in_year(year: Year) -> u8 {
    unsafe { ffi::g_date_get_sunday_weeks_in_year(year) }
}

/// Returns true if the day of the month is valid (a day is valid if it's between 1 and
/// 31 inclusive).
pub fn is_valid_day(day: Day) -> bool {
    unsafe { super::to_bool(ffi::g_date_valid_day(day as c_int)) }
}

/// Returns true if the month value is valid. The 12 GDateMonth enumeration values are
/// the only valid months.
pub fn is_valid_month(month: Month) -> bool {
    unsafe { super::to_bool(ffi::g_date_valid_month(month as c_int)) }
}

/// Returns true if the year is valid. Any year greater than 0 is valid, though there
/// is a 16-bit limit to what GDate will understand.
pub fn is_valid_year(year: Year) -> bool {
    unsafe { super::to_bool(ffi::g_date_valid_year(year)) }
}

/// Returns true if the day-month-year triplet forms a valid, existing day in the range
/// of days GDate understands (Year 1 or later, no more than a few thousand years in the
/// future).
pub fn is_valid_dmy(day: Day, month: Month, year: Year) -> bool {
    unsafe { super::to_bool(ffi::g_date_valid_dmy(day as c_int, month as c_int, year)) }
}

/// Returns true if the Julian day is valid. Anything greater than zero is basically a
/// valid Julian, though there is a 32-bit limit.
pub fn is_valid_julian(julian: u32) -> bool {
    unsafe { super::to_bool(ffi::g_date_valid_julian(julian)) }
}

/// Returns true if the weekday is valid. The seven GDateWeekday enumeration values are
/// the only valid weekdays.
pub fn is_valid_weekday(day: Weekday) -> bool {
    unsafe { super::to_bool(ffi::g_date_valid_weekday(day as c_int)) }
}
