# BeckN Protocol SDK — Rust

Rust SDK for the BeckN Protocol, implementing the Beckn protocol specification for domain-agnostic digital commerce.

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
beckn-sdk = { path = "path/to/beckn-sdk" }
```

Or from crates.io:

```toml
[dependencies]
beckn-sdk = "1.0.0"
```

## Quick Start

### Initialize Client

```rust
use beckn_sdk::{BeckNClient, ClientConfig};

let client = BeckNClient::new(ClientConfig::default()
    .with_base_url("https://api.beckn.network/v1")
    .with_api_key("bk_your_api_key_here"));
```

### Register a BAP

```rust
use beckn_sdk::BapCreate;

let bap = client.create_bap(&BapCreate {
    id: "my-bap-001".to_string(),
    name: "My Store App".to_string(),
    country: Some("US".to_string()),
    lat: Some(40.7128),
    lon: Some(-74.0060),
    company_id: None,
    ..Default::default()
}).await.unwrap();
```

### Register a BPP

```rust
use beckn_sdk::BppCreate;

let bpp = client.create_bpp(&BppCreate {
    id: "my-bpp-001".to_string(),
    name: "My Provider App".to_string(),
    country: Some("US".to_string()),
    currency: Some("USD".to_string()),
    lat: Some(40.7128),
    lon: Some(-74.0060),
    ..Default::default()
}).await.unwrap();
```

### Discover Nearby BPPs (GeoDNS)

```rust
// Find nearby BPPs by geographic coordinates
let bpps = client.discover_marketplace(40.7128, -74.0060, 50, 10).await.unwrap();

// Discover A2A agents near a location (cross-protocol)
let agents = client.discover_nearest_bap(Some("US"), Some("NYC")).await.unwrap();
```

### Companies & GBP Sync

```rust
// Create a company for multi-tenant B2B isolation
let company = client.create_company(&CompanyCreate {
    name: "Acme Corp".to_string(),
    domain: Some("acme.example.com".to_string()),
    country: Some("US".to_string()),
    ..Default::default()
}).await.unwrap();

// GBP account + sync (registers agent cards automatically)
let account = client.create_gbp_account(&GbpAccountCreate {
    email: "merchant@example.com".to_string(),
    account_name: Some("accounts/1234567890".to_string()),
    ..Default::default()
}).await.unwrap();

let synced = client.sync_gbp_locations(&account.id).await.unwrap();
```

## API Reference

### ClientConfig

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `base_url` | `String` | `"http://localhost:4000/v1"` | API base URL |
| `api_key` | `Option<String>` | `None` | API key for authentication |
| `timeout_seconds` | `u64` | `30` | Request timeout in seconds |

### BeckNClient Methods

#### Orders
- `list_orders() -> Result<Vec<Order>, BeckNError>`
- `create_order(data: &OrderCreate) -> Result<Order, BeckNError>`
- `get_order(id: &str) -> Result<Order, BeckNError>`
- `update_order(id: &str, data: &Order) -> Result<Order, BeckNError>`
- `delete_order(id: &str) -> Result<(), BeckNError>`

#### BAPs & BPPs
- `list_baps() -> Result<Vec<Bap>, BeckNError>`
- `create_bap(data: &BapCreate) -> Result<Bap, BeckNError>`
- `get_bap(id: &str) -> Result<Bap, BeckNError>`
- `list_bpps() -> Result<Vec<Bpp>, BeckNError>`
- `create_bpp(data: &BppCreate) -> Result<Bpp, BeckNError>`
- `get_bpp(id: &str) -> Result<Bpp, BeckNError>`

#### GeoDNS Discovery
- `discover_nearest_bap(country: Option<&str>, city: Option<&str>) -> Result<GeoDnsResult, BeckNError>`
- `discover_nearest_bpp(country: Option<&str>, city: Option<&str>) -> Result<GeoDnsResult, BeckNError>`
- `discover_marketplace(lat: f64, lng: f64, radius_km: u32, limit: u32) -> Result<Vec<GeoDnsResult>, BeckNError>`

#### API Keys
- `list_api_keys() -> Result<Vec<ApiKey>, BeckNError>`
- `create_api_key(data: &ApiKeyCreate) -> Result<ApiKeyResponse, BeckNError>`
- `verify_api_key(secret: &str) -> Result<ApiKeyVerifyResponse, BeckNError>`
- `revoke_api_key(id: &str) -> Result<ApiKey, BeckNError>`
- `rotate_api_key(id: &str) -> Result<ApiKeyResponse, BeckNError>`

#### A2A (Agent-to-Agent)
- `list_agent_cards() -> Result<Vec<AgentCard>, BeckNError>`
- `register_agent_card(data: &AgentCardRegister) -> Result<AgentCard, BeckNError>`
- `list_tasks() -> Result<Vec<A2ATask>, BeckNError>`
- `create_task(data: &A2ATaskCreate) -> Result<A2ATask, BeckNError>`
- `send_message(data: &A2ATaskMessageSend) -> Result<A2ATaskMessage, BeckNError>`
- `list_messages() -> Result<Vec<A2ATaskMessage>, BeckNError>`
- `list_artifacts() -> Result<Vec<A2AArtifact>, BeckNError>`
- `create_artifact(data: &A2AArtifactCreate) -> Result<A2AArtifact, BeckNError>`

#### MCP (Model Context Protocol)
- `list_tools() -> Result<Vec<McpTool>, BeckNError>`
- `create_tool(data: &McpToolCreate) -> Result<McpTool, BeckNError>`
- `list_resources() -> Result<Vec<McpResource>, BeckNError>`
- `create_resource(data: &McpResourceCreate) -> Result<McpResource, BeckNError>`
- `list_prompts() -> Result<Vec<McpPrompt>, BeckNError>`
- `create_prompt(data: &McpPromptCreate) -> Result<McpPrompt, BeckNError>`
- `list_clients() -> Result<Vec<McpClient>, BeckNError>`
- `register_client(data: &McpClientRegister) -> Result<McpClient, BeckNError>`

#### ACP (Agent Communication Protocol)
- `list_issuers() -> Result<Vec<AcpIssuer>, BeckNError>`
- `register_issuer(data: &AcpIssuerRegister) -> Result<AcpIssuer, BeckNError>`
- `list_tokens() -> Result<Vec<AcpToken>, BeckNError>`
- `issue_token(data: &AcpTokenIssue) -> Result<AcpToken, BeckNError>`
- `introspect_token(token: &str) -> Result<AcpToken, BeckNError>`
- `list_presentations() -> Result<Vec<AcpPresentation>, BeckNError>`
- `submit_presentation(data: &AcpPresentationSubmit) -> Result<AcpPresentation, BeckNError>`
- `list_policies() -> Result<Vec<AcpAccessPolicy>, BeckNError>`
- `create_policy(data: &AcpAccessPolicyCreate) -> Result<AcpAccessPolicy, BeckNError>`

#### ANP (Agent Network Protocol)
- `list_announcements() -> Result<Vec<AnpAnnouncement>, BeckNError>`
- `announce(data: &AnpAnnouncementCreate) -> Result<AnpAnnouncement, BeckNError>`
- `list_witnesses() -> Result<Vec<AnpWitness>, BeckNError>`
- `register_witness(data: &AnpWitnessRegister) -> Result<AnpWitness, BeckNError>`
- `verify(data: &AnpVerificationVerify) -> Result<AnpVerification, BeckNError>`

#### GBP (Google Business Profile)
- `list_gbp_accounts() -> Result<Vec<GbpAccount>, BeckNError>`
- `create_gbp_account(data: &GbpAccountCreate) -> Result<GbpAccount, BeckNError>`
- `sync_gbp_locations(account_id: &str) -> Result<HashMap<String, Vec<Bpp>>, BeckNError>`

#### Subscriptions
- `list_subscriptions() -> Result<Vec<Subscription>, BeckNError>`
- `create_subscription(data: &Subscription) -> Result<Subscription, BeckNError>`
- `cancel_subscription(id: &str) -> Result<Subscription, BeckNError>`
- `renew_subscription(id: &str) -> Result<Subscription, BeckNError>`

#### Items & Providers
- `list_items() -> Result<Vec<Item>, BeckNError>`
- `create_item(data: &Item) -> Result<Item, BeckNError>`
- `list_providers() -> Result<Vec<Provider>, BeckNError>`
- `create_provider(data: &Provider) -> Result<Provider, BeckNError>`

#### Fulfillments
- `list_fulfillments() -> Result<Vec<Fulfillment>, BeckNError>`
- `create_fulfillment(data: &FulfillmentCreate) -> Result<Fulfillment, BeckNError>`

## License

Apache-2.0
