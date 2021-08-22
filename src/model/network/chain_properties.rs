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

/// ChainPropertiesDto : Chain related configuration properties.
///
#[serde(rename_all = "camelCase")]
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ChainProperties {
    /// Set to true if block chain should calculate state hashes so that state is fully verifiable at each block.
    #[serde(
    rename = "enableVerifiableState",
    skip_serializing_if = "Option::is_none"
    )]
    pub enable_verifiable_state: Option<bool>,
    /// Set to true if block chain should calculate receipts so that state changes are fully verifiable at each block.
    #[serde(
    rename = "enableVerifiableReceipts",
    skip_serializing_if = "Option::is_none"
    )]
    pub enable_verifiable_receipts: Option<bool>,
    /// Mosaic id used as primary chain currency.
    #[serde(rename = "currencyMosaicId", skip_serializing_if = "Option::is_none")]
    pub currency_mosaic_id: Option<String>,
    /// Mosaic id used to provide harvesting ability.
    #[serde(rename = "harvestingMosaicId", skip_serializing_if = "Option::is_none")]
    pub harvesting_mosaic_id: Option<String>,
    /// Targeted time between blocks.
    #[serde(
    rename = "blockGenerationTargetTime",
    skip_serializing_if = "Option::is_none"
    )]
    pub block_generation_target_time: Option<String>,
    /// A higher value makes the network more biased.
    #[serde(
    rename = "blockTimeSmoothingFactor",
    skip_serializing_if = "Option::is_none"
    )]
    pub block_time_smoothing_factor: Option<String>,
    /// Number of blocks between successive finalization attempts.
    #[serde(
    rename = "blockFinalizationInterval",
    skip_serializing_if = "Option::is_none"
    )]
    pub block_finalization_interval: Option<String>,
    /// Number of blocks that should be treated as a group for importance purposes.
    #[serde(rename = "importanceGrouping", skip_serializing_if = "Option::is_none")]
    pub importance_grouping: Option<String>,
    /// Percentage of importance resulting from fee generation and beneficiary usage.
    #[serde(
    rename = "importanceActivityPercentage",
    skip_serializing_if = "Option::is_none"
    )]
    pub importance_activity_percentage: Option<String>,
    /// Maximum number of blocks that can be rolled back.
    #[serde(rename = "maxRollbackBlocks", skip_serializing_if = "Option::is_none")]
    pub max_rollback_blocks: Option<String>,
    /// Maximum number of blocks to use in a difficulty calculation.
    #[serde(
    rename = "maxDifficultyBlocks",
    skip_serializing_if = "Option::is_none"
    )]
    pub max_difficulty_blocks: Option<String>,
    /// Default multiplier to use for dynamic fees.
    #[serde(
    rename = "defaultDynamicFeeMultiplier",
    skip_serializing_if = "Option::is_none"
    )]
    pub default_dynamic_fee_multiplier: Option<String>,
    /// Maximum lifetime a transaction can have before it expires.
    #[serde(
    rename = "maxTransactionLifetime",
    skip_serializing_if = "Option::is_none"
    )]
    pub max_transaction_lifetime: Option<String>,
    /// Maximum future time of a block that can be accepted.
    #[serde(rename = "maxBlockFutureTime", skip_serializing_if = "Option::is_none")]
    pub max_block_future_time: Option<String>,
    /// Initial currency atomic units available in the network.
    #[serde(
    rename = "initialCurrencyAtomicUnits",
    skip_serializing_if = "Option::is_none"
    )]
    pub initial_currency_atomic_units: Option<String>,
    /// Maximum atomic units (total-supply * 10 ^ divisibility) of a mosaic allowed in the network.
    #[serde(
    rename = "maxMosaicAtomicUnits",
    skip_serializing_if = "Option::is_none"
    )]
    pub max_mosaic_atomic_units: Option<String>,
    /// Total whole importance units available in the network.
    #[serde(
    rename = "totalChainImportance",
    skip_serializing_if = "Option::is_none"
    )]
    pub total_chain_importance: Option<String>,
    /// Minimum number of harvesting mosaic atomic units needed for an account to be eligible for harvesting.
    #[serde(
    rename = "minHarvesterBalance",
    skip_serializing_if = "Option::is_none"
    )]
    pub min_harvester_balance: Option<String>,
    /// Maximum number of harvesting mosaic atomic units needed for an account to be eligible for harvesting.
    #[serde(
    rename = "maxHarvesterBalance",
    skip_serializing_if = "Option::is_none"
    )]
    pub max_harvester_balance: Option<String>,
    /// Minimum number of harvesting mosaic atomic units needed for an account to be eligible for voting.
    #[serde(rename = "minVoterBalance", skip_serializing_if = "Option::is_none")]
    pub min_voter_balance: Option<String>,
    /// Maximum number of voting keys that can be registered at once per account.
    #[serde(
    rename = "maxVotingKeysPerAccount",
    skip_serializing_if = "Option::is_none"
    )]
    pub max_voting_keys_per_account: Option<String>,
    /// Minimum number of finalization rounds for which voting key can be registered.
    #[serde(
    rename = "minVotingKeyLifetime",
    skip_serializing_if = "Option::is_none"
    )]
    pub min_voting_key_lifetime: Option<String>,
    /// Maximum number of finalization rounds for which voting key can be registered.
    #[serde(
    rename = "maxVotingKeyLifetime",
    skip_serializing_if = "Option::is_none"
    )]
    pub max_voting_key_lifetime: Option<String>,
    /// Percentage of the harvested fee that is collected by the beneficiary account.
    #[serde(
    rename = "harvestBeneficiaryPercentage",
    skip_serializing_if = "Option::is_none"
    )]
    pub harvest_beneficiary_percentage: Option<String>,
    /// Percentage of the harvested fee that is collected by the network.
    #[serde(
    rename = "harvestNetworkPercentage",
    skip_serializing_if = "Option::is_none"
    )]
    pub harvest_network_percentage: Option<String>,
    /// Address encoded using a 32-character set.
    #[serde(
    rename = "harvestNetworkFeeSinkAddress",
    skip_serializing_if = "Option::is_none"
    )]
    pub harvest_network_fee_sink_address: Option<String>,
    /// Number of blocks between cache pruning.
    #[serde(rename = "blockPruneInterval", skip_serializing_if = "Option::is_none")]
    pub block_prune_interval: Option<String>,
    /// Maximum number of transactions per block.
    #[serde(
    rename = "maxTransactionsPerBlock",
    skip_serializing_if = "Option::is_none"
    )]
    pub max_transactions_per_block: Option<String>,
}

impl fmt::Display for ChainProperties {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(&self).unwrap_or_default()
        )
    }
}
