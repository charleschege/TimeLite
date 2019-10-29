/// The number of nanoseconds in a microsecond.
pub const NANOS_PER_MICRO: u32 = 1000;
/// The number of nanoseconds in a millisecond.
pub const NANOS_PER_MILLI: u32 = 1000_000;
/// The number of nanoseconds in seconds.
pub const NANOS_PER_SEC: u32 = 1_000_000_000;
/// The number of microseconds per second.
pub const MICROS_PER_SEC: i64 = 1000_000;
/// The number of milliseconds per second.
pub const MILLIS_PER_SEC: i64 = 1000;
/// The number of seconds in one minute.
pub const SECONDS_PER_MINUTE: u64 = 60;

/// The number of seconds in one hour.
pub const SECONDS_PER_HOUR: u64 = 60 * SECONDS_PER_MINUTE;

/// The number of seconds in one day.
pub const SECONDS_PER_DAY: u64 = 24 * SECONDS_PER_HOUR;

/// The number of seconds in one week.
pub const SECONDS_PER_WEEK: u64 = 7 * SECONDS_PER_DAY;
