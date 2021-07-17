/*
 * // Copyright 2021 BlockPuppets.
 * //
 * // Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
 * // https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
 * // <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your
 * // option. This file may not be copied, modified, or distributed
 * // except according to those terms.
 */

use std::fmt;

/// Node equality strategy.
/// Defines if the identifier for the node must be its public key or host.
#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub enum NodeIdentityEqualityStrategy {
    #[serde(rename = "host")]
    Host,
    #[serde(rename = "public-key")]
    PublicKey,
}

impl fmt::Display for NodeIdentityEqualityStrategy {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(&self).unwrap_or_default()
        )
    }
}
