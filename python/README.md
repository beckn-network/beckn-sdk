# BeckN Protocol SDK — Python

Python SDK for the BeckN Protocol, implementing the Beckn protocol specification for domain-agnostic digital commerce.

## Installation

```bash
pip install beckn-sdk
```

## Quick Start

### Initialize Client

```python
from beckn import BeckNClient, BeckNClientConfig

client = BeckNClient(BeckNClientConfig(
    base_url="https://api.beckn.network/v1",
    api_key="bk_your_api_key_here",
    timeout=30000,
))
```

### Register a BAP

```python
bap = client.create_bap(BapCreate(
    id="my-bap-001",
    name="My Store App",
    endpoint="https://store.example.com",
    country="US",
    lat=40.7128,
    lon=-74.0060,
))
```

### Register a BPP

```python
bpp = client.create_bpp(BppCreate(
    id="my-bpp-001",
    name="My Provider App",
    endpoint="https://provider.example.com",
    country="US",
    currency="USD",
    lat=40.7128,
    lon=-74.0060,
))
```

### Create an Order

```python
order = client.create_order(OrderCreate(
    id="order-001",
    transaction_id="txn-001",
    bap_id="my-bap-001",
    bpp_id="my-bpp-001",
))
```

### Discover Nearby BPPs (GeoDNS)

```python
# Find BPPs in a specific country
bpps = client.discover_nearest_bap(country="US")

# Find BPPs near a geographic location
nearby = client.discover_marketplace(lat=40.7128, lng=-74.0060, radius_km=50, limit=10)
```

### GBP (Google Business Profile)

```python
# Register a GBP account
account = client.create_gbp_account(GbpAccountCreate(
    email="merchant@example.com",
    account_name="accounts/1234567890",
))

# Sync GBP locations as BPPs (auto-registers A2A agent cards)
synced = client.sync_gbp_locations(account.id)
print(f"{len(synced['bpps'])} BPPs synced")
```

### A2A Agent Discovery

```python
# Discover agents near a location (cross-protocol with GeoDNS)
nearby = client.discover_marketplace(lat=40.7128, lng=-74.0060, radius_km=50, limit=10)

# Register an agent card
agent = client.register_agent_card(AgentCardRegister(
    agent_id="agent-001",
    name="Travel Booking Agent",
    url="https://travel.example.com",
    capabilities={"streaming": True},
    skills=[{"id": "search", "name": "Travel Search"}],
))

# Discover agents by skill
results = client.discover_agents(skill="booking")
```

### ANP (Agent Network Protocol)

```python
# Announce a DID to the network
announcement = client.announce(AnpAnnouncementCreate(
    announcement_id="ann-001",
    did="did:example:123",
    service_endpoint="https://agent.example.com",
))

# Register a witness node
witness = client.register_witness(AnpWitnessRegister(
    witness_id="witness-001",
    did="did:example:witness-1",
    endpoint="https://witness.example.com",
    protocols=["gossip", "http"],
))

# Verify a DID document
verification = client.verify(AnpVerificationVerify(
    did="did:example:123",
    did_document={"id": "did:example:123"},
    method="key",
))
```

### ACP (Agent Communication Protocol)

```python
# Register a credential issuer
issuer = client.register_issuer(AcpIssuerRegister(
    issuer_id="issuer-001",
    name="Test Issuer",
    authorization_endpoint="https://issuer.example.com/authorize",
    token_endpoint="https://issuer.example.com/token",
    jwks_uri="https://issuer.example.com/.well-known/jwks.json",
))

# Issue a token
token = client.issue_token(AcpTokenIssue(
    token_value="eyJhbGciOiJSUzI1NiIs...",
    subject="did:example:subject-1",
))

# Submit a verifiable presentation
presentation = client.submit_presentation(AcpPresentationSubmit(
    presentation_id="vp-123",
    holder_did="did:example:holder-1",
    issuer_id="issuer-001",
    claims={"age": 25},
))
```

### Google Business Profile Sync

```python
# Register a GBP account
account = client.create_gbp_account({
    "email": "merchant@example.com",
    "account_name": "accounts/1234567890",
})

# Sync GBP locations as BPPs (auto-registers A2A agent cards)
result = client.sync_gbp_locations(account["id"])
print(f"{len(result['bpps'])} BPPs synced")
```

### A2A Agent Discovery

```python
# Discover nearby BPPs via GeoDNS (maps to A2A agents)
bpps = client.discover_nearest_bpp(country="US")

# Register an agent card
agent_card = client.register_agent_card({
    "agent_id": "agent-001",
    "name": "Travel Booking Agent",
    "url": "https://travel.example.com",
    "capabilities": {"streaming": True},
    "skills": [{"id": "search", "name": "Travel Search"}],
})

# Send a message to a task
message = client.send_message({
    "message_id": "msg-001",
    "context_id": "ctx-1",
    "role": "user",
    "parts": [{"type": "text", "text": "Find flights to NYC"}],
})
```

### ANP (Agent Network Protocol)

```python
# Announce a DID to the network
announcement = client.announce({
    "announcement_id": "ann-001",
    "did": "did:example:123",
    "service_endpoint": "https://agent.example.com",
})

# Register a witness node
witness = client.register_witness({
    "witness_id": "witness-001",
    "did": "did:example:witness-1",
    "endpoint": "https://witness.example.com",
    "protocols": ["gossip", "http"],
})
```

### ACP (Agent Communication Protocol)

```python
# Register a credential issuer
issuer = client.register_issuer({
    "issuer_id": "issuer-001",
    "name": "Test Issuer",
    "authorization_endpoint": "https://issuer.example.com/authorize",
    "token_endpoint": "https://issuer.example.com/token",
    "jwks_uri": "https://issuer.example.com/.well-known/jwks.json",
})

# Issue a token
token = client.issue_token({
    "token_value": "eyJhbGciOiJSUzI1NiIs...",
    "subject": "did:example:subject-1",
})

# Submit a verifiable presentation
presentation = client.submit_presentation({
    "id": "pres-001",
    "presentation_id": "vp-123",
    "vp": {"@context": ["..."], "type": "VerifiablePresentation"},
    "credential_issuer_id": "issuer-001",
    "holder": "did:example:holder-1",
})
```

## API Reference

### BeckNClientConfig

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `base_url` | `str` | `'http://localhost:4000/v1'` | API base URL |
| `api_key` | `Optional[str]` | `None` | API key for authentication |
| `timeout` | `int` | `30000` | Request timeout in milliseconds |

### Methods

#### Orders
- `list_orders() -> list[Order]`
- `create_order(data: OrderCreate) -> Order`
- `get_order(id: str) -> Order`
- `delete_order(id: str) -> None`

#### BAPs
- `list_baps() -> list[Bap]`
- `list_baps_by_company(company_id: str) -> list[Bap]`
- `list_baps_by_country(country: str) -> list[Bap]`
- `list_baps_nearby(lat: float, lng: float, radius_km: int = 50) -> list[Bap]`
- `create_bap(data: BapCreate) -> Bap`
- `get_bap(bap_id: str) -> Bap`
- `update_bap(bap_id: str, data: BapUpdate) -> Bap`

#### BPPs
- `list_bpps() -> list[Bpp]`
- `list_bpps_by_company(company_id: str) -> list[Bpp]`
- `list_bpps_by_country(country: str) -> list[Bpp]`
- `list_bpps_nearby(lat: float, lng: float, radius_km: int = 50) -> list[Bpp]`
- `create_bpp(data: BppCreate) -> Bpp`
- `get_bpp(bpp_id: str) -> Bpp`
- `update_bpp(bpp_id: str, data: BppUpdate) -> Bpp`

#### Companies (B2B Multi-Tenant)
- `list_companies() -> list[Company]`
- `create_company(data: CompanyCreate) -> Company`
- `get_company(company_id: str) -> Company`
- `list_companies_by_domain(domain: str) -> list[Company]`

#### API Keys
- `list_api_keys() -> list[ApiKey]`
- `create_api_key(data: ApiKeyCreate) -> ApiKeyResponse`
- `verify_api_key(secret: str) -> ApiKeyVerifyResponse`
- `revoke_api_key(key_id: str) -> ApiKey`
- `rotate_api_key(key_id: str) -> ApiKeyResponse`

#### GeoDNS Discovery
- `discover_nearest_bap(country: str = None, city: str = None) -> GeoDnsResult`
- `discover_nearest_bpp(country: str = None, city: str = None) -> GeoDnsResult`
- `discover_marketplace(lat: float, lng: float, radius_km: int = 50, limit: int = 10) -> list[GeoDnsResult]`

#### GBP (Google Business Profile)
- `list_gbp_accounts() -> list[GbpAccount]`
- `create_gbp_account(data: dict) -> GbpAccount`
- `sync_gbp_locations(account_id: str) -> dict`

#### A2A (Agent-to-Agent)
- `list_agent_cards() -> list[AgentCard]`
- `register_agent_card(data: AgentCardRegister) -> AgentCard`
- `list_tasks() -> list[A2ATask]`
- `create_task(data: A2ATaskCreate) -> A2ATask`
- `send_message(data: A2ATaskMessageSend) -> A2ATaskMessage`
- `list_messages() -> list[A2ATaskMessage]`
- `list_artifacts() -> list[A2AArtifact]`
- `create_artifact(data: A2AArtifactCreate) -> A2AArtifact`

#### MCP (Model Context Protocol)
- `list_tools() -> list[McpTool]`
- `create_tool(data: McpToolCreate) -> McpTool`
- `list_resources() -> list[McpResource]`
- `create_resource(data: McpResourceCreate) -> McpResource`
- `list_prompts() -> list[McpPrompt]`
- `create_prompt(data: McpPromptCreate) -> McpPrompt`

#### ACP (Agent Communication Protocol)
- `list_issuers() -> list[AcpIssuer]`
- `register_issuer(data: AcpIssuerRegister) -> AcpIssuer`
- `list_tokens() -> list[AcpToken]`
- `issue_token(data: AcpTokenIssue) -> AcpToken`
- `introspect_token(token: str) -> AcpToken`
- `list_presentations() -> list[AcpPresentation]`
- `submit_presentation(data: AcpPresentationSubmit) -> AcpPresentation`
- `list_policies() -> list[AcpAccessPolicy]`
- `create_policy(data: AcpAccessPolicyCreate) -> AcpAccessPolicy`

#### ANP (Agent Network Protocol)
- `list_announcements() -> list[AnpAnnouncement]`
- `announce(data: AnpAnnouncementCreate) -> AnpAnnouncement`
- `list_witnesses() -> list[AnpWitness]`
- `register_witness(data: AnpWitnessRegister) -> AnpWitness`
- `verify(data: AnpVerificationVerify) -> AnpVerification`

#### Subscriptions
- `list_subscriptions() -> list[Subscription]`
- `create_subscription(data: SubscriptionCreate) -> Subscription`
- `cancel_subscription(sub_id: str) -> Subscription`
- `renew_subscription(sub_id: str) -> Subscription`

## License

Apache-2.0
