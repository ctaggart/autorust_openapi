mod common;
use common::*;

// cargo test --test azure_rest_api_specs
// These tests require cloning azure-rest-api-specs.
// git clone git@github.com:Azure/azure-rest-api-specs.git ../azure-rest-api-specs
#[test]
fn can_deserialize_azure_rest_api_specs() -> Result<()> {
    assert_deserialize_without_ignored(vec![
        "../azure-rest-api-specs/specification/vmware/resource-manager/Microsoft.AVS/stable/2020-03-20/vmware.json",
        "../azure-rest-api-specs/specification/batch/data-plane/Microsoft.Batch/stable/2020-03-01.11.0/BatchService.json",
        "../azure-rest-api-specs/specification/security/resource-manager/common/v1/types.json",
        "../azure-rest-api-specs/specification/cosmos-db/resource-manager/Microsoft.DocumentDB/stable/2020-04-01/cosmos-db.json",
        "../azure-rest-api-specs/specification/alertsmanagement/resource-manager/Microsoft.AlertsManagement/preview/2019-05-05-preview/ActionRules.json"
    ])
}

#[test]
fn can_roundtrip_azure_rest_api_specs() -> Result<()> {
    assert_roundtrip_eq(vec![
        "../azure-rest-api-specs/specification/vmware/resource-manager/Microsoft.AVS/stable/2020-03-20/vmware.json",
        "../azure-rest-api-specs/specification/batch/data-plane/Microsoft.Batch/stable/2020-03-01.11.0/BatchService.json",
        "../azure-rest-api-specs/specification/security/resource-manager/common/v1/types.json",
        "../azure-rest-api-specs/specification/cosmos-db/resource-manager/Microsoft.DocumentDB/stable/2020-04-01/cosmos-db.json",
        "../azure-rest-api-specs/specification/alertsmanagement/resource-manager/Microsoft.AlertsManagement/preview/2019-05-05-preview/ActionRules.json"
    ])
}
