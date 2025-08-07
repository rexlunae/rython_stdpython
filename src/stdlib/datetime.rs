//! Python datetime module implementation
//! 
//! This module provides classes for manipulating dates and times.
//! Implementation matches Python's datetime module API.

use crate::PyException;
use std::time::{SystemTime, Duration, UNIX_EPOCH};
use std::fmt;

// Days in each month (non-leap year)
const DAYS_IN_MONTH: [u32; 12] = [31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];

/// date - represents a date (year, month, day)
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct date {
    pub year: i32,
    pub month: u32,
    pub day: u32,
}

impl date {
    /// Create a new date
    pub fn new(year: i32, month: u32, day: u32) -> Result<Self, PyException> {
        if !(1..=12).contains(&month) {
            return Err(crate::value_error("month must be in 1..12"));
        }
        
        let max_day = if month == 2 && is_leap_year(year) { 29 } else { DAYS_IN_MONTH[month as usize - 1] };
        
        if !(1..=max_day).contains(&day) {
            return Err(crate::value_error(format!("day must be in 1..{}", max_day)));
        }
        
        Ok(Self { year, month, day })
    }
    
    /// Get today's date
    pub fn today() -> Self {
        let now = SystemTime::now();
        let duration = now.duration_since(UNIX_EPOCH).unwrap_or(Duration::from_secs(0));
        let days_since_epoch = duration.as_secs() / 86400;
        days_to_date(days_since_epoch as i64 + 719163) // Unix epoch offset
    }
    
    /// Create date from ordinal day
    pub fn fromordinal(ordinal: i64) -> Result<Self, PyException> {
        if ordinal < 1 {
            return Err(crate::value_error("ordinal must be >= 1"));
        }
        Ok(days_to_date(ordinal))
    }
    
    /// Convert to ordinal day
    pub fn toordinal(&self) -> i64 {
        date_to_days(*self)
    }
    
    /// Get weekday (0=Monday, 6=Sunday)
    pub fn weekday(&self) -> u32 {
        (self.toordinal() % 7) as u32
    }
    
    /// Get ISO weekday (1=Monday, 7=Sunday)
    pub fn isoweekday(&self) -> u32 {
        let wd = self.weekday();
        if wd == 0 { 7 } else { wd }
    }
    
    /// Get ISO calendar (year, week, weekday)
    pub fn isocalendar(&self) -> (i32, u32, u32) {
        let year = self.year;
        let ordinal = self.toordinal();
        let jan1_ordinal = date::new(year, 1, 1).unwrap().toordinal();
        let jan1_weekday = (jan1_ordinal % 7) as u32;
        
        let week = ((ordinal - jan1_ordinal + jan1_weekday as i64 + 7) / 7) as u32;
        (year, week.max(1), self.isoweekday())
    }
    
    /// Format as ISO string
    pub fn isoformat(&self) -> String {
        format!("{:04}-{:02}-{:02}", self.year, self.month, self.day)
    }
    
    /// Format with strftime
    pub fn strftime(&self, fmt: &str) -> String {
        // Simplified strftime implementation
        fmt.replace("%Y", &format!("{:04}", self.year))
           .replace("%m", &format!("{:02}", self.month))
           .replace("%d", &format!("{:02}", self.day))
           .replace("%B", month_name(self.month))
           .replace("%b", month_abbr(self.month))
           .replace("%A", weekday_name(self.weekday()))
           .replace("%a", weekday_abbr(self.weekday()))
    }
    
    /// Replace components
    pub fn replace(&self, year: Option<i32>, month: Option<u32>, day: Option<u32>) -> Result<Self, PyException> {
        Self::new(
            year.unwrap_or(self.year),
            month.unwrap_or(self.month),
            day.unwrap_or(self.day)
        )
    }
}

impl fmt::Display for date {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.isoformat())
    }
}

/// time - represents a time (hour, minute, second, microsecond)
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct time {
    pub hour: u32,
    pub minute: u32,
    pub second: u32,
    pub microsecond: u32,
}

impl time {
    /// Create a new time
    pub fn new(hour: u32, minute: u32, second: Option<u32>, microsecond: Option<u32>) -> Result<Self, PyException> {
        if hour >= 24 {
            return Err(crate::value_error("hour must be in 0..23"));
        }
        if minute >= 60 {
            return Err(crate::value_error("minute must be in 0..59"));
        }
        let second = second.unwrap_or(0);
        if second >= 60 {
            return Err(crate::value_error("second must be in 0..59"));
        }
        let microsecond = microsecond.unwrap_or(0);
        if microsecond >= 1_000_000 {
            return Err(crate::value_error("microsecond must be in 0..999999"));
        }
        
        Ok(Self { hour, minute, second, microsecond })
    }
    
    /// Format as ISO string
    pub fn isoformat(&self, timespec: Option<&str>) -> String {
        match timespec {
            Some("hours") => format!("{:02}", self.hour),
            Some("minutes") => format!("{:02}:{:02}", self.hour, self.minute),
            Some("seconds") => format!("{:02}:{:02}:{:02}", self.hour, self.minute, self.second),
            _ => {
                if self.microsecond == 0 {
                    format!("{:02}:{:02}:{:02}", self.hour, self.minute, self.second)
                } else {
                    format!("{:02}:{:02}:{:02}.{:06}", self.hour, self.minute, self.second, self.microsecond)
                }
            }
        }
    }
    
    /// Format with strftime
    pub fn strftime(&self, fmt: &str) -> String {
        fmt.replace("%H", &format!("{:02}", self.hour))
           .replace("%M", &format!("{:02}", self.minute))
           .replace("%S", &format!("{:02}", self.second))
           .replace("%f", &format!("{:06}", self.microsecond))
    }
    
    /// Replace components
    pub fn replace(&self, hour: Option<u32>, minute: Option<u32>, second: Option<u32>, microsecond: Option<u32>) -> Result<Self, PyException> {
        Self::new(
            hour.unwrap_or(self.hour),
            minute.unwrap_or(self.minute),
            Some(second.unwrap_or(self.second)),
            Some(microsecond.unwrap_or(self.microsecond))
        )
    }
}

impl fmt::Display for time {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.isoformat(None))
    }
}

/// datetime - represents a datetime (date + time)
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct datetime {
    pub date: date,
    pub time: time,
}

impl datetime {
    /// Create a new datetime
    pub fn new(year: i32, month: u32, day: u32, hour: Option<u32>, minute: Option<u32>, 
               second: Option<u32>, microsecond: Option<u32>) -> Result<Self, PyException> {
        let date = date::new(year, month, day)?;
        let time = time::new(
            hour.unwrap_or(0),
            minute.unwrap_or(0),
            second,
            microsecond
        )?;
        Ok(Self { date, time })
    }
    
    /// Get current datetime
    pub fn now() -> Self {
        let now = SystemTime::now();
        let duration = now.duration_since(UNIX_EPOCH).unwrap_or(Duration::from_secs(0));
        let total_seconds = duration.as_secs();
        let microseconds = duration.subsec_micros();
        
        let days_since_epoch = total_seconds / 86400;
        let seconds_today = total_seconds % 86400;
        
        let date = days_to_date(days_since_epoch as i64 + 719163);
        let hour = (seconds_today / 3600) as u32;
        let minute = ((seconds_today % 3600) / 60) as u32;
        let second = (seconds_today % 60) as u32;
        
        Self {
            date,
            time: time::new(hour, minute, Some(second), Some(microseconds)).unwrap(),
        }
    }
    
    /// Get UTC datetime
    pub fn utcnow() -> Self {
        Self::now() // Simplified - same as now() for this implementation
    }
    
    /// Create from timestamp
    pub fn fromtimestamp(timestamp: f64) -> Result<Self, PyException> {
        let duration = Duration::from_secs_f64(timestamp);
        let total_seconds = duration.as_secs();
        let microseconds = duration.subsec_micros();
        
        let days_since_epoch = (total_seconds / 86400) as i64;
        let seconds_today = total_seconds % 86400;
        
        let date = days_to_date(days_since_epoch + 719163);
        let hour = (seconds_today / 3600) as u32;
        let minute = ((seconds_today % 3600) / 60) as u32;
        let second = (seconds_today % 60) as u32;
        
        Ok(Self {
            date,
            time: time::new(hour, minute, Some(second), Some(microseconds))?,
        })
    }
    
    /// Convert to timestamp
    pub fn timestamp(&self) -> f64 {
        let days_since_epoch = self.date.toordinal() - 719163;
        let seconds_since_midnight = self.time.hour as u64 * 3600 + 
                                   self.time.minute as u64 * 60 + 
                                   self.time.second as u64;
        let microseconds = self.time.microsecond as u64;
        
        (days_since_epoch as u64 * 86400 + seconds_since_midnight) as f64 + microseconds as f64 / 1_000_000.0
    }
    
    /// Get date component
    pub fn date_component(&self) -> date {
        self.date
    }
    
    /// Get time component
    pub fn time_component(&self) -> time {
        self.time
    }
    
    /// Format as ISO string
    pub fn isoformat(&self, sep: Option<char>, timespec: Option<&str>) -> String {
        let sep = sep.unwrap_or('T');
        format!("{}{}{}", self.date.isoformat(), sep, self.time.isoformat(timespec))
    }
    
    /// Format with strftime
    pub fn strftime(&self, fmt: &str) -> String {
        self.date.strftime(&self.time.strftime(fmt))
    }
    
    /// Replace components
    pub fn replace(&self, year: Option<i32>, month: Option<u32>, day: Option<u32>,
                   hour: Option<u32>, minute: Option<u32>, second: Option<u32>, 
                   microsecond: Option<u32>) -> Result<Self, PyException> {
        let new_date = self.date.replace(year, month, day)?;
        let new_time = self.time.replace(hour, minute, second, microsecond)?;
        Ok(Self { date: new_date, time: new_time })
    }
}

impl fmt::Display for datetime {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.isoformat(None, None))
    }
}

/// timedelta - represents a duration
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct timedelta {
    pub days: i64,
    pub seconds: i64,
    pub microseconds: i64,
}

impl timedelta {
    /// Create a new timedelta
    pub fn new(days: Option<i64>, seconds: Option<i64>, microseconds: Option<i64>,
               milliseconds: Option<i64>, minutes: Option<i64>, hours: Option<i64>,
               weeks: Option<i64>) -> Self {
        let mut total_days = days.unwrap_or(0);
        let mut total_seconds = seconds.unwrap_or(0);
        let mut total_microseconds = microseconds.unwrap_or(0);
        
        if let Some(ms) = milliseconds {
            total_microseconds += ms * 1000;
        }
        if let Some(min) = minutes {
            total_seconds += min * 60;
        }
        if let Some(hr) = hours {
            total_seconds += hr * 3600;
        }
        if let Some(wk) = weeks {
            total_days += wk * 7;
        }
        
        // Normalize
        if total_microseconds >= 1_000_000 {
            total_seconds += total_microseconds / 1_000_000;
            total_microseconds %= 1_000_000;
        } else if total_microseconds < 0 {
            total_seconds += (total_microseconds - 999_999) / 1_000_000;
            total_microseconds = ((total_microseconds % 1_000_000) + 1_000_000) % 1_000_000;
        }
        
        if total_seconds >= 86400 {
            total_days += total_seconds / 86400;
            total_seconds %= 86400;
        } else if total_seconds < 0 {
            total_days += (total_seconds - 86399) / 86400;
            total_seconds = ((total_seconds % 86400) + 86400) % 86400;
        }
        
        Self {
            days: total_days,
            seconds: total_seconds,
            microseconds: total_microseconds,
        }
    }
    
    /// Get total seconds
    pub fn total_seconds(&self) -> f64 {
        self.days as f64 * 86400.0 + self.seconds as f64 + self.microseconds as f64 / 1_000_000.0
    }
}

impl fmt::Display for timedelta {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.days != 0 {
            write!(f, "{} day{}, ", self.days, if self.days == 1 { "" } else { "s" })?;
        }
        
        let hours = self.seconds / 3600;
        let minutes = (self.seconds % 3600) / 60;
        let seconds = self.seconds % 60;
        
        if self.microseconds != 0 {
            write!(f, "{}:{:02}:{:02}.{:06}", hours, minutes, seconds, self.microseconds)
        } else {
            write!(f, "{}:{:02}:{:02}", hours, minutes, seconds)
        }
    }
}

// Helper functions
fn is_leap_year(year: i32) -> bool {
    (year % 4 == 0 && year % 100 != 0) || (year % 400 == 0)
}

fn days_to_date(ordinal: i64) -> date {
    // Simplified algorithm - not historically accurate for very old dates
    let mut year = 1;
    let mut remaining_days = ordinal - 1;
    
    // Find the year
    while remaining_days >= 365 {
        let year_days = if is_leap_year(year) { 366 } else { 365 };
        if remaining_days < year_days {
            break;
        }
        remaining_days -= year_days;
        year += 1;
    }
    
    // Find the month and day
    let mut month = 1;
    while month <= 12 {
        let month_days = if month == 2 && is_leap_year(year) { 29 } else { DAYS_IN_MONTH[month as usize - 1] };
        if remaining_days < month_days as i64 {
            break;
        }
        remaining_days -= month_days as i64;
        month += 1;
    }
    
    date::new(year, month, remaining_days as u32 + 1).unwrap()
}

fn date_to_days(d: date) -> i64 {
    let mut days = 0i64;
    
    // Add days for complete years
    for y in 1..d.year {
        days += if is_leap_year(y) { 366 } else { 365 };
    }
    
    // Add days for complete months in the current year
    for m in 1..d.month {
        let month_days = if m == 2 && is_leap_year(d.year) { 29 } else { DAYS_IN_MONTH[m as usize - 1] };
        days += month_days as i64;
    }
    
    // Add remaining days
    days + d.day as i64
}

fn month_name(month: u32) -> &'static str {
    match month {
        1 => "January", 2 => "February", 3 => "March", 4 => "April",
        5 => "May", 6 => "June", 7 => "July", 8 => "August",
        9 => "September", 10 => "October", 11 => "November", 12 => "December",
        _ => "Unknown"
    }
}

fn month_abbr(month: u32) -> &'static str {
    match month {
        1 => "Jan", 2 => "Feb", 3 => "Mar", 4 => "Apr",
        5 => "May", 6 => "Jun", 7 => "Jul", 8 => "Aug",
        9 => "Sep", 10 => "Oct", 11 => "Nov", 12 => "Dec",
        _ => "Unk"
    }
}

fn weekday_name(weekday: u32) -> &'static str {
    match weekday {
        0 => "Monday", 1 => "Tuesday", 2 => "Wednesday", 3 => "Thursday",
        4 => "Friday", 5 => "Saturday", 6 => "Sunday",
        _ => "Unknown"
    }
}

fn weekday_abbr(weekday: u32) -> &'static str {
    match weekday {
        0 => "Mon", 1 => "Tue", 2 => "Wed", 3 => "Thu",
        4 => "Fri", 5 => "Sat", 6 => "Sun",
        _ => "Unk"
    }
}

// Constants
pub const MINYEAR: i32 = 1;
pub const MAXYEAR: i32 = 9999;

// Module-level functions
/// Get current UTC time
pub fn utcnow() -> datetime {
    datetime::utcnow()
}