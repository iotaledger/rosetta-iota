// Copyright 2020 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use serde::{Deserialize, Serialize};

// Objects

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
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct OperationStatus {
    pub status: String,
    pub successful: bool,
}

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
pub enum ExemptionType {
    #[serde(rename = "greater_or_equal")]
    GreaterOrEqual,
    #[serde(rename = "less_or_equal")]
    LessOrEqual,
    #[serde(rename = "dynamic")]
    Dynanic,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Block {
    pub block_identifier: BlockIdentifier,
    pub parent_block_identifier: BlockIdentifier,
    pub timestamp: u64,
    pub transactions: Vec<Transaction>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Coin {
    pub coin_identifier: CoinIdentifier,
    pub amount: Amount,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Amount {
    pub value: String,
    pub currency: Currency,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct Currency {
    pub symbol: String,
    pub decimals: u64,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CoinChange {
    pub coin_identifier: CoinIdentifier,
    pub coin_action: String
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Error {
    pub code: u64,
    pub message: String,
    pub retriable: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub details: Option<ErrorDetails>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Operation {
    pub operation_identifier: OperationIdentifier,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub related_operations: Option<Vec<OperationIdentifier>>,
    #[serde(rename = "type")]
    pub type_: String,
    pub status: Option<String>,
    pub account: AccountIdentifier,
    pub amount: Amount,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub coin_change: Option<CoinChange>,
    pub metadata: OperationMetadata,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct OperationMetadata {
    pub is_spent: String
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct SigningPayload {
    pub account_identifier: AccountIdentifier,
    pub hex_bytes: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub signature_type: Option<SignatureType>,
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

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Transaction {
    pub transaction_identifier: TransactionIdentifier,
    pub operations: Vec<Operation>,
    // pub related_transactions // todo
}

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

// Miscellaneous

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub enum CurveType {
    #[serde(rename = "edwards25519")]
    Edwards25519,
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

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub enum SignatureType {
    #[serde(rename = "ed25519")]
    Edwards25519,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Version {
    pub rosetta_version: String,
    pub node_version: String,
    pub middleware_version: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct MetadataOptions {
    /// The account that will construct the transaction
    pub sender_address: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ErrorDetails {
    /// The detailed error
    pub error: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ConstructionPreprocessResponseOptions {

}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ConstructionMetadataResponseMetadata {

}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ConstructionSubmitResponseMetadata {
    pub message_id: String,
}
