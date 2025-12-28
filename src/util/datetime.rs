// Date utilities based on Howard Hinnant's algorithms
// https://howardhinnant.github.io/date_algorithms.html

use crate::error::{Error, Result};
use serde::Deserialize;
use std::fmt;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

// ----------------------------------------------------------------------------
const SECONDS_PER_DAY: u64 = 86_400;

// ----------------------------------------------------------------------------
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Weekday {
    Mon,
    Tue,
    Wed,
    Thu,
    Fri,
    Sat,
    Sun,
}

// ----------------------------------------------------------------------------
impl TryFrom<i32> for Weekday {
    type Error = Error;

    fn try_from(value: i32) -> Result<Self> {
        match value {
            0 => Ok(Weekday::Mon),
            1 => Ok(Weekday::Tue),
            2 => Ok(Weekday::Wed),
            3 => Ok(Weekday::Thu),
            4 => Ok(Weekday::Fri),
            5 => Ok(Weekday::Sat),
            6 => Ok(Weekday::Sun),
            _ => Err(Error::InvalidDate),
        }
    }
}

// ----------------------------------------------------------------------------
#[derive(Clone, Copy, Debug)]
pub enum Month {
    Jan = 1,
    Feb = 2,
    Mar = 3,
    Apr = 4,
    May = 5,
    Jun = 6,
    Jul = 7,
    Aug = 8,
    Sep = 9,
    Oct = 10,
    Nov = 11,
    Dec = 12,
}

// ----------------------------------------------------------------------------
impl From<Month> for i32 {
    fn from(value: Month) -> i32 {
        value as i32
    }
}

// ----------------------------------------------------------------------------
impl fmt::Display for Month {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let num: i32 = (*self).into();
        write!(f, "{num}")
    }
}

// ----------------------------------------------------------------------------
const fn is_leap_year(year: i32) -> bool {
    (year % 4 == 0 && year % 100 != 0) || (year % 400 == 0)
}

// ----------------------------------------------------------------------------
const fn days_in_month(year: i32, month: i32) -> Result<i32> {
    match month {
        1 | 3 | 5 | 7 | 8 | 10 | 12 => Ok(31),
        4 | 6 | 9 | 11 => Ok(30),
        2 => {
            if is_leap_year(year) {
                Ok(29)
            } else {
                Ok(28)
            }
        }
        _ => Err(Error::InvalidDate),
    }
}

// ----------------------------------------------------------------------------
// Using 32 bit arithmetic, overflow occurs at +/- 5.8 million years
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Date(i32);

impl Date {
    // ------------------------------------------------------------------------
    pub fn new(days: i32) -> Self {
        Date(days)
    }

    // ------------------------------------------------------------------------
    pub fn from_ymd(year: i32, month: i32, day: i32) -> Result<Self> {
        let dim = days_in_month(year, month)?;
        if day < 1 || day > dim {
            return Err(Error::InvalidDate);
        }
        Ok(Date(days_from_gregorian(year, month, day)))
    }

    // ------------------------------------------------------------------------
    pub fn today() -> Self {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or(Duration::ZERO);

        let today = now.as_secs().div_euclid(SECONDS_PER_DAY);

        // will not overflow for at least 5.8 million years
        Date(today as i32)
    }

    // ------------------------------------------------------------------------
    pub const fn to_ymd(self) -> (i32, Month, i32) {
        gregorian_from_days(self.0)
    }

    // ------------------------------------------------------------------------
    pub const fn weekday(&self) -> Weekday {
        // 1970-01-01 = Thursday = ISO weekday 3 (Mon=0)
        const THURSDAY: i32 = 3;
        let day = self.0 + THURSDAY;
        match day.rem_euclid(7) {
            0 => Weekday::Mon,
            1 => Weekday::Tue,
            2 => Weekday::Wed,
            3 => Weekday::Thu,
            4 => Weekday::Fri,
            5 => Weekday::Sat,
            6 => Weekday::Sun,
            _ => unreachable!(),
        }
    }
}

// ----------------------------------------------------------------------------
#[rustfmt::skip]
const fn days_from_gregorian(year: i32, month: i32, day: i32) -> i32 {
    // March-based year and month
    let y = year + if month > 2 { 0 } else { -1 };
    let m = month + if month > 2 { -3 } else { 9 };                 // [0, 11]

    let era = y.div_euclid(400);
    let yoe = y.rem_euclid(400);                                    // [0, 399]

    let doy = (153 * m + 2) / 5 + day - 1;                          // [0, 365]
    let doe = yoe * 365 + yoe / 4 - yoe / 100 + doy;                // [0, 146096]

    era * 146_097 + doe - 719_468
}

// ----------------------------------------------------------------------------
#[rustfmt::skip]
const fn gregorian_from_days(days: i32) -> (i32, Month, i32) {
    const MONTHS: [(Month, i32); 12] = [
        (Month::Mar, 0), (Month::Apr, 0), (Month::May, 0), (Month::Jun, 0), 
        (Month::Jul, 0), (Month::Aug, 0), (Month::Sep, 0), (Month::Oct, 0), 
        (Month::Nov, 0), (Month::Dec, 0), (Month::Jan, 1), (Month::Feb, 1),
    ];
    let z = days + 719_468;
    let era = z.div_euclid(146_097);
    let doe = z.rem_euclid(146_097);                                // [0, 146096]
    let yoe = (doe - doe / 1460 + doe / 36524 - doe / 146096) / 365; // [0, 399]
    let y = yoe + era * 400;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);              // [0, 365]
    let mp = (5 * doy + 2) / 153;                                   // [0, 11]
    let day = doy - (153 * mp + 2) / 5 + 1;                         // [1, 31]
    let (month, year_offset) = MONTHS[mp as usize];
    let year = y + year_offset;
    (year, month, day)
}

// ----------------------------------------------------------------------------
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Time(u32);

impl Time {
    // ------------------------------------------------------------------------
    pub fn new(seconds: u64) -> Result<Self> {
        if seconds >= SECONDS_PER_DAY {
            return Err(Error::InvalidTime);
        }
        Ok(Time(seconds as u32))
    }

    // ------------------------------------------------------------------------
    pub fn from_hms(hour: u32, minute: u32, second: u32) -> Result<Self> {
        if hour >= 24 || minute >= 60 || second >= 60 {
            return Err(Error::InvalidTime);
        }
        Ok(Time(hour * 3600 + minute * 60 + second))
    }

    // ------------------------------------------------------------------------
    pub fn now() -> Self {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or(Duration::ZERO);

        let seconds = now.as_secs().rem_euclid(SECONDS_PER_DAY);

        // will never overflow because 0 ≤ seconds_today < SECONDS_PER_DAY
        Time(seconds as u32)
    }

    // ------------------------------------------------------------------------
    pub const fn to_hms(self) -> (u32, u32, u32) {
        let hour = self.0 / 3600;
        let minute = (self.0 % 3600) / 60;
        let second = self.0 % 60;
        (hour, minute, second)
    }
}

// ----------------------------------------------------------------------------
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct DateTime {
    pub date: Date,
    pub time: Time,
}

impl DateTime {
    // ------------------------------------------------------------------------
    pub fn now() -> Self {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or(Duration::ZERO);

        let days = now.as_secs().div_euclid(SECONDS_PER_DAY);
        let seconds = now.as_secs().rem_euclid(SECONDS_PER_DAY);

        DateTime {
            date: Date(days as i32),
            time: Time(seconds as u32),
        }
    }

    // ------------------------------------------------------------------------
    pub fn from_iso8601(s: &str) -> Result<Self> {
        // Strict minimal form: YYYY-MM-DDTHH:MM:SS(Z optional)
        let s = s.trim_end_matches('Z');
        let s = s.get(0..19).ok_or(Error::InvalidDate)?;

        let year = s[0..4].parse()?;
        let month = s[5..7].parse()?;
        let day = s[8..10].parse()?;
        let hour = s[11..13].parse()?;
        let minute = s[14..16].parse()?;
        let second = s[17..19].parse()?;

        let date = Date::from_ymd(year, month, day)?;
        let time = Time::from_hms(hour, minute, second)?;

        Ok(Self { date, time })
    }

    // ------------------------------------------------------------------------
    pub fn as_iso8601(&self) -> String {
        let (year, month, day) = self.date.to_ymd();
        let (hour, minute, second) = self.time.to_hms();
        format!("{year:04}-{month:02}-{day:02}T{hour:02}:{minute:02}:{second:02}Z")
    }

    // ------------------------------------------------------------------------
    pub fn as_timestamp(&self) -> String {
        let (year, month, day) = self.date.to_ymd();
        let (hour, minute, second) = self.time.to_hms();
        format!("{year:04}{month:02}{day:02}_{hour:02}{minute:02}{second:02}")
    }
}

// ----------------------------------------------------------------------------
impl fmt::Display for DateTime {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let (year, month, day) = self.date.to_ymd();
        let (hour, minute, second) = self.time.to_hms();
        write!(
            f,
            "{year:04}/{month:02}/{day:02}-{hour:02}:{minute:02}:{second:02}"
        )
    }
}

// ----------------------------------------------------------------------------
impl<'a> Deserialize<'a> for DateTime {
    fn deserialize<D: serde::Deserializer<'a>>(
        deserializer: D,
    ) -> std::result::Result<Self, D::Error> {
        let s: &str = serde::Deserialize::deserialize(deserializer)?;
        DateTime::from_iso8601(s).map_err(serde::de::Error::custom)
    }
}

// ----------------------------------------------------------------------------
#[cfg(test)]
mod tests {
    use super::*;

    /// (days_since_1970, year, month, day, weekday)
    #[rustfmt::skip]
    pub const TEST_DATES: [(i32, i32, i32, i32, Weekday); 16] = [
        // Epoch
        (   0,  1970,  1,  1, Weekday::Thu),
        (   1,  1970,  1,  2, Weekday::Fri),
        (  30,  1970,  1, 31, Weekday::Sat),
        (  31,  1970,  2,  1, Weekday::Sun),
        (  59,  1970,  3,  1, Weekday::Sun),
        ( 364,  1970, 12, 31, Weekday::Thu),

        // Year transitions
        ( 365,  1971,  1,  1, Weekday::Fri),
        ( 730,  1972,  1,  1, Weekday::Sat),
        ( 731,  1972,  1,  2, Weekday::Sun),
        (1095,  1972, 12, 31, Weekday::Sun),

        // Millennium leap year
        (10957, 2000,  1,  1, Weekday::Sat),
        (11322, 2000, 12, 31, Weekday::Sun),

        // Negative days (before 1970)
        (-1,    1969, 12, 31, Weekday::Wed),
        (-365,  1969,  1,  1, Weekday::Wed),
        (-366,  1968, 12, 31, Weekday::Tue), // (leap year)
        (-719528,  0,  1,  1, Weekday::Sat), // (proleptic Gregorian start)
    ];

    #[test]
    fn test_date_conversion() {
        for (days, year, month, day, weekday) in TEST_DATES.iter() {
            let date = Date(*days);
            let (y, m, d) = date.to_ymd();
            assert_eq!((*year, *month, *day), (y, m.into(), d));

            let computed_days = days_from_gregorian(y, m.into(), d);
            assert_eq!(*days, computed_days);

            let wd = date.weekday();
            assert_eq!(*weekday, wd, "Weekday mismatch for {year}-{month}-{day}",);
        }
    }

    #[test]
    fn test_current_time() {
        let now = DateTime::now();
        let week_day = now.date.weekday();
        println!("Today is {week_day:?}, {now}");
    }
}
