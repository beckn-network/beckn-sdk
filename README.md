# BeckN Multi-Language Developer SDK

A multi-language SDK for the BeckN Protocol — an Elixir/Ash implementation of the [Beckn protocol](https://github.com/beckn/protocol-specifications) for domain-agnostic digital commerce.

## Overview

The BeckN SDK enables businesses and developers to interact with BeckN-powered platforms across three programming languages. It provides type-safe access to all BeckN protocol resources (Orders, BAPs, BPPs, Items, etc.), agent protocols (A2A, ANP), model context protocols (MCP), credential protocols (ACP), GeoDNS discovery, and Google Business Profile integration.

BeckN serves as a unified agentic system — when GBP locations are synced, they are automatically registered as A2A agent cards, discoverable via GeoDNS proximity search.

## Supported Languages

| Language | Package | Status |
|----------|---------|--------|
| TypeScript/Node.js | `@beckn/sdk` | ✅ Available |
| Python | `beckn-sdk` | ✅ Available |
| Rust | `beckn-sdk` | ✅ Available |

## Installation

### TypeScript
```bash
npm install @beckn/sdk
```

### Python
```bash
pip install beckn-sdk
```

### Rust
```toml
[dependencies]
beckn-sdk = "1.0.0"
```

## Quick Start

```typescript
import { BeckNClient } from '@beckn/sdk';

const client = new BeckNClient({
  baseUrl: 'https://api.beckn.network/v1',
  apiKey: 'bk_your_api_key'
});

// Register a BAP
const bap = await client.createBap({
  id: 'my-bap',
  name: 'My Store',
  country: 'US',
});

// Discover nearby BPPs via GeoDNS
const bpps = await client.discoverNearestBpp('US');

// Create an order
const order = await client.createOrder({
  id: 'order-001',
  transaction_id: 'txn-001',
});
```

## Documentation

- [TypeScript SDK](./typescript/README.md)
- [Python SDK](./python/README.md)
- [Rust SDK](./rust/README.md)
- [API Specification](./spec/openapi.yaml)
- [Architecture Plan](./PLAN.md)

## API Coverage

The SDK provides typed access to all BeckN API endpoints:

| Category | Resources |
|----------|-----------|
| **Commerce** | Orders, Items, Providers, Fulfillments |
| **BeckN Network** | BAPs, BPPs, Subscriptions, API Keys |
| **GeoDNS** | Geographic discovery, proximity search, country/region routing |
| **Companies** | Multi-tenant B2B organization support |
| **GBP** | Google Business Profile integration and sync |
| **A2A** | Agent-to-Agent communication (agent cards, tasks, messages, artifacts) |
| **MCP** | Model Context Protocol (tools, resources, prompts, clients) |
| **ACP** | Agent Credentials Protocol (issuers, tokens, presentations, policies) |
| **ANP** | Agent Network Protocol (announcements, witnesses, verifications) |

## License

Apache-2.0
