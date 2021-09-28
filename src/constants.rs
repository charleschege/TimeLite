use core::fmt;

pub(crate) const NOT_LEAP_YEAR_MONTHS: [i64; 12] = [31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];
pub(crate) const LEAP_YEAR_MONTHS: [i64; 12] = [31, 29, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];

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
            Self::Tai64 => write!(f, "{}", "TimeStandard::TAI64"),
            Self::Tai64N => write!(f, "{}", "TimeStandard::TAI64N"),
            Self::Tai64NA => write!(f, "{}", "TimeStandard::TAI64NA"),
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
