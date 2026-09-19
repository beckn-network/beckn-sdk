use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum OrderState {
    Pending,
    Accepted,
    #[serde(rename = "in_fulfillment")]
    InFulfillment,
    Completed,
    Cancelled,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SubscriptionStatus {
    Trial,
    Active,
    #[serde(rename = "past_due")]
    PastDue,
    #[serde(rename = "canceled")]
    Canceled,
    Unpaid,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SubscriberType {
    Bap,
    Bpp,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TaskStatus {
    Submitted,
    Working,
    #[serde(rename = "input_required")]
    InputRequired,
    Completed,
    Failed,
    #[serde(rename = "canceled")]
    Canceled,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum TransportType {
    Stdio,
    StreamableHttp,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum MessageRole {
    User,
    Agent,
    System,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Bap {
    pub id: String,
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub did: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub endpoint: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub public_key: Option<String>,
    #[serde(default = "default_true")]
    pub active: bool,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub capabilities: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub city: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BapCreate {
    pub id: String,
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub did: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub endpoint: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub public_key: Option<String>,
    #[serde(default = "default_true")]
    pub active: bool,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub capabilities: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BapUpdate {
    pub name: Option<String>,
    pub endpoint: Option<String>,
    pub active: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Bpp {
    pub id: String,
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub did: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub endpoint: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub public_key: Option<String>,
    #[serde(default = "default_true")]
    pub active: bool,
    #[serde(default)]
    pub capabilities: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub symbol: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Order {
    pub id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_state: Option<OrderState>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provider_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bpp_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bap_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transaction_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrderCreate {
    pub id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_state: Option<OrderState>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provider_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bpp_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bap_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transaction_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Item {
    pub id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provider_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub descriptor: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Provider {
    pub id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub descriptor: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Fulfillment {
    pub id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fulfillment_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tracking_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Subscription {
    pub id: String,
    pub subscriber_id: String,
    pub subscriber_type: SubscriberType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub plan_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<SubscriptionStatus>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub current_period_end: Option<String>,
    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub cancel_at_period_end: bool,
    #[serde(default)]
    pub metadata: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiKey {
    pub id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owner: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub scopes: Vec<String>,
    #[serde(default = "default_true")]
    pub active: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expires_at: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiKeyResponse {
    pub id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secret: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owner: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiKeyVerifyResponse {
    pub valid: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owner: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scopes: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ErrorResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeoLocation {
    pub lat: f64,
    pub lng: f64,
    pub country: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub city: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeoDnsResult {
    pub hostname: String,
    pub endpoints: Vec<String>,
    pub location: GeoLocation,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub latency_ms: Option<u64>,
}

// A2A types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentCard {
    pub agent_id: String,
    pub name: String,
    pub url: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentCardRegister {
    pub agent_id: String,
    pub name: String,
    pub url: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct A2ATask {
    pub task_id: String,
    pub context_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<TaskStatus>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Message {
    pub message_id: String,
    pub role: MessageRole,
    pub parts: Vec<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub context_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Artifact {
    pub artifact_id: String,
    pub task_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

// MCP types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct McpTool {
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_schema: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub handler: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct McpToolCreate {
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub handler: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct McpResource {
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uri: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mime_type: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct McpResourceCreate {
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uri: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct McpPrompt {
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct McpClientInfo {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_info: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub capabilities: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transport: Option<TransportType>,
}

// ACP types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AcpIssuer {
    pub issuer_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authorization_endpoint: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub token_endpoint: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AcpIssuerRegister {
    pub issuer_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authorization_endpoint: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AcpToken {
    #[serde(rename = "token_value")]
    pub token: String,
    #[serde(default = "default_bearer")]
    pub token_type: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AcpTokenIssue {
    pub token: String,
    #[serde(default = "default_bearer")]
    pub token_type: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AcpPresentation {
    pub presentation_id: String,
    pub holder_did: String,
    pub issuer_id: String,
    pub claims: serde_json::Value,
    #[serde(default)]
    pub verified: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AcpPresentationSubmit {
    pub presentation_id: String,
    pub holder_did: String,
    pub issuer_id: String,
    pub claims: serde_json::Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AcpAccessPolicy {
    pub policy_id: String,
    pub action: String,
    pub resource: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scope_required: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AcpAccessPolicyCreate {
    pub policy_id: String,
    pub action: String,
    pub resource: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scope_required: Option<String>,
}

// ANP types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnpAnnouncement {
    pub announcement_id: String,
    pub did: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_endpoint: Option<String>,
    #[serde(default = "default_ttl")]
    pub ttl: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnpAnnouncementCreate {
    pub announcement_id: String,
    pub did: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_endpoint: Option<String>,
    #[serde(default = "default_ttl")]
    pub ttl: u64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proof: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnpWitness {
    pub witness_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub did: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub endpoint: Option<String>,
    #[serde(default)]
    pub protocols: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnpWitnessRegister {
    pub witness_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub did: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub endpoint: Option<String>,
    #[serde(default)]
    pub protocols: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnpVerification {
    pub proof_id: String,
    pub did: String,
    pub verification_method: String,
    #[serde(default)]
    pub valid: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnpVerificationVerify {
    pub proof_id: String,
    pub did: String,
    pub verification_method: String,
}

fn default_true() -> bool { true }
fn default_bearer() -> String { "Bearer".into() }
fn default_ttl() -> u64 { 3600 }

#[derive(Debug, Error)]
pub enum BeckNError {
    #[error("HTTP error {status}: {message}")]
    Http { status: u16, message: String },
    #[error(transparent)]
    Reqwest(#[from] reqwest::Error),
    #[error(transparent)]
    Json(#[from] serde_json::Error),
    #[error("{0}")]
    Other(String),
}

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
    pub fn new() -> Self { Self::default() }
    pub fn base_url(mut self, url: impl Into<String>) -> Self { self.base_url = url.into(); self }
    pub fn api_key(mut self, key: impl Into<String>) -> Self { self.api_key = Some(key.into()); self }
    pub fn timeout(mut self, secs: u64) -> Self { self.timeout_seconds = secs; self }
}
