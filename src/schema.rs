use serde::{Deserialize, Serialize};
use indexmap::IndexMap;
use crate::*;

// http://json.schemastore.org/swagger-2.0

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

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Default)]
#[serde(rename_all = "lowercase")]
pub struct Tag {
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_docs: Option<ExternalDoc>,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Default)]
pub struct ExternalDoc {
    pub url: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

/// General information about the API.
///
/// https://github.com/OAI/OpenAPI-Specification/blob/master/versions/2.0.md#info-object
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Default)]
#[serde(rename_all = "lowercase")]
pub struct Info {
    /// A unique and precise title of the API.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    /// A semantic version number of the API.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "termsOfService", skip_serializing_if = "Option::is_none")]
    pub terms_of_service: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contact: Option<Contact>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub license: Option<License>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}

/// https://github.com/OAI/OpenAPI-Specification/blob/master/versions/2.0.md#contactObject
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Default)]
pub struct Contact {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    // TODO: Make sure the url is a valid URL
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    // TODO: Make sure the email is a valid email
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
}

/// https://github.com/OAI/OpenAPI-Specification/blob/master/versions/2.0.md#licenseObject
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Default)]
pub struct License {
    /// The name of the license type. It's encouraged to use an OSI
    /// compatible license.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// The URL pointing to the license.
    // TODO: Make sure the url is a valid URL
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}

/// https://github.com/OAI/OpenAPI-Specification/blob/master/versions/2.0.md#path-item-object
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Default)]
pub struct PathItem {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub get: Option<Operation>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub post: Option<Operation>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub put: Option<Operation>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub patch: Option<Operation>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delete: Option<Operation>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub options: Option<Operation>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub head: Option<Operation>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub parameters: Vec<ReferenceOr<Parameter>>,
}

/// https://github.com/OAI/OpenAPI-Specification/blob/master/versions/2.0.md#securityRequirementObject
pub type SecurityRequirement = IndexMap<String, Vec<String>>;

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

/// https://swagger.io/docs/specification/2-0/describing-responses/
/// https://github.com/OAI/OpenAPI-Specification/blob/master/versions/2.0.md#responseObject
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Default)]
pub struct Response {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schema: Option<Schema>,
    #[serde(default, skip_serializing_if = "IndexMap::is_empty")]
    pub headers: IndexMap<String, Header>,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
#[serde(tag = "type")]
pub enum Security {
    #[serde(rename = "apiKey")]
    ApiKey {
        name: String,
        #[serde(rename = "in")]
        in_: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        description: Option<String>,
    },
    #[serde(rename = "oauth2")]
    Oauth2 {
        flow: Flow,
        #[serde(rename = "authorizationUrl")]
        authorization_url: String,
        #[serde(rename = "tokenUrl")]
        #[serde(skip_serializing_if = "Option::is_none")]
        token_url: Option<String>,
        /// Required. The available scopes for the OAuth2 security scheme.
        scopes: IndexMap<String, String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        description: Option<String>,
    },
    #[serde(rename = "basic")]
    Basic {
        #[serde(skip_serializing_if = "Option::is_none")]
        description: Option<String>,
    },
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub enum Flow {
    Implicit,
    Password,
    Application,
    AccessCode,
}

/// https://github.com/Azure/autorest/blob/master/docs/extensions/readme.md#x-ms-enum
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct MsEnum {
    pub name: String,
    #[serde(default, skip_serializing_if = "is_false")]
    pub model_as_string: bool,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub values: Vec<MsEnumValue>,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
#[serde()]
pub struct MsEnumValue {
    pub value: serde_json::Value,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

/// https://github.com/OAI/OpenAPI-Specification/blob/master/versions/2.0.md#schemaObject
/// A [JSON schema](http://json-schema.org/) definition describing
/// the shape and properties of an object.
///
/// This may also contain a `$ref` to another definition
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Default)]
pub struct Schema {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "$ref")]
    pub ref_: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "type")]
    pub type_: Option<DataType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub format: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    #[serde(rename = "enum")]
    pub enum_: Vec<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub required: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub items: Option<Box<Schema>>,
    // implies object
    #[serde(default, skip_serializing_if = "IndexMap::is_empty")]
    pub properties: IndexMap<String, Schema>,
    #[serde(rename = "additionalProperties", skip_serializing_if = "Option::is_none")]
    pub additional_properties: Option<Box<Schema>>,
    // composition
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    #[serde(rename = "allOf")]
    pub all_of: Vec<Box<Schema>>,
    #[serde(rename = "readOnly", default, skip_serializing_if = "is_false")]
    pub read_only: bool,
    #[serde(default, skip_serializing_if = "is_false")]
    pub default: bool,

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

/// see Response Headers https://swagger.io/docs/specification/2-0/describing-responses/
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Default)]
pub struct Header {
    #[serde(rename = "type")]
    pub type_: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub format: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json;
    use indexmap::IndexMap;
    use crate::{ReferenceOr};

    #[test]
    fn security_api_deserializes() {
        let json = r#"{"type":"apiKey", "name":"foo", "in": "query"}"#;
        assert_eq!(
            serde_json::from_str::<Security>(&json).unwrap(),
            Security::ApiKey {
                name: "foo".into(),
                in_: "query".into(),
                description: None,
            }
        );
    }

    #[test]
    fn security_api_serializes() {
        let json = r#"{"type":"apiKey","name":"foo","in":"query"}"#;
        assert_eq!(
            serde_json::to_string(&Security::ApiKey {
                name: "foo".into(),
                in_: "query".into(),
                description: None,
            })
            .unwrap(),
            json
        );
    }

    #[test]
    fn security_basic_deserializes() {
        let json = r#"{"type":"basic"}"#;
        assert_eq!(
            serde_json::from_str::<Security>(&json).unwrap(),
            Security::Basic { description: None }
        );
    }

    #[test]
    fn security_basic_serializes() {
        let json = r#"{"type":"basic"}"#;
        assert_eq!(json, serde_json::to_string(&Security::Basic { description: None }).unwrap());
    }

    #[test]
    fn security_oauth_deserializes() {
        let json = r#"{"type":"oauth2","flow":"implicit","authorizationUrl":"foo/bar","scopes":{"foo":"bar"}}"#;
        let mut scopes = IndexMap::new();
        scopes.insert("foo".into(), "bar".into());
        assert_eq!(
            serde_json::from_str::<Security>(&json).unwrap(),
            Security::Oauth2 {
                flow: Flow::Implicit,
                authorization_url: "foo/bar".into(),
                token_url: None,
                scopes: scopes,
                description: None,
            }
        );
    }

    #[test]
    fn security_oauth_serializes() {
        let json = r#"{"type":"oauth2","flow":"implicit","authorizationUrl":"foo/bar","scopes":{"foo":"bar"}}"#;
        let mut scopes = IndexMap::new();
        scopes.insert("foo".into(), "bar".into());
        assert_eq!(
            json,
            serde_json::to_string(&Security::Oauth2 {
                flow: Flow::Implicit,
                authorization_url: "foo/bar".into(),
                token_url: None,
                scopes: scopes,
                description: None,
            })
            .unwrap()
        );
    }

    #[test]
    fn parameter_or_ref_deserializes_ref() {
        let json = r#"{"$ref":"foo/bar"}"#;
        assert_eq!(
            serde_json::from_str::<ReferenceOr<Parameter>>(&json).unwrap(),
            ReferenceOr::<Parameter>::Reference { reference: "foo/bar".into() }
        );
    }

    #[test]
    fn parameter_or_ref_serializes_pref() {
        let json = r#"{"$ref":"foo/bar"}"#;
        assert_eq!(
            json,
            serde_json::to_string(&ReferenceOr::<Parameter>::Reference { reference: "foo/bar".into() }).unwrap()
        );
    }
}
