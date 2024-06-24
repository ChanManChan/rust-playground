use std::str::FromStr;

use serde::{Deserialize, Serialize};

macro_rules! new_id {
    ($name: ident) => {
        #[derive(
            Clone, Copy, Debug, Eq, PartialEq, Hash, Serialize, Deserialize, PartialOrd, Ord,
        )]
        #[cfg_attr(feature = "query", derive(DieselNewType))]
        pub struct $name(uuid::Uuid);

        impl $name {
            pub fn new() -> Self {
                Self(uuid::Uuid::new_v4())
            }
            pub fn into_inner(self) -> uuid::Uuid {
                self.0
            }
            pub fn as_uuid(&self) -> &uuid::Uuid {
                &self.0
            }
            pub fn to_string(&self) -> String {
                self.0.to_string()
            }
        }

        impl Default for $name {
            fn default() -> Self {
                Self::new()
            }
        }

        impl From<uuid::Uuid> for $name {
            fn from(id: uuid::Uuid) -> Self {
                Self(id)
            }
        }

        impl FromStr for $name {
            type Err = IdError;
            fn from_str(id: &str) -> Result<Self, Self::Err> {
                uuid::Uuid::try_parse(id)
                    .map(|id| id.into())
                    .map_err(|_| IdError::Parse)
            }
        }
    };
}

new_id!(UserId);
new_id!(SessionId);
new_id!(PostId);

#[derive(Debug, thiserror::Error)]
pub enum IdError {
    #[error("failed to parse ID")]
    Parse,
}
