use std::str::FromStr;
use crate::error::RHSError;

pub trait OnRollback<T> {
    type Error;

    fn on_rollback<F: Fn(usize) -> Self::Error>(self, f: F) -> Result<T, Self::Error>;
}

impl<T> OnRollback<T> for Result<T, surrealdb::Error> {
    type Error = RHSError;

    fn on_rollback<F: Fn(usize) -> Self::Error>(self, f: F) -> Result<T, Self::Error> {
        self.map_err(|err| match &err {
            surrealdb::Error::Db(surrealdb::error::Db::FieldValue { thing, value, .. }) => {
                if !thing.starts_with("rollback:") {
                    RHSError::from(err)
                } else {
                    if let Ok(code) = usize::from_str(value) {
                        f(code)
                    } else {
                        RHSError::from(err)
                    }
                }
            }
            _ => RHSError::from(err)
        })
    }
}
