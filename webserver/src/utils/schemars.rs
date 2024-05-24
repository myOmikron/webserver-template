//! Utilities for patching missing [`JsonSchema`] implementations

use std::borrow::Cow;

use schemars::gen::SchemaGenerator;
use schemars::schema::Schema;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

/// Wrap any type to "provide" a [`JsonSchema`] implementation of [`String`]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub struct SchemaString<T>(pub T);
impl<T> JsonSchema for SchemaString<T> {
    fn is_referenceable() -> bool {
        <String as JsonSchema>::is_referenceable()
    }

    fn schema_name() -> String {
        <String as JsonSchema>::schema_name()
    }

    fn schema_id() -> Cow<'static, str> {
        <String as JsonSchema>::schema_id()
    }

    fn json_schema(gen: &mut SchemaGenerator) -> Schema {
        <String as JsonSchema>::json_schema(gen)
    }
}
