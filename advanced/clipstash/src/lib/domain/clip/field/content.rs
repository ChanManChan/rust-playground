use super::super::ClipError;
use rocket::form::{FromFormField, ValueField};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Content(String);

impl Content {
    pub fn new(content: &str) -> Result<Self, ClipError> {
        if content.trim().is_empty() {
            Err(ClipError::EmptyContent)
        } else {
            Ok(Self(content.to_owned()))
        }
    }
    pub fn into_inner(self) -> String {
        self.0
    }
    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }
}

#[rocket::async_trait]
impl<'r> FromFormField<'r> for Content {
    fn from_value(field: ValueField<'r>) -> rocket::form::Result<'r, Self> {
        Ok(
            Self::new(field.value)
                .map_err(|e| rocket::form::Error::validation(format!("{}", e)))?,
        )
    }
}
