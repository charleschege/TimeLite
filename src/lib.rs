#![deny(missing_docs)]
#![deny(missing_doc_code_examples)]
#![deny(unsafe_code)]

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

mod constants;
pub use constants::{
    MICROS_PER_SEC, MILLIS_PER_SEC, NANOS_PER_MICRO, NANOS_PER_MILLI, NANOS_PER_SEC,
    SECONDS_PER_DAY, SECONDS_PER_HOUR, SECONDS_PER_MINUTE, SECONDS_PER_WEEK,
};

mod duration;
pub use duration::LiteDuration;
