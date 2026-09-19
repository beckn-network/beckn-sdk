# BeckN SDK

Multi-language SDKs for the BeckN Protocol — a unified agentic discovery and commerce protocol API.

## Supported Languages

| Language | Package | Registry | Status |
|----------|---------|----------|--------|
| TypeScript/Node.js | `@beckn-network/sdk` | npm | ✅ Available |
| Python | `beckn-sdk` | PyPI | ✅ Available |
| Rust | `beckn-sdk` | crates.io | ⏳ Coming soon |

## Installation

### TypeScript
```bash
npm install @beckn-network/sdk
```

### Python
```bash
pip install beckn-sdk
```

### Rust
```toml
# Cargo.toml
[dependencies]
beckn-sdk = "1.0.0"
```

## Quick Start

### TypeScript
```typescript
import { BeckNClient } from '@beckn-network/sdk';

const client = new BeckNClient({
  apiKey: 'bk_your_api_key_here',
  baseUrl: 'https://api.beckn.network'
});

// Check health
const health = await client.healthCheck();

// Create an order
const order = await client.createOrder({
  id: 'order_001',
  order_state: 'pending',
  bap_id: 'bap_001',
  bpp_id: 'bpp_001'
});
```

### Python
```python
from beckn import BeckNClient, BeckNClientConfig

client = BeckNClient(BeckNClientConfig(
    base_url="https://api.beckn.network/v1",
    api_key="bk_your_api_key_here",
    timeout=30000,
))

# Check health
health = client.health_check()

# Register a BAP
bap = client.create_bap({
    "id": "my-bap-001",
    "name": "My Store App",
    "endpoint": "https://store.beckn.network",
    "country": "US",
    "lat": 40.7128,
    "lon": -74.0060,
})
```

### Rust
```rust
use beckn_sdk::{BeckNClient, ClientConfig};

let client = BeckNClient::new(ClientConfig {
    base_url: "https://api.beckn.network/v1".to_string(),
    api_key: Some("bk_your_api_key_here".to_string()),
    timeout: 30000,
});

// Check health
let health = client.health_check().await?;

// Create an order
let order = client.create_order(&OrderCreate {
    id: "order_001".to_string(),
    bap_id: "bap_001".to_string(),
    bpp_id: "bpp_001".to_string(),
    ..Default::default()
}).await?;
```

## Protocol Support

All SDKs provide unified endpoints for:

- **BeckN Core** — Orders, BAPs, BPPs, Items, Providers, Fulfillments, Subscriptions
- **GeoDNS** — Proximity-based discovery with earth_distance/PostGIS
- **A2A (Agent-to-Agent)** — Agent cards, tasks, messages, artifacts
- **MCP (Model Context Protocol)** — Tools, prompts, resources, clients
- **ANP (Agent Network Protocol)** — Announcements, verifications, witnesses
- **ACP (Agent Credential Protocol)** — Tokens, credential issuers, presentations, access policies
- **GBP Sync** — Google Business Profile OAuth and location sync

## Development

```bash
# TypeScript
cd typescript && npm install && npm run build && npm test

# Python
cd python && pip install -e ".[dev]" && pytest tests/

# Rust
cd rust && cargo test
```

## License

Apache-2.0
