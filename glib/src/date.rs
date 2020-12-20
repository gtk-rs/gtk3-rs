// Take a look at the license at the top of the repository in the LICENSE file.

use crate::translate::*;
use crate::BoolError;
use crate::DateDay;
use crate::DateMonth;
use crate::DateWeekday;
use crate::DateYear;
use std::cmp;
use std::fmt;
use std::hash;

wrapper! {
    pub struct Date(Boxed<ffi::GDate>);

    match fn {
        copy => |ptr| gobject_ffi::g_boxed_copy(ffi::g_date_get_type(), ptr as *const _) as *mut _,
        free => |ptr| ffi::g_date_free(ptr),
        init => |_ptr| (),
        clear => |ptr| ffi::g_date_clear(ptr, 1),
        get_type => || ffi::g_date_get_type(),
    }
}

unsafe impl Send for Date {}
unsafe impl Sync for Date {}

impl Date {
    #[doc(alias = "g_date_valid_dmy")]
    pub fn new_dmy(day: DateDay, month: DateMonth, year: DateYear) -> Result<Date, BoolError> {
        let month = month.to_glib();
        unsafe {
            let check: bool = from_glib(ffi::g_date_valid_dmy(day, month, year));
            if !check {
                Err(bool_error!("Invalid date"))
            } else {
                Ok(from_glib_full(ffi::g_date_new_dmy(day, month, year)))
            }
        }
    }

    #[doc(alias = "g_date_new_julian")]
    pub fn new_julian(julian_day: u32) -> Result<Date, BoolError> {
        if !Self::valid_julian(julian_day) {
            Err(bool_error!("Invalid date"))
        } else {
            unsafe { Ok(from_glib_full(ffi::g_date_new_julian(julian_day))) }
        }
    }

    #[doc(alias = "g_date_add_days")]
    pub fn add_days(&mut self, n_days: u32) -> Result<(), BoolError> {
        let julian_days = self.get_julian();
        if julian_days == 0 || n_days > std::u32::MAX - julian_days {
            Err(bool_error!("Invalid date"))
        } else {
            unsafe {
                ffi::g_date_add_days(self.to_glib_none_mut().0, n_days);
            }
            Ok(())
        }
    }

    #[doc(alias = "g_date_add_months")]
    pub fn add_months(&mut self, n_months: u32) -> Result<(), BoolError> {
        // The checks for this function are just a mess in the C code, allowing intermediate
        // unknown state. So for now, nothing can be done...
        unsafe {
            ffi::g_date_add_months(self.to_glib_none_mut().0, n_months);
        }
        Ok(())
    }

    #[doc(alias = "g_date_add_years")]
    pub fn add_years(&mut self, n_years: u16) -> Result<(), BoolError> {
        let year = self.get_year();
        if n_years > std::u16::MAX - year {
            Err(bool_error!("Invalid date"))
        } else {
            unsafe {
                ffi::g_date_add_years(self.to_glib_none_mut().0, n_years as _);
            }
            Ok(())
        }
    }

    #[doc(alias = "g_date_clamp")]
    pub fn clamp(&mut self, min_date: &Date, max_date: &Date) -> Result<(), BoolError> {
        if min_date >= max_date {
            Err(bool_error!("`min_date` must be before `max_date`"))
        } else {
            unsafe {
                ffi::g_date_clamp(
                    self.to_glib_none_mut().0,
                    min_date.to_glib_none().0,
                    max_date.to_glib_none().0,
                );
            }
            Ok(())
        }
    }

    #[doc(alias = "g_date_compare")]
    fn compare(&self, rhs: &Date) -> i32 {
        unsafe { ffi::g_date_compare(self.to_glib_none().0, rhs.to_glib_none().0) }
    }

    #[doc(alias = "g_date_days_between")]
    pub fn days_between(&self, date2: &Date) -> i32 {
        unsafe { ffi::g_date_days_between(self.to_glib_none().0, date2.to_glib_none().0) }
    }

    #[doc(alias = "g_date_get_day")]
    pub fn get_day(&self) -> DateDay {
        unsafe { ffi::g_date_get_day(self.to_glib_none().0) }
    }

    #[doc(alias = "g_date_get_day_of_year")]
    pub fn get_day_of_year(&self) -> u32 {
        unsafe { ffi::g_date_get_day_of_year(self.to_glib_none().0) }
    }

    #[doc(alias = "g_date_get_iso8601_week_of_year")]
    pub fn get_iso8601_week_of_year(&self) -> u32 {
        unsafe { ffi::g_date_get_iso8601_week_of_year(self.to_glib_none().0) }
    }

    #[doc(alias = "g_date_get_julian")]
    pub fn get_julian(&self) -> u32 {
        unsafe { ffi::g_date_get_julian(self.to_glib_none().0) }
    }

    #[doc(alias = "g_date_get_monday_week_of_year")]
    pub fn get_monday_week_of_year(&self) -> u32 {
        unsafe { ffi::g_date_get_monday_week_of_year(self.to_glib_none().0) }
    }

    #[doc(alias = "g_date_get_month")]
    pub fn get_month(&self) -> DateMonth {
        unsafe { from_glib(ffi::g_date_get_month(self.to_glib_none().0)) }
    }

    #[doc(alias = "g_date_get_sunday_week_of_year")]
    pub fn get_sunday_week_of_year(&self) -> u32 {
        unsafe { ffi::g_date_get_sunday_week_of_year(self.to_glib_none().0) }
    }

    #[doc(alias = "g_date_get_weekday")]
    pub fn get_weekday(&self) -> DateWeekday {
        unsafe { from_glib(ffi::g_date_get_weekday(self.to_glib_none().0)) }
    }

    #[doc(alias = "g_date_get_year")]
    pub fn get_year(&self) -> DateYear {
        unsafe { ffi::g_date_get_year(self.to_glib_none().0) }
    }

    #[doc(alias = "g_date_is_first_of_month")]
    pub fn is_first_of_month(&self) -> bool {
        unsafe { from_glib(ffi::g_date_is_first_of_month(self.to_glib_none().0)) }
    }

    #[doc(alias = "g_date_is_last_of_month")]
    pub fn is_last_of_month(&self) -> bool {
        unsafe { from_glib(ffi::g_date_is_last_of_month(self.to_glib_none().0)) }
    }

    #[doc(alias = "g_date_order")]
    pub fn order(&mut self, date2: &mut Date) {
        unsafe {
            ffi::g_date_order(self.to_glib_none_mut().0, date2.to_glib_none_mut().0);
        }
    }

    #[doc(alias = "g_date_set_day")]
    pub fn set_day(&mut self, day: DateDay) -> Result<(), BoolError> {
        if !Self::valid_dmy(day, self.get_month(), self.get_year()) {
            Err(bool_error!("invalid day"))
        } else {
            unsafe {
                ffi::g_date_set_day(self.to_glib_none_mut().0, day);
            }
            Ok(())
        }
    }

    #[doc(alias = "g_date_set_dmy")]
    pub fn set_dmy(
        &mut self,
        day: DateDay,
        month: DateMonth,
        y: DateYear,
    ) -> Result<(), BoolError> {
        if !Self::valid_dmy(day, month, y) {
            Err(bool_error!("invalid date"))
        } else {
            unsafe {
                ffi::g_date_set_dmy(self.to_glib_none_mut().0, day, month.to_glib(), y);
            }
            Ok(())
        }
    }

    #[doc(alias = "g_date_set_julian")]
    pub fn set_julian(&mut self, julian_date: u32) -> Result<(), BoolError> {
        if !Self::valid_julian(julian_date) {
            Err(bool_error!("invalid date"))
        } else {
            unsafe {
                ffi::g_date_set_julian(self.to_glib_none_mut().0, julian_date);
            }
            Ok(())
        }
    }

    #[doc(alias = "g_date_set_month")]
    pub fn set_month(&mut self, month: DateMonth) -> Result<(), BoolError> {
        if !Self::valid_dmy(self.get_day(), month, self.get_year()) {
            Err(bool_error!("invalid month"))
        } else {
            unsafe {
                ffi::g_date_set_month(self.to_glib_none_mut().0, month.to_glib());
            }
            Ok(())
        }
    }

    #[doc(alias = "g_date_set_parse")]
    pub fn set_parse(&mut self, str: &str) -> Result<(), BoolError> {
        let mut c = self.clone();
        if !unsafe {
            ffi::g_date_set_parse(c.to_glib_none_mut().0, str.to_glib_none().0);
            ffi::g_date_valid(c.to_glib_none().0) == 0
        } {
            Err(bool_error!("invalid parse string"))
        } else {
            *self = c;
            Ok(())
        }
    }

    #[doc(alias = "g_date_set_time_t")]
    pub fn set_time(&mut self, time_: u32) -> Result<(), BoolError> {
        let mut c = self.clone();
        unsafe {
            ffi::g_date_set_time_t(c.to_glib_none_mut().0, time_ as _);
        }
        if !Self::valid_dmy(c.get_day(), c.get_month(), c.get_year()) {
            Err(bool_error!("invalid time"))
        } else {
            *self = c;
            Ok(())
        }
    }

    //pub fn set_time_val(&mut self, timeval: /*Ignored*/&mut TimeVal) {
    //    unsafe { TODO: call ffi::g_date_set_time_val() }
    //}

    #[doc(alias = "g_date_set_year")]
    pub fn set_year(&mut self, year: DateYear) -> Result<(), BoolError> {
        if !Self::valid_dmy(self.get_day(), self.get_month(), year) {
            Err(bool_error!("invalid year"))
        } else {
            unsafe {
                ffi::g_date_set_year(self.to_glib_none_mut().0, year);
            }
            Ok(())
        }
    }

    #[doc(alias = "g_date_subtract_days")]
    pub fn subtract_days(&mut self, n_days: u32) -> Result<(), BoolError> {
        let julian = self.get_julian();
        if julian > n_days {
            Err(bool_error!("invalid number of days"))
        } else {
            unsafe {
                ffi::g_date_subtract_days(self.to_glib_none_mut().0, n_days);
            }
            Ok(())
        }
    }

    #[doc(alias = "g_date_subtract_months")]
    pub fn subtract_months(&mut self, n_months: u32) -> Result<(), BoolError> {
        // The checks for this function are just a mess in the C code, allowing intermediate
        // unknown state. So for now, nothing can be done...
        unsafe {
            ffi::g_date_subtract_months(self.to_glib_none_mut().0, n_months);
        }
        Ok(())
    }

    #[doc(alias = "g_date_subtract_years")]
    pub fn subtract_years(&mut self, n_years: u16) -> Result<(), BoolError> {
        if self.get_year() < n_years {
            Err(bool_error!("invalid number of years"))
        } else {
            unsafe {
                ffi::g_date_subtract_years(self.to_glib_none_mut().0, n_years as _);
            }
            Ok(())
        }
    }

    //#[doc(alias="g_date_to_struct_tm")]
    //pub fn to_struct_tm(&self, tm: /*Unimplemented*/Fundamental: Pointer) {
    //    unsafe { TODO: call ffi::g_date_to_struct_tm() }
    //}

    #[doc(alias = "g_date_valid")]
    pub fn valid(&self) -> bool {
        unsafe { from_glib(ffi::g_date_valid(self.to_glib_none().0)) }
    }

    #[doc(alias = "g_date_get_days_in_month")]
    pub fn get_days_in_month(month: DateMonth, year: DateYear) -> u8 {
        unsafe { ffi::g_date_get_days_in_month(month.to_glib(), year) }
    }

    #[doc(alias = "g_date_get_monday_weeks_in_year")]
    pub fn get_monday_weeks_in_year(year: DateYear) -> u8 {
        unsafe { ffi::g_date_get_monday_weeks_in_year(year) }
    }

    #[doc(alias = "g_date_get_sunday_weeks_in_year")]
    pub fn get_sunday_weeks_in_year(year: DateYear) -> u8 {
        unsafe { ffi::g_date_get_sunday_weeks_in_year(year) }
    }

    #[doc(alias = "g_date_is_leap_year")]
    pub fn is_leap_year(year: DateYear) -> bool {
        unsafe { from_glib(ffi::g_date_is_leap_year(year)) }
    }

    #[doc(alias = "g_date_strftime")]
    pub fn strftime(s: &str, format: &str, date: &Date) -> usize {
        let slen = s.len() as usize;
        unsafe {
            ffi::g_date_strftime(
                s.to_glib_none().0,
                slen,
                format.to_glib_none().0,
                date.to_glib_none().0,
            )
        }
    }

    #[doc(alias = "g_date_valid_day")]
    pub fn valid_day(day: DateDay) -> bool {
        unsafe { from_glib(ffi::g_date_valid_day(day)) }
    }

    #[doc(alias = "g_date_valid_dmy")]
    pub fn valid_dmy(day: DateDay, month: DateMonth, year: DateYear) -> bool {
        unsafe { from_glib(ffi::g_date_valid_dmy(day, month.to_glib(), year)) }
    }

    #[doc(alias = "g_date_valid_julian")]
    pub fn valid_julian(julian_date: u32) -> bool {
        unsafe { from_glib(ffi::g_date_valid_julian(julian_date)) }
    }

    #[doc(alias = "g_date_valid_month")]
    pub fn valid_month(month: DateMonth) -> bool {
        unsafe { from_glib(ffi::g_date_valid_month(month.to_glib())) }
    }

    #[doc(alias = "g_date_valid_weekday")]
    pub fn valid_weekday(weekday: DateWeekday) -> bool {
        unsafe { from_glib(ffi::g_date_valid_weekday(weekday.to_glib())) }
    }

    #[doc(alias = "g_date_valid_year")]
    pub fn valid_year(year: DateYear) -> bool {
        unsafe { from_glib(ffi::g_date_valid_year(year)) }
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

impl fmt::Debug for Date {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("Date")
            .field("year", &self.get_year())
            .field("month", &self.get_month())
            .field("day", &self.get_day())
            .finish()
    }
}

impl hash::Hash for Date {
    fn hash<H>(&self, state: &mut H)
    where
        H: hash::Hasher,
    {
        self.get_year().hash(state);
        self.get_month().hash(state);
        self.get_day().hash(state);
    }
}
