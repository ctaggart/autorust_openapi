use crate::*;
use serde::{Deserialize, Serialize};

/// https://swagger.io/docs/specification/using-ref/
/// https://github.com/OAI/OpenAPI-Specification/blob/master/versions/2.0.md#referenceObject
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(untagged)]
pub enum ReferenceOr<T> {
    Reference {
        #[serde(rename = "$ref")]
        reference: String,
        // $ref with sibling elements are not OpenAPI spec compliant
        // https://github.com/ctaggart/autorust_openapi/issues/13
        #[serde(skip_serializing_if = "Option::is_none")]
        title: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        description: Option<String>,
        // specifying "type" feels like a bug in the spec
        #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
        type_: Option<DataType>,
        #[serde(rename = "readOnly", skip_serializing_if = "Option::is_none")]
        read_only: Option<bool>,

        /// flattens client model property or parameter
        /// https://github.com/Azure/autorest/blob/master/docs/extensions/readme.md#x-ms-client-flatten
        #[serde(rename = "x-ms-client-flatten", skip_serializing_if = "Option::is_none")]
        x_ms_client_flatten: Option<bool>,
    },
    Item(T),
}

#[cfg(test)]
mod tests {
    use crate::{Parameter, ReferenceOr};
    use serde_json;

    #[test]
    fn deserializes() {
        let json = r#"{"$ref":"foo/bar"}"#;
        assert_eq!(
            serde_json::from_str::<ReferenceOr<Parameter>>(&json).unwrap(),
            ReferenceOr::<Parameter>::Reference {
                reference: "foo/bar".into(),
                title: None,
                description: None,
                x_ms_client_flatten: None,
                type_: None,
                read_only: None
            }
        );
    }

    #[test]
    fn serializes() {
        let json = r#"{"$ref":"foo/bar"}"#;
        assert_eq!(
            json,
            serde_json::to_string(&ReferenceOr::<Parameter>::Reference {
                reference: "foo/bar".into(),
                title: None,
                description: None,
                x_ms_client_flatten: None,
                type_: None,
                read_only: None
            })
            .unwrap()
        );
    }
}
