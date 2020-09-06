use crate::*;
use serde::{Deserialize, Serialize};
use indexmap::IndexMap;

/// This is the root document object for the API specification.
/// It combines what previously was the Resource Listing and API Declaration (version 1.2 and earlier) together into one document.
/// https://github.com/OAI/OpenAPI-Specification/blob/master/versions/2.0.md#swagger-object
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Default)]
#[serde(rename_all = "camelCase")]
pub struct OpenAPI {
    /// The Swagger version of this document.
    pub swagger: String,
    pub info: Info,
    /// The host (name or ip) of the API. Example: 'swagger.io'
    #[serde(skip_serializing_if = "Option::is_none")]
    pub host: Option<String>,
    /// The base path to the API. Example: '/api'.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub base_path: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub schemes: Vec<Scheme>,
    /// A list of MIME types accepted by the API.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub consumes: Vec<String>,
    /// A list of MIME types the API can produce.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub produces: Vec<String>,
    /// Relative paths to the individual endpoints. They must be relative to the 'basePath'.
    #[serde(default, skip_serializing_if = "IndexMap::is_empty")]
    pub paths: Paths,
    #[serde(default, skip_serializing_if = "IndexMap::is_empty")]
    pub definitions: IndexMap<String, ReferenceOr<Schema>>,
    #[serde(default, skip_serializing_if = "IndexMap::is_empty")]
    pub parameters: IndexMap<String, Parameter>,
    /// mappings to http response codes or "default"
    #[serde(default, skip_serializing_if = "IndexMap::is_empty")]
    pub responses: IndexMap<String, Response>,
    #[serde(default, skip_serializing_if = "IndexMap::is_empty")]
    pub security_definitions: IndexMap<String, Security>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub security: Vec<IndexMap<String, Vec<String>>>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub tags: Vec<Tag>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_docs: Option<ExternalDocumentation>,
}
