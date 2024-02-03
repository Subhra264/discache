pub mod cluster;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
struct GetResponse {
    value: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum HttpError {
    KeyNotFound,
    UnknownError,
    BadRequest,
}

#[derive(Serialize, Deserialize, Debug)]
struct GetErrorResponse {
    error: HttpError,
}

#[derive(Serialize, Deserialize, Debug)]
struct PutResponse {
    success: bool,
}

#[derive(Serialize, Deserialize, Debug)]
struct EntryRequestBody {
    key: String,
    value: String,
}
