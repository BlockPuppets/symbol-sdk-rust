/*
 * // Copyright 2021 BlockPuppets developers.
 * //
 * // Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
 * // https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
 * // <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your
 * // option. This file may not be copied, modified, or distributed
 * // except according to those terms.
 */

use std::fmt;
use std::ops::Deref;

/// `TransactionVersion` struct containing transaction version constants.
///
/// Transaction format versions are defined in catapult-server in each transaction's plugin source code.
///
/// In [catapult-server](https://github.com/nemtech/catapult-server), the `DEFINE_TRANSACTION_CONSTANTS` macro
/// is used to define the `TYPE` and `VERSION` of the transaction format.
///
/// @see https://github.com/nemtech/catapult-server/blob/main/plugins/txes/transfer/src/model/TransferTransaction.h#L37
///
#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize, Hash)]
pub struct TransactionVersion(u8);

impl TransactionVersion {
    /// Transfer Transaction transaction version.
    ///
    pub const TRANSFER: TransactionVersion = TransactionVersion(1);

    /// Register namespace transaction version.
    ///
    pub const NAMESPACE_REGISTRATION: TransactionVersion = TransactionVersion(1);

    /// Mosaic definition transaction version.
    ///
    pub const MOSAIC_DEFINITION: TransactionVersion = TransactionVersion(1);

    /// Mosaic supply change transaction version.
    ///
    pub const MOSAIC_SUPPLY_CHANGE: TransactionVersion = TransactionVersion(1);

    /// Modify multisig account transaction version.
    ///
    pub const MULTISIG_ACCOUNT_MODIFICATION: TransactionVersion = TransactionVersion(1);

    /// Aggregate complete transaction version.
    ///
    pub const AGGREGATE_COMPLETE: TransactionVersion = TransactionVersion(1);

    /// Aggregate bonded transaction version.
    ///
    pub const AGGREGATE_BONDED: TransactionVersion = TransactionVersion(1);

    /// Lock transaction version.
    ///
    pub const HASH_LOCK: TransactionVersion = TransactionVersion(1);

    /// Secret Lock transaction version.
    ///
    pub const SECRET_LOCK: TransactionVersion = TransactionVersion(1);

    /// Secret Proof transaction version.
    ///
    pub const SECRET_PROOF: TransactionVersion = TransactionVersion(1);

    /// Address Alias transaction version.
    ///
    pub const ADDRESS_ALIAS: TransactionVersion = TransactionVersion(1);

    /// Mosaic Alias transaction version.
    ///
    pub const MOSAIC_ALIAS: TransactionVersion = TransactionVersion(1);

    /// Mosaic global restriction transaction version.
    ///
    pub const MOSAIC_GLOBAL_RESTRICTION: TransactionVersion = TransactionVersion(1);

    /// Mosaic address restriction transaction version.
    ///
    pub const MOSAIC_ADDRESS_RESTRICTION: TransactionVersion = TransactionVersion(1);

    /// Account Restriction address transaction version.
    ///
    pub const ACCOUNT_ADDRESS_RESTRICTION: TransactionVersion = TransactionVersion(1);

    /// Account Restriction mosaic transaction version.
    ///
    pub const ACCOUNT_MOSAIC_RESTRICTION: TransactionVersion = TransactionVersion(1);

    /// Account Restriction operation transaction version.
    ///
    pub const MODIFY_ACCOUNT_RESTRICTION_ENTITY_TYPE: TransactionVersion = TransactionVersion(1);

    /// Link account transaction version.
    ///
    pub const ACCOUNT_KEY_LINK: TransactionVersion = TransactionVersion(1);

    /// Account metadata transaction version.
    ///
    pub const ACCOUNT_METADATA: TransactionVersion = TransactionVersion(1);

    /// Mosaic metadata transaction version.
    ///
    pub const MOSAIC_METADATA: TransactionVersion = TransactionVersion(1);

    /// Namespace metadata transaction version.
    ///
    pub const NAMESPACE_METADATA: TransactionVersion = TransactionVersion(1);

    /// Vrf key link transaction version.
    ///
    pub const VRF_KEY_LINK: TransactionVersion = TransactionVersion(1);

    /// Voting key link transaction version.
    ///
    pub const VOTING_KEY_LINK: TransactionVersion = TransactionVersion(1);

    /// Node key link transaction version.
    ///
    pub const NODE_KEY_LINK: TransactionVersion = TransactionVersion(1);

    pub fn to_bytes(&self) -> [u8; 1] {
        self.to_le_bytes()
    }
}

impl Deref for TransactionVersion {
    type Target = u8;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl fmt::Display for TransactionVersion {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.0)
    }
}
