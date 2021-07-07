/*
 * // Copyright 2021 BlockPuppets.
 * //
 * // Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
 * // https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
 * // <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your
 * // option. This file may not be copied, modified, or distributed
 * // except according to those terms.
 */

use std::str::FromStr;

use anyhow::Result;

use crate::model_dto::CommunicationTimestampsDto;
use crate::node::NodeTime;

#[derive(Clone, Serialize, Deserialize)]
pub struct NodeTimeDto {
    #[serde(rename = "communicationTimestamps")]
    pub communication_timestamps: CommunicationTimestampsDto,
}

impl NodeTimeDto {
    pub fn to_compact(&self) -> Result<NodeTime> {
        Ok(NodeTime {
            send_timestamp: u64::from_str(self.communication_timestamps.send_timestamp.as_str())?,
            receive_timestamp: u64::from_str(self.communication_timestamps.receive_timestamp.as_str())?,
        })
    }
}
