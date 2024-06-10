//! Wrapper around [String] that requires not to be empty

use std::ops::Deref;

use schemars::gen::SchemaGenerator;
use schemars::schema::InstanceType;
use schemars::schema::Schema;
use schemars::schema::SchemaObject;
use schemars::schema::StringValidation;
use schemars::JsonSchema;
use serde::de::Error;
use serde::Deserialize;
use serde::Deserializer;
use serde::Serialize;
use serde::Serializer;
use thiserror::Error;

/// A wrapper around a T, most likely a string, which checks the
/// minimum and maximum length of the string
#[derive(Debug, Clone)]
pub struct CheckedString<const MIN_LEN: u32 = 0, const MAX_LEN: u32 = 255, T = String>(T)
where
    T: Deref<Target = str>;

impl<const MIN_LEN: u32, const MAX_LEN: u32, T> CheckedString<MIN_LEN, MAX_LEN, T>
where
    T: Deref<Target = str>,
{
    /// Instantiate a new Checked string
    pub fn new(str: T) -> Result<Self, ConstraintsViolated> {
        let len = str.len() as u32;
        if len < MIN_LEN || (MAX_LEN > 0 && len > MAX_LEN) {
            return Err(ConstraintsViolated);
        }

        Ok(Self(str))
    }

    /// Convert the string into its inner type
    pub fn into_inner(self) -> T {
        self.0
    }
}

impl<const MIN_LEN: u32, const MAX_LEN: u32, T> JsonSchema for CheckedString<MIN_LEN, MAX_LEN, T>
where
    T: Deref<Target = str>,
{
    fn is_referenceable() -> bool {
        false
    }
    fn schema_name() -> String {
        "CheckedString".to_string()
    }

    fn json_schema(_gen: &mut SchemaGenerator) -> Schema {
        if MIN_LEN != 0 && MIN_LEN > MAX_LEN {
            // compile_error!("Minimum length must not exceed maximum length: {MIN_LENG} > {MAX_LEN}");
        }

        SchemaObject {
            instance_type: Some(InstanceType::String.into()),
            format: None,
            string: Some(Box::new(StringValidation {
                max_length: if MAX_LEN == 0 { None } else { Some(MAX_LEN) },
                min_length: Some(MIN_LEN),
                pattern: None,
            })),
            ..Default::default()
        }
        .into()
    }
}

impl<'de, const MIN_LEN: u32, const MAX_LEN: u32, T> Deserialize<'de>
    for CheckedString<MIN_LEN, MAX_LEN, T>
where
    T: Deref<Target = str> + Deserialize<'de>,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let str = T::deserialize(deserializer)?;

        let len = str.len() as u32;

        if len < MIN_LEN {
            return Err(Error::custom(format!(
                "Minimum of {MIN_LEN} violated: {len} < {MIN_LEN}"
            )));
        } else if MAX_LEN > 0 && len > MAX_LEN {
            return Err(Error::custom(format!(
                "Maximum of {MAX_LEN} violated: {len} > {MAX_LEN}"
            )));
        }

        Ok(CheckedString(str))
    }
}

impl<const MIN_LEN: u32, const MAX_LEN: u32, T> Serialize for CheckedString<MIN_LEN, MAX_LEN, T>
where
    T: Deref<Target = str>,
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.0)
    }
}

impl<const MIN_LEN: u32, const MAX_LEN: u32, T> Deref for CheckedString<MIN_LEN, MAX_LEN, T>
where
    T: Deref<Target = str>,
{
    type Target = str;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

/// Empty string found
#[derive(Debug, Error)]
#[error("String constraints were violated")]
pub struct ConstraintsViolated;
