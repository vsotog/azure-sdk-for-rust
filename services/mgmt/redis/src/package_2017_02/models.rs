#![doc = "generated by AutoRust 0.1.0"]
#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::{Deserialize, Serialize};
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Sku {
    pub name: sku::Name,
    pub family: sku::Family,
    pub capacity: i32,
}
pub mod sku {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Name {
        Basic,
        Standard,
        Premium,
    }
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Family {
        C,
        P,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RedisProperties {
    #[serde(rename = "redisConfiguration", skip_serializing_if = "Option::is_none")]
    pub redis_configuration: Option<serde_json::Value>,
    #[serde(rename = "enableNonSslPort", skip_serializing_if = "Option::is_none")]
    pub enable_non_ssl_port: Option<bool>,
    #[serde(rename = "tenantSettings", skip_serializing_if = "Option::is_none")]
    pub tenant_settings: Option<serde_json::Value>,
    #[serde(rename = "shardCount", skip_serializing_if = "Option::is_none")]
    pub shard_count: Option<i32>,
    #[serde(rename = "subnetId", skip_serializing_if = "Option::is_none")]
    pub subnet_id: Option<String>,
    #[serde(rename = "staticIP", skip_serializing_if = "Option::is_none")]
    pub static_ip: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RedisCreateProperties {
    #[serde(flatten)]
    pub redis_properties: RedisProperties,
    pub sku: Sku,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RedisUpdateProperties {
    #[serde(flatten)]
    pub redis_properties: RedisProperties,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sku: Option<Sku>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Resource {
    #[serde(skip_serializing)]
    pub id: Option<String>,
    #[serde(skip_serializing)]
    pub name: Option<String>,
    #[serde(rename = "type", skip_serializing)]
    pub type_: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ProxyResource {
    #[serde(flatten)]
    pub resource: Resource,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TrackedResource {
    #[serde(flatten)]
    pub resource: Resource,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
    pub location: String,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RedisCreateParameters {
    #[serde(flatten)]
    pub tracked_resource: TrackedResource,
    pub properties: RedisCreateProperties,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RedisUpdateParameters {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<RedisUpdateProperties>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RedisAccessKeys {
    #[serde(rename = "primaryKey", skip_serializing)]
    pub primary_key: Option<String>,
    #[serde(rename = "secondaryKey", skip_serializing)]
    pub secondary_key: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RedisFirewallRule {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    pub properties: RedisFirewallRuleProperties,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RedisFirewallRuleProperties {
    #[serde(rename = "startIP")]
    pub start_ip: String,
    #[serde(rename = "endIP")]
    pub end_ip: String,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RedisFirewallRuleListResult {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<RedisFirewallRule>,
    #[serde(rename = "nextLink", skip_serializing)]
    pub next_link: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RedisResourceProperties {
    #[serde(flatten)]
    pub redis_properties: RedisProperties,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sku: Option<Sku>,
    #[serde(rename = "redisVersion", skip_serializing)]
    pub redis_version: Option<String>,
    #[serde(rename = "provisioningState", skip_serializing)]
    pub provisioning_state: Option<String>,
    #[serde(rename = "hostName", skip_serializing)]
    pub host_name: Option<String>,
    #[serde(skip_serializing)]
    pub port: Option<i32>,
    #[serde(rename = "sslPort", skip_serializing)]
    pub ssl_port: Option<i32>,
    #[serde(rename = "accessKeys", skip_serializing_if = "Option::is_none")]
    pub access_keys: Option<RedisAccessKeys>,
    #[serde(rename = "linkedServers", skip_serializing_if = "Option::is_none")]
    pub linked_servers: Option<RedisLinkedServerList>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RedisResource {
    #[serde(flatten)]
    pub tracked_resource: TrackedResource,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<RedisResourceProperties>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RedisListResult {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<RedisResource>,
    #[serde(rename = "nextLink", skip_serializing)]
    pub next_link: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RedisRegenerateKeyParameters {
    #[serde(rename = "keyType")]
    pub key_type: redis_regenerate_key_parameters::KeyType,
}
pub mod redis_regenerate_key_parameters {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum KeyType {
        Primary,
        Secondary,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RedisRebootParameters {
    #[serde(rename = "rebootType")]
    pub reboot_type: redis_reboot_parameters::RebootType,
    #[serde(rename = "shardId", skip_serializing_if = "Option::is_none")]
    pub shard_id: Option<i32>,
}
pub mod redis_reboot_parameters {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum RebootType {
        PrimaryNode,
        SecondaryNode,
        AllNodes,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ExportRdbParameters {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub format: Option<String>,
    pub prefix: String,
    pub container: String,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ImportRdbParameters {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub format: Option<String>,
    pub files: Vec<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ScheduleEntry {
    #[serde(rename = "dayOfWeek")]
    pub day_of_week: schedule_entry::DayOfWeek,
    #[serde(rename = "startHourUtc")]
    pub start_hour_utc: i32,
    #[serde(rename = "maintenanceWindow", skip_serializing_if = "Option::is_none")]
    pub maintenance_window: Option<String>,
}
pub mod schedule_entry {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum DayOfWeek {
        Monday,
        Tuesday,
        Wednesday,
        Thursday,
        Friday,
        Saturday,
        Sunday,
        Everyday,
        Weekend,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ScheduleEntries {
    #[serde(rename = "scheduleEntries")]
    pub schedule_entries: Vec<ScheduleEntry>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RedisPatchSchedule {
    #[serde(skip_serializing)]
    pub id: Option<String>,
    #[serde(skip_serializing)]
    pub name: Option<String>,
    #[serde(rename = "type", skip_serializing)]
    pub type_: Option<String>,
    #[serde(skip_serializing)]
    pub location: Option<String>,
    pub properties: ScheduleEntries,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RedisForceRebootResponse {
    #[serde(rename = "Message", skip_serializing)]
    pub message: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RedisLinkedServer {
    #[serde(skip_serializing)]
    pub id: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RedisLinkedServerWithProperties {
    #[serde(skip_serializing)]
    pub id: Option<String>,
    #[serde(skip_serializing)]
    pub name: Option<String>,
    #[serde(rename = "type", skip_serializing)]
    pub type_: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<RedisLinkedServerProperties>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RedisLinkedServerProperties {
    #[serde(flatten)]
    pub redis_linked_server_create_properties: RedisLinkedServerCreateProperties,
    #[serde(rename = "provisioningState", skip_serializing)]
    pub provisioning_state: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RedisLinkedServerList {
    pub value: Vec<RedisLinkedServer>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RedisLinkedServerWithPropertiesList {
    pub value: Vec<RedisLinkedServerWithProperties>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RedisLinkedServerCreateProperties {
    #[serde(rename = "linkedRedisCacheId")]
    pub linked_redis_cache_id: String,
    #[serde(rename = "linkedRedisCacheLocation")]
    pub linked_redis_cache_location: String,
    #[serde(rename = "serverRole")]
    pub server_role: redis_linked_server_create_properties::ServerRole,
}
pub mod redis_linked_server_create_properties {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum ServerRole {
        Primary,
        Secondary,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RedisLinkedServerCreateParameters {
    pub properties: RedisLinkedServerCreateProperties,
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
        pub operation: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub resource: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub description: Option<String>,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct OperationListResult {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<Operation>,
    #[serde(rename = "nextLink", skip_serializing)]
    pub next_link: Option<String>,
}
