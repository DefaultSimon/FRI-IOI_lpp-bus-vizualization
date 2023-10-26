use reqwest::StatusCode;
use thiserror::Error;

#[derive(Error, Debug, PartialEq, Eq, Clone)]
pub enum FullUrlConstructionError {
    #[error("failed to join sub-URL onto base: {reason}.")]
    FailedToJoinUrl {
        #[from]
        reason: url::ParseError,
    },
}


#[derive(Error, Debug)]
pub enum LppApiFetchError {
    #[error("URL construction error: {0}")]
    UrlError(#[from] FullUrlConstructionError),

    #[error("Failed to perform request: {0}")]
    RequestError(reqwest::Error),

    /// This can happend when e.g. the `success` field is set to `false` in the JSON response.
    #[error("Request was not successful: {reason}")]
    APIResponseNotSuccessful { reason: String },

    #[error("HTTP request failed with client error: {0}")]
    ClientHTTPError(StatusCode),

    #[error("HTTP request failed with server error: {0}")]
    ServerHTTPError(StatusCode),

    #[error("Failed to decode JSON response: {0}")]
    ResponseDecodingError(reqwest::Error),
}