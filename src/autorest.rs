//! AutoRest Extensions for OpenAPI 2.0
//! https://github.com/Azure/autorest/blob/master/docs/extensions/readme.md

use serde::{Deserialize, Serialize};
use indexmap::IndexMap;
use crate::{Operation, ReferenceOr};

/// provides insight to Autorest on how to generate code. It doesn't alter the modeling of what is actually sent on the wire
/// https://github.com/Azure/autorest/blob/master/docs/extensions/readme.md#x-ms-mutability
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum MsMutability {
    Create,
    Read,
    Update,
}

/// allows paging through lists of data
/// https://github.com/Azure/autorest/blob/master/docs/extensions/readme.md#x-ms-pageable
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Default)]
#[serde(rename_all = "camelCase")]
pub struct MsPageable {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub item_name: Option<String>,
    // nextLinkName is required, null is valid
    pub next_link_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operation_name: Option<String>,
}

/// describes the format for specifying examples for request and response of an operation
/// https://github.com/Azure/autorest/blob/master/docs/extensions/readme.md#x-ms-examples
pub type MsExamples = IndexMap<String, ReferenceOr<Operation>>;

/// https://github.com/Azure/autorest/blob/master/docs/extensions/readme.md#x-ms-long-running-operation-options
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Default)]
#[serde(rename_all = "camelCase")]
pub struct MsLongRunningOperationOptions {
    #[serde(rename = "final-state-via")]
    pub final_state_via: MsLongRunningOperationOptionsFinalStateVia,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "kebab-case")]
pub enum MsLongRunningOperationOptionsFinalStateVia {
    AzureAsyncOperation,
    Location,
    OriginalUri,
}

impl Default for MsLongRunningOperationOptionsFinalStateVia {
    fn default() -> Self {
        MsLongRunningOperationOptionsFinalStateVia::AzureAsyncOperation
    }
}

/// https://github.com/Azure/autorest/blob/master/docs/extensions/readme.md#x-ms-parameter-location
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub enum MsParameterLocation {
    Client,
    Method,
}

#[cfg(test)]
mod tests {
    use serde_json;
    use crate::{MsPageable};

    #[test]
    fn pageable_nextlinkname_may_be_null() {
        let json = r#"{"x-ms-pageable":{"nextLinkName":null}}"#;
        serde_json::from_str::<MsPageable>(json).unwrap();
    }
}