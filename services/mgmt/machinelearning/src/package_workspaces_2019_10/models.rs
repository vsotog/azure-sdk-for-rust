#![doc = "generated by AutoRust 0.1.0"]
#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::{Deserialize, Serialize};
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ErrorResponse {
    pub code: String,
    pub message: String,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Workspace {
    #[serde(flatten)]
    pub resource: Resource,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<WorkspaceProperties>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct WorkspaceProperties {
    #[serde(rename = "userStorageAccountId")]
    pub user_storage_account_id: String,
    #[serde(rename = "ownerEmail")]
    pub owner_email: String,
    #[serde(rename = "workspaceType", skip_serializing)]
    pub workspace_type: Option<workspace_properties::WorkspaceType>,
    #[serde(rename = "workspaceState", skip_serializing)]
    pub workspace_state: Option<workspace_properties::WorkspaceState>,
    #[serde(rename = "workspaceId", skip_serializing)]
    pub workspace_id: Option<String>,
    #[serde(rename = "creationTime", skip_serializing)]
    pub creation_time: Option<String>,
    #[serde(rename = "studioEndpoint", skip_serializing)]
    pub studio_endpoint: Option<String>,
    #[serde(rename = "keyVaultIdentifierId", skip_serializing_if = "Option::is_none")]
    pub key_vault_identifier_id: Option<String>,
}
pub mod workspace_properties {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum WorkspaceType {
        Production,
        Free,
        Anonymous,
        PaidStandard,
        PaidPremium,
    }
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum WorkspaceState {
        Deleted,
        Enabled,
        Disabled,
        Migrated,
        Updated,
        Registered,
        Unregistered,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct WorkspaceUpdateParameters {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<WorkspacePropertiesUpdateParameters>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct WorkspacePropertiesUpdateParameters {
    #[serde(rename = "workspaceState", skip_serializing_if = "Option::is_none")]
    pub workspace_state: Option<workspace_properties_update_parameters::WorkspaceState>,
    #[serde(rename = "keyVaultIdentifierId", skip_serializing_if = "Option::is_none")]
    pub key_vault_identifier_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sku: Option<Sku>,
}
pub mod workspace_properties_update_parameters {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum WorkspaceState {
        Deleted,
        Enabled,
        Disabled,
        Migrated,
        Updated,
        Registered,
        Unregistered,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct WorkspaceKeysResponse {
    #[serde(rename = "primaryToken", skip_serializing_if = "Option::is_none")]
    pub primary_token: Option<String>,
    #[serde(rename = "secondaryToken", skip_serializing_if = "Option::is_none")]
    pub secondary_token: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct WorkspaceListResult {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<Workspace>,
    #[serde(rename = "nextLink", skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Operation {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display: Option<operation::Display>,
}
pub mod operation {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub struct Display {
        #[serde(skip_serializing_if = "Option::is_none")]
        pub provider: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub resource: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub operation: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub description: Option<String>,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct OperationListResult {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<Operation>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Resource {
    #[serde(skip_serializing)]
    pub id: Option<String>,
    #[serde(skip_serializing)]
    pub name: Option<String>,
    #[serde(rename = "type", skip_serializing)]
    pub type_: Option<String>,
    pub location: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sku: Option<Sku>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Sku {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tier: Option<String>,
}
