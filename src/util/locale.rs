#![allow(dead_code)]
use crate::util::datetime::{Date, Month, Time, Weekday};
use std::fmt;

pub trait DateLocale {
    fn date_format(&self) -> DatePattern;
    fn time_format(&self) -> TimePattern;
    fn weekday_name(&self, weekday: &Weekday) -> (&'static str, &'static str);
    fn month_name(&self, month: &Month) -> (&'static str, &'static str);
}

#[derive(Clone, Copy)]
pub enum DatePattern {
    YmdDash,
    DmyDot,
    MdySlash,
}

#[derive(Clone, Copy)]
pub enum TimePattern {
    HmsColon12,
    HmsColon24,
}

pub fn fmt_short(date: &Date, locale: &dyn DateLocale) -> String {
    let (year, month, day) = date.to_ymd();

    match locale.date_format() {
        DatePattern::YmdDash => {
            format!("{year:04}-{month:02}-{day:02}")
        }
        DatePattern::DmyDot => {
            format!("{day:02}.{month:02}.{year:04}")
        }
        DatePattern::MdySlash => {
            format!("{month:02}/{day:02}/{year:04}")
        }
    }
}

// Example: "Monday, 10. March 2025"
pub fn fmt_long(date: &Date, locale: &dyn DateLocale) -> String {
    let (year, month, day) = date.to_ymd();
    let (_, weekday) = locale.weekday_name(&date.weekday());
    let (_, month) = locale.month_name(&month);
    format!("{weekday}, {day:02}. {month} {year:04}",)
}

pub trait TimeFormat {
    fn fmt(&self, locale: &impl DateLocale, f: &mut fmt::Formatter<'_>) -> fmt::Result;
}

impl TimeFormat for Time {
    fn fmt(&self, locale: &impl DateLocale, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let (hour, minute, second) = self.to_hms();
        match locale.time_format() {
            TimePattern::HmsColon12 => {
                let (hour, suffix) = if hour == 0 {
                    (12, "AM")
                } else if hour < 12 {
                    (hour, "AM")
                } else if hour == 12 {
                    (12, "PM")
                } else {
                    (hour - 12, "PM")
                };

                write!(f, "{hour}:{minute:02}:{second:02} {suffix}")
            }
            TimePattern::HmsColon24 => {
                write!(f, "{hour:02}:{minute:02}:{second:02}")
            }
        }
    }
}

pub struct LocaleUs;

impl DateLocale for LocaleUs {
    fn date_format(&self) -> DatePattern {
        DatePattern::MdySlash
    }

    fn time_format(&self) -> TimePattern {
        TimePattern::HmsColon12
    }

    fn weekday_name(&self, wd: &Weekday) -> (&'static str, &'static str) {
        match wd {
            Weekday::Mon => ("Mon", "Monday"),
            Weekday::Tue => ("Tue", "Tuesday"),
            Weekday::Wed => ("Wed", "Wednesday"),
            Weekday::Thu => ("Thu", "Thursday"),
            Weekday::Fri => ("Fri", "Friday"),
            Weekday::Sat => ("Sat", "Saturday"),
            Weekday::Sun => ("Sun", "Sunday"),
        }
    }

    fn month_name(&self, m: &Month) -> (&'static str, &'static str) {
        match m {
            Month::Jan => ("Jan", "January"),
            Month::Feb => ("Feb", "February"),
            Month::Mar => ("Mar", "March"),
            Month::Apr => ("Apr", "April"),
            Month::May => ("May", "May"),
            Month::Jun => ("Jun", "June"),
            Month::Jul => ("Jul", "July"),
            Month::Aug => ("Aug", "August"),
            Month::Sep => ("Sep", "September"),
            Month::Oct => ("Oct", "October"),
            Month::Nov => ("Nov", "November"),
            Month::Dec => ("Dec", "December"),
        }
    }
}

pub struct LocaleGerman;

impl DateLocale for LocaleGerman {
    fn date_format(&self) -> DatePattern {
        DatePattern::DmyDot
    }

    fn time_format(&self) -> TimePattern {
        TimePattern::HmsColon24
    }

    fn weekday_name(&self, wd: &Weekday) -> (&'static str, &'static str) {
        match wd {
            Weekday::Mon => ("Mo", "Montag"),
            Weekday::Tue => ("Di", "Dienstag"),
            Weekday::Wed => ("Mi", "Mittwoch"),
            Weekday::Thu => ("Do", "Donnerstag"),
            Weekday::Fri => ("Fr", "Freitag"),
            Weekday::Sat => ("Sa", "Samstag"),
            Weekday::Sun => ("So", "Sonntag"),
        }
    }

    fn month_name(&self, m: &Month) -> (&'static str, &'static str) {
        match m {
            Month::Jan => ("Jan", "Januar"),
            Month::Feb => ("Feb", "Februar"),
            Month::Mar => ("Mär", "März"),
            Month::Apr => ("Apr", "April"),
            Month::May => ("Mai", "Mai"),
            Month::Jun => ("Jun", "Juni"),
            Month::Jul => ("Jul", "Juli"),
            Month::Aug => ("Aug", "August"),
            Month::Sep => ("Sep", "September"),
            Month::Oct => ("Okt", "Oktober"),
            Month::Nov => ("Nov", "November"),
            Month::Dec => ("Dez", "Dezember"),
        }
    }
}
