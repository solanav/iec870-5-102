pub enum TimeLabel {
    A(TimeLabelA),
    B(TimeLabelB),
}

pub struct TimeLabelA {
    minute: u8,
    TIS: u8,
    IV: u8,
    hour: u8,
    RES1: u8,
    SU: u8,
    month_day: u8,
    week_dat: u8,
    month: u8,
    ETI: u8,
    PTI: u8,
    year: u8,
    RES2: u8,
}

pub struct TimeLabelB {
    minute: u8,
    TIS: u8,
    IV: u8,
    hour: u8,
    RES1: u8,
    SU: u8,
    month_day: u8,
    week_dat: u8,
    month: u8,
    ETI: u8,
    PTI: u8,
    year: u8,
    RES2: u8,
    seconds: u8,
    milliseconds: u8,
}