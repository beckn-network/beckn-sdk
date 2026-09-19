# BeckN Multi-Language Developer SDK Plan

## Overview

BeckN implements the Beckn protocol (an open, domain-agnostic protocol for digital commerce) as an Elixir/Ash library. This SDK plan enables customers to interact with BeckN-powered platforms from multiple languages.

## Architecture

```
beckn-sdk/
├── spec/                    # OpenAPI specification (single source of truth)
│   └── openapi.yaml
├── typescript/              # TypeScript/Node.js SDK
│   ├── src/
│   ├── tests/
│   ├── package.json
│   └── README.md
├── python/                  # Python SDK
│   ├── beckn/
│   ├── tests/
│   ├── pyproject.toml
│   └── README.md
├── rust/                    # Rust SDK
│   ├── src/
│   ├── tests/
│   ├── Cargo.toml
│   └── README.md
└── README.md                # Multi-language SDK overview
```

## API Surface

### Core Resources
| Resource | Operations |
|----------|-----------|
| Order | create, get, list, update, destroy |
| Item | create, get, list |
| Provider | create, get, list, destroy |
| Fulfillment | create, track, list |
| Bap (Buyer App) | register, get, list, update |
| Bpp (Provider App) | register, get, list, update |
| Subscription | create, get, list, cancel, renew |
| ApiKey | create, verify, revoke, rotate |

### API Endpoints

| HTTP Method | Path | Operation |
|-------------|------|-----------|
| POST | /api/orders | Create order |
| GET | /api/orders/{id} | Get order |
| GET | /api/orders | List orders |
| PATCH | /api/orders/{id} | Update order |
| DELETE | /api/orders/{id} | Delete order |
| POST | /api/items | Create item |
| GET | /api/items/{id} | Get item |
| GET | /api/items | List items |
| POST | /api/providers | Create provider |
| GET | /api/providers/{id} | Get provider |
| GET | /api/providers | List providers |
| POST | /api/fulfillments | Create fulfillment |
| GET | /api/fulfillments/tracking/{tracking_id} | Track fulfillment |
| GET | /api/fulfillments | List fulfillments |
| POST | /api/baps | Register BAP |
| GET | /api/baps/{id} | Get BAP |
| GET | /api/baps | List BAPs |
| PATCH | /api/baps/{id} | Update BAP |
| POST | /api/bpps | Register BPP |
| GET | /api/bpps/{id} | Get BPP |
| GET | /api/bpps | List BPPs |
| PATCH | /api/bpps/{id} | Update BPP |
| POST | /api/subscriptions | Create subscription |
| GET | /api/subscriptions/{id} | Get subscription |
| GET | /api/subscriptions | List subscriptions |
| POST | /api/subscriptions/{id}/cancel | Cancel subscription |
| POST | /api/subscriptions/{id}/renew | Renew subscription |
| POST | /api/api-keys | Create API key |
| POST | /api/api-keys/verify | Verify API key |
| POST | /api/api-keys/{id}/revoke | Revoke API key |
| POST | /api/api-keys/{id}/rotate | Rotate API key |

## Authentication

- BeckN API keys prefixed with `bk_`
- Sent via `Authorization: Bearer <key>` header
- Server validates using SHA-256 hash comparison

## SDK Features

### Shared Across All Languages
- HTTP client with retry logic
- Typed models for all resources
- Authentication via API key
- Error handling with typed error classes
- Request/response logging
- Connection pooling

### TypeScript SDK
- Fetch-based HTTP client (Node.js 18+ and browser)
- Full TypeScript types
- Async/await interface
- Zod schemas for runtime validation
- Jest tests

### Python SDK
- httpx-based async/sync client
- Pydantic models for typed data
- Dataclasses for model definitions
- pytest tests

### Rust SDK
- Reqwest-based HTTP client
- Serde for serialization
- Typed structs and enums
- tokio async runtime
- cargo test

## Implementation Order
1. OpenAPI spec (foundation for all SDKs)
2. TypeScript SDK (most popular for web/mobile)
3. Python SDK (common for backend/data)
4. Rust SDK (performance-sensitive use cases)
5. Tests for all three
