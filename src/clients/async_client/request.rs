use std::collections::HashMap;

use hex::ToHex;
use reqwest::Method;
use serde_json::json;

use crate::clients::search_criteria::BlockSearchCriteria;
use crate::{Order, H256};

/// Type alias to improve readability.
pub(crate) type RoutePathName = &'static str;

#[derive(Debug)]
pub struct Request {
    pub(crate) base_path: &'static str,
    pub(crate) query_params: HashMap<&'static str, String>,
    pub(crate) path_params: HashMap<&'static str, String>,
    pub(crate) method: Method,
}

impl Request {
    fn new(
        base_path: &'static str,
        path_params: HashMap<&'static str, String>,
        query_params: HashMap<&'static str, String>,
        method: Method,
    ) -> Self {
        Request {
            base_path,
            query_params,
            path_params,
            method,
        }
    }

    pub fn to_json(&self, id: usize) -> serde_json::Value {
        json!({"jsonrpc": "2.0", "id": id, "method": self.method.as_str()})
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

        Self::new(
            Self::BLOCKS_HEIGHT_PATH,
            path_params,
            Default::default(),
            Method::GET,
        )
    }

    pub fn get_merkle_receipts(height: u64, hash: H256) -> Self {
        let mut path_params = HashMap::new();
        path_params.insert("height", height.to_string());
        path_params.insert("hash", hash.encode_hex_upper::<String>());

        Self::new(
            Self::BLOCKS_MERKLE_RECEIPTS_PATH,
            path_params,
            Default::default(),
            Method::GET,
        )
    }

    pub fn get_merkle_transaction(height: u64, hash: H256) -> Self {
        let mut path_params = HashMap::new();
        path_params.insert("height", height.to_string());
        path_params.insert("hash", hash.encode_hex_upper::<String>());

        Self::new(
            Self::BLOCKS_MERKLE_TRANSACTION_PATH,
            path_params,
            Default::default(),
            Method::GET,
        )
    }

    pub fn search_blocks(
        criteria: Option<BlockSearchCriteria>,
        page_size: Option<i32>,
        page_number: Option<i32>,
        offset: Option<&str>,
        order: Option<Order>,
    ) -> Self {
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
        }

        if let Some(value) = page_size {
            query_params.insert("pageSize", value.to_string());
        }
        if let Some(value) = page_number {
            query_params.insert("pageNumber", value.to_string());
        }
        if let Some(value) = offset {
            query_params.insert("offset", value.to_string());
        }
        if let Some(value) = order {
            query_params.insert("order", value.to_string());
        }

        Self::new(
            Self::BLOCKS_SEARCH_PATH,
            Default::default(),
            query_params,
            Method::GET,
        )
    }
}

// Blocks requests
impl Request {
    pub const CHAIN_INFO_PATH: RoutePathName = "/chain/info";

    pub fn get_chain_info() -> Self {
        Self::new(
            Self::CHAIN_INFO_PATH,
            Default::default(),
            Default::default(),
            Method::GET,
        )
    }
}
