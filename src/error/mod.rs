mod http;
mod conversions;
mod rollback;

use std::fmt::{Display, Formatter};
use crate::utils::AnyString;
pub use rollback::*;

#[derive(Serialize, Debug, Clone)]
#[serde(tag = "code", content = "data")]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub enum RHSError {
    // --- MISC
    #[serde(rename="error.miscUnknown")]
    Unknown,
    #[serde(rename="error.miscUnimplemented")]
    Unimplemented,
    /// Error that shouldn't happen - happened
    #[serde(rename="error.miscUnexpected")]
    Unexpected,

    // --- HTTP
    #[serde(rename="error.httpInvalidEndpoint")]
    InvalidEndpoint {
        endpoint: AnyString,
    },
    #[serde(rename="error.httpMissingHeader")]
    MissingHeader {
        header: AnyString,
    },
    #[serde(rename="error.httpInvalidHeader")]
    InvalidHeader {
        header: AnyString,
    },
    /// Returned when server cannot read one of the frames of the body or if the body is too large.
    #[serde(rename="error.httpRejectedBody")]
    RejectedBody,
    /// Returned when data that server expected to receive in the body was not valid.
    #[serde(rename="error.httpInvalidBody")]
    InvalidBody,
    #[serde(rename="error.httpMethodNotAccepted")]
    MethodNotAccepted,
    #[serde(rename="error.httpActionNotAccepted")]
    ActionNotAccepted,

    // --- WEB SOCKET
    #[serde(rename="error.wsMissingSecWebSocketKey")]
    MissingSecWebSocketKey,
    #[serde(rename="error.wsMissingSecWebSocketVersionHeader")]
    MissingSecWebSocketVersionHeader,
    #[serde(rename="error.wsDisallowedWebSocketVersion")]
    DisallowedWebSocketVersion {
        supported_version: AnyString,
    },
    #[serde(rename="error.wsHandshakeIncomplete")]
    HandshakeIncomplete,

    // --- CLIENT
    #[serde(rename = "error.clientArgumentInvalid")]
    ArgumentInvalid {
        arg: AnyString,
    },
    #[serde(rename = "error.clientArgumentTooShort")]
    ArgumentTooShort {
        arg: AnyString,
        val: u64,
    },
    #[serde(rename = "error.clientArgumentTooLong")]
    ArgumentTooLong {
        arg: AnyString,
        val: u64,
    },
    #[serde(rename = "error.clientArgumentOutOfRange")]
    ArgumentOutOfRange {
        arg: AnyString,
        min: u64,
        max: u64,
    },
    #[serde(rename = "error.clientArgumentInvalidCharacter")]
    ArgumentInvalidCharacter {
        arg: AnyString,
        char: char,
        pos: u64,
    },
    #[serde(rename = "error.clientArgumentMissing")]
    ArgumentMissing {
        arg: AnyString,
    },

    // --- SERVER
    #[serde(rename="error.miscServerConfigUninitialized")]
    ServerConfigUninitialized,
    #[serde(rename = "error.serverEntityAlreadyExists")]
    EntityAlreadyExists {
        entity: AnyString,
    },
    #[serde(rename = "error.serverEntityNotFound")]
    EntityNotFound {
        entity: AnyString,
    },

    // -- DATABASE
    #[serde(rename = "error.databaseDatabaseFailure")]
    DatabaseFailure {
        msg: Option<AnyString>,
    },
    #[serde(rename = "error.databaseInputInvalid")]
    InputInvalid {
        input: AnyString,
    },
    #[serde(rename = "error.databaseInputInvalidID")]
    InputInvalidID,
    #[serde(rename = "error.databaseTransactionNotExecuted")]
    TransactionNotExecuted {
        msg: Option<AnyString>,
    },
    #[serde(rename = "error.databaseQueryInvalid")]
    QueryInvalid {
        msg: Option<AnyString>,
    },
    #[serde(rename = "error.databaseUnexpectedResponseMissing")]
    UnexpectedResponseMissing {
        entity: AnyString,
    },
    #[serde(rename = "error.databaseUnexpectedResponseEmpty")]
    UnexpectedResponseEmpty {
        expected: Option<AnyString>,
    },
    #[serde(rename = "error.databaseUnexpectedResponseType")]
    UnexpectedResponseType {
        expected: Option<AnyString>,
        provided: Option<AnyString>,
    },
    #[serde(rename = "error.databaseUnexpectedResponseLen")]
    UnexpectedResponseLen {
        expected: usize,
        provided: usize,
    },
    #[serde(rename = "error.databaseUnexpectedResponseError")]
    UnexpectedResponseError {
        msg: Option<AnyString>,
    },
    #[serde(rename = "error.databaseInvalidRollbackCode")]
    InvalidRollbackCode {
        code: u64,
    },
    #[serde(rename = "error.databaseSystemHasNoAssignedKeys")]
    SystemHasNoAssignedKeys,
    #[serde(rename = "error.databaseSystemAssignedKeyIsInvalid")]
    SystemAssignedKeyIsInvalid,

    // --- ACCESS CODE
    #[serde(rename = "error.accessCodeInvalidFormat")]
    AccessCodeInvalidFormat,
    #[serde(rename = "error.accessCodeUnexpectedPayload")]
    AccessCodeUnexpectedPayload,
    #[serde(rename = "error.accessCodeTampered")]
    AccessCodeTampered,
    #[serde(rename = "error.accessCodeExpired")]
    AccessCodeExpired,
}

impl RHSError {
    fn get_http_code(&self) -> u16 {
        use RHSError::*;

        match self {
            InvalidEndpoint{..} | MissingHeader{..} | InvalidBody |
            ArgumentInvalid{..} | ArgumentTooShort{..} | ArgumentOutOfRange{..} |
            ArgumentInvalidCharacter{..} | ArgumentMissing{..} => 400,
            MethodNotAccepted => 405,
            RejectedBody => 413,
            Unimplemented => 501,
            _ => 500,
        }
    }
}

impl Display for RHSError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl std::error::Error for RHSError {}
