use crate::{
    types::{self, ErrorDetails},
};
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
    #[error("unable to get milestone")]
    UnableToGetMilestone,
}

impl ApiError {
    pub fn code(&self) -> u64 {
        match self {
            ApiError::BadNetwork => 10,
            ApiError::NotImplemented => 20,
            ApiError::UnableToBuildClient => 30,
            ApiError::UnableToGetNodeInfo => 40,
            ApiError::UnableToGetMilestone => 50,
        }
    }

    pub fn retriable(&self) -> bool {
        match self {
            ApiError::BadNetwork => false,
            ApiError::NotImplemented => false,
            ApiError::UnableToBuildClient => false,
            ApiError::UnableToGetNodeInfo => false,
            ApiError::UnableToGetMilestone => false,
        }
    }

    pub fn status_code(&self) -> StatusCode {
        match self {
            ApiError::BadNetwork => StatusCode::BAD_REQUEST,
            ApiError::NotImplemented => StatusCode::BAD_REQUEST,
            ApiError::UnableToBuildClient => StatusCode::INTERNAL_SERVER_ERROR,
            ApiError::UnableToGetNodeInfo => StatusCode::INTERNAL_SERVER_ERROR,
            ApiError::UnableToGetMilestone => StatusCode::INTERNAL_SERVER_ERROR,
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

/* commented out after bumping to warp 0.3
remember to use impl<T: Reject> From<T> for Rejection */
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
