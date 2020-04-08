use chrono::{DateTime, Utc, NaiveDate};
use std::fmt;
use std::fmt::Formatter;

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
    tis: u8, // tariff info
    iv: u8, // valid
    hour: u8,
    res1: u8, // reserve 1
    su: u8, // summer time
    month_day: u8,
    week_day: u8,
    month: u8,
    eti: u8, // energy tariff
    pti: u8, // power tariff
    year: u8,
    res2: u8, // reserve 2
}

impl TimeLabelA {
    pub fn from_be_bytes(bin: [u8; 5]) -> Self {
        let minute = bin[0] & 0b00111111;
        let tis    = bin[0] & 0b01000000;
        let iv     = bin[0] & 0b10000000;

        let hour= bin[1] & 0b00011111;
        let res1= bin[1] & 0b01100000;
        let su  = bin[1] & 0b10000000;

        let month_day = bin[2] & 0b00011111;
        let week_day  = bin[2] & 0b11100000;

        let month = bin[3] & 0b00001111;
        let eti   = bin[3] & 0b00110000;
        let pti   = bin[3] & 0b11000000;

        let year = bin[4] & 0b01111111;
        let res2 = bin[4] & 0b10000000;

        TimeLabelA {
            minute,
            tis,
            iv,
            hour,
            res1,
            su,
            month_day,
            week_day,
            month,
            eti,
            pti,
            year,
            res2,
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
    tis: u8, // tariff info
    iv: u8, // valid
    hour: u8,
    res1: u8, // reserve 1
    su: u8, // summer time
    month_day: u8,
    week_day: u8,
    month: u8,
    eti: u8, // energy tariff
    pti: u8, // power tariff
    year: u8,
    res2: u8, // reserve 2
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