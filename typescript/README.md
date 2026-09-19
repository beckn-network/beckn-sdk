# BeckN TypeScript SDK

A TypeScript/JavaScript SDK for the BeckN Protocol — a unified agentic discovery and commerce protocol API.

Supports: BeckN Core, GeoDNS, A2A (Agent-to-Agent), MCP (Model Context Protocol), ANP (Agent Network Protocol), ACP (Agent Credential Protocol), and GBP (Google Business Profile) sync.

## Installation

```bash
npm install @beckn-network/sdk
```

## Quick Start

```typescript
import { BeckNClient, BeckNError } from '@beckn-network/sdk';

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

// Register a BAP
const bap = await client.createBap({
  id: 'bap_001',
  name: 'My Store',
  country: 'US',
  lat: 40.7128,
  lon: -74.0060,
});

// Discover nearby BPPs (GeoDNS)
const bpps = await client.findNearbyBpps({ lat: 40.7128, lng: -74.0060, radiusKm: 50 });

// Register an agent card (A2A)
const agent = await client.registerAgentCard({
  agent_id: 'agent-001',
  name: 'Travel Booking Agent',
  url: 'https://travel.example.com',
  capabilities: { streaming: true },
  skills: [{ id: 'search', name: 'Travel Search' }],
});
```

## API Reference

### BeckNClient

```typescript
new BeckNClient({
  apiKey: string,              // Required: Your BeckN API key (bk_*)
  baseUrl?: string,            // Default: 'https://api.beckn.network'
  timeout?: number,            // Default: 30000 (ms)
  headers?: Record<string, string>  // Custom headers
})
```

### Core Commerce

#### Orders
- `createOrder(attrs: OrderAttrs): Promise<Order>`
- `getOrder(id: string): Promise<Order>`
- `listOrders(opts?: ListOpts): Promise<Order[]>`
- `updateOrder(id: string, attrs: Partial<OrderAttrs>): Promise<Order>`
- `deleteOrder(id: string): Promise<void>`

#### Items
- `createItem(attrs: ItemAttrs): Promise<Item>`
- `getItem(id: string): Promise<Item>`
- `listItems(opts?: ListOpts): Promise<Item[]>`

#### Providers
- `createProvider(attrs: ProviderAttrs): Promise<Provider>`
- `getProvider(id: string): Promise<Provider>`
- `listProviders(opts?: ListOpts): Promise<Provider[]>`
- `deleteProvider(id: string): Promise<void>`

#### Fulfillments
- `createFulfillment(attrs: FulfillmentAttrs): Promise<Fulfillment>`
- `listFulfillments(opts?: ListOpts): Promise<Fulfillment[]>`
- `trackFulfillment(trackingId: string): Promise<Fulfillment>`

#### BAPs & BPPs
- `createBap(attrs: BapAttrs): Promise<Bap>`
- `getBap(id: string): Promise<Bap>`
- `listBaps(opts?: ListOpts): Promise<Bap[]>`
- `updateBap(id: string, attrs: Partial<BapAttrs>): Promise<Bap>`
- `createBpp(attrs: BppAttrs): Promise<Bpp>`
- `getBpp(id: string): Promise<Bpp>`
- `listBpps(opts?: ListOpts): Promise<Bpp[]>`
- `updateBpp(id: string, attrs: Partial<BppAttrs>): Promise<Bpp>`

#### Subscriptions
- `createSubscription(attrs: SubscriptionAttrs): Promise<Subscription>`
- `getSubscription(id: string): Promise<Subscription>`
- `listSubscriptions(opts?: ListOpts): Promise<Subscription[]>`
- `cancelSubscription(id: string): Promise<Subscription>`
- `renewSubscription(id: string): Promise<Subscription>`

#### Companies (B2B Multi-Tenant)
- `createCompany(attrs: CompanyAttrs): Promise<Company>`
- `getCompany(id: string): Promise<Company>`
- `listCompanies(opts?: ListOpts): Promise<Company[]>`
- `companiesByDomain(domain: string): Promise<Company[]>`
- `updateCompany(id: string, attrs: Partial<CompanyAttrs>): Promise<Company>`

#### API Keys
- `createApiKey(attrs: ApiKeyAttrs): Promise<ApiKeyResponse>`
- `verifyApiKey(secret: string): Promise<ApiKeyVerifyResponse>`
- `revokeApiKey(id: string): Promise<void>`
- `rotateApiKey(id: string): Promise<ApiKeyResponse>`
- `listApiKeys(opts?: ListOpts): Promise<ApiKey[]>`

### GeoDNS

| Method | Parameters | Returns |
|--------|-----------|---------|
| `findNearbyBapps` | `{ lat, lng, radiusKm? }` | `Promise<AgentCard[]>` |
| `findNearbyBpps` | `{ lat, lng, radiusKm? }` | `Promise<Bpp[]>` |
| `findBapsByCountry` | `country: string` | `Promise<Bap[]>` |
| `findBppsByCountry` | `country: string` | `Promise<Bpp[]>` |

### A2A (Agent-to-Agent)

| Method | Parameters | Returns |
|--------|-----------|---------|
| `registerAgentCard` | `attrs: AgentCardAttrs` | `Promise<AgentCard>` |
| `listAgentCards` | `opts?: ListOpts` | `Promise<AgentCard[]>` |
| `discoverAgents` | `skill: string` | `Promise<AgentCard[]>` |
| `createTask` | `attrs: TaskAttrs` | `Promise<A2ATask>` |
| `getTask` | `id: string` | `Promise<A2ATask>` |
| `listTasks` | `opts?: ListOpts` | `Promise<A2ATask[]>` |
| `sendMessage` | `attrs: MessageAttrs` | `Promise<Message>` |
| `listMessages` | `opts?: ListOpts` | `Promise<Message[]>` |
| `createArtifact` | `attrs: ArtifactAttrs` | `Promise<Artifact>` |
| `listArtifacts` | `opts?: ListOpts` | `Promise<Artifact[]>` |

### MCP (Model Context Protocol)

| Method | Parameters | Returns |
|--------|-----------|---------|
| `createTool` | `attrs: ToolAttrs` | `Promise<McpTool>` |
| `listTools` | `opts?: ListOpts` | `Promise<McpTool[]>` |
| `createPrompt` | `attrs: PromptAttrs` | `Promise<McpPrompt>` |
| `listPrompts` | `opts?: ListOpts` | `Promise<McpPrompt[]>` |
| `createResource` | `attrs: ResourceAttrs` | `Promise<McpResource>` |
| `listResources` | `opts?: ListOpts` | `Promise<McpResource[]>` |
| `registerClient` | `attrs: ClientAttrs` | `Promise<McpClient>` |
| `listClients` | `opts?: ListOpts` | `Promise<McpClient[]>` |

### ANP (Agent Network Protocol)

| Method | Parameters | Returns |
|--------|-----------|---------|
| `announce` | `attrs: AnnouncementAttrs` | `Promise<Announcement>` |
| `listAnnouncements` | `opts?: ListOpts` | `Promise<Announcement[]>` |
| `verify` | `attrs: VerificationAttrs` | `Promise<Verification>` |
| `listVerifications` | `opts?: ListOpts` | `Promise<Verification[]>` |
| `registerWitness` | `attrs: WitnessAttrs` | `Promise<Witness>` |
| `listWitnesses` | `opts?: ListOpts` | `Promise<Witness[]>` |

### ACP (Agent Credential Protocol)

| Method | Parameters | Returns |
|--------|-----------|---------|
| `issueToken` | `attrs: TokenAttrs` | `Promise<AcpToken>` |
| `listTokens` | `opts?: ListOpts` | `Promise<AcpToken[]>` |
| `introspectToken` | `token: string` | `Promise<IntrospectionResult>` |
| `registerIssuer` | `attrs: IssuerAttrs` | `Promise<Issuer>` |
| `listIssuers` | `opts?: ListOpts` | `Promise<Issuer[]>` |
| `submitPresentation` | `attrs: PresentationAttrs` | `Promise<Presentation>` |
| `listPresentations` | `opts?: ListOpts` | `Promise<Presentation[]>` |
| `createPolicy` | `attrs: PolicyAttrs` | `Promise<AccessPolicy>` |
| `listPolicies` | `opts?: ListOpts` | `Promise<AccessPolicy[]>` |

### GBP Sync

| Method | Parameters | Returns |
|--------|-----------|---------|
| `createAccount` | `attrs: AccountAttrs` | `Promise<GbpAccount>` |
| `listAccounts` | `opts?: ListOpts` | `Promise<GbpAccount[]>` |
| `syncLocations` | `accountId: string` | `Promise<SyncResult>` |

### Health
- `healthCheck(): Promise<HealthCheck>`

## Error Handling

```typescript
import { BeckNError } from '@beckn-network/sdk';

try {
  const order = await client.getOrder('nonexistent');
} catch (error) {
  if (error instanceof BeckNError) {
    console.error(`HTTP ${error.status}: ${error.message}`);
    console.error('Error code:', error.code);
    console.error('Retry after:', error.retryAfter);
  }
}
```

### Error Codes

| Code | HTTP Status | Description |
|------|-------------|-------------|
| `validation_error` | 400 | Invalid request data |
| `not_found` | 404 | Resource not found |
| `api_error` | 422/500 | Server error |
| `invalid_api_key` | 401 | Invalid API key |
| `expired_api_key` | 401 | API key expired |
| `rate_limit_exceeded` | 429 | Too many requests |

## Development

```bash
# Install dependencies
npm install

# Build
npm run build

# Type check
npx tsc --noEmit

# Run tests
npm test
```

## License

Apache-2.0
