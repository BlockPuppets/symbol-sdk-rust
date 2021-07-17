/*
 * // Copyright 2021 BlockPuppets.
 * //
 * // Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
 * // https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
 * // <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your
 * // option. This file may not be copied, modified, or distributed
 * // except according to those terms.
 */

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ServerInfo {
    /// catapult-rest component version.
    #[serde(rename = "restVersion")]
    pub rest_version: String,
    /// catapult-sdk component version.
    #[serde(rename = "sdkVersion")]
    pub sdk_version: String,
}
