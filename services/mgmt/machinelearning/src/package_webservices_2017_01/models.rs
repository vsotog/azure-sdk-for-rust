#![doc = "generated by AutoRust 0.1.0"]
#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::{Deserialize, Serialize};
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Resource {
    #[serde(skip_serializing)]
    pub id: Option<String>,
    #[serde(skip_serializing)]
    pub name: Option<String>,
    pub location: String,
    #[serde(rename = "type", skip_serializing)]
    pub type_: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PatchedResource {
    #[serde(skip_serializing)]
    pub id: Option<String>,
    #[serde(skip_serializing)]
    pub name: Option<String>,
    #[serde(skip_serializing)]
    pub location: Option<String>,
    #[serde(rename = "type", skip_serializing)]
    pub type_: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct WebService {
    #[serde(flatten)]
    pub resource: Resource,
    pub properties: WebServiceProperties,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PatchedWebService {
    #[serde(flatten)]
    pub patched_resource: PatchedResource,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<WebServiceProperties>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct WebServiceProperties {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "createdOn", skip_serializing)]
    pub created_on: Option<String>,
    #[serde(rename = "modifiedOn", skip_serializing)]
    pub modified_on: Option<String>,
    #[serde(rename = "provisioningState", skip_serializing)]
    pub provisioning_state: Option<web_service_properties::ProvisioningState>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub keys: Option<WebServiceKeys>,
    #[serde(rename = "readOnly", skip_serializing_if = "Option::is_none")]
    pub read_only: Option<bool>,
    #[serde(rename = "swaggerLocation", skip_serializing)]
    pub swagger_location: Option<String>,
    #[serde(rename = "exposeSampleData", skip_serializing_if = "Option::is_none")]
    pub expose_sample_data: Option<bool>,
    #[serde(rename = "realtimeConfiguration", skip_serializing_if = "Option::is_none")]
    pub realtime_configuration: Option<RealtimeConfiguration>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub diagnostics: Option<DiagnosticsConfiguration>,
    #[serde(rename = "storageAccount", skip_serializing_if = "Option::is_none")]
    pub storage_account: Option<StorageAccount>,
    #[serde(rename = "machineLearningWorkspace", skip_serializing_if = "Option::is_none")]
    pub machine_learning_workspace: Option<MachineLearningWorkspace>,
    #[serde(rename = "commitmentPlan", skip_serializing_if = "Option::is_none")]
    pub commitment_plan: Option<CommitmentPlan>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input: Option<ServiceInputOutputSpecification>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output: Option<ServiceInputOutputSpecification>,
    #[serde(rename = "exampleRequest", skip_serializing_if = "Option::is_none")]
    pub example_request: Option<ExampleRequest>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub assets: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameters: Option<serde_json::Value>,
    #[serde(rename = "packageType")]
    pub package_type: web_service_properties::PackageType,
    #[serde(rename = "payloadsInBlobStorage", skip_serializing_if = "Option::is_none")]
    pub payloads_in_blob_storage: Option<bool>,
    #[serde(rename = "payloadsLocation", skip_serializing_if = "Option::is_none")]
    pub payloads_location: Option<BlobLocation>,
}
pub mod web_service_properties {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum ProvisioningState {
        Unknown,
        Provisioning,
        Succeeded,
        Failed,
    }
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum PackageType {
        Graph,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct WebServicePropertiesForGraph {
    #[serde(flatten)]
    pub web_service_properties: WebServiceProperties,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub package: Option<GraphPackage>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct WebServiceKeys {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub primary: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secondary: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PaginatedWebServicesList {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<WebService>,
    #[serde(rename = "nextLink", skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RealtimeConfiguration {
    #[serde(rename = "maxConcurrentCalls", skip_serializing_if = "Option::is_none")]
    pub max_concurrent_calls: Option<i64>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DiagnosticsConfiguration {
    pub level: diagnostics_configuration::Level,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expiry: Option<String>,
}
pub mod diagnostics_configuration {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Level {
        None,
        Error,
        All,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct StorageAccount {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MachineLearningWorkspace {
    pub id: String,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CommitmentPlan {
    pub id: String,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ServiceInputOutputSpecification {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "type")]
    pub type_: String,
    pub properties: serde_json::Value,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TableSpecification {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "type")]
    pub type_: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub format: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<serde_json::Value>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ColumnSpecification {
    #[serde(rename = "type")]
    pub type_: column_specification::Type,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub format: Option<column_specification::Format>,
    #[serde(rename = "enum", skip_serializing_if = "Vec::is_empty")]
    pub enum_: Vec<serde_json::Value>,
    #[serde(rename = "x-ms-isnullable", skip_serializing_if = "Option::is_none")]
    pub x_ms_isnullable: Option<bool>,
    #[serde(rename = "x-ms-isordered", skip_serializing_if = "Option::is_none")]
    pub x_ms_isordered: Option<bool>,
}
pub mod column_specification {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Type {
        Boolean,
        Integer,
        Number,
        String,
    }
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Format {
        Byte,
        Char,
        Complex64,
        Complex128,
        #[serde(rename = "Date-time")]
        DateTime,
        #[serde(rename = "Date-timeOffset")]
        DateTimeOffset,
        Double,
        Duration,
        Float,
        Int8,
        Int16,
        Int32,
        Int64,
        Uint8,
        Uint16,
        Uint32,
        Uint64,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ExampleRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inputs: Option<serde_json::Value>,
    #[serde(rename = "globalParameters", skip_serializing_if = "Option::is_none")]
    pub global_parameters: Option<serde_json::Value>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AssetItem {
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "type")]
    pub type_: asset_item::Type,
    #[serde(rename = "locationInfo")]
    pub location_info: BlobLocation,
    #[serde(rename = "inputPorts", skip_serializing_if = "Option::is_none")]
    pub input_ports: Option<serde_json::Value>,
    #[serde(rename = "outputPorts", skip_serializing_if = "Option::is_none")]
    pub output_ports: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub parameters: Vec<ModuleAssetParameter>,
}
pub mod asset_item {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Type {
        Module,
        Resource,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct BlobLocation {
    pub uri: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub credentials: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ModuleAssetParameter {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "parameterType", skip_serializing_if = "Option::is_none")]
    pub parameter_type: Option<String>,
    #[serde(rename = "modeValuesInfo", skip_serializing_if = "Option::is_none")]
    pub mode_values_info: Option<serde_json::Value>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ModeValueInfo {
    #[serde(rename = "interfaceString", skip_serializing_if = "Option::is_none")]
    pub interface_string: Option<String>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub parameters: Vec<ModuleAssetParameter>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct InputPort {
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub type_: Option<input_port::Type>,
}
pub mod input_port {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Type {
        Dataset,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct OutputPort {
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub type_: Option<output_port::Type>,
}
pub mod output_port {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Type {
        Dataset,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct GraphPackage {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nodes: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub edges: Vec<GraphEdge>,
    #[serde(rename = "graphParameters", skip_serializing_if = "Option::is_none")]
    pub graph_parameters: Option<serde_json::Value>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct GraphNode {
    #[serde(rename = "assetId", skip_serializing_if = "Option::is_none")]
    pub asset_id: Option<String>,
    #[serde(rename = "inputId", skip_serializing_if = "Option::is_none")]
    pub input_id: Option<String>,
    #[serde(rename = "outputId", skip_serializing_if = "Option::is_none")]
    pub output_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameters: Option<serde_json::Value>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct GraphEdge {
    #[serde(rename = "sourceNodeId", skip_serializing_if = "Option::is_none")]
    pub source_node_id: Option<String>,
    #[serde(rename = "sourcePortId", skip_serializing_if = "Option::is_none")]
    pub source_port_id: Option<String>,
    #[serde(rename = "targetNodeId", skip_serializing_if = "Option::is_none")]
    pub target_node_id: Option<String>,
    #[serde(rename = "targetPortId", skip_serializing_if = "Option::is_none")]
    pub target_port_id: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct GraphParameter {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "type")]
    pub type_: graph_parameter::Type,
    pub links: Vec<GraphParameterLink>,
}
pub mod graph_parameter {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Type {
        String,
        Int,
        Float,
        Enumerated,
        Script,
        Mode,
        Credential,
        Boolean,
        Double,
        ColumnPicker,
        ParameterRange,
        DataGatewayName,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct GraphParameterLink {
    #[serde(rename = "nodeId")]
    pub node_id: String,
    #[serde(rename = "parameterKey")]
    pub parameter_key: String,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct WebServiceParameter {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<serde_json::Value>,
    #[serde(rename = "certificateThumbprint", skip_serializing_if = "Option::is_none")]
    pub certificate_thumbprint: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AsyncOperationStatus {
    #[serde(skip_serializing)]
    pub id: Option<String>,
    #[serde(skip_serializing)]
    pub name: Option<String>,
    #[serde(rename = "provisioningState", skip_serializing)]
    pub provisioning_state: Option<async_operation_status::ProvisioningState>,
    #[serde(rename = "startTime", skip_serializing)]
    pub start_time: Option<String>,
    #[serde(rename = "endTime", skip_serializing)]
    pub end_time: Option<String>,
    #[serde(rename = "percentComplete", skip_serializing)]
    pub percent_complete: Option<f64>,
    #[serde(rename = "errorInfo", skip_serializing_if = "Option::is_none")]
    pub error_info: Option<AsyncOperationErrorInfo>,
}
pub mod async_operation_status {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum ProvisioningState {
        Unknown,
        Provisioning,
        Succeeded,
        Failed,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AsyncOperationErrorInfo {
    #[serde(skip_serializing)]
    pub code: Option<String>,
    #[serde(skip_serializing)]
    pub target: Option<String>,
    #[serde(skip_serializing)]
    pub message: Option<String>,
    #[serde(skip_serializing)]
    pub details: Vec<AsyncOperationErrorInfo>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct OperationEntityListResult {
    #[serde(skip_serializing)]
    pub value: Vec<OperationEntity>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct OperationEntity {
    #[serde(skip_serializing)]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display: Option<OperationDisplayInfo>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct OperationDisplayInfo {
    #[serde(skip_serializing)]
    pub description: Option<String>,
    #[serde(skip_serializing)]
    pub operation: Option<String>,
    #[serde(skip_serializing)]
    pub provider: Option<String>,
    #[serde(skip_serializing)]
    pub resource: Option<String>,
}
