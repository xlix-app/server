use serde::{Deserialize, Deserializer};
use serde::de::SeqAccess;
use surrealdb::sql::{Id, Thing};

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(untagged)]
pub enum ID {
    Pointer(Thing),
    Raw(String),
}

impl ID {
    pub fn from_raw(id: impl ToString) -> Self {
        Self::Raw(id.to_string())
    }

    pub fn into_raw(self) -> String {
        match self {
            Self::Pointer(thing) => thing.id.to_raw(),
            Self::Raw(raw) => raw,
        }
    }

    pub fn into_thing(self, table: impl ToString) -> Thing {
        let table = table.to_string();

        match self {
            Self::Pointer(mut thing) => {
                thing.tb = table;
                thing
            },
            Self::Raw(id) => Thing {
                tb: table,
                id: Id::String(id),
            }
        }
    }

    pub fn map_to_raw(self) -> Self {
        match self {
            ID::Pointer(thing) => match thing.id {
                Id::String(raw) => {
                    let mut inner = String::new();
                    let _ = std::mem::replace(&mut inner, raw);
                    Self::Raw(inner)
                },
                raw => Self::Raw(raw.to_string()),
            }
            id => id,
        }
    }
}

pub struct Pointer;

impl Pointer {
    pub fn new(table: impl ToString, id: impl ToString) -> Thing {
        Thing {
            tb: table.to_string(),
            id: Id::String(id.to_string()),
        }
    }
}
