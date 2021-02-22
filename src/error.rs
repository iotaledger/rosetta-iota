// Copyright 2020 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use crate::types::{self, ErrorDetails};
use thiserror::Error;
use warp::{http::StatusCode, reply::Reply};

#[derive(Debug, Error)]
pub enum ApiError {
    #[error("bad network")]
    BadNetwork,
    #[error("Unimplemented")]
    NotImplemented,
    #[error("unable to build iota client")]
    UnableToBuildClient,
    #[error("unable to get node info")]
    UnableToGetNodeInfo,
    #[error("unable to get milestone {0}")]
    UnableToGetMilestone(u32),
    #[error("unable to get peers")]
    UnableToGetPeers,
    #[error("bad block/milestone request")]
    BadMilestoneRequest,
    #[error("unable to get milestone utxo changes")]
    UnableToGetMilestoneUTXOChanges,
    #[error("unable to get transaction outputs")]
    UnableToGetOutput,
    #[error("unable to get genesis milestone. try pointing to a permanode instead")]
    UnableToGetGenesisMilestone,
    #[error("Historical balances are not supported.")]
    HistoricalBalancesUnsupported,
    #[error("bad construction request: {0}")]
    BadConstructionRequest(String),
    #[error("unable to get balance")]
    UnableToGetBalance,
    #[error("unable to get outputs from address")]
    UnableToGetOutputsFromAddress,
    #[error("unsupported curve")]
    UnsupportedCurve,
    #[error("hex decoding failed: {0:?}")]
    HexDecodingFailed(#[from] hex::FromHexError),
    #[error("{0:?}")]
    BeeMessageError(#[from] bee_message::Error),
    #[error("{0:?}")]
    IotaClientError(#[from] iota::client::Error),
    #[error("unsupported offline")]
    UnavailableOffline,
    #[error("output has already been spent")]
    UnableToSpend,
    #[error("unknown operation type")]
    UnknownOperationType,
}

impl ApiError {
    pub fn code(&self) -> u64 {
        match self {
            ApiError::BadNetwork => 10,
            ApiError::NotImplemented => 20,
            ApiError::UnableToBuildClient => 30,
            ApiError::UnableToGetNodeInfo => 40,
            ApiError::UnableToGetMilestone(_) => 50,
            ApiError::UnableToGetPeers => 60,
            ApiError::BadMilestoneRequest => 70,
            ApiError::UnableToGetMilestoneUTXOChanges => 80,
            ApiError::UnableToGetOutput => 90,
            ApiError::UnableToGetGenesisMilestone => 100,
            ApiError::HistoricalBalancesUnsupported => 110,
            ApiError::BadConstructionRequest(_) => 120,
            ApiError::UnableToGetBalance => 130,
            ApiError::UnableToGetOutputsFromAddress => 140,
            ApiError::UnsupportedCurve => 150,
            ApiError::HexDecodingFailed(_) => 160,
            ApiError::BeeMessageError(_) => 170,
            ApiError::IotaClientError(_) => 180,
            ApiError::UnavailableOffline => 190,
            ApiError::UnableToSpend => 200,
            ApiError::UnknownOperationType => 210,
        }
    }

    pub fn retriable(&self) -> bool {
        match self {
            ApiError::BadNetwork => false,
            ApiError::NotImplemented => false,
            ApiError::UnableToBuildClient => false,
            ApiError::UnableToGetNodeInfo => true,
            ApiError::UnableToGetMilestone(_) => true,
            ApiError::UnableToGetPeers => false,
            ApiError::BadMilestoneRequest => false,
            ApiError::UnableToGetMilestoneUTXOChanges => true,
            ApiError::UnableToGetOutput => false,
            ApiError::UnableToGetGenesisMilestone => false,
            ApiError::HistoricalBalancesUnsupported => false,
            ApiError::BadConstructionRequest(_) => false,
            ApiError::UnableToGetBalance => false,
            ApiError::UnableToGetOutputsFromAddress => false,
            ApiError::UnsupportedCurve => false,
            ApiError::HexDecodingFailed(_) => false,
            ApiError::BeeMessageError(_) => false,
            ApiError::IotaClientError(_) => false,
            ApiError::UnavailableOffline => false,
            ApiError::UnableToSpend => false,
            ApiError::UnknownOperationType => false,
        }
    }

    pub fn status_code(&self) -> StatusCode {
        match self {
            ApiError::BadNetwork => StatusCode::BAD_REQUEST,
            ApiError::NotImplemented => StatusCode::BAD_REQUEST,
            ApiError::UnableToBuildClient => StatusCode::INTERNAL_SERVER_ERROR,
            ApiError::UnableToGetNodeInfo => StatusCode::INTERNAL_SERVER_ERROR,
            ApiError::UnableToGetMilestone(_) => StatusCode::INTERNAL_SERVER_ERROR,
            ApiError::UnableToGetPeers => StatusCode::INTERNAL_SERVER_ERROR,
            ApiError::BadMilestoneRequest => StatusCode::BAD_REQUEST,
            ApiError::UnableToGetMilestoneUTXOChanges => StatusCode::INTERNAL_SERVER_ERROR,
            ApiError::UnableToGetOutput => StatusCode::INTERNAL_SERVER_ERROR,
            ApiError::UnableToGetGenesisMilestone => StatusCode::INTERNAL_SERVER_ERROR,
            ApiError::HistoricalBalancesUnsupported => StatusCode::BAD_REQUEST,
            ApiError::BadConstructionRequest(_) => StatusCode::BAD_REQUEST,
            ApiError::UnableToGetBalance => StatusCode::INTERNAL_SERVER_ERROR,
            ApiError::UnableToGetOutputsFromAddress => StatusCode::INTERNAL_SERVER_ERROR,
            ApiError::UnsupportedCurve => StatusCode::BAD_REQUEST,
            ApiError::HexDecodingFailed(_) => StatusCode::BAD_REQUEST,
            ApiError::BeeMessageError(_) => StatusCode::BAD_REQUEST,
            ApiError::IotaClientError(_) => StatusCode::BAD_REQUEST,
            ApiError::UnavailableOffline => StatusCode::INTERNAL_SERVER_ERROR,
            ApiError::UnableToSpend => StatusCode::BAD_REQUEST,
            ApiError::UnknownOperationType => StatusCode::BAD_REQUEST,
        }
    }

    pub fn message(&self) -> String {
        let full = format!("{}", self);
        let parts: Vec<_> = full.split(":").collect();
        parts[0].to_string()
    }

    pub(crate) fn details(&self) -> ErrorDetails {
        let error = format!("{}", self);
        ErrorDetails { error }
    }

    pub(crate) fn all_errors() -> Vec<types::Error> {
        vec![
            types::Error {
                message: "Bad Network".to_string(),
                code: 10,
                retriable: false,
                details: None,
            },
            types::Error {
                message: "Endpoint not implemented".to_string(),
                code: 20,
                retriable: false,
                details: None,
            },
            types::Error {
                message: "Unable to build IOTA Client".to_string(),
                code: 30,
                retriable: false,
                details: None,
            },
            types::Error {
                message: "Unable to get Node Info".to_string(),
                code: 40,
                retriable: false,
                details: None,
            },
            types::Error {
                message: "Unable to get Milestone".to_string(),
                code: 50,
                retriable: false,
                details: None,
            },
            types::Error {
                message: "Unable to get Peers".to_string(),
                code: 60,
                retriable: false,
                details: None,
            },
            types::Error {
                message: "Bad Milestone Request".to_string(),
                code: 70,
                retriable: false,
                details: None,
            },
            types::Error {
                message: "Unable to get Milestone UTXO Changes".to_string(),
                code: 80,
                retriable: false,
                details: None,
            },
            types::Error {
                message: "Unable to get Transaction Outputs".to_string(),
                code: 90,
                retriable: false,
                details: None,
            },
            types::Error {
                message: "Unable to get Genesis Milestone, try pointing to a Permanode instead.".to_string(),
                code: 100,
                retriable: false,
                details: None,
            },
            types::Error {
                message: "Historical balances not supported.".to_string(),
                code: 110,
                retriable: false,
                details: None,
            },
            types::Error {
                message: "Bad Construction Request".to_string(),
                code: 120,
                retriable: false,
                details: None,
            },
            types::Error {
                message: "Unable to get Balance".to_string(),
                code: 130,
                retriable: false,
                details: None,
            },
            types::Error {
                message: "Unable to get Outputs from Address".to_string(),
                code: 140,
                retriable: false,
                details: None,
            },
            types::Error {
                message: "Unsupported Curve".to_string(),
                code: 150,
                retriable: false,
                details: None,
            },
            types::Error {
                message: "HEX decoding failed".to_string(),
                code: 160,
                retriable: false,
                details: None,
            },
            types::Error {
                message: "Bee Message Error".to_string(),
                code: 170,
                retriable: false,
                details: None,
            },
            types::Error {
                message: "IOTA Client Error".to_string(),
                code: 180,
                retriable: false,
                details: None,
            },
            types::Error {
                message: "Unavailable Offline".to_string(),
                code: 190,
                retriable: false,
                details: None,
            },
            types::Error {
                message: "Already Spent".to_string(),
                code: 200,
                retriable: false,
                details: None,
            },
            types::Error {
                message: "Unknown Operation Type".to_string(),
                code: 210,
                retriable: false,
                details: None,
            },
        ]
    }

    pub fn into_error(self) -> types::Error {
        types::Error {
            message: self.message(),
            code: self.code(),
            retriable: self.retriable(),
            details: Some(self.details()),
        }
    }
}

impl warp::reject::Reject for ApiError {}

// commented out after bumping to warp 0.3
// remember to use impl<T: Reject> From<T> for Rejection
// impl std::convert::From<ApiError> for warp::reject::Rejection {
//     fn from(api_error: ApiError) -> Self {
//         warp::reject::custom(api_error)
//     }
// }

impl Reply for ApiError {
    fn into_response(self) -> warp::reply::Response {
        warp::reply::json(&self.into_error()).into_response()
    }
}
