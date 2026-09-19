use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use thiserror::Error;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Bap {
    pub id: String,
    pub name: String,
    pub did: Option<String>,
    pub endpoint: Option<String>,
    pub public_key: Option<String>,
    pub active: Option<bool>,
    #[serde(default)]
    pub capabilities: Vec<String>,
    pub country: Option<String>,
    pub city: Option<String>,
    pub region: Option<String>,
    pub lat: Option<f64>,
    pub lon: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BapCreate {
    pub id: String,
    pub name: String,
    pub did: Option<String>,
    pub endpoint: Option<String>,
    pub public_key: Option<String>,
    pub active: Option<bool>,
    pub capabilities: Vec<String>,
    pub country: Option<String>,
    pub city: Option<String>,
    pub region: Option<String>,
    pub lat: Option<f64>,
    pub lon: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Bpp {
    pub id: String,
    pub name: String,
    pub did: Option<String>,
    pub endpoint: Option<String>,
    pub public_key: Option<String>,
    pub active: Option<bool>,
    #[serde(default)]
    pub capabilities: Vec<String>,
    pub country: Option<String>,
    pub city: Option<String>,
    pub symbol: Option<String>,
    pub currency: Option<String>,
    pub region: Option<String>,
    pub lat: Option<f64>,
    pub lon: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BppCreate {
    pub id: String,
    pub name: String,
    pub did: Option<String>,
    pub endpoint: Option<String>,
    pub public_key: Option<String>,
    pub active: Option<bool>,
    pub capabilities: Vec<String>,
    pub country: Option<String>,
    pub city: Option<String>,
    pub symbol: Option<String>,
    pub currency: Option<String>,
    pub region: Option<String>,
    pub lat: Option<f64>,
    pub lon: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Order {
    pub id: String,
    #[serde(rename = "order_state")]
    pub order_state: Option<String>,
    #[serde(rename = "provider_id")]
    pub provider_id: Option<String>,
    #[serde(rename = "bpp_id")]
    pub bpp_id: Option<String>,
    #[serde(rename = "bap_id")]
    pub bap_id: Option<String>,
    #[serde(rename = "transaction_id")]
    pub transaction_id: Option<String>,
    pub billing: Option<HashMap<String, serde_json::Value>>,
    pub fulfillments: Option<Vec<serde_json::Value>>,
    pub quote: Option<HashMap<String, serde_json::Value>>,
    pub payment: Option<HashMap<String, serde_json::Value>>,
    pub items: Option<Vec<serde_json::Value>>,
    #[serde(rename = "created_at")]
    pub created_at: Option<String>,
    #[serde(rename = "updated_at")]
    pub updated_at: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrderCreate {
    pub id: String,
    #[serde(rename = "order_state")]
    pub order_state: Option<String>,
    #[serde(rename = "provider_id")]
    pub provider_id: Option<String>,
    #[serde(rename = "bpp_id")]
    pub bpp_id: Option<String>,
    #[serde(rename = "bap_id")]
    pub bap_id: Option<String>,
    #[serde(rename = "transaction_id")]
    pub transaction_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum OrderState {
    #[serde(rename = "pending")]
    Pending,
    #[serde(rename = "accepted")]
    Accepted,
    #[serde(rename = "rejected")]
    Rejected,
    #[serde(rename = "cancelled")]
    Cancelled,
    #[serde(rename = "filled")]
    Filled,
    #[serde(rename = "completed")]
    Completed,
    #[serde(rename = "expired")]
    Expired,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Item {
    pub id: String,
    pub name: Option<String>,
    pub description: Option<String>,
    pub price: Option<Price>,
    pub category: Option<String>,
    pub item_type: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Price {
    pub currency: Option<String>,
    pub value: Option<String>,
    #[serde(rename = "listed_value")]
    pub listed_value: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Provider {
    pub id: String,
    pub name: Option<String>,
    pub description: Option<String>,
    pub locations: Option<Vec<Location>>,
    pub tags: Option<HashMap<String, String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Location {
    pub id: Option<String>,
    pub name: Option<String>,
    pub lat: Option<f64>,
    pub lng: Option<f64>,
    pub address: Option<HashMap<String, String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Fulfillment {
    pub id: String,
    #[serde(rename = "fulfillment_id")]
    pub fulfillment_id: Option<String>,
    #[serde(rename = "tracking_id")]
    pub tracking_id: Option<String>,
    #[serde(rename = "type")]
    pub fulfillment_type: Option<String>,
    pub state: Option<HashMap<String, String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FulfillmentCreate {
    pub id: String,
    #[serde(rename = "tracking_id")]
    pub tracking_id: Option<String>,
    #[serde(rename = "type")]
    pub fulfillment_type: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Subscription {
    pub id: String,
    #[serde(rename = "subscriber_id")]
    pub subscriber_id: String,
    #[serde(rename = "subscriber_type")]
    pub subscriber_type: String,
    #[serde(rename = "plan_id")]
    pub plan_id: Option<String>,
    pub status: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubscriptionCreate {
    pub id: String,
    #[serde(rename = "subscriber_id")]
    pub subscriber_id: String,
    #[serde(rename = "subscriber_type")]
    pub subscriber_type: String,
    #[serde(rename = "plan_id")]
    pub plan_id: Option<String>,
    pub activate: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiKey {
    pub id: String,
    pub description: Option<String>,
    pub owner: Option<String>,
    pub scopes: Vec<String>,
    pub active: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiKeyCreate {
    pub description: Option<String>,
    pub owner: Option<String>,
    pub scopes: Option<Vec<String>>,
    pub expires_at: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiKeyResponse {
    pub id: String,
    pub secret: Option<String>,
    pub owner: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiKeyVerifyResponse {
    pub valid: bool,
    pub owner: Option<String>,
    pub scopes: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ErrorResponse {
    pub message: Option<String>,
    pub code: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeoDnsResult {
    pub hostname: String,
    pub endpoints: Vec<String>,
    pub country: String,
    pub city: String,
    pub region: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeoDnsDiscoveryOptions {
    pub country: Option<String>,
    pub city: Option<String>,
    pub limit: Option<u32>,
    pub radius_km: Option<u32>,
}

// A2A Types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentCard {
    pub agent_id: String,
    pub name: String,
    pub url: String,
    pub description: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentCardRegister {
    pub agent_id: String,
    pub name: String,
    pub url: String,
    pub description: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct A2ATask {
    pub id: String,
    pub state: String,
    pub message: Option<Message>,
    pub artifacts: Option<Vec<Artifact>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct A2ATaskCreate {
    pub id: String,
    pub message: Message,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct A2AArtifact {
    pub id: String,
    pub name: Option<String>,
    pub description: Option<String>,
    pub content: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Artifact {
    #[serde(rename = "text")]
    Text(TextArtifact),
    #[serde(rename = "data")]
    Data(DataArtifact),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TextArtifact {
    pub text: String,
    pub description: Option<String>,
    pub content_type: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DataArtifact {
    pub data: serde_json::Value,
    pub description: Option<String>,
    pub content_type: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Message {
    pub id: Option<String>,
    pub role: String,
    pub content: Vec<MessageContent>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MessageContent {
    #[serde(rename = "text")]
    Text(TextContent),
    #[serde(rename = "data")]
    Data(DataContent),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TextContent {
    pub text: String,
    pub content_type: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DataContent {
    pub data: serde_json::Value,
    pub content_type: Option<String>,
}

// MCP Types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct McpTool {
    pub name: String,
    pub description: Option<String>,
    pub handler: Option<String>,
    pub input_schema: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct McpToolCreate {
    pub name: String,
    pub handler: Option<String>,
    pub description: Option<String>,
    pub input_schema: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct McpResource {
    pub id: String,
    pub uri: String,
    pub name: Option<String>,
    pub description: Option<String>,
    pub mime_type: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct McpResourceCreate {
    pub uri: String,
    pub name: Option<String>,
    pub description: Option<String>,
    pub mime_type: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct McpPrompt {
    pub name: String,
    pub description: Option<String>,
    pub arguments: Option<Vec<McpPromptArgument>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct McpPromptCreate {
    pub name: String,
    pub description: Option<String>,
    pub arguments: Option<Vec<McpPromptArgument>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct McpPromptArgument {
    pub name: String,
    pub description: Option<String>,
    pub required: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct McpClientInfo {
    pub id: String,
    pub name: String,
    pub version: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct McpClientRegister {
    pub id: String,
    pub name: String,
    pub version: String,
}

// ACP Types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AcpIssuer {
    #[serde(rename = "issuer_id")]
    pub issuer_id: String,
    pub name: Option<String>,
    pub jwks_uri: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AcpIssuerRegister {
    #[serde(rename = "issuer_id")]
    pub issuer_id: String,
    pub name: Option<String>,
    pub jwks_uri: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AcpToken {
    #[serde(rename = "token_value")]
    pub token_value: String,
    #[serde(rename = "token_type")]
    pub token_type: Option<String>,
    #[serde(rename = "expires_in")]
    pub expires_in: Option<u64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AcpTokenIssue {
    #[serde(rename = "token_value")]
    pub token_value: String,
    #[serde(rename = "token_type")]
    pub token_type: Option<String>,
    pub scope: Option<String>,
    #[serde(rename = "expires_at")]
    pub expires_at: Option<String>,
    #[serde(rename = "credential_issuer_id")]
    pub credential_issuer_id: Option<String>,
    pub subject: Option<String>,
    pub audience: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AcpPresentation {
    pub id: String,
    #[serde(rename = "presentation_id")]
    pub presentation_id: Option<String>,
    #[serde(rename = "token_type")]
    pub token_type: Option<String>,
    pub credential: Option<serde_json::Value>,
    pub holder: Option<String>,
    pub issued_at: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AcpPresentationSubmit {
    #[serde(rename = "presentation_id")]
    pub presentation_id: Option<String>,
    pub credential: serde_json::Value,
    pub holder: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AcpAccessPolicy {
    pub id: String,
    pub name: Option<String>,
    pub description: Option<String>,
    pub rules: Option<Vec<serde_json::Value>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AcpAccessPolicyCreate {
    pub name: String,
    pub description: Option<String>,
    pub rules: Option<Vec<serde_json::Value>>,
}

// ANP Types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnpAnnouncement {
    #[serde(rename = "announcement_id")]
    pub announcement_id: String,
    pub did: String,
    #[serde(rename = "did_document_hash")]
    pub did_document_hash: Option<String>,
    pub service_endpoint: Option<String>,
    pub timestamp: Option<String>,
    pub ttl: Option<u64>,
    pub proof: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnpAnnouncementCreate {
    #[serde(rename = "announcement_id")]
    pub announcement_id: String,
    pub did: String,
    #[serde(rename = "did_document_hash")]
    pub did_document_hash: Option<String>,
    pub service_endpoint: Option<String>,
    pub timestamp: Option<String>,
    pub ttl: Option<u64>,
    pub proof: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnpWitness {
    pub id: String,
    #[serde(rename = "did")]
    pub did: String,
    #[serde(rename = "service_endpoint")]
    pub service_endpoint: Option<String>,
    #[serde(rename = "registered_at")]
    pub registered_at: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnpWitnessRegister {
    #[serde(rename = "did")]
    pub did: String,
    #[serde(rename = "service_endpoint")]
    pub service_endpoint: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AnpVerificationStatus {
    #[serde(rename = "verified")]
    Verified,
    #[serde(rename = "revoked")]
    Revoked,
    #[serde(rename = "expired")]
    Expired,
    #[serde(rename = "unknown")]
    Unknown,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnpVerification {
    pub id: String,
    #[serde(rename = "announcement_id")]
    pub announcement_id: Option<String>,
    pub did: Option<String>,
    pub status: Option<AnpVerificationStatus>,
    #[serde(rename = "verified_at")]
    pub verified_at: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnpVerificationVerify {
    #[serde(rename = "announcement_id")]
    pub announcement_id: Option<String>,
    pub did: Option<String>,
}

#[derive(Error, Debug)]
pub enum BeckNError {
    #[error("HTTP error: {status}")]
    Http { status: u16, message: String },
    #[error("Request error: {0}")]
    Request(#[from] reqwest::Error),
    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),
    #[error("Other error: {0}")]
    Other(String),
}

#[derive(Debug, Clone)]
pub struct ClientConfig {
    pub base_url: String,
    pub api_key: Option<String>,
    pub timeout_seconds: u64,
}

impl Default for ClientConfig {
    fn default() -> Self {
        Self {
            base_url: "http://localhost:4000/v1".to_string(),
            api_key: std::env::var("BECKN_API_KEY").ok(),
            timeout_seconds: 30,
        }
    }
}

impl ClientConfig {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn with_api_key(mut self, key: impl Into<String>) -> Self {
        self.api_key = Some(key.into());
        self
    }
    pub fn with_base_url(mut self, url: impl Into<String>) -> Self {
        self.base_url = url.into();
        self
    }
    pub fn with_timeout(mut self, seconds: u64) -> Self {
        self.timeout_seconds = seconds;
        self
    }
}
