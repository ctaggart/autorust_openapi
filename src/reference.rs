use serde::{Deserialize, Serialize};

/// https://swagger.io/docs/specification/using-ref/
/// https://github.com/OAI/OpenAPI-Specification/blob/master/versions/2.0.md#referenceObject
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(untagged)]
pub enum ReferenceOr<T> {
    Reference {
        #[serde(rename = "$ref")]
        reference: String,
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
                reference: "foo/bar".into()
            }
        );
    }

    #[test]
    fn serializes() {
        let json = r#"{"$ref":"foo/bar"}"#;
        assert_eq!(
            json,
            serde_json::to_string(&ReferenceOr::<Parameter>::Reference {
                reference: "foo/bar".into()
            })
            .unwrap()
        );
    }
}
