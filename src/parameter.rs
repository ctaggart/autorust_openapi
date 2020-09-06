
use serde::{Deserialize, Serialize};
use crate::Schema;
use crate::is_false;

/// https://github.com/OAI/OpenAPI-Specification/blob/master/versions/2.0.md#parameter-object
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Default)]
#[serde(rename_all = "camelCase")]
pub struct Parameter {
    /// both bodyParameter and nonBodyParameter in one for now
    /// The name of the parameter.
    pub name: String,
    /// values depend on parameter type
    /// may be `header`, `query`, 'path`, `formData`
    #[serde(rename = "in")]
    pub in_: String,
    #[serde(default, skip_serializing_if = "is_false")]
    pub required: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schema: Option<Schema>,
    #[serde(default, skip_serializing_if = "is_false")]
    pub unique_items: bool,
    /// string, number, boolean, integer, array, file ( only for formData )
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "type")]
    pub type_: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub items: Option<Box<Schema>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub format: Option<String>,
    /// A brief description of the parameter. This could contain examples
    /// of use.  GitHub Flavored Markdown is allowed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "collectionFormat", skip_serializing_if = "Option::is_none")]
    pub collection_format: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default: Option<serde_json::Value>,
    // maximum ?
    // exclusiveMaximum ??
    // minimum ??
    // exclusiveMinimum ??
    // maxLength ??
    // minLength ??
    // pattern ??
    // maxItems ??
    // minItems ??
    // enum ??
    // multipleOf ??
    // allowEmptyValue ( for query / body params )
    #[serde(rename = "x-ms-parameter-location", skip_serializing_if = "Option::is_none")]
    pub x_ms_parameter_location: Option<String>,
}