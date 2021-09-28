use crate::{
    TimeStandard, UnixTimestamp, YearType, LEAP_YEAR_MONTHS, LEAP_YEAR_MONTHS_SECONDS,
    NOT_LEAP_YEAR_MONTHS, NOT_LEAP_YEAR_MONTHS_SECONDS, SECONDS_PER_DAY,
};
use core::fmt;
pub struct LiteDateTime {
    unix_timestamp: UnixTimestamp,
    year: i64,
    month: u8,
    day: u8,
    hour: u8,
    minutes: u8,
    seconds: u8,
    year_type: YearType,
    time_standard: TimeStandard,
}

impl LiteDateTime {
    pub fn new(unix_timestamp: UnixTimestamp) -> Self {
        Self {
            unix_timestamp,
            year: i64::default(),
            month: u8::default(),
            day: u8::default(),
            hour: u8::default(),
            minutes: u8::default(),
            seconds: u8::default(),
            year_type: YearType::NotLeapYear,
            time_standard: TimeStandard::Utc,
        }
    }

    /// Convert Unix epoch seconds into date time.
    pub fn seconds_to_datetime_mut(&mut self) -> &mut Self {
        self.convert_to_self()
    }

    /// Convert Unix epoch seconds into date time.

    /// Convert Unix epoch seconds into date time.
    pub fn seconds_to_datetime(&mut self) -> &Self {
        self.convert_to_self()
    }
    fn convert_to_self(&mut self) -> &mut Self {
        let mut digital_genesis_time = 1970;

        let day_clock: i64 = self.unix_timestamp % 86400;
        let mut day_count: i64 = self.unix_timestamp / 86400;

        self.seconds = (day_clock % 60) as u8;
        self.minutes = ((day_clock % 3600) / 60) as u8;
        self.hour = (day_clock / 3600) as u8;

        let is_leap_year = LiteDateTime::check_is_leap_year(digital_genesis_time);

        if is_leap_year {
            self.year_type = YearType::IsLeapYear;
        } else {
            self.year_type = YearType::NotLeapYear;
        }

        loop {
            let year_size = match is_leap_year {
                true => 366,
                false => 365,
            };

            if day_count >= year_size {
                day_count -= year_size;
                digital_genesis_time += 1;
            } else {
                break;
            }
        }
        self.year = digital_genesis_time as i64;
        self.time_standard = TimeStandard::Utc;

        let mut month = 0;

        while day_count
            >= if is_leap_year {
                LEAP_YEAR_MONTHS[month]
            } else {
                NOT_LEAP_YEAR_MONTHS[month]
            }
        {
            day_count -= if is_leap_year {
                LEAP_YEAR_MONTHS[month]
            } else {
                NOT_LEAP_YEAR_MONTHS[month]
            };
            month += 1;
        }
        self.month = month as u8;
        self.day = day_count as u8 + 1;

        self
    }

    pub fn check_is_leap_year(value: i64) -> bool {
        if value % 4 == 0 && (value % 100 != 0 || value % 400 == 0) {
            true
        } else {
            false
        }
    }

    pub fn date(&self) -> String {
        format!("{:04}-{:02}-{:02}-UTC", self.year, self.month, self.day)
    }

    pub fn year_month(&self) -> String {
        format!("{:04}-{:02}-UTC", self.year, self.month)
    }

    pub fn time(&self) -> String {
        format!(
            "{:02}:{:02}:{:02}+0300",
            self.hour, self.minutes, self.seconds
        )
    }

    pub fn next_unix_date(&self, month_count: MonthCount) -> LiteDateTime {
        let next_unix_timestamp = self.unix_timestamp + (month_count as i64 * SECONDS_PER_DAY);
        let mut next_datetime = LiteDateTime::new(next_unix_timestamp);
        
        next_datetime.seconds_to_datetime().clone()
    }
}

impl fmt::Debug for LiteDateTime {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("LiteDateTime")
            .field("year", &self.year)
            .field("month", &self.month)
            .field("day", &self.day)
            .field("hours", &self.hour)
            .field("minutes", &self.minutes)
            .field("seconds", &self.seconds)
            .field("year_type", &self.year_type)
            .field("time_standard", &self.time_standard)
            .field("unix_timestamp", &self.unix_timestamp)
            .finish()
    }
}

impl fmt::Display for LiteDateTime {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{:04}-{:02}-{:02} {:02}:{:02}:{:02}+0000",
            self.year, self.month, self.day, self.hour, self.minutes, self.seconds,
        )
    }
}

impl core::cmp::PartialEq for LiteDateTime {
    fn eq(&self, other: &Self) -> bool {
        if self.year == other.year
            && self.month == other.month
            && self.day == other.day
            && self.hour == other.hour
            && self.minutes == other.minutes
            && self.seconds == other.seconds
        {
            true
        } else {
            false
        }
    }
}
impl core::cmp::Eq for LiteDateTime {}

impl core::marker::Copy for LiteDateTime {}

impl core::clone::Clone for LiteDateTime {
    fn clone(&self) -> LiteDateTime {
        *self
    }
}

pub enum MonthCount {
    OneMonth = 1,
    TwoMonths = 2,
    ThreeMonths = 3,
    FourMonths = 4,
    FiveMonths = 5,
    SixMonths = 6,
    SevenMonths = 7,
    EightMonths = 8,
    NineMonths = 9,
    TenMonths = 10,
    ElevenMonths = 11,
    TwelveMonths = 12,
}
