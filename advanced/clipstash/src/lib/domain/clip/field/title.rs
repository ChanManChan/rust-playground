use super::super::ClipError;
use rocket::form::{FromFormField, ValueField};
use serde::{Deserialize, Serialize};
use std::str::FromStr;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Title(Option<String>);

impl Title {
    pub fn new<T: Into<Option<String>>>(title: T) -> Self {
        let title: Option<String> = title.into();

        match title {
            Some(title) => {
                if title.trim().is_empty() {
                    Self(None)
                } else {
                    Self(Some(title))
                }
            }
            None => Self(None),
        }
    }
    pub fn into_inner(self) -> Option<String> {
        self.0
    }
}

impl Default for Title {
    fn default() -> Self {
        Self::new(None)
    }
}

impl FromStr for Title {
    type Err = ClipError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self::new(s.to_owned()))
    }
}

#[rocket::async_trait]
impl<'r> FromFormField<'r> for Title {
    fn from_value(field: ValueField<'r>) -> rocket::form::Result<'r, Self> {
        Ok(Self::from_str(field.value)
            .map_err(|e| rocket::form::Error::validation(format!("{}", e)))?)
    }
}
