#![doc = "generated by AutoRust 0.1.0"]
#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::{Deserialize, Serialize};
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SkuListResult {
    #[serde(skip_serializing)]
    pub value: Vec<CatalogSku>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CatalogSku {
    #[serde(rename = "resourceType", skip_serializing)]
    pub resource_type: Option<String>,
    #[serde(skip_serializing)]
    pub name: Option<String>,
    #[serde(skip_serializing)]
    pub tier: Option<String>,
    #[serde(skip_serializing)]
    pub locations: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub capacity: Option<SkuCapacity>,
    #[serde(skip_serializing)]
    pub capabilities: Vec<SkuCapability>,
    #[serde(skip_serializing)]
    pub costs: Vec<SkuCost>,
    #[serde(skip_serializing)]
    pub restrictions: Vec<SkuRestrictions>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SkuCapacity {
    #[serde(skip_serializing)]
    pub minimum: Option<i64>,
    #[serde(skip_serializing)]
    pub maximum: Option<i64>,
    #[serde(skip_serializing)]
    pub default: Option<i64>,
    #[serde(rename = "scaleType", skip_serializing)]
    pub scale_type: Option<sku_capacity::ScaleType>,
}
pub mod sku_capacity {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum ScaleType {
        Automatic,
        Manual,
        None,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SkuCost {
    #[serde(rename = "meterID", skip_serializing)]
    pub meter_id: Option<String>,
    #[serde(skip_serializing)]
    pub quantity: Option<i64>,
    #[serde(rename = "extendedUnit", skip_serializing)]
    pub extended_unit: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SkuCapability {
    #[serde(skip_serializing)]
    pub name: Option<String>,
    #[serde(skip_serializing)]
    pub value: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SkuRestrictions {
    #[serde(rename = "type", skip_serializing)]
    pub type_: Option<sku_restrictions::Type>,
    #[serde(skip_serializing)]
    pub values: Vec<String>,
    #[serde(rename = "reasonCode", skip_serializing)]
    pub reason_code: Option<sku_restrictions::ReasonCode>,
}
pub mod sku_restrictions {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Type {
        #[serde(rename = "location")]
        Location,
        #[serde(rename = "zone")]
        Zone,
    }
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum ReasonCode {
        QuotaId,
        NotAvailableForSubscription,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Tags {}
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
    pub tags: Option<Tags>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CommitmentAssociation {
    #[serde(flatten)]
    pub resource: Resource,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub etag: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<CommitmentAssociationProperties>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CommitmentAssociationProperties {
    #[serde(rename = "associatedResourceId", skip_serializing)]
    pub associated_resource_id: Option<String>,
    #[serde(rename = "commitmentPlanId", skip_serializing)]
    pub commitment_plan_id: Option<String>,
    #[serde(rename = "creationDate", skip_serializing)]
    pub creation_date: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ResourceSku {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub capacity: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tier: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CommitmentAssociationListResult {
    #[serde(rename = "nextLink", skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<CommitmentAssociation>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MoveCommitmentAssociationRequest {
    #[serde(rename = "destinationPlanId", skip_serializing_if = "Option::is_none")]
    pub destination_plan_id: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CommitmentPlanPatchPayload {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Tags>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sku: Option<ResourceSku>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CommitmentPlan {
    #[serde(flatten)]
    pub resource: Resource,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub etag: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<CommitmentPlanProperties>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sku: Option<ResourceSku>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CommitmentPlanProperties {
    #[serde(rename = "chargeForOverage", skip_serializing)]
    pub charge_for_overage: Option<bool>,
    #[serde(rename = "chargeForPlan", skip_serializing)]
    pub charge_for_plan: Option<bool>,
    #[serde(rename = "creationDate", skip_serializing)]
    pub creation_date: Option<String>,
    #[serde(rename = "includedQuantities", skip_serializing)]
    pub included_quantities: Option<serde_json::Value>,
    #[serde(rename = "maxAssociationLimit", skip_serializing)]
    pub max_association_limit: Option<i32>,
    #[serde(rename = "maxCapacityLimit", skip_serializing)]
    pub max_capacity_limit: Option<i32>,
    #[serde(rename = "minCapacityLimit", skip_serializing)]
    pub min_capacity_limit: Option<i32>,
    #[serde(rename = "planMeter", skip_serializing)]
    pub plan_meter: Option<String>,
    #[serde(rename = "refillFrequencyInDays", skip_serializing)]
    pub refill_frequency_in_days: Option<i32>,
    #[serde(rename = "suspendPlanOnOverage", skip_serializing)]
    pub suspend_plan_on_overage: Option<bool>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PlanQuantity {
    #[serde(skip_serializing)]
    pub allowance: Option<f64>,
    #[serde(skip_serializing)]
    pub amount: Option<f64>,
    #[serde(rename = "includedQuantityMeter", skip_serializing)]
    pub included_quantity_meter: Option<String>,
    #[serde(rename = "overageMeter", skip_serializing)]
    pub overage_meter: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CommitmentPlanListResult {
    #[serde(rename = "nextLink", skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<CommitmentPlan>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PlanUsageHistoryListResult {
    #[serde(rename = "nextLink", skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<PlanUsageHistory>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PlanUsageHistory {
    #[serde(rename = "planDeletionOverage", skip_serializing_if = "Option::is_none")]
    pub plan_deletion_overage: Option<serde_json::Value>,
    #[serde(rename = "planMigrationOverage", skip_serializing_if = "Option::is_none")]
    pub plan_migration_overage: Option<serde_json::Value>,
    #[serde(rename = "planQuantitiesAfterUsage", skip_serializing_if = "Option::is_none")]
    pub plan_quantities_after_usage: Option<serde_json::Value>,
    #[serde(rename = "planQuantitiesBeforeUsage", skip_serializing_if = "Option::is_none")]
    pub plan_quantities_before_usage: Option<serde_json::Value>,
    #[serde(rename = "planUsageOverage", skip_serializing_if = "Option::is_none")]
    pub plan_usage_overage: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub usage: Option<serde_json::Value>,
    #[serde(rename = "usageDate", skip_serializing_if = "Option::is_none")]
    pub usage_date: Option<String>,
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
