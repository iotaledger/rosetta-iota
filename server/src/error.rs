// Copyright 2020 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use crate::types::{self, ErrorDetails};

use thiserror::Error;
use warp::{http::StatusCode, reply::Reply};

#[derive(Debug, Error)]
pub enum ApiError {
    #[error("non retriable error")]
    NonRetriable(String),
    #[error("retriable error")]
    Retriable(String),
}

impl ApiError {
    pub fn code(&self) -> u64 {
        match self {
            ApiError::NonRetriable(_) => 1,
            ApiError::Retriable(_) => 2,
        }
    }

    pub fn retriable(&self) -> bool {
        match self {
            ApiError::NonRetriable(_) => false,
            ApiError::Retriable(_) => true,
        }
    }

    pub fn message(&self) -> String {
        format!("{}", self)
    }

    pub(crate) fn details(&self) -> ErrorDetails {
        let error = match self {
            ApiError::NonRetriable(e) => e.clone(),
            ApiError::Retriable(e) => e.clone(),
        };
        ErrorDetails { error }
    }

    pub(crate) fn status_code(&self) -> StatusCode {
        match self {
            ApiError::NonRetriable(_) => StatusCode::BAD_REQUEST,
            ApiError::Retriable(_) => StatusCode::SERVICE_UNAVAILABLE,
        }
    }

    pub(crate) fn all_errors() -> Vec<types::Error> {
        vec![
            types::Error {
                message: "non retriable error".to_string(),
                code: 1,
                retriable: false,
                details: None,
            },
            types::Error {
                message: "retriable error".to_string(),
                code: 2,
                retriable: true,
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

impl Reply for ApiError {
    fn into_response(self) -> warp::reply::Response {
        warp::reply::json(&self.into_error()).into_response()
    }
}
