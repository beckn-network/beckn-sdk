# BeckN Protocol SDK — Python

Python SDK for the BeckN Protocol — a unified agentic discovery and commerce protocol API.

Supports: BeckN Core, GeoDNS, A2A (Agent-to-Agent), MCP (Model Context Protocol), ANP (Agent Network Protocol), ACP (Agent Credential Protocol), and GBP (Google Business Profile) sync.

## Installation

```bash
pip install beckn-sdk
```

## Quick Start

```python
from beckn import BeckNClient, BeckNClientConfig

client = BeckNClient(BeckNClientConfig(
    base_url="https://api.beckn.network/v1",
    api_key="bk_your_api_key_here",
    timeout=30000,
))

# Check health
health = client.health_check()

# Create an order
order = client.create_order({
    "id": "order-001",
    "transaction_id": "txn-001",
    "bap_id": "my-bap-001",
    "bpp_id": "my-bpp-001",
})

# Discover nearby BPPs
nearby = client.discover_marketplace(lat=40.7128, lng=-74.0060, radius_km=50)

# Register an agent card
agent = client.register_agent_card({
    "agent_id": "agent-001",
    "name": "Travel Booking Agent",
    "url": "https://travel.beckn.network",
    "capabilities": {"streaming": True},
})
```

## Protocol Support

### BeckN Core
```python
# Orders
order = client.create_order({"id": "order-001", "bap_id": "bap-001", "bpp_id": "bpp-001"})

# BAPs (BeckN Application Platform)
bap = client.create_bap({"id": "my-bap-001", "name": "My Store App", "country": "US"})

# BPPs (BeckN Provider Platform)
bpp = client.create_bpp({"id": "my-bpp-001", "name": "My Provider", "country": "US"})

# Items
item = client.create_item({"id": "item-001", "name": "Product", "price": 99})

# Companies (B2B Multi-Tenant)
company = client.create_company({"name": "Acme Corp", "domain": "acme.beckn"})

# API Keys
key = client.create_api_key({"name": "my-api-key"})

# Subscriptions
sub = client.create_subscription({"id": "sub-001", "type": "order"})
```

### GeoDNS Discovery
```python
# Find BAPs/BPPs by country
baps = client.discover_nearest_bap(country="US")
bpps = client.discover_nearest_bpp(country="US")

# Find nearby by coordinates
nearby = client.discover_marketplace(lat=40.7128, lng=-74.0060, radius_km=50, limit=10)
```

### A2A (Agent-to-Agent)
```python
# Register an agent card
agent = client.register_agent_card({
    "agent_id": "agent-001",
    "name": "Travel Booking Agent",
    "url": "https://travel.beckn.network",
    "capabilities": {"streaming": True},
    "skills": [{"id": "search", "name": "Travel Search"}],
})

# Create a task
task = client.create_task({
    "task_id": "task-123",
    "context_id": "ctx-456",
    "status": "pending",
})

# Send a message
message = client.send_message({
    "message_id": "msg-001",
    "context_id": "ctx-1",
    "task_id": "task-123",
    "role": "user",
    "parts": [{"type": "text", "text": "Hello, agent!"}],
})
```

### MCP (Model Context Protocol)
```python
# Register a tool
tool = client.create_tool({
    "id": "tool-1",
    "name": "search",
    "input_schema": {"type": "object"},
    "handler": "MyApp.Tools.Search",
})

# Create a resource
resource = client.create_resource({
    "uri_template": "file:///data/{id}",
    "name": "data-file",
    "mime_type": "application/json",
})

# Register a client
client_obj = client.register_client({
    "client_info": {"name": "my-app", "version": "1.0.0"},
    "capabilities": {"tools": True},
})
```

### ANP (Agent Network Protocol)
```python
# Announce a DID
announcement = client.announce({
    "announcement_id": "ann-001",
    "did": "did:example:123",
    "service_endpoint": "https://agent.beckn.network",
})

# Register a witness
witness = client.register_witness({
    "witness_id": "witness-001",
    "did": "did:example:witness-1",
    "endpoint": "https://witness.beckn.network",
})

# Verify a DID document
verification = client.verify({
    "did": "did:example:123",
    "did_document": {"id": "did:example:123"},
    "method": "key",
})
```

### ACP (Agent Credential Protocol)
```python
# Register a credential issuer
issuer = client.register_issuer({
    "issuer_id": "issuer-001",
    "name": "Test Issuer",
    "authorization_endpoint": "https://issuer.beckn.network/authorize",
    "token_endpoint": "https://issuer.beckn.network/token",
    "jwks_uri": "https://issuer.beckn.network/.well-known/jwks.json",
})

# Issue a token
token = client.issue_token({
    "token_value": "eyJhbGciOiJSUzI1NiIs...",
    "scope": "openid profile offline_access",
    "subject": "did:example:subject-1",
})

# Submit a verifiable presentation
presentation = client.submit_presentation({
    "presentation_id": "vp-123",
    "holder": "did:example:holder-1",
    "credential_issuer_id": "issuer-001",
})
```

### GBP Sync
```python
# Register a GBP account
account = client.create_gbp_account({
    "email": "merchant@beckn.network",
    "account_name": "accounts/1234567890",
})

# Sync GBP locations (auto-registers A2A agent cards)
result = client.sync_gbp_locations(account["id"])
print(f"{result['bpps_synced']} BPPs synced")
```

## API Reference

### BeckNClientConfig

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `base_url` | `str` | `'https://api.beckn.network/v1'` | API base URL |
| `api_key` | `Optional[str]` | `None` | API key for authentication |
| `timeout` | `int` | `30000` | Request timeout in milliseconds |

### Core Methods

| Method | Description |
|--------|-------------|
| `health_check()` | Check API health |
| `list_orders()` | List all orders |
| `create_order(data)` | Create a new order |
| `get_order(id)` | Get order by ID |
| `list_baps()` | List BAPs |
| `create_bap(data)` | Register a BAP |
| `list_bpps()` | List BPPs |
| `create_bpp(data)` | Register a BPP |
| `list_items()` | List items |
| `create_item(data)` | Create an item |
| `list_providers()` | List providers |
| `create_provider(data)` | Register a provider |
| `list_api_keys()` | List API keys |
| `create_api_key(data)` | Create API key |
| `revoke_api_key(id)` | Revoke API key |
| `list_companies()` | List companies |
| `create_company(data)` | Create company |

### GeoDNS Methods

| Method | Description |
|--------|-------------|
| `discover_nearest_bap(country?, city?)` | Find BAPs by country/city |
| `discover_nearest_bpp(country?, city?)` | Find BPPs by country/city |
| `discover_marketplace(lat, lng, radius_km, limit)` | Find nearby providers |

### A2A Methods

| Method | Description |
|--------|-------------|
| `list_agent_cards()` | List agent cards |
| `register_agent_card(data)` | Register an agent card |
| `discover_agents(skill)` | Find agents by skill |
| `list_tasks()` | List tasks |
| `create_task(data)` | Create a task |
| `list_messages()` | List messages |
| `send_message(data)` | Send a message |
| `list_artifacts()` | List artifacts |
| `create_artifact(data)` | Create an artifact |

### MCP Methods

| Method | Description |
|--------|-------------|
| `list_tools()` | List MCP tools |
| `create_tool(data)` | Create a tool |
| `list_resources()` | List resources |
| `create_resource(data)` | Create a resource |
| `list_prompts()` | List prompts |
| `create_prompt(data)` | Create a prompt |
| `list_clients()` | List MCP clients |
| `register_client(data)` | Register a client |

### ANP Methods

| Method | Description |
|--------|-------------|
| `list_announcements()` | List announcements |
| `announce(data)` | Announce a DID |
| `list_witnesses()` | List witnesses |
| `register_witness(data)` | Register a witness |
| `verify(data)` | Verify a DID document |

### ACP Methods

| Method | Description |
|--------|-------------|
| `list_issuers()` | List credential issuers |
| `register_issuer(data)` | Register an issuer |
| `list_tokens()` | List tokens |
| `issue_token(data)` | Issue a token |
| `introspect_token(token)` | Introspect a token |
| `list_presentations()` | List presentations |
| `submit_presentation(data)` | Submit a presentation |
| `list_policies()` | List access policies |
| `create_policy(data)` | Create an access policy |

### GBP Methods

| Method | Description |
|--------|-------------|
| `list_gbp_accounts()` | List GBP accounts |
| `create_gbp_account(data)` | Register a GBP account |
| `sync_gbp_locations(account_id)` | Sync locations (creates BPPs + A2A agents) |

## Development

```bash
# Install with dev dependencies
pip install -e ".[dev]"

# Run tests
pytest tests/ -v

# Type check
mypy beckn/

# Lint
ruff check beckn/
```

## License

Apache-2.0
