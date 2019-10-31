use crate::{MICROS_PER_SEC, MILLIS_PER_SEC, NANOS_PER_SEC, SECONDS_PER_DAY, SECONDS_PER_HOUR,SECONDS_PER_MINUTE, SECONDS_PER_WEEK};

    /// The main struct that handles conversion from human readable time formats to seconds
    /// ### Structure
    /// ```
    /// #[derive(Debug)]
    /// pub struct LiteDuration;
    /// ```
    ///
    /// #### Usage
    /// ```
    /// use timelite::LiteDuration;
    /// LiteDuration::nanos(1);
    /// ```
#[derive(Debug)]
pub struct LiteDuration;
    
    /// #### Usage
    /// ```
    /// use timelite::LiteDuration;
    /// LiteDuration::nanos(1);
    /// ```
impl LiteDuration {
        /// #### Usage
        /// ```
        /// use timelite::LiteDuration;
        /// LiteDuration::nanos(1);
        /// ```
    pub fn nanos(value: u32) -> f32 {
        value as f32 / NANOS_PER_SEC as f32
    }
        /// #### Usage
        /// ```
        /// use timelite::LiteDuration;
        /// LiteDuration::micros(1);
        /// ```
    pub fn micros(value: u32) -> f32 {
        value as f32 / MICROS_PER_SEC as f32
    }
        /// #### Usage
        /// ```
        /// use timelite::LiteDuration;
        /// LiteDuration::millis(1);
        /// ```
    pub fn millis(value: u32) -> f32 {
        value as f32 / MILLIS_PER_SEC as f32
    }
        /// #### Usage
        /// ```
        /// use timelite::LiteDuration;
        /// LiteDuration::seconds(1);
        /// ```
    pub fn seconds(value: u64) -> u64 {
        value
    }
        /// #### Usage
        /// ```
        /// use timelite::LiteDuration;
        /// LiteDuration::minutes(1);
        /// ```
    pub fn minutes(value: u64) -> u64 {
        value * SECONDS_PER_MINUTE
    }
        /// #### Usage
        /// ```
        /// use timelite::LiteDuration;
        /// LiteDuration::hours(1);
        /// ```
    pub fn hours(value: u64) -> u64 {
        value * SECONDS_PER_HOUR
    }
        /// #### Usage
        /// ```
        /// use timelite::LiteDuration;
        /// LiteDuration::days(1);
        /// ```
    pub fn days(value: u64) -> u64 {
        value * SECONDS_PER_DAY
    }
        /// #### Usage
        /// ```
        /// use timelite::LiteDuration;
        /// LiteDuration::weeks(1);
        /// ```
    pub fn weeks(value: u64) -> u64 {
        value * SECONDS_PER_WEEK
    }
        /// #### Usage
        /// ```
        /// use timelite::LiteDuration;
        /// LiteDuration::months(1);
        /// ```
    pub fn months(value: u64) -> u64 {
        value * SECONDS_PER_WEEK * 4
    }
        /// #### Usage
        /// ```
        /// use timelite::LiteDuration;
        /// LiteDuration::years(1);
        /// ```
    pub fn years(value: u64) -> u64 {
        value * SECONDS_PER_WEEK * 4 * 12
    }
}
