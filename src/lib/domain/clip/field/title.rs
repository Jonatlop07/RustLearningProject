use crate::domain::clip::ClipError;
use rocket::form::{self, DataField, FromFormField, ValueField};
use serde::{Serialize, Deserialize};
use std::str::FromStr;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Title(Option<String>);

impl Title {
    pub fn new<T: Into<Option<String>>>(title: T) -> Self {
        let title: Option<String> = title.into();
        let empty_title = Self(None);
        match title {
            Some(title) => {
                if !title.trim().is_empty() {
                    Self(Some(title));
                }
                empty_title
            },
            None => empty_title
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
        Ok(Self::new(s.to_string()))
    }
}

#[rocket::async_trait]
impl<'r> FromFormField<'r> for Title {
    fn from_value(field: ValueField<'r>) -> form::Result<'r, Self> {
        Ok(Self::new(field.value.to_owned()))
    }
}