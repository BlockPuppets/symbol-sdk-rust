/*
 * // Copyright 2021 BlockPuppets developers.
 * //
 * // Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
 * // https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
 * // <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your
 * // option. This file may not be copied, modified, or distributed
 * // except according to those terms.
 */

use anyhow::{ensure, Result};
use chrono::{DateTime, Duration, Local, TimeZone};
use std::ops::{AddAssign, Deref};
use std::time::UNIX_EPOCH;

/// The deadline of the transaction.
/// The deadline is given as the number of seconds elapsed since the creation of the nemesis block.
/// If a transaction does not get included in a block before the deadline is reached, it is deleted.
///
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct DeadLine(u64);

impl DeadLine {
    /// Create DeadLine.
    ///
    /// # Inputs
    ///
    /// * `epoch_adjustment:` The network's epoch adjustment (seconds). Defined in the network/properties, e.g. 1573430400;
    /// * `duration`: The time duration with nanosecond precision.
    ///
    /// # Example
    ///
    /// ```
    /// use chrono::Duration;
    /// use symbol_sdk::DeadLine;
    ///
    /// #
    /// # fn main() {
    /// #
    /// let deadline = DeadLine::create(1573430400, Duration::hours(2)).unwrap();
    /// # println!("{}", deadline);
    /// # }
    /// ```
    ///
    pub fn create(epoch_adjustment: u64, duration: Duration) -> Result<Self> {
        let mut deadline_date_time = Local::now();
        ensure!(
            duration.num_milliseconds() > 0,
            "deadline should be greater than 0"
        );

        deadline_date_time = deadline_date_time.checked_add_signed(duration).unwrap();

        deadline_date_time = deadline_date_time
            .checked_sub_signed(Duration::seconds(epoch_adjustment as i64))
            .unwrap();

        Ok(Self(deadline_date_time.timestamp_millis() as u64))
    }

    /// Returns deadline as local date time.
    ///
    pub fn to_local_date_time(&self, epoch_adjustment: u64) -> DateTime<Local> {
        return self.to_local_date_time_given_time_zone(epoch_adjustment, Local);
    }

    /// Returns deadline as local date time.
    ///
    pub fn to_local_date_time_given_time_zone<Tz: TimeZone>(
        &self,
        epoch_adjustment: u64,
        zone_id: Tz,
    ) -> DateTime<Tz> {
        let mut naive_date_time =
            UNIX_EPOCH + Duration::milliseconds(self.0 as i64).to_std().unwrap();

        naive_date_time.add_assign(Duration::seconds(epoch_adjustment as i64).to_std().unwrap());
        DateTime::<Local>::from(naive_date_time).with_timezone(&zone_id)
    }

    pub fn to_bytes(&self) -> [u8; 8] {
        self.to_le_bytes()
    }
}

impl Deref for DeadLine {
    type Target = u64;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl core::fmt::Display for DeadLine {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.deref())
    }
}

#[cfg(test)]
mod tests {
    use crate::DeadLine;
    use chrono::{Datelike, Duration, Local};

    const EPOCH_ADJUSTMENT: u64 = 1573430400;

    #[test]
    fn test_should_create_timestamp_today() {
        let deadline = DeadLine::create(EPOCH_ADJUSTMENT, Duration::hours(2)).unwrap();

        // avoid SYSTEM and UTC differences
        let mut network_time_stamp = Local::now();

        network_time_stamp = network_time_stamp
            .checked_add_signed(Duration::hours(2))
            .unwrap();

        assert_eq!(
            deadline.to_local_date_time(EPOCH_ADJUSTMENT).day(),
            network_time_stamp.day()
        );
        assert_eq!(
            deadline.to_local_date_time(EPOCH_ADJUSTMENT).month(),
            network_time_stamp.month()
        );
        assert_eq!(
            deadline.to_local_date_time(EPOCH_ADJUSTMENT).year(),
            network_time_stamp.year()
        );
    }
}
