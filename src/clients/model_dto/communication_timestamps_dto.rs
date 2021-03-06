/*
 * // Copyright 2021 BlockPuppets.
 * //
 * // Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
 * // https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
 * // <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your
 * // option. This file may not be copied, modified, or distributed
 * // except according to those terms.
 */

#[serde(rename_all = "camelCase")]
#[derive(Clone, Serialize, Deserialize)]
pub struct CommunicationTimestampsDto {
    pub send_timestamp: String,
    pub receive_timestamp: String,
}
