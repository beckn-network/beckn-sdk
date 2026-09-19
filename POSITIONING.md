# BeckN Protocol — Positioning & Usage Guidelines

Version 2.0.0 | Public Beta

---

## 1. Product Positioning

### What BeckN Is

**BeckN** is a unified agentic discovery and commerce protocol. It is an open, domain-agnostic infrastructure layer that enables interoperable digital commerce between distributed applications, agents, and services. BeckN unifies five protocol families into a single Ash-based Elixir core:

| Protocol | Purpose | Key Resources |
|----------|---------|---------------|
| **BeckN Core** | Commerce primitives (orders, items, providers, BAP/BPP) | Order, Item, Provider, Fulfillment, Bap, Bpp |
| **A2A** (Agent-to-Agent) | Agent discovery, task execution, artifact/message exchange | A2ATask, A2AAgentCard, A2AArtifact, A2ATaskMessage |
| **MCP** (Model Context Protocol) | Tool registration and execution for AI agents | McpTool, McpPrompt, McpServerResource, McpClient |
| **ANP** (Agent Network Protocol) | DID-based announcements, verification, witnessing | AnpAnnouncement, AnpVerification, AnpWitness |
| **ACP** (Agent Credential Protocol) | Verifiable credentials, token issuance, access policies | AcpToken, AcpCredentialIssuer, AcpPresentation, AcpAccessPolicy |

**BeckN is infrastructure, not a product.** It is an open protocol specification implemented as an Elixir library and exposed via a hosted API (api.beckn.network). Developers build applications *on top of* BeckN — they do not run BeckN itself in production (though they may run it in development for testing).

### What BeckN Is Not

- **Not a marketplace.** BeckN does not sell products or services directly. It provides the protocol for BAPs (Buyer Applications) and BPPs (Provider Applications) to discover and transact with each other.
- **Not a hosted SaaS application.** BeckN does not provide a UI for end-users. It is an API-first protocol.
- **Not a data warehouse.** BeckN stores transient protocol state (tasks, agent cards, credentials) in Ets/in-memory for protocol resources and PostgreSQL for core commerce resources. Data retention is limited.
- **Not a replacement for existing identity systems.** BeckN integrates with DID ecosystems but does not provide identity-as-a-service. ANP verification and ACP presentation resources are protocol-level attestations, not general-purpose identity providers.

### Target Personas

| Persona | Description | Use Case |
|---------|-------------|----------|
| **Platform Engineer** | Building B2B/B2C platforms that need BeckN protocol compliance | Integrates existing platform into BeckN network via SDK |
| **Agent Developer** | Building AI agents that need to discover and transact with services | Uses A2A/MCP resources to register agents and tools |
| **Enterprise Architect** | Evaluating protocol adoption for large organizations | Deploys BeckN in dev, uses hosted API in prod, manages via Company resource |
| **Startup Founder** | Building a new commerce app on top of BeckN | Uses SDK to register BAP/BPP, handles API keys for auth |
| **Protocol Researcher** | Studying decentralized commerce protocols | Runs tests, contributes to spec, uses Ets for experiment isolation |

### Positioning Statement

> **For** developers building decentralized commerce applications, **BeckN** is the open protocol infrastructure that unifies agentic discovery, verifiable credentials, and GeoDNS-based buyer/provider matching across A2A, MCP, ANP, and ACP protocol families. Unlike proprietary commerce APIs, BeckN provides an open, domain-agnostic foundation with multi-language SDKs and a hosted API endpoint.

### Competitive Landscape

| System | Scope | Protocol Families | SDK Languages |
|--------|-------|-------------------|---------------|
| **BeckN** | Full-stack agentic commerce | BeckN + A2A + MCP + ANP + ACP | TS, Python, Rust |
| Beckn (original) | Commerce domain only | BeckN only | JS, Go |
| MCP (Anthropic) | AI tooling only | MCP only | TS, Python, Rust, Go, Java |
| A2A (Google) | Agent task execution only | A2A only | TS, Python |
| ANP (W3C DID) | Decentralized identity only | ANP only | TS, Rust |
| ACP (DIF) | Verifiable credentials only | ACP only | TS, Python, Rust |

BeckN uniquely **unifies** these into a single Ash domain with shared authentication, GeoDNS routing, and B2B multi-tenant isolation via the Company resource.

---

## 2. Usage Guidelines

### 2.1 Authentication & API Keys

All requests to `api.beckn.network/v1` require a valid API key:

```
Authorization: Bearer bk_<your_api_key>
```

**Key Policies:**
- Keys are prefixed with `bk_` and are SHA-256 hashed before storage
- The raw secret is **only** returned once at creation time — store it securely
- Keys can have scopes (e.g., `read:orders`, `write:bpps`, `admin:keys`)
- Keys can be rotated or revoked at any time via `/api/api-keys/{id}/rotate` and `/api/api-keys/{id}/revoke`
- Expired keys are automatically rejected
- **Never** commit API keys to version control, share keys in chat, or expose them in client-side code

### 2.2 Rate Limits

Rate limits are enforced per API key at the load balancer layer:

| Resource | Limit | Notes |
|----------|-------|-------|
| General API requests | 1000 req/min | All endpoints except GBP sync |
| GBP sync endpoints | 10 req/min | `/api/gbp/accounts/{id}/sync` |
| A2A agent registration | 60 req/min | `/api/a2a/agent-cards` POST |
| ANP announcements | 30 req/min | `/api/anp/announcements` POST |
| Health check | Unlimited | `/api/health` |

Exceeding limits returns HTTP 429. Check `X-RateLimit-Remaining` and `Retry-After` headers.

### 2.3 Data Handling & Privacy

- **Protocol resources** (A2A tasks, MCP tools, ANP announcements, ACP tokens) are stored in-memory (Ets) and are **not persisted**. They are suitable for testing and development but will be lost on restart.
- **Core commerce resources** (Orders, BAPs, BPPs, Companies, Subscriptions, API Keys) are stored in PostgreSQL and are persisted.
- **GBP refresh tokens** are stored encrypted in PostgreSQL and are **never** returned in API responses.
- **ACP token values** are stored hashed — the raw token value is only available at issuance time.
- **PII**: BAPs/BPPs may store DIDs, public keys, and endpoints. These are public by design in the BeckN protocol. Do not store sensitive PII (SSNs, credit cards) in protocol resource metadata fields.
- **Data retention**: Protocol resources in Ets are ephemeral. PostgreSQL resources persist indefinitely unless explicitly deleted. There is no automated archival; operators must manage their own data lifecycle.

### 2.4 Responsible Use

**Acceptable Uses:**
- Building compliant BAP/BPP applications for commerce
- Registering AI agents for task execution and tool discovery
- Issuing verifiable credentials for agent authentication
- Announcing DIDs to the ANP network
- Syncing Google Business Profile locations for commerce discovery

**Prohibited Uses:**
- Spamming agent registrations or announcements
- Storing illegal content in protocol resource metadata
- Attempting to bypass rate limits or authentication
- Using BeckN for high-frequency trading or financial services without explicit permission
- Impersonating other BAPs/BPPs or agents
- Running production services on Ets data layer (use PostgreSQL in prod)

### 2.5 Multi-Tenant B2B Isolation

BeckN supports B2B multi-tenant isolation via the `Company` resource. Each company has its own ID and can filter BAPs/BPPs by `company_id`. When registering BAPs or BPPs, always include `company_id` for tenant isolation:

```json
{
  "id": "bpp-001",
  "name": "Acme Store",
  "company_id": "company-uuid",
  "country": "US",
  "lat": 40.7128,
  "lon": -74.0060
}
```

GeoDNS discovery can be scoped to a company:

```
GET /api/geodns/bpps?company_id=company-uuid&country=US
```

---

## 3. Integration Patterns

### 3.1 SDK Quick Start (TypeScript Example)

```typescript
import { BeckNClient } from '@beckn-network/sdk';

const client = new BeckNClient({
  baseUrl: 'https://api.beckn.network/v1',
  apiKey: 'bk_your_api_key',
});

// 1. Register a company for multi-tenant isolation
const company = await client.createCompany({
  name: 'Acme Corp',
  domain: 'acme.example.com',
  country: 'US',
});

// 2. Register your BPP
await client.createBpp({
  id: 'bpp-001',
  name: 'Acme Store',
  company_id: company.id,
  country: 'US',
  lat: 40.7128,
  lon: -74.0060,
});

// 3. Discover nearby BPPs via GeoDNS
const bpps = await client.discoverMarketplace(40.7128, -74.0060, 50, 10);

// 4. Register an A2A agent card
await client.registerAgentCard({
  agent_id: 'agent-001',
  name: 'Travel Booking Agent',
  url: 'https://travel.example.com',
  skills: [{ id: 'search', name: 'Travel Search' }],
});

// 5. Issue an ACP token
const token = await client.issueToken({
  token_value: 'eyJhbGciOiJSUzI1NiIs...',
  subject: 'did:example:subject-1',
});
```

### 3.2 Onboarding Flow

1. **Create a Company** — Establishes B2B tenant isolation
2. **Generate an API Key** — For authenticating all subsequent requests
3. **Register as a BAP or BPP** — With GeoDNS fields (country, city, lat, lon, company_id)
4. **Optional: Register GBP Account** — For Google Business Profile synchronization
5. **Sync GBP Locations** — Auto-creates BPPs with GeoDNS metadata + A2A agent cards
6. **Register A2A Agents** — If you're building AI agents on top of BeckN
7. **Issue ACP Tokens** — For verifiable credential-based authentication
8. **Discover Peers** — Use GeoDNS (`/api/geodns/*`) to find nearby BAPs/BPPs/agents

### 3.3 Cross-Protocol Integration

BeckN's strength is cross-protocol integration:

| Flow | Source Protocol | Target Protocol | API Endpoint |
|------|----------------|-----------------|--------------|
| GBP locations → agent cards | GBP → A2A | `GbpSync.register_agent_card/3` | Auto during sync |
| GeoDNS proximity → agent discovery | GeoDNS → A2A | `GeoDNS.discover_agents_nearby/3` | `/api/geodns/agents` |
| Company → BAP/BPP isolation | Company → BeckN Core | Filter by `company_id` | All BAP/BPP endpoints |

### 3.4 Environment Variables

For Google Business Profile sync, the following environment variables must be set:

```
GOOGLE_CLIENT_ID=your_google_oauth_client_id
GOOGLE_CLIENT_SECRET=your_google_oauth_client_secret
GOOGLE_REDIRECT_URI=https://api.beckn.network/api/gbp/oauth/callback
DATABASE_URL=postgresql://user:pass@localhost:5432/beckn_prod
```

### 3.5 Development Mode

For local development, BeckN can be run without external services:

```bash
cd beckn
mix ecto.create
mix ecto.migrate
mix phx.server
# API available at http://localhost:4000/v1
```

All protocol resources use in-memory Ets storage for testing. PostgreSQL persists core commerce resources.

---

## 4. Operational Guidelines

### 4.1 SLA & Uptime

- **Target Uptime**: 99.9% monthly uptime
- **Maintenance Windows**: Sundays 2:00–4:00 UTC (communicated 72h in advance)
- **Incident Response**: Critical issues (P0) acknowledged within 15 minutes during business hours (9:00–18:00 UTC)
- **Data Backups**: PostgreSQL daily backups, 7-day retention, point-in-time recovery available
- **No SLA on Ets-stored protocol resources** — these are ephemeral by design

### 4.2 Versioning

- API versioning uses URL path: `https://api.beckn.network/v1/`
- SDK versioning follows semver, aligned with API version
- Breaking changes announced 60 days in advance via changelog and email
- Deprecation headers: `Deprecation: true` and `Sunset: <date>` included in responses

### 4.3 Monitoring & Observability

- All requests are logged with structured JSON (request ID, path, method, status, latency)
- API key usage is tracked for rate limiting and abuse detection
- Health check endpoint: `GET /api/health` returns `{"status": "ok"}`
- Metrics available via Prometheus endpoint: `/metrics` (internal use only)

### 4.4 Support & Community

- **Documentation**: https://docs.beckn.network
- **GitHub Issues**: https://github.com/beckn/ecosystem/issues
- **Community Discord**: https://discord.gg/beckn
- **Email Support**: support@beckn.network (business hours: 9:00–18:00 UTC)
- **SLA Response**: 
  - P0 (system down): 15 min
  - P1 (major functionality broken): 1 hour
  - P2 (minor issues): 8 hours
  - P3 (general questions): 24 hours

### 4.5 Beta Limitations

During public beta:
- Ets data layer is used for protocol resources (non-persistent)
- Rate limits are relaxed (2000 req/min for general, 20 req/min for GBP sync)
- Data deletion is not automated — resources persist indefinitely unless explicitly removed
- No uptime SLA guarantee during beta

---

## 5. Brand Guidelines

### Naming Conventions

- **Correct**: BeckN, BeckN Protocol, BeckN SDK, beckn.network
- **Incorrect**: BeckN Protocol API, Beckn, BECKN, beckn
- **Protocol names**: A2A, MCP, ANP, ACP (never lowercase, never "a2a protocol")
- **Resource names**: Use full names (e.g., "A2AAgentCard", not "agent card")

### Terminology

| Term | Meaning |
|------|---------|
| BAP | Buyer Application Platform — consumer-facing app |
| BPP | Provider Application Platform — seller/provider app |
| BG | Beckn Gateway — intermediary for discovery |
| DID | Decentralized Identifier (W3C standard) |
| Agent Card | A2A resource describing an agent's capabilities |
| Tool | MCP resource for AI agent capabilities |
| Issuer | ACP resource for credential issuance |
| Announcement | ANP resource for DID broadcast |
| GeoDNS | Geographic discovery service in BeckN |

---

## License

Apache-2.0 — See [LICENSE](https://github.com/beckn/protocol-specifications)

---

*For questions about these guidelines, contact: product@beckn.network*