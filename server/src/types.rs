// Copyright 2020 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use serde::{Deserialize, Serialize};
use iota::OutputResponse;
use std::collections::HashMap;
use bee_message::prelude::*;

/// Full reference: https://www.rosetta-api.org/docs/Reference.html#models

/// Objects

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Allow {
    pub operation_statuses: Vec<OperationStatus>,
    pub operation_types: Vec<String>,
    pub errors: Vec<Error>,
    pub historical_balance_lookup: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timestamp_start_index: Option<u64>,
    pub call_methods: Vec<String>,
    pub balance_exemptions: Vec<BalanceExemption>,
    pub mempool_coins: bool,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Amount {
    pub value: String,
    pub currency: Currency,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<AmountMetadata>
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct AmountMetadata;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct BalanceExemption {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sub_account_address: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<Currency>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exemption_type: Option<ExemptionType>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Block {
    pub block_identifier: BlockIdentifier,
    pub parent_block_identifier: BlockIdentifier,
    pub timestamp: u64,
    pub transactions: Vec<Transaction>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<BlockMetadata>
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct BlockMetadata;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Coin {
    pub coin_identifier: CoinIdentifier,
    pub amount: Amount,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum CoinAction {
    #[serde(rename = "coin_created")]
    CoinCreated,
    #[serde(rename = "coin_spent")]
    CoinSpent
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CoinChange {
    pub coin_identifier: CoinIdentifier,
    pub coin_action: CoinAction
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct Currency {
    pub symbol: String,
    pub decimals: u64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<CurrencyMetadata>
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct CurrencyMetadata;

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub enum CurveType {
    #[serde(rename = "secp256k1")]
    Secp256K1,
    #[serde(rename = "secp256r1")]
    Secp256R1,
    #[serde(rename = "edwards25519")]
    Edwards25519,
    #[serde(rename = "tweedle")]
    Tweedle,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum ExemptionType {
    #[serde(rename = "greater_or_equal")]
    GreaterOrEqual,
    #[serde(rename = "less_or_equal")]
    LessOrEqual,
    #[serde(rename = "dynamic")]
    Dynamic,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Operation {
    pub operation_identifier: OperationIdentifier,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub related_operations: Option<Vec<OperationIdentifier>>,
    #[serde(rename = "type")]
    pub type_: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account: Option<AccountIdentifier>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<Amount>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub coin_change: Option<CoinChange>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<OperationMetadata>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct OperationMetadata {
    pub is_spent: String, // TODO: bool
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct PublicKey {
    pub hex_bytes: String,
    pub curve_type: CurveType,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Signature {
    pub signing_payload: SigningPayload,
    pub public_key: PublicKey,
    pub signature_type: SignatureType,
    pub hex_bytes: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub enum SignatureType {
    #[serde(rename = "ecdsa")]
    Ecdsa,
    #[serde(rename = "ecdsa_recovery")]
    EcdsaRecovery,
    #[serde(rename = "ed25519")]
    Edwards25519,
    #[serde(rename = "schnorr_1")]
    Schnorr1,
    #[serde(rename = "schnorr_poseidon")]
    SchnorrPoseidon,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct SigningPayload {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_identifier: Option<AccountIdentifier>,
    pub hex_bytes: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub signature_type: Option<SignatureType>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Transaction {
    pub transaction_identifier: TransactionIdentifier,
    pub operations: Vec<Operation>,
    //pub related_transactions: Option<RelatedTransaction>, TODO
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<TransactionMetadata>
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct TransactionMetadata;

// Identifiers

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct AccountIdentifier {
    pub address: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sub_account: Option<SubAccountIdentifier>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct BlockIdentifier {
    pub index: u32,
    pub hash: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CoinIdentifier {
    pub identifier: String
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct NetworkIdentifier {
    pub blockchain: String,
    pub network: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sub_network_identifier: Option<SubNetworkIdentifier>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct OperationIdentifier {
    pub index: u64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_index: Option<u64>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct PartialBlockIdentifier {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub index: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hash: Option<String>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct SubAccountIdentifier {
    pub address: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct SubNetworkIdentifier {
    pub network: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct TransactionIdentifier {
    pub hash: String,
}

/// Miscellaneous

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Error {
    pub code: u64,
    pub message: String,
    pub retriable: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub details: Option<ErrorDetails>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ErrorDetails {
    /// The detailed error
    pub error: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct OperationStatus {
    pub status: String,
    pub successful: bool,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Peer {
    pub peer_id: String,
    pub metadata: PeerMetadata,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct PeerMetadata {
    pub multi_addresses: Vec<String>,
    pub alias: Option<String>,
    pub connected: bool,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Version {
    pub rosetta_version: String,
    pub node_version: String,
    pub middleware_version: String,
}

/// Self-defined objects

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct UnsignedTransaction {
    essence: Essence,
    inputs_metadata: HashMap<String, OutputResponse>,
}

impl UnsignedTransaction {
    pub fn new(transaction_essence: Essence, inputs_metadata: HashMap<String, OutputResponse>) -> Self {
        Self {
            essence: transaction_essence,
            inputs_metadata
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
pub struct SignedTransaction {
    transaction: TransactionPayload,
    inputs_metadata: HashMap<String, OutputResponse>,
}

impl SignedTransaction {
    pub fn new(transaction: TransactionPayload, inputs_metadata: HashMap<String, OutputResponse>) -> Self {
        Self {
            transaction,
            inputs_metadata
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
pub struct PreprocessOptions {
    pub inputs: Vec<String>
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ConstructionMetadata {
    pub inputs_metadata: HashMap<String, OutputResponse>
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ConstructionSubmitResponseMetadata {
    pub message_id: String,
}






