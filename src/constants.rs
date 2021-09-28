use core::fmt;

pub(crate) const NOT_LEAP_YEAR_MONTHS: [i64; 12] = [31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];
pub(crate) const LEAP_YEAR_MONTHS: [i64; 12] = [31, 29, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];
pub const SECONDS_PER_DAY: i64 = 60 * 60 * 24; //86400 seconds per 24 hours
pub const NOT_LEAP_YEAR_MONTHS_SECONDS: [i64; 12] = [
    NOT_LEAP_YEAR_MONTHS[0] * SECONDS_PER_DAY,
    NOT_LEAP_YEAR_MONTHS[1] * SECONDS_PER_DAY,
    NOT_LEAP_YEAR_MONTHS[2] * SECONDS_PER_DAY,
    NOT_LEAP_YEAR_MONTHS[3] * SECONDS_PER_DAY,
    NOT_LEAP_YEAR_MONTHS[4] * SECONDS_PER_DAY,
    NOT_LEAP_YEAR_MONTHS[5] * SECONDS_PER_DAY,
    NOT_LEAP_YEAR_MONTHS[6] * SECONDS_PER_DAY,
    NOT_LEAP_YEAR_MONTHS[7] * SECONDS_PER_DAY,
    NOT_LEAP_YEAR_MONTHS[8] * SECONDS_PER_DAY,
    NOT_LEAP_YEAR_MONTHS[9] * SECONDS_PER_DAY,
    NOT_LEAP_YEAR_MONTHS[10] * SECONDS_PER_DAY,
    NOT_LEAP_YEAR_MONTHS[11] * SECONDS_PER_DAY,
];
pub const LEAP_YEAR_MONTHS_SECONDS: [i64; 12] = [
    LEAP_YEAR_MONTHS[0] * SECONDS_PER_DAY,
    LEAP_YEAR_MONTHS[1] * SECONDS_PER_DAY,
    LEAP_YEAR_MONTHS[2] * SECONDS_PER_DAY,
    LEAP_YEAR_MONTHS[3] * SECONDS_PER_DAY,
    LEAP_YEAR_MONTHS[4] * SECONDS_PER_DAY,
    LEAP_YEAR_MONTHS[5] * SECONDS_PER_DAY,
    LEAP_YEAR_MONTHS[6] * SECONDS_PER_DAY,
    LEAP_YEAR_MONTHS[7] * SECONDS_PER_DAY,
    LEAP_YEAR_MONTHS[8] * SECONDS_PER_DAY,
    LEAP_YEAR_MONTHS[9] * SECONDS_PER_DAY,
    LEAP_YEAR_MONTHS[10] * SECONDS_PER_DAY,
    LEAP_YEAR_MONTHS[11] * SECONDS_PER_DAY,
];
pub type UnixTimestamp = i64;

pub enum YearType {
    IsLeapYear,
    NotLeapYear,
}

impl fmt::Debug for YearType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::IsLeapYear => write!(f, "{}", "IsLeapYear"),
            Self::NotLeapYear => write!(f, "{}", "NotLeapYear"),
        }
    }
}

impl fmt::Display for YearType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::IsLeapYear => write!(f, "{}", "YearType::IsLeapYear"),
            Self::NotLeapYear => write!(f, "{}", "YearType::NotLeapYear"),
        }
    }
}

impl core::cmp::PartialEq for YearType {
    fn eq(&self, other: &Self) -> bool {
        if self == other {
            true
        } else {
            false
        }
    }
}
impl core::cmp::Eq for YearType {}

impl core::marker::Copy for YearType {}

impl core::clone::Clone for YearType {
    fn clone(&self) -> YearType {
        *self
    }
}

pub enum TimeStandard {
    Utc,
    Tai64,
    Tai64N,
    Tai64NA,
    UtcOffset,
}

impl fmt::Debug for TimeStandard {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Utc => write!(f, "{}", "Utc"),
            Self::Tai64 => write!(f, "{}", "Tai64"),
            Self::Tai64N => write!(f, "{}", "Tai64N"),
            Self::Tai64NA => write!(f, "{}", "Tai64NA"),
            Self::UtcOffset => write!(f, "{}", "UtcOffset"),
        }
    }
}

impl fmt::Display for TimeStandard {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Utc => write!(f, "{}", "TimeStandard::UTC"),
            Self::Tai64 => write!(f, "{}", "TimeStandard::TAi64"),
            Self::Tai64N => write!(f, "{}", "TimeStandard::TAi64N"),
            Self::Tai64NA => write!(f, "{}", "TimeStandard::TAi64NA"),
            Self::UtcOffset => write!(f, "{}", "TimeStandard::UTC OFFSET"),
        }
    }
}

impl core::cmp::PartialEq for TimeStandard {
    fn eq(&self, other: &Self) -> bool {
        if self == other {
            true
        } else {
            false
        }
    }
}
impl core::cmp::Eq for TimeStandard {}

impl core::marker::Copy for TimeStandard {}

impl core::clone::Clone for TimeStandard {
    fn clone(&self) -> TimeStandard {
        *self
    }
}
