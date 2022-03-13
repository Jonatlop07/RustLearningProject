use crate::domain::clip::ClipError;
use rocket::form::{self, DataField, FromFormField, ValueField};
use serde::{Serialize, Deserialize};
use std::str::FromStr;

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, PartialOrd)]
pub struct Password(Option<String>);

impl Password {
    pub fn new<T: Into<Option<String>>>(password: T) -> Result<Self, ClipError> {
        let password: Option<String> = password.into();
        let empty_password = Ok(Self(None));
        match password {
            Some(password) => {
                if !password.trim().is_empty() {
                    return Ok(Self(Some(password)));
                }
                empty_password
            },
            None => empty_password
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
    fn from_value(field: ValueField<'r>) -> form::Result<'r, Self> {
        Ok(
            Self::new(field.value.to_owned())
                .map_err(
                    |e|
                        form::Error::validation(format!("{}", e))
                )?
        )
    }
}
