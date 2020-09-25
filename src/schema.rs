use crate::*;
use indexmap::IndexMap;
use serde::{Deserialize, Serialize};

// http://json.schemastore.org/swagger-2.0

/// The transfer protocol of the API. Values MUST be from the list: "http", "https", "ws", "wss".
/// If the schemes is not included, the default scheme to be used is the one used to access the Swagger definition itself.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum Scheme {
    Http,
    Https,
    Ws,
    Wss,
}

impl Default for Scheme {
    fn default() -> Self {
        Scheme::Http
    }
}

/// https://swagger.io/docs/specification/data-models/data-types/
/// https://github.com/OAI/OpenAPI-Specification/blob/master/versions/2.0.md#data-types
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum DataType {
    String,
    Number,
    Integer,
    Boolean,
    Array,
    Object,
}

/// https://github.com/OAI/OpenAPI-Specification/blob/master/versions/2.0.md#securityRequirementObject
pub type SecurityRequirement = IndexMap<String, Vec<String>>;

/// https://swagger.io/docs/specification/2-0/describing-responses/
/// https://github.com/OAI/OpenAPI-Specification/blob/master/versions/2.0.md#responseObject
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Default)]
pub struct Response {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schema: Option<ReferenceOr<Schema>>,
    #[serde(default, skip_serializing_if = "IndexMap::is_empty")]
    pub headers: IndexMap<String, ReferenceOr<Header>>,
}

/// https://github.com/OAI/OpenAPI-Specification/blob/master/versions/2.0.md#schemaObject
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Default)]
#[serde(rename_all = "camelCase")]
pub struct Schema {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub required: Vec<String>,
    // implies object
    #[serde(default, skip_serializing_if = "IndexMap::is_empty")]
    pub properties: IndexMap<String, ReferenceOr<Schema>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub additional_properties: Box<Option<ReferenceOr<Schema>>>,
    // composition
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub all_of: Vec<ReferenceOr<Schema>>,
    #[serde(default, skip_serializing_if = "is_false")]
    pub read_only: bool,

    // fields also found in Parameter Object
    // https://github.com/OAI/OpenAPI-Specification/blob/master/versions/2.0.md#parameter-object
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub type_: Option<DataType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub format: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub items: Box<Option<ReferenceOr<Schema>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maximum: Option<f64>,
    #[serde(default, skip_serializing_if = "is_false")]
    pub exclusive_maximum: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub minimum: Option<f64>,
    #[serde(default, skip_serializing_if = "is_false")]
    pub exclusive_minimum: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_length: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_length: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pattern: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_items: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_items: Option<usize>,
    #[serde(default, skip_serializing_if = "is_false")]
    pub unique_items: bool,
    #[serde(rename = "enum", default, skip_serializing_if = "Vec::is_empty")]
    pub enum_: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub multiple_of: Option<f64>,

    /// flattens client model property or parameter
    /// https://github.com/Azure/autorest/blob/master/docs/extensions/readme.md#x-ms-client-flatten
    #[serde(rename = "x-ms-client-flatten", default, skip_serializing_if = "is_false")]
    pub x_ms_client_flatten: bool,

    /// additional metadata for enums
    /// https://github.com/Azure/autorest/blob/master/docs/extensions/readme.md#x-ms-enum
    #[serde(rename = "x-ms-enum", skip_serializing_if = "Option::is_none")]
    pub x_ms_enum: Option<MsEnum>,

    #[serde(rename = "x-ms-secret", default, skip_serializing_if = "is_false")]
    pub x_ms_secret: bool,

    /// indicates that the Definition Schema Object is a resource as defined by the Resource Manager API
    /// https://github.com/Azure/autorest/blob/master/docs/extensions/readme.md#x-ms-azure-resource
    #[serde(rename = "x-ms-azure-resource", default, skip_serializing_if = "is_false")]
    pub x_ms_azure_resource: bool,

    /// provides insight to Autorest on how to generate code. It doesn't alter the modeling of what is actually sent on the wire
    /// https://github.com/Azure/autorest/blob/master/docs/extensions/readme.md#x-ms-mutability
    #[serde(rename = "x-ms-mutability", default, skip_serializing_if = "Vec::is_empty")]
    pub x_ms_mutability: Vec<MsMutability>,

    /// allows specific Definition Objects to be excluded from code generation
    /// https://github.com/Azure/autorest/blob/master/docs/extensions/readme.md#x-ms-external
    #[serde(rename = "x-ms-external", default, skip_serializing_if = "is_false")]
    pub x_ms_external: bool,
}
