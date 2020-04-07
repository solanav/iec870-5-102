use chrono::{DateTime, Utc, NaiveDate};
use std::str::FromStr;
use std::fmt;
use std::fmt::{Formatter, Error};

#[derive(Debug, Copy, Clone)]
pub enum TimeLabel {
    A(TimeLabelA),
    B(TimeLabelB),
}

impl fmt::Display for TimeLabel {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TimeLabel::A(t) => t.fmt(f),
            TimeLabel::B(t) => t.fmt(f),
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub struct TimeLabelA {
    minute: u8,
    TIS: u8, // tariff info
    IV: u8, // valid
    hour: u8,
    RES1: u8, // reserve 1
    SU: u8, // summer time
    month_day: u8,
    week_day: u8,
    month: u8,
    ETI: u8, // energy tariff
    PTI: u8, // power tariff
    year: u8,
    RES2: u8, // reserve 2
}

impl From<[u8; 5]> for TimeLabelA {
    fn from(bin: [u8; 5]) -> Self {
        let minute = bin[0] & 0b00111111;
        let hour = bin[1] & 0b00011111;
        let month_day = bin[2] & 0b00011111;
        let month = bin[3] & 0b00001111;
        let year = bin[4] & 0b01111111;

        TimeLabelA {
            minute,
            TIS: 0,
            IV: 0,
            hour,
            RES1: 0,
            SU: 0,
            month_day,
            week_day: 0,
            month,
            ETI: 0,
            PTI: 0,
            year,
            RES2: 0
        }
    }
}

impl From<TimeLabelA> for DateTime<Utc> {
    fn from(t: TimeLabelA) -> Self {
        let ndate = NaiveDate::from_ymd(
            t.year as i32,
            t.month as u32,
            t.month_day as u32)
            .and_hms(
                t.hour as u32,
                t.minute as u32,
                0u32,
            );

        DateTime::<Utc>::from_utc(ndate, Utc)
    }
}

impl fmt::Display for TimeLabelA {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "[{}/{}/{}] [{:02}:{:02}]",
            self.month_day, self.month, self.year,
            self.hour, self.minute
        )
    }
}

#[derive(Debug, Copy, Clone)]
pub struct TimeLabelB {
    minute: u8,
    TIS: u8, // tariff info
    IV: u8, // valid
    hour: u8,
    RES1: u8, // reserve 1
    SU: u8, // summer time
    month_day: u8,
    week_day: u8,
    month: u8,
    ETI: u8, // energy tariff
    PTI: u8, // power tariff
    year: u8,
    RES2: u8, // reserve 2
    seconds: u8,
    milliseconds: u8,
}

impl From<TimeLabelB> for DateTime<Utc> {
    fn from(t: TimeLabelB) -> Self {
        let ndate = NaiveDate::from_ymd(
            t.year as i32,
            t.month as u32,
            t.month_day as u32)
            .and_hms_milli(
                t.hour as u32,
                t.minute as u32,
                t.seconds as u32,
                t.milliseconds as u32,
            );

        DateTime::<Utc>::from_utc(ndate, Utc)
    }
}

impl fmt::Display for TimeLabelB {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "[{}/{}/{}] [{:02}:{:02}:{:02}:{}]",
               self.month_day, self.month, self.year,
               self.hour, self.minute, self.seconds, self.milliseconds
        )
    }
}