//! TimeLite is a simple library to convert days, weeks, months and years into seconds.
//!
//! This library is not necessary if you are using something like `chrono` crate but its purpose is to be
//! very egonormic for users to write seconds on a human level understanding of minutes, hours, days, weeks, months and years.
//!
//!
//! ## Examples
//!
//! ### 1. Handling Nanoseconds
//!
//! ```
//! use timelite::LiteDuration;
//!
//! let timer = LiteDuration::nanos(100);
//! ```
//!
//! ### 2. Handling Microseconds
//!
//! ```
//! use timelite::LiteDuration;
//!
//! let timer = LiteDuration::micros(1);
//! ```
//!
//! ### 3. Handling Milliseconds
//!
//! ```
//! use timelite::LiteDuration;
//!
//! let timer = LiteDuration::millis(1);
//! ```
//!
//! ### 4. Handling Seconds
//!
//! ```
//! use timelite::LiteDuration;
//!
//! let timer = LiteDuration::seconds(1);
//! ```
//!
//! ### 5. Handling Minutes
//!
//! ```
//! use timelite::LiteDuration;
//!
//! let timer = LiteDuration::minutes(1);
//! ```
//!
//! ### 6. Handling Hours
//!
//! ```
//! use timelite::LiteDuration;
//!
//! let timer = LiteDuration::hours(1);
//! ```
//!
//! ### 7. Handling Days
//!
//! ```
//! use timelite::LiteDuration;
//!
//! let timer = LiteDuration::days(1);
//! ```
//!
//! ### 8. Handling Weeks
//!
//! ```
//! use timelite::LiteDuration;
//! let timer = LiteDuration::weeks(1);
//! ```
//!
//! ### 9. Handling Months
//!
//! ```
//! use timelite::LiteDuration;
//! let timer = LiteDuration::months(1);
//! ```
//!
//! ### 10. Handling Years
//!
//! ```
//! use timelite::LiteDuration;
//! let timer = LiteDuration::years(1);
//! ```

//#![deny(missing_docs)]
//#![deny(missing_doc_code_examples)]
//#![forbid(unsafe_code)]

/// Module that handles time constants
#[allow(missing_doc_code_examples)]
mod constants;
pub use constants::*;

/// The module that handles conversions
///
/// ### Usage
/// ```
/// use timelite::LiteDateTime;
/// ```
mod duration;
pub use duration::*;

// FIXME fix this test
/*
#[test]
fn check_date() {
    use tai64::TAI64;
    let now = TAI64::now();

    let mut date_time = LiteDateTime::new(now.to_unix());
    let secs_to_datetime = date_time.seconds_to_datetime();

    println!("{}", &secs_to_datetime);
    dbg!(&secs_to_datetime);
    dbg!(&secs_to_datetime.date());
    dbg!(&secs_to_datetime.year_month());
    dbg!(&secs_to_datetime.time());
    dbg!(&secs_to_datetime.next_unix_date(MonthCount::TwoMonths));
}
*/
