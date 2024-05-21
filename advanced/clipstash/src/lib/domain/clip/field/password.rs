use super::super::ClipError;
use rocket::form::{FromFormField, ValueField};
use serde::{Deserialize, Serialize};
use std::str::FromStr;

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, PartialOrd)]
pub struct Password(Option<String>);

impl Password {
    pub fn new<T: Into<Option<String>>>(password: T) -> Result<Self, ClipError> {
        let password: Option<String> = password.into();
        match password {
            Some(password) => {
                if password.trim().is_empty() {
                    Ok(Self(None))
                } else {
                    Ok(Self(Some(password)))
                }
            }
            None => Ok(Self(None)),
        }
    }
    pub fn into_inner(self) -> Option<String> {
        self.0
    }
    pub fn has_password(&self) -> bool {
        self.0.is_some()
    }
}

impl Default for Password {
    fn default() -> Self {
        Self(None)
    }
}

impl FromStr for Password {
    type Err = ClipError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::new(s.to_string())
    }
}

#[rocket::async_trait]
impl<'r> FromFormField<'r> for Password {
    fn from_value(field: ValueField<'r>) -> rocket::form::Result<'r, Self> {
        Ok(Self::new(field.value.to_owned())
            .map_err(|e| rocket::form::Error::validation(format!("{}", e)))?)
    }
}
