use crate::database::types::ID;
use super::*;

impl From<surrealdb::Error> for RHSError {
    fn from(err: surrealdb::Error) -> Self {
        match err {
            surrealdb::Error::Db(err_db) => {
                use surrealdb::error::Db::*;
                match err_db {
                    Tx(msg) => {
                        RHSError::TransactionNotExecuted {
                            #[cfg(debug_assertions)]
                            msg: Some(msg.into()),
                            #[cfg(not(debug_assertions))]
                            msg: None,
                        }
                    },
                    TxFailure => {
                        RHSError::TransactionNotExecuted {
                            #[cfg(debug_assertions)]
                            msg: Some("Failed to start the transaction.".into()),
                            #[cfg(not(debug_assertions))]
                            msg: None,
                        }
                    },
                    TxFinished => {
                        RHSError::TransactionNotExecuted {
                            #[cfg(debug_assertions)]
                            msg: Some("Couldn't update a finished transaction.".into()),
                            #[cfg(not(debug_assertions))]
                            msg: None,
                        }
                    },
                    TxReadonly => {
                        RHSError::TransactionNotExecuted {
                            #[cfg(debug_assertions)]
                            msg: Some("Couldn't write to a read only transaction.".into()),
                            #[cfg(not(debug_assertions))]
                            msg: None,
                        }
                    },
                    TxConditionNotMet => {
                        RHSError::TransactionNotExecuted {
                            #[cfg(debug_assertions)]
                            msg: Some("Value being checked was not correct.".into()),
                            #[cfg(not(debug_assertions))]
                            msg: None,
                        }
                    },
                    TxKeyAlreadyExistsCategory(_) => {
                        RHSError::TransactionNotExecuted {
                            #[cfg(debug_assertions)]
                            msg: Some("The key being inserted already exists.".into()),
                            #[cfg(not(debug_assertions))]
                            msg: None,
                        }
                    },
                    InvalidQuery { .. } => {
                        RHSError::QueryInvalid {
                            #[cfg(debug_assertions)]
                            msg: Some(format!(
                                "Tried to execute an invalid query. [{}]", err_db
                            ).into()),
                            #[cfg(not(debug_assertions))]
                            msg: None,
                        }
                    }
                    TbNotFound { value } => {
                        // todo: Find out what the `value` is.
                        RHSError::EntityNotFound {
                            entity: value.into(),
                        }
                    },
                    FieldValue { thing, .. } => {
                        RHSError::EntityAlreadyExists {
                            entity: match ID::from_raw(thing) {
                                ID::Pointer(thing) => thing.tb.into(),
                                ID::Raw(id) => id.into(),
                            },
                        }
                    },
                    IndexExists { thing, .. } => {
                        RHSError::EntityAlreadyExists {
                            entity: match ID::from_raw(thing) {
                                ID::Pointer(thing) => thing.tb.into(),
                                ID::Raw(id) => id.into(),
                            },
                        }
                    },
                    RecordExists { thing } => {
                        RHSError::EntityAlreadyExists {
                            entity: match ID::from_raw(thing) {
                                ID::Pointer(thing) => thing.tb.into(),
                                ID::Raw(id) => id.into(),
                            },
                        }
                    },
                    IdInvalid { .. } => {
                        RHSError::InputInvalidID
                    }
                    _ => RHSError::DatabaseFailure {
                        #[cfg(debug_assertions)]
                        msg: Some(format!("{}", err_db).into()),
                        #[cfg(not(debug_assertions))]
                        msg: None,
                    }
                }
            }
            surrealdb::Error::Api(err_api) => {
                match err_api {
                    _ => RHSError::DatabaseFailure {
                        #[cfg(debug_assertions)]
                        msg: Some(format!("{}", err_api).into()),
                        #[cfg(not(debug_assertions))]
                        msg: None,
                    }
                }
            }
        }
    }
}
