/*
 * // Copyright 2021 Developers of the Symbol sdk Rust project.
 * //
 * // Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
 * // https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
 * // <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your
 * // option. This file may not be copied, modified, or distributed
 * // except according to those terms.
 */

use std::fmt;

use chrono::{DateTime, NaiveDateTime, Utc};

#[serde(rename_all = "camelCase")]
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct NodeTime {
    pub send_timestamp: u64,
    pub receive_timestamp: u64,
}

impl NodeTime {
    fn timestamp_format(&self, timestamp: u64) -> String {
        let naive = NaiveDateTime::from_timestamp(timestamp as i64, 0);
        // Create DateTime from SystemTime
        let datetime: DateTime<Utc> = DateTime::from_utc(naive, Utc);
        // Formats the combined date and time with the specified format string.
        datetime.format("%Y-%m-%d %H:%M:%S").to_string()
    }
}

impl fmt::Display for NodeTime {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{{\n  \"receive_timestamp:\" {}, \n  \"send_timestamp:\" {} \n}},",
            self.timestamp_format(self.receive_timestamp),
            self.timestamp_format(self.send_timestamp)
        )
    }
}
