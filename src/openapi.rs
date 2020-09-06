use crate::*;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

/// https://github.com/OAI/OpenAPI-Specification/blob/master/versions/2.0.md
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
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub tags: Vec<Tag>,
    /// Relative paths to the individual endpoints. They must be relative
    /// to the 'basePath'.
    pub paths: BTreeMap<String, PathItem>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub definitions: Option<BTreeMap<String, Schema>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameters: Option<BTreeMap<String, Parameter>>,
    /// mappings to http response codes or "default"
    #[serde(skip_serializing_if = "Option::is_none")]
    pub responses: Option<BTreeMap<String, Response>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_definitions: Option<BTreeMap<String, Security>>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub security: Vec<BTreeMap<String, Vec<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_docs: Option<ExternalDoc>,
}