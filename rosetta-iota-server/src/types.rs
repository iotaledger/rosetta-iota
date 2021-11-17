// Copyright 2020 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use bee_message::prelude::*;
use bee_rest_api::types::responses::OutputResponse;

use serde::{Deserialize, Serialize};

use std::collections::HashMap;

/// Full reference: https://www.rosetta-api.org/docs/Reference.html#models

/// Objects

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Allow {
    pub operation_statuses: Vec<OperationStatus>,
    pub operation_types: Vec<String>,
    pub errors: Vec<Error>,
    pub historical_balance_lookup: bool,
    pub call_methods: Vec<String>,
    pub balance_exemptions: Vec<BalanceExemption>,
    pub mempool_coins: bool,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Amount {
    pub value: String,
    pub currency: Currency,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct BalanceExemption {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sub_account_address: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<Currency>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exemption_type: Option<ExemptionType>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Block {
    pub block_identifier: BlockIdentifier,
    pub parent_block_identifier: BlockIdentifier,
    pub timestamp: u64,
    pub transactions: Vec<BlockTransaction>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Coin {
    pub coin_identifier: CoinIdentifier,
    pub amount: Amount,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub enum CoinAction {
    #[serde(rename = "coin_created")]
    CoinCreated,
    #[serde(rename = "coin_spent")]
    CoinSpent,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct CoinChange {
    pub coin_identifier: CoinIdentifier,
    pub coin_action: CoinAction,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Currency {
    pub symbol: String,
    pub decimals: u64,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub enum CurveType {
    #[serde(rename = "edwards25519")]
    Edwards25519,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub enum ExemptionType {
    #[serde(rename = "greater_or_equal")]
    GreaterOrEqual,
    #[serde(rename = "less_or_equal")]
    LessOrEqual,
    #[serde(rename = "dynamic")]
    Dynamic,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Operation {
    pub operation_identifier: OperationIdentifier,
    #[serde(rename = "type")]
    pub type_: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    pub account: Option<AccountIdentifier>,
    pub amount: Option<Amount>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub coin_change: Option<CoinChange>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct PublicKey {
    pub hex_bytes: String,
    pub curve_type: CurveType,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Signature {
    pub signing_payload: SigningPayload,
    pub public_key: PublicKey,
    pub signature_type: SignatureType,
    pub hex_bytes: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub enum SignatureType {
    #[serde(rename = "ed25519")]
    Edwards25519,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct SigningPayload {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<String>, // DEPRECIATED BUT IS NEEDED https://github.com/coinbase/rosetta-cli/issues/256
    pub account_identifier: AccountIdentifier,
    pub hex_bytes: String,
    pub signature_type: SignatureType,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct BlockTransaction {
    pub transaction_identifier: TransactionIdentifier,
    pub operations: Vec<Operation>,
}

// Identifiers

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct AccountIdentifier {
    pub address: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct BlockIdentifier {
    pub index: u32,
    pub hash: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct CoinIdentifier {
    pub identifier: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct NetworkIdentifier {
    pub blockchain: String,
    pub network: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct OperationIdentifier {
    pub index: u64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_index: Option<u64>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct PartialBlockIdentifier {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub index: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hash: Option<String>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct TransactionIdentifier {
    pub hash: String,
}

/// Miscellaneous

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Error {
    pub code: u64,
    pub message: String,
    pub retriable: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub details: Option<ErrorDetails>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ErrorDetails {
    pub error: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct OperationStatus {
    pub status: String,
    pub successful: bool,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Peer {
    pub peer_id: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Version {
    pub rosetta_version: String,
    pub node_version: String,
}

/// Self-defined objects

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct UnsignedTransaction {
    essence: Essence,
    inputs_metadata: HashMap<String, OutputResponse>,
}

impl UnsignedTransaction {
    pub fn new(transaction_essence: Essence, inputs_metadata: HashMap<String, OutputResponse>) -> Self {
        Self {
            essence: transaction_essence,
            inputs_metadata,
        }
    }
    pub fn essence(&self) -> &Essence {
        &self.essence
    }
    pub fn inputs_metadata(&self) -> &HashMap<String, OutputResponse> {
        &self.inputs_metadata
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct SignedTransaction {
    transaction: TransactionPayload,
    inputs_metadata: HashMap<String, OutputResponse>,
}

impl SignedTransaction {
    pub fn new(transaction: TransactionPayload, inputs_metadata: HashMap<String, OutputResponse>) -> Self {
        Self {
            transaction,
            inputs_metadata,
        }
    }
    pub fn transaction(&self) -> &bee_message::prelude::TransactionPayload {
        &self.transaction
    }
    pub fn inputs_metadata(&self) -> &HashMap<String, OutputResponse> {
        &self.inputs_metadata
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct PreprocessOptions {
    pub utxo_inputs: Vec<String>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ConstructionMetadata {
    pub utxo_inputs_metadata: HashMap<String, OutputResponse>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ConstructionSubmitResponseMetadata {
    pub message_id: String,
}
