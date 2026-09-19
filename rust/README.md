# BeckN Protocol SDK — Rust

Rust SDK for the BeckN Protocol — a unified agentic discovery and commerce protocol API.

Supports: BeckN Core, GeoDNS, A2A (Agent-to-Agent), MCP (Model Context Protocol), ANP (Agent Network Protocol), ACP (Agent Credential Protocol), and GBP (Google Business Profile) sync.

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
beckn-sdk = "1.0.0"
```

## Quick Start

```rust
use beckn_sdk::{BeckNClient, ClientConfig};

let client = BeckNClient::new(ClientConfig::default()
    .with_base_url("https://api.beckn.network/v1")
    .with_api_key("bk_your_api_key_here"));

// Check health
let health = client.health_check().await?;

// Create an order
let order = client.create_order(&OrderCreate {
    id: "order-001".to_string(),
    bap_id: "my-bap-001".to_string(),
    bpp_id: "my-bpp-001".to_string(),
    ..Default::default()
}).await?;

// Discover nearby BPPs (GeoDNS)
let nearby = client.discover_marketplace(40.7128, -74.0060, 50, 10).await?;

// Register an agent card (A2A)
let agent = client.register_agent_card(&AgentCardRegister {
    agent_id: "agent-001".to_string(),
    name: "Travel Booking Agent".to_string(),
    url: "https://travel.beckn.network".to_string(),
    ..Default::default()
}).await?;
```

## API Reference

### ClientConfig

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `base_url` | `String` | `"https://api.beckn.network/v1"` | API base URL |
| `api_key` | `Option<String>` | `None` | API key for authentication |
| `timeout_seconds` | `u64` | `30` | Request timeout in seconds |

### Methods

#### Health
- `health_check() -> Result<HealthCheck, BeckNError>`

#### Orders
- `create_order(data: &OrderCreate) -> Result<Order, BeckNError>`
- `get_order(id: &str) -> Result<Order, BeckNError>`
- `list_orders() -> Result<Vec<Order>, BeckNError>`
- `update_order(id: &str, data: &Order) -> Result<Order, BeckNError>`
- `delete_order(id: &str) -> Result<(), BeckNError>`

#### BAPs & BPPs
- `create_bap(data: &BapCreate) -> Result<Bap, BeckNError>`
- `get_bap(id: &str) -> Result<Bap, BeckNError>`
- `list_baps() -> Result<Vec<Bap>, BeckNError>`
- `create_bpp(data: &BppCreate) -> Result<Bpp, BeckNError>`
- `get_bpp(id: &str) -> Result<Bpp, BeckNError>`
- `list_bpps() -> Result<Vec<Bpp>, BeckNError>`

#### GeoDNS Discovery
- `discover_nearest_bap(country: Option<&str>, city: Option<&str>) -> Result<GeoDnsResult, BeckNError>`
- `discover_nearest_bpp(country: Option<&str>, city: Option<&str>) -> Result<GeoDnsResult, BeckNError>`
- `discover_marketplace(lat: f64, lng: f64, radius_km: u32, limit: u32) -> Result<Vec<GeoDnsResult>, BeckNError>`

#### Companies & GBP Sync
- `create_company(data: &CompanyCreate) -> Result<Company, BeckNError>`
- `create_gbp_account(data: &GbpAccountCreate) -> Result<GbpAccount, BeckNError>`
- `sync_gbp_locations(account_id: &str) -> Result<HashMap<String, Vec<Bpp>>, BeckNError>`

#### A2A (Agent-to-Agent)
- `register_agent_card(data: &AgentCardRegister) -> Result<AgentCard, BeckNError>`
- `list_agent_cards() -> Result<Vec<AgentCard>, BeckNError>`
- `create_task(data: &A2ATaskCreate) -> Result<A2ATask, BeckNError>`
- `list_tasks() -> Result<Vec<A2ATask>, BeckNError>`
- `send_message(data: &A2ATaskMessageSend) -> Result<A2ATaskMessage, BeckNError>`
- `list_messages() -> Result<Vec<A2ATaskMessage>, BeckNError>`
- `create_artifact(data: &A2AArtifactCreate) -> Result<A2AArtifact, BeckNError>`
- `list_artifacts() -> Result<Vec<A2AArtifact>, BeckNError>`

#### MCP (Model Context Protocol)
- `create_tool(data: &McpToolCreate) -> Result<McpTool, BeckNError>`
- `list_tools() -> Result<Vec<McpTool>, BeckNError>`
- `create_prompt(data: &McpPromptCreate) -> Result<McpPrompt, BeckNError>`
- `list_prompts() -> Result<Vec<McpPrompt>, BeckNError>`
- `create_resource(data: &McpResourceCreate) -> Result<McpResource, BeckNError>`
- `list_resources() -> Result<Vec<McpResource>, BeckNError>`
- `register_client(data: &McpClientRegister) -> Result<McpClient, BeckNError>`
- `list_clients() -> Result<Vec<McpClient>, BeckNError>`

#### ACP (Agent Credential Protocol)
- `register_issuer(data: &AcpIssuerRegister) -> Result<AcpIssuer, BeckNError>`
- `list_issuers() -> Result<Vec<AcpIssuer>, BeckNError>`
- `issue_token(data: &AcpTokenIssue) -> Result<AcpToken, BeckNError>`
- `list_tokens() -> Result<Vec<AcpToken>, BeckNError>`
- `introspect_token(token: &str) -> Result<AcpToken, BeckNError>`
- `submit_presentation(data: &AcpPresentationSubmit) -> Result<AcpPresentation, BeckNError>`
- `list_presentations() -> Result<Vec<AcpPresentation>, BeckNError>`
- `create_policy(data: &AcpAccessPolicyCreate) -> Result<AcpAccessPolicy, BeckNError>`
- `list_policies() -> Result<Vec<AcpAccessPolicy>, BeckNError>`

#### ANP (Agent Network Protocol)
- `announce(data: &AnpAnnouncementCreate) -> Result<AnpAnnouncement, BeckNError>`
- `list_announcements() -> Result<Vec<AnpAnnouncement>, BeckNError>`
- `register_witness(data: &AnpWitnessRegister) -> Result<AnpWitness, BeckNError>`
- `list_witnesses() -> Result<Vec<AnpWitness>, BeckNError>`
- `verify(data: &AnpVerificationVerify) -> Result<AnpVerification, BeckNError>`

#### API Keys
- `create_api_key(data: &ApiKeyCreate) -> Result<ApiKeyResponse, BeckNError>`
- `verify_api_key(secret: &str) -> Result<ApiKeyVerifyResponse, BeckNError>`
- `revoke_api_key(id: &str) -> Result<ApiKey, BeckNError>`
- `rotate_api_key(id: &str) -> Result<ApiKeyResponse, BeckNError>`
- `list_api_keys() -> Result<Vec<ApiKey>, BeckNError>`

#### Items & Providers
- `create_item(data: &Item) -> Result<Item, BeckNError>`
- `list_items() -> Result<Vec<Item>, BeckNError>`
- `create_provider(data: &Provider) -> Result<Provider, BeckNError>`
- `list_providers() -> Result<Vec<Provider>, BeckNError>`

#### Fulfillments
- `create_fulfillment(data: &FulfillmentCreate) -> Result<Fulfillment, BeckNError>`
- `list_fulfillments() -> Result<Vec<Fulfillment>, BeckNError>`

#### Subscriptions
- `create_subscription(data: &Subscription) -> Result<Subscription, BeckNError>`
- `list_subscriptions() -> Result<Vec<Subscription>, BeckNError>`
- `cancel_subscription(id: &str) -> Result<Subscription, BeckNError>`
- `renew_subscription(id: &str) -> Result<Subscription, BeckNError>`

## License

Apache-2.0
