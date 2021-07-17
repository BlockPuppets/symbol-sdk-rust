/*
 * // Copyright 2021 BlockPuppets.
 * //
 * // Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
 * // https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
 * // <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your
 * // option. This file may not be copied, modified, or distributed
 * // except according to those terms.
 */

use std::collections::HashMap;

use hex::ToHex;
use reqwest::Method;

use crate::clients::search_criteria::BlockSearchCriteria;
use crate::mosaic::MosaicId;
use crate::{MosaicIds, MosaicSearchCriteria, H256};

/// Type alias to improve readability.
pub(crate) type RoutePathName = &'static str;

#[derive(Debug)]
pub struct Request {
    pub(crate) base_path: &'static str,
    pub(crate) query_params: HashMap<&'static str, String>,
    pub(crate) path_params: HashMap<&'static str, String>,
    pub(crate) serialized_body: Option<String>,
    pub(crate) method: Method,
}

impl Request {
    fn new_path(base_path: &'static str) -> Self {
        Request {
            base_path,
            query_params: Default::default(),
            path_params: Default::default(),
            serialized_body: None,
            method: Default::default(),
        }
    }

    fn from_path_params(
        base_path: &'static str,
        path_params: HashMap<&'static str, String>,
        method: Method,
    ) -> Self {
        Request {
            base_path,
            query_params: Default::default(),
            path_params,
            serialized_body: None,
            method,
        }
    }

    fn from_query_params(
        base_path: &'static str,
        query_params: HashMap<&'static str, String>,
        method: Method,
    ) -> Self {
        Request {
            base_path,
            query_params,
            path_params: Default::default(),
            serialized_body: None,
            method,
        }
    }

    fn from_serialized_body<T: serde::Serialize>(base_path: &'static str, body: T) -> Self {
        let serialized_body = Some(serde_json::to_string(&body).unwrap());

        Request {
            base_path,
            query_params: Default::default(),
            path_params: Default::default(),
            serialized_body,
            method: Method::POST,
        }
    }
}

// Blocks requests
impl Request {
    pub const BLOCKS_SEARCH_PATH: RoutePathName = "/blocks";
    pub const BLOCKS_HEIGHT_PATH: RoutePathName = "/blocks/{height}";
    pub const BLOCKS_MERKLE_RECEIPTS_PATH: RoutePathName =
        "/blocks/{height}/statements/{hash}/merkle";
    pub const BLOCKS_MERKLE_TRANSACTION_PATH: RoutePathName =
        "/blocks/{height}/transactions/{hash}/merkle";

    pub fn get_block_by_height(height: u64) -> Self {
        let mut path_params = HashMap::new();
        path_params.insert("height", height.to_string());

        Self::from_path_params(Self::BLOCKS_HEIGHT_PATH, path_params, Method::GET)
    }

    pub fn get_merkle_receipts(height: u64, hash: H256) -> Self {
        let mut path_params = HashMap::new();
        path_params.insert("height", height.to_string());
        path_params.insert("hash", hash.encode_hex_upper::<String>());

        Self::from_path_params(Self::BLOCKS_MERKLE_RECEIPTS_PATH, path_params, Method::GET)
    }

    pub fn get_merkle_transaction(height: u64, hash: H256) -> Self {
        let mut path_params = HashMap::new();
        path_params.insert("height", height.to_string());
        path_params.insert("hash", hash.encode_hex_upper::<String>());

        Self::from_path_params(
            Self::BLOCKS_MERKLE_TRANSACTION_PATH,
            path_params,
            Method::GET,
        )
    }

    pub fn search_blocks(criteria: Option<BlockSearchCriteria>) -> Self {
        let mut query_params = HashMap::new();

        if let Some(c) = criteria {
            if let Some(value) = c.signer_public_key {
                query_params.insert("signerPublicKey", value.public_key_to_hex());
            }
            if let Some(value) = c.beneficiary_address {
                query_params.insert("beneficiaryAddress", value.address_str());
            }
            if let Some(value) = c.order_by {
                query_params.insert("orderBy", value.to_string());
            }

            if let Some(param) = c.param {
                if let Some(value) = param.page_size {
                    query_params.insert("pageSize", value.to_string());
                }
                if let Some(value) = param.page_number {
                    query_params.insert("pageNumber", value.to_string());
                }
                if let Some(value) = param.offset {
                    query_params.insert("offset", value.to_string());
                }
                if let Some(value) = param.order {
                    query_params.insert("order", value.to_string());
                }
            }
        }

        Self::from_query_params(Self::BLOCKS_SEARCH_PATH, query_params, Method::GET)
    }
}

// Chain requests
impl Request {
    pub const CHAIN_INFO_PATH: RoutePathName = "/chain/info";

    pub fn get_chain_info() -> Self {
        Self::new_path(Self::CHAIN_INFO_PATH)
    }
}

// Network requests
impl Request {
    pub const NETWORK_NAME_PATH: RoutePathName = "/network";
    pub const NETWORK_PROPERTIES_PATH: RoutePathName = "/network/properties";
    pub const NETWORK_RENTAL_FEES_PATH: RoutePathName = "/network/fees/rental";
    pub const NETWORK_TRANSACTION_FEES_PATH: RoutePathName = "/network/fees/transaction";

    pub fn get_network_name() -> Self {
        Self::new_path(Self::NETWORK_NAME_PATH)
    }

    pub fn get_network_properties() -> Self {
        Self::new_path(Self::NETWORK_PROPERTIES_PATH)
    }

    pub fn get_rental_fees() -> Self {
        Self::new_path(Self::NETWORK_RENTAL_FEES_PATH)
    }

    pub fn get_transaction_fees() -> Self {
        Self::new_path(Self::NETWORK_TRANSACTION_FEES_PATH)
    }
}

// Mosaic requests
impl Request {
    pub const MOSAIC_INFO_PATH: RoutePathName = "/mosaics/{mosaicId}";
    pub const MOSAIC_INFO_MERKLE_PATH: RoutePathName = "/mosaics/{mosaicId}/merkle";
    pub const MOSAICS_INFO_PATH: RoutePathName = "/mosaics";

    pub fn get_mosaic(mosaic_id: MosaicId) -> Self {
        let mut path_params = HashMap::new();
        path_params.insert("mosaicId", mosaic_id.to_hex());
        Self::from_path_params(Self::MOSAIC_INFO_PATH, path_params, Method::GET)
    }

    pub fn get_mosaics(mosaic_ids: MosaicIds) -> Self {
        Self::from_serialized_body(Self::MOSAICS_INFO_PATH, mosaic_ids)
    }

    pub fn search_mosaics(criteria: Option<MosaicSearchCriteria>) -> Self {
        let mut query_params = HashMap::new();

        if let Some(c) = criteria {
            if let Some(value) = c.owner_address {
                query_params.insert("ownerAddress", value.address_str());
            }

            if let Some(param) = c.param {
                if let Some(value) = param.page_size {
                    query_params.insert("pageSize", value.to_string());
                }
                if let Some(value) = param.page_number {
                    query_params.insert("pageNumber", value.to_string());
                }
                if let Some(value) = param.offset {
                    query_params.insert("offset", value.to_string());
                }
                if let Some(value) = param.order {
                    query_params.insert("order", value.to_string());
                }
            }
        }

        Self::from_query_params(Self::MOSAICS_INFO_PATH, query_params, Method::GET)
    }

    pub fn get_mosaic_merkle(mosaic_id: MosaicId) -> Self {
        let mut path_params = HashMap::new();
        path_params.insert("mosaicId", mosaic_id.to_hex());
        Self::from_path_params(Self::MOSAIC_INFO_MERKLE_PATH, path_params, Method::GET)
    }
}

// Node requests
impl Request {
    pub const NODE_HEALTH_PATH: RoutePathName = "/node/health";
    pub const NODE_INFO_PATH: RoutePathName = "/node/info";
    pub const NODE_PEERS_PATH: RoutePathName = "/node/peers";
    pub const NODE_STORAGE_INFO_PATH: RoutePathName = "/node/storage";
    pub const NODE_NODE_TIME_PATH: RoutePathName = "/node/time";
    pub const NODE_SERVER_INFO_PATH: RoutePathName = "/node/server";
    pub const NODE_UNLOCKED_ACCOUNTS_PATH: RoutePathName = "/node/unlockedaccount";

    pub fn get_node_health() -> Self {
        Self::new_path(Self::NODE_HEALTH_PATH)
    }

    pub fn get_node_info() -> Self {
        Self::new_path(Self::NODE_INFO_PATH)
    }

    pub fn get_node_peers() -> Self {
        Self::new_path(Self::NODE_PEERS_PATH)
    }

    pub fn get_storage_info() -> Self {
        Self::new_path(Self::NODE_STORAGE_INFO_PATH)
    }

    pub fn get_node_time() -> Self {
        Self::new_path(Self::NODE_NODE_TIME_PATH)
    }

    pub fn get_server_info() -> Self {
        Self::new_path(Self::NODE_SERVER_INFO_PATH)
    }

    pub fn get_unlocked_accounts() -> Self {
        Self::new_path(Self::NODE_UNLOCKED_ACCOUNTS_PATH)
    }
}
