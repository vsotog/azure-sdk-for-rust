#![doc = "generated by AutoRust 0.1.0"]
#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::{Deserialize, Serialize};
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AzureResource {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub identity: Option<String>,
    pub location: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TestNotificationResponse {
    #[serde(rename = "notificationId")]
    pub notification_id: String,
    #[serde(rename = "correlationId")]
    pub correlation_id: String,
    #[serde(rename = "createdTime")]
    pub created_time: String,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TestNotificationDetailsResponse {
    #[serde(rename = "Context", default, skip_serializing_if = "Option::is_none")]
    pub context: Option<Context>,
    #[serde(rename = "State")]
    pub state: String,
    #[serde(rename = "CompletedTime", default, skip_serializing_if = "Option::is_none")]
    pub completed_time: Option<String>,
    #[serde(rename = "CreatedTime", default, skip_serializing_if = "Option::is_none")]
    pub created_time: Option<String>,
    #[serde(rename = "ActionDetails", default, skip_serializing_if = "Vec::is_empty")]
    pub action_details: Vec<ActionDetail>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ActionDetail {
    #[serde(rename = "MechanismType", default, skip_serializing_if = "Option::is_none")]
    pub mechanism_type: Option<String>,
    #[serde(rename = "Name", default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "Status", default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "SubState", default, skip_serializing_if = "Option::is_none")]
    pub sub_state: Option<String>,
    #[serde(rename = "SendTime", default, skip_serializing_if = "Option::is_none")]
    pub send_time: Option<String>,
    #[serde(rename = "Detail", default, skip_serializing_if = "Option::is_none")]
    pub detail: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Context {
    #[serde(rename = "NotificationSource", default, skip_serializing_if = "Option::is_none")]
    pub notification_source: Option<String>,
    #[serde(rename = "ContextType", default, skip_serializing_if = "Option::is_none")]
    pub context_type: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ActionGroupResource {
    #[serde(flatten)]
    pub azure_resource: AzureResource,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<ActionGroup>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ActionGroupList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<ActionGroupResource>,
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct NotificationRequestBody {
    #[serde(rename = "alertType")]
    pub alert_type: String,
    #[serde(rename = "emailReceivers", default, skip_serializing_if = "Vec::is_empty")]
    pub email_receivers: Vec<EmailReceiver>,
    #[serde(rename = "smsReceivers", default, skip_serializing_if = "Vec::is_empty")]
    pub sms_receivers: Vec<SmsReceiver>,
    #[serde(rename = "webhookReceivers", default, skip_serializing_if = "Vec::is_empty")]
    pub webhook_receivers: Vec<WebhookReceiver>,
    #[serde(rename = "itsmReceivers", default, skip_serializing_if = "Vec::is_empty")]
    pub itsm_receivers: Vec<ItsmReceiver>,
    #[serde(rename = "azureAppPushReceivers", default, skip_serializing_if = "Vec::is_empty")]
    pub azure_app_push_receivers: Vec<AzureAppPushReceiver>,
    #[serde(rename = "automationRunbookReceivers", default, skip_serializing_if = "Vec::is_empty")]
    pub automation_runbook_receivers: Vec<AutomationRunbookReceiver>,
    #[serde(rename = "voiceReceivers", default, skip_serializing_if = "Vec::is_empty")]
    pub voice_receivers: Vec<VoiceReceiver>,
    #[serde(rename = "logicAppReceivers", default, skip_serializing_if = "Vec::is_empty")]
    pub logic_app_receivers: Vec<LogicAppReceiver>,
    #[serde(rename = "azureFunctionReceivers", default, skip_serializing_if = "Vec::is_empty")]
    pub azure_function_receivers: Vec<AzureFunctionReceiver>,
    #[serde(rename = "armRoleReceivers", default, skip_serializing_if = "Vec::is_empty")]
    pub arm_role_receivers: Vec<ArmRoleReceiver>,
    #[serde(rename = "eventHubReceivers", default, skip_serializing_if = "Vec::is_empty")]
    pub event_hub_receivers: Vec<EventHubReceiver>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ActionGroup {
    #[serde(rename = "groupShortName")]
    pub group_short_name: String,
    pub enabled: bool,
    #[serde(rename = "emailReceivers", default, skip_serializing_if = "Vec::is_empty")]
    pub email_receivers: Vec<EmailReceiver>,
    #[serde(rename = "smsReceivers", default, skip_serializing_if = "Vec::is_empty")]
    pub sms_receivers: Vec<SmsReceiver>,
    #[serde(rename = "webhookReceivers", default, skip_serializing_if = "Vec::is_empty")]
    pub webhook_receivers: Vec<WebhookReceiver>,
    #[serde(rename = "itsmReceivers", default, skip_serializing_if = "Vec::is_empty")]
    pub itsm_receivers: Vec<ItsmReceiver>,
    #[serde(rename = "azureAppPushReceivers", default, skip_serializing_if = "Vec::is_empty")]
    pub azure_app_push_receivers: Vec<AzureAppPushReceiver>,
    #[serde(rename = "automationRunbookReceivers", default, skip_serializing_if = "Vec::is_empty")]
    pub automation_runbook_receivers: Vec<AutomationRunbookReceiver>,
    #[serde(rename = "voiceReceivers", default, skip_serializing_if = "Vec::is_empty")]
    pub voice_receivers: Vec<VoiceReceiver>,
    #[serde(rename = "logicAppReceivers", default, skip_serializing_if = "Vec::is_empty")]
    pub logic_app_receivers: Vec<LogicAppReceiver>,
    #[serde(rename = "azureFunctionReceivers", default, skip_serializing_if = "Vec::is_empty")]
    pub azure_function_receivers: Vec<AzureFunctionReceiver>,
    #[serde(rename = "armRoleReceivers", default, skip_serializing_if = "Vec::is_empty")]
    pub arm_role_receivers: Vec<ArmRoleReceiver>,
    #[serde(rename = "eventHubReceivers", default, skip_serializing_if = "Vec::is_empty")]
    pub event_hub_receivers: Vec<EventHubReceiver>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EmailReceiver {
    pub name: String,
    #[serde(rename = "emailAddress")]
    pub email_address: String,
    #[serde(rename = "useCommonAlertSchema", default, skip_serializing_if = "Option::is_none")]
    pub use_common_alert_schema: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<ReceiverStatus>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SmsReceiver {
    pub name: String,
    #[serde(rename = "countryCode")]
    pub country_code: String,
    #[serde(rename = "phoneNumber")]
    pub phone_number: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<ReceiverStatus>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct WebhookReceiver {
    pub name: String,
    #[serde(rename = "serviceUri")]
    pub service_uri: String,
    #[serde(rename = "useCommonAlertSchema", default, skip_serializing_if = "Option::is_none")]
    pub use_common_alert_schema: Option<bool>,
    #[serde(rename = "useAadAuth", default, skip_serializing_if = "Option::is_none")]
    pub use_aad_auth: Option<bool>,
    #[serde(rename = "objectId", default, skip_serializing_if = "Option::is_none")]
    pub object_id: Option<String>,
    #[serde(rename = "identifierUri", default, skip_serializing_if = "Option::is_none")]
    pub identifier_uri: Option<String>,
    #[serde(rename = "tenantId", default, skip_serializing_if = "Option::is_none")]
    pub tenant_id: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ItsmReceiver {
    pub name: String,
    #[serde(rename = "workspaceId")]
    pub workspace_id: String,
    #[serde(rename = "connectionId")]
    pub connection_id: String,
    #[serde(rename = "ticketConfiguration")]
    pub ticket_configuration: String,
    pub region: String,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EventHubReceiver {
    pub name: String,
    #[serde(rename = "eventHubNameSpace")]
    pub event_hub_name_space: String,
    #[serde(rename = "eventHubName")]
    pub event_hub_name: String,
    #[serde(rename = "useCommonAlertSchema", default, skip_serializing_if = "Option::is_none")]
    pub use_common_alert_schema: Option<bool>,
    #[serde(rename = "tenantId", default, skip_serializing_if = "Option::is_none")]
    pub tenant_id: Option<String>,
    #[serde(rename = "subscriptionId")]
    pub subscription_id: String,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AzureAppPushReceiver {
    pub name: String,
    #[serde(rename = "emailAddress")]
    pub email_address: String,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AutomationRunbookReceiver {
    #[serde(rename = "automationAccountId")]
    pub automation_account_id: String,
    #[serde(rename = "runbookName")]
    pub runbook_name: String,
    #[serde(rename = "webhookResourceId")]
    pub webhook_resource_id: String,
    #[serde(rename = "isGlobalRunbook")]
    pub is_global_runbook: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "serviceUri", default, skip_serializing_if = "Option::is_none")]
    pub service_uri: Option<String>,
    #[serde(rename = "useCommonAlertSchema", default, skip_serializing_if = "Option::is_none")]
    pub use_common_alert_schema: Option<bool>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct VoiceReceiver {
    pub name: String,
    #[serde(rename = "countryCode")]
    pub country_code: String,
    #[serde(rename = "phoneNumber")]
    pub phone_number: String,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LogicAppReceiver {
    pub name: String,
    #[serde(rename = "resourceId")]
    pub resource_id: String,
    #[serde(rename = "callbackUrl")]
    pub callback_url: String,
    #[serde(rename = "useCommonAlertSchema", default, skip_serializing_if = "Option::is_none")]
    pub use_common_alert_schema: Option<bool>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AzureFunctionReceiver {
    pub name: String,
    #[serde(rename = "functionAppResourceId")]
    pub function_app_resource_id: String,
    #[serde(rename = "functionName")]
    pub function_name: String,
    #[serde(rename = "httpTriggerUrl")]
    pub http_trigger_url: String,
    #[serde(rename = "useCommonAlertSchema", default, skip_serializing_if = "Option::is_none")]
    pub use_common_alert_schema: Option<bool>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ArmRoleReceiver {
    pub name: String,
    #[serde(rename = "roleId")]
    pub role_id: String,
    #[serde(rename = "useCommonAlertSchema", default, skip_serializing_if = "Option::is_none")]
    pub use_common_alert_schema: Option<bool>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum ReceiverStatus {
    NotSpecified,
    Enabled,
    Disabled,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EnableRequest {
    #[serde(rename = "receiverName")]
    pub receiver_name: String,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ErrorResponse {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ActionGroupPatchBody {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<ActionGroupPatch>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ActionGroupPatch {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
}