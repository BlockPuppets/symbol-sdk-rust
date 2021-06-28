/*
 * // Copyright 2021 BlockPuppets developers.
 * //
 * // Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
 * // https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
 * // <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your
 * // option. This file may not be copied, modified, or distributed
 * // except according to those terms.
 */

use std::convert::TryFrom;
use std::str::FromStr;

use anyhow::Result;

use crate::{H256, H512};
use crate::account::{Address, PublicAccount};
use crate::blockchain::{BlockInfo, BlockType, NemesisImportanceBlockInfo};
use crate::network::NetworkType;

use super::{block_dto::BlockDto, block_meta_dto::BlockMetaDto};

#[derive(Serialize, Deserialize)]
pub struct BlockInfoDto {
    /// Internal resource identifier.
    id: String,
    meta: BlockMetaDto,
    block: BlockDto,
}

impl BlockInfoDto {
    pub fn to_compact(&self) -> Result<BlockInfo> {
        let network_type = NetworkType::try_from(self.block.network)?;
        let signer =
            PublicAccount::from_public_key(&self.block.signer_public_key, network_type)?;

        let beneficiary_address = if !self.block.beneficiary_address.is_empty() {
            Some(Address::from_encoded(
                &self.block.beneficiary_address,
            )?)
        } else {
            None
        };

        let state_hash_sub_cache_merkle_roots = self
            .meta
            .state_hash_sub_cache_merkle_roots
            .iter()
            .map(|h| H256::from_str(h).unwrap())
            .collect();

        let nemesis_importance_info = if let Some(_) = &self.block.total_voting_balance {
            Some(NemesisImportanceBlockInfo {
                voting_eligible_accounts_count: self.block.voting_eligible_accounts_count.unwrap(),
                harvesting_eligible_accounts_count: u64::from_str(
                    self.block
                        .harvesting_eligible_accounts_count
                        .as_ref()
                        .unwrap(),
                )?,
                total_voting_balance: u64::from_str(
                    self.block.total_voting_balance.as_ref().unwrap(),
                )?,
                previous_importance_block_hash: H256::from_str(
                    self.block.previous_importance_block_hash.as_ref().unwrap(),
                )?,
            })
        } else {
            None
        };

        Ok(BlockInfo {
            hash: H256::from_str(&self.meta.hash)?,
            generation_hash: H256::from_str(&self.meta.generation_hash)?,
            fee_multiplier: self.block.fee_multiplier,
            signature: H512::from_str(&self.block.signature)?,
            signer,
            network_type,
            version: self.block.version,
            r#type: BlockType::try_from(self.block.type_field)?,
            height: u64::from_str(&self.block.height)?,
            timestamp: u64::from_str(&self.block.timestamp)?,
            difficulty: u64::from_str(&self.block.difficulty)?,
            previous_block_hash: H256::from_str(&self.block.previous_block_hash)?,
            block_transactions_hash: H256::from_str(&self.block.transactions_hash)?,
            block_receipts_hash: H256::from_str(&self.block.receipts_hash)?,
            state_hash: self.block.state_hash.to_string(),
            proof_gamma: H256::from_str(&self.block.proof_gamma)?,
            proof_scalar: H256::from_str(&self.block.proof_scalar)?,
            proof_verification_hash: self.block.proof_verification_hash.to_string(),
            beneficiary_address,
            record_id: self.id.to_string(),
            size: self.block.size,
            total_fee: u64::from_str(&self.meta.total_fee)?,
            state_hash_sub_cache_merkle_roots,
            total_transactions_count: self.meta.total_transactions_count,
            transactions_count: self.meta.transactions_count,
            statements_count: self.meta.statements_count,
            nemesis_importance_info,
        })
    }
}