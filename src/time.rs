pub enum TimeLabel {
    A(TimeLabelA),
    B(TimeLabelB),
}

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