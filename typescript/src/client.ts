import type {
  Bap, BapCreate, BapUpdate, Bpp, BppCreate, BppUpdate,
  Order, OrderCreate, OrderUpdate, Item, ItemCreate,
  Provider, ProviderCreate, Fulfillment, FulfillmentCreate,
  Subscription, SubscriptionCreate,
  ApiKey, ApiKeyCreate, ApiKeyResponse, ApiKeyVerifyRequest, ApiKeyVerifyResponse,
  ErrorResponse, GeoDnsResult, Company, CompanyCreate, CompanyUpdate,
  GbpAccount, GbpAccountCreate,
  AgentCard, AgentCardRegister, A2ATask, A2ATaskCreate,
  A2ATaskMessage, A2ATaskMessageSend, A2AArtifact, A2AArtifactCreate,
  McpTool, McpToolCreate, McpResource, McpResourceCreate,
  McpPrompt, McpPromptCreate, McpClient as IMcpClient, McpClientRegister,
  AcpIssuer, AcpIssuerRegister, AcpToken, AcpTokenIssue,
  AcpPresentation, AcpPresentationSubmit, AcpAccessPolicy, AcpAccessPolicyCreate,
  AnpAnnouncement, AnpAnnouncementCreate, AnpWitness, AnpWitnessRegister,
  AnpVerification, AnpVerificationVerify,
  BeckNClientConfig,
} from './types.js';

export class BeckNError extends Error {
  constructor(message: string, public status?: number, public response?: ErrorResponse) {
    super(message);
    this.name = 'BeckNError';
  }
}

const DEFAULT_BASE_URL = process.env.BECKN_API_URL || 'https://api.beckn.network/v1';
const DEFAULT_TIMEOUT = 30000;

export class BeckNClient {
  private baseUrl: string;
  private apiKey?: string;
  private timeout: number;

  constructor(config: BeckNClientConfig = {}) {
    this.baseUrl = config.baseUrl || DEFAULT_BASE_URL;
    this.apiKey = config.apiKey || process.env.BECKN_API_KEY;
    this.timeout = config.timeout || DEFAULT_TIMEOUT;
  }

  getCredential(): string | undefined { return this.apiKey; }
  getTimeout(): number { return this.timeout; }

  protected async request<T>(path: string, options: RequestInit = {}): Promise<T> {
    const headers: Record<string, string> = {
      'Content-Type': 'application/json',
      Accept: 'application/json',
      ...(options.headers as Record<string, string> || {}),
    };

    if (this.apiKey) headers['x-api-key'] = this.apiKey;

    const url = `${this.baseUrl}${path}`;
    const controller = new AbortController();
    const timeoutId = setTimeout(() => controller.abort(), this.timeout);

    try {
      const response = await fetch(url, { ...options, headers, signal: controller.signal });
      clearTimeout(timeoutId);

      if (!response.ok) {
        const errorBody = await response.text();
        let parsedError: ErrorResponse | undefined;
        try { parsedError = JSON.parse(errorBody) as ErrorResponse; } catch { parsedError = { message: errorBody }; }
        throw new BeckNError(parsedError.message || `HTTP ${response.status}: ${response.statusText}`, response.status, parsedError);
      }
      if (response.status === 204) return undefined as T;
      return (await response.json()) as T;
    } catch (error) {
      clearTimeout(timeoutId);
      if (error instanceof BeckNError) throw error;
      if (error instanceof Error && error.name === 'AbortError') throw new BeckNError('Request timed out', 408);
      throw new BeckNError(error instanceof Error ? error.message : String(error), 500);
    }
  }

  protected post<T>(path: string, body: unknown): Promise<T> { return this.request<T>(path, { method: 'POST', body: JSON.stringify(body) }); }
  protected get<T>(path: string): Promise<T> { return this.request<T>(path, { method: 'GET' }); }
  protected patch<T>(path: string, body: unknown): Promise<T> { return this.request<T>(path, { method: 'PATCH', body: JSON.stringify(body) }); }
  protected delete<T = void>(path: string): Promise<T> { return this.request<T>(path, { method: 'DELETE' }); }

  // === GeoDNS Discovery ===
  async discoverNearestBap(companyId?: string, country?: string, city?: string): Promise<GeoDnsResult> {
    const params = new URLSearchParams();
    if (companyId) params.set('company_id', companyId);
    if (country) params.set('country', country);
    if (city) params.set('city', city);
    return this.get<GeoDnsResult>(`/geodns/baps?${params.toString()}`);
  }

  async discoverNearestBpp(companyId?: string, country?: string, city?: string): Promise<GeoDnsResult> {
    const params = new URLSearchParams();
    if (companyId) params.set('company_id', companyId);
    if (country) params.set('country', country);
    if (city) params.set('city', city);
    return this.get<GeoDnsResult>(`/geodns/bpps?${params.toString()}`);
  }

  async discoverMarketplace(lat: number, lng: number, radiusKm = 50, limit = 10): Promise<GeoDnsResult[]> {
    return this.get<GeoDnsResult[]>(`/geodns/marketplaces?lat=${lat}&lng=${lng}&radius_km=${radiusKm}&limit=${limit}`);
  }

  // === Companies (B2B Multi-Tenant) ===
  async listCompanies(): Promise<Company[]> { return this.get<Company[]>('/companies'); }
  async createCompany(data: CompanyCreate): Promise<Company> { return this.post<Company>('/companies', data); }
  async getCompany(id: string): Promise<Company> { return this.get<Company>(`/companies/${encodeURIComponent(id)}`); }
  async updateCompany(id: string, data: CompanyUpdate): Promise<Company> { return this.patch<Company>(`/companies/${encodeURIComponent(id)}`, data); }
  async listCompaniesByDomain(domain: string): Promise<Company[]> { return this.get<Company[]>(`/companies?domain=${encodeURIComponent(domain)}`); }

  // === BeckN Orders ===
  async listOrders(): Promise<Order[]> { return this.get<Order[]>('/orders'); }
  async createOrder(data: OrderCreate): Promise<Order> { return this.post<Order>('/orders', data); }
  async getOrder(id: string): Promise<Order> { return this.get<Order>(`/orders/${encodeURIComponent(id)}`); }
  async updateOrder(id: string, data: OrderUpdate): Promise<Order> { return this.patch<Order>(`/orders/${encodeURIComponent(id)}`, data); }
  async deleteOrder(id: string): Promise<void> { return this.delete<void>(`/orders/${encodeURIComponent(id)}`); }

  // === BeckN BAPs ===
  async listBaps(): Promise<Bap[]> { return this.get<Bap[]>('/baps'); }
  async listBapsByCompany(companyId: string): Promise<Bap[]> { return this.get<Bap[]>(`/baps?company_id=${encodeURIComponent(companyId)}`); }
  async listBapsByCountry(country: string): Promise<Bap[]> { return this.get<Bap[]>(`/baps?country=${encodeURIComponent(country)}`); }
  async listBapsNearby(lat: number, lng: number, radiusKm = 50): Promise<Bap[]> { return this.get<Bap[]>(`/baps/nearby?lat=${lat}&lng=${lng}&radius_km=${radiusKm}`); }
  async createBap(data: BapCreate): Promise<Bap> { return this.post<Bap>('/baps', data); }
  async getBap(id: string): Promise<Bap> { return this.get<Bap>(`/baps/${encodeURIComponent(id)}`); }
  async updateBap(id: string, data: BapUpdate): Promise<Bap> { return this.patch<Bap>(`/baps/${encodeURIComponent(id)}`, data); }

  // === BeckN BPPs ===
  async listBpps(): Promise<Bpp[]> { return this.get<Bpp[]>('/bpps'); }
  async listBppsByCompany(companyId: string): Promise<Bpp[]> { return this.get<Bpp[]>(`/bpps?company_id=${encodeURIComponent(companyId)}`); }
  async listBppsByCountry(country: string): Promise<Bpp[]> { return this.get<Bpp[]>(`/bpps?country=${encodeURIComponent(country)}`); }
  async listBppsNearby(lat: number, lng: number, radiusKm = 50): Promise<Bpp[]> { return this.get<Bpp[]>(`/bpps/nearby?lat=${lat}&lng=${lng}&radius_km=${radiusKm}`); }
  async createBpp(data: BppCreate): Promise<Bpp> { return this.post<Bpp>('/bpps', data); }
  async getBpp(id: string): Promise<Bpp> { return this.get<Bpp>(`/bpps/${encodeURIComponent(id)}`); }
  async updateBpp(id: string, data: BppUpdate): Promise<Bpp> { return this.patch<Bpp>(`/bpps/${encodeURIComponent(id)}`, data); }

  // === BeckN Items ===
  async listItems(): Promise<Item[]> { return this.get<Item[]>('/items'); }
  async createItem(data: ItemCreate): Promise<Item> { return this.post<Item>('/items', data); }
  async getItem(id: string): Promise<Item> { return this.get<Item>(`/items/${encodeURIComponent(id)}`); }

  // === BeckN Providers ===
  async listProviders(): Promise<Provider[]> { return this.get<Provider[]>('/providers'); }
  async createProvider(data: ProviderCreate): Promise<Provider> { return this.post<Provider>('/providers', data); }
  async getProvider(id: string): Promise<Provider> { return this.get<Provider>(`/providers/${encodeURIComponent(id)}`); }
  async deleteProvider(id: string): Promise<void> { return this.delete<void>(`/providers/${encodeURIComponent(id)}`); }

  // === BeckN Fulfillments ===
  async listFulfillments(): Promise<Fulfillment[]> { return this.get<Fulfillment[]>('/fulfillments'); }
  async createFulfillment(data: FulfillmentCreate): Promise<Fulfillment> { return this.post<Fulfillment>('/fulfillments', data); }
  async trackFulfillment(trackingId: string): Promise<Fulfillment[]> { return this.post<Fulfillment[]>('/fulfillments/track', { tracking_id: trackingId }); }

  // === BeckN Subscriptions ===
  async listSubscriptions(): Promise<Subscription[]> { return this.get<Subscription[]>('/subscriptions'); }
  async createSubscription(data: SubscriptionCreate): Promise<Subscription> { return this.post<Subscription>('/subscriptions', data); }
  async getSubscription(id: string): Promise<Subscription> { return this.get<Subscription>(`/subscriptions/${encodeURIComponent(id)}`); }
  async listActiveSubscriptions(): Promise<Subscription[]> { return this.get<Subscription[]>('/subscriptions/active'); }
  async cancelSubscription(id: string, data?: Record<string, unknown>): Promise<Subscription> { return this.post(`/subscriptions/${encodeURIComponent(id)}/cancel`, data || {}); }
  async renewSubscription(id: string, data?: Record<string, unknown>): Promise<Subscription> { return this.post(`/subscriptions/${encodeURIComponent(id)}/renew`, data || {}); }

  // === BeckN API Keys ===
  async listApiKeys(): Promise<ApiKey[]> { return this.get<ApiKey[]>('/api-keys'); }
  async createApiKey(data: ApiKeyCreate): Promise<ApiKeyResponse> { return this.post<ApiKeyResponse>('/api-keys', data); }
  async getApiKey(id: string): Promise<ApiKey> { return this.get<ApiKey>(`/api-keys/${encodeURIComponent(id)}`); }
  async verifyApiKey(secret: string): Promise<ApiKeyVerifyResponse> { return this.post<ApiKeyVerifyResponse>('/api-keys/verify', { secret } as ApiKeyVerifyRequest); }
  async listApiKeysByOwner(ownerId: string): Promise<ApiKey[]> { return this.get<ApiKey[]>(`/api-keys/owner/${encodeURIComponent(ownerId)}`); }
  async revokeApiKey(id: string): Promise<ApiKey> { return this.post<ApiKey>(`/api-keys/${encodeURIComponent(id)}/revoke`, {}); }
  async rotateApiKey(id: string): Promise<ApiKeyResponse> { return this.post<ApiKeyResponse>(`/api-keys/${encodeURIComponent(id)}/rotate`, {}); }

  // === A2A API ===
  async listAgentCards(): Promise<AgentCard[]> { return this.get<AgentCard[]>('/a2a/agent-cards'); }
  async registerAgentCard(data: AgentCardRegister): Promise<AgentCard> { return this.post<AgentCard>('/a2a/agent-cards', data); }
  async getAgentCard(id: string): Promise<AgentCard> { return this.get<AgentCard>(`/a2a/agent-cards/${encodeURIComponent(id)}`); }

  async listTasks(): Promise<A2ATask[]> { return this.get<A2ATask[]>('/a2a/tasks'); }
  async createTask(data: A2ATaskCreate): Promise<A2ATask> { return this.post<A2ATask>('/a2a/tasks', data); }
  async getTask(id: string): Promise<A2ATask> { return this.get<A2ATask>(`/a2a/tasks/${encodeURIComponent(id)}`); }

  async listMessages(): Promise<A2ATaskMessage[]> { return this.get<A2ATaskMessage[]>('/a2a/messages'); }
  async sendMessage(data: A2ATaskMessageSend): Promise<A2ATaskMessage> { return this.post<A2ATaskMessage>('/a2a/messages', data); }

  async listArtifacts(): Promise<A2AArtifact[]> { return this.get<A2AArtifact[]>('/a2a/artifacts'); }
  async createArtifact(data: A2AArtifactCreate): Promise<A2AArtifact> { return this.post<A2AArtifact>('/a2a/artifacts', data); }

  // === MCP API ===
  async listTools(): Promise<McpTool[]> { return this.get<McpTool[]>('/mcp/tools'); }
  async createTool(data: McpToolCreate): Promise<McpTool> { return this.post<McpTool>('/mcp/tools', data); }
  async listResources(): Promise<McpResource[]> { return this.get<McpResource[]>('/mcp/resources'); }
  async createResource(data: McpResourceCreate): Promise<McpResource> { return this.post<McpResource>('/mcp/resources', data); }
  async listPrompts(): Promise<McpPrompt[]> { return this.get<McpPrompt[]>('/mcp/prompts'); }
  async createPrompt(data: McpPromptCreate): Promise<McpPrompt> { return this.post<McpPrompt>('/mcp/prompts', data); }
  async listClients(): Promise<IMcpClient[]> { return this.get<IMcpClient[]>('/mcp/clients'); }
  async registerClient(data: McpClientRegister): Promise<IMcpClient> { return this.post<IMcpClient>('/mcp/clients', data); }

  // === ACP API ===
  async listIssuers(): Promise<AcpIssuer[]> { return this.get<AcpIssuer[]>('/acp/issuers'); }
  async registerIssuer(data: AcpIssuerRegister): Promise<AcpIssuer> { return this.post<AcpIssuer>('/acp/issuers', data); }
  async getIssuer(id: string): Promise<AcpIssuer> { return this.get<AcpIssuer>(`/acp/issuers/${encodeURIComponent(id)}`); }

  async listTokens(): Promise<AcpToken[]> { return this.get<AcpToken[]>('/acp/tokens'); }
  async issueToken(data: AcpTokenIssue): Promise<AcpToken> { return this.post<AcpToken>('/acp/tokens', data); }
  async introspectToken(token: string): Promise<AcpToken> { return this.post<AcpToken>('/acp/tokens/introspect', { token }); }

  async listPresentations(): Promise<AcpPresentation[]> { return this.get<AcpPresentation[]>('/acp/presentations'); }
  async submitPresentation(data: AcpPresentationSubmit): Promise<AcpPresentation> { return this.post<AcpPresentation>('/acp/presentations', data); }

  async listPolicies(): Promise<AcpAccessPolicy[]> { return this.get<AcpAccessPolicy[]>('/acp/policies'); }
  async createPolicy(data: AcpAccessPolicyCreate): Promise<AcpAccessPolicy> { return this.post<AcpAccessPolicy>('/acp/policies', data); }

  // === ANP API ===
  async listAnnouncements(): Promise<AnpAnnouncement[]> { return this.get<AnpAnnouncement[]>('/anp/announcements'); }
  async announce(data: AnpAnnouncementCreate): Promise<AnpAnnouncement> { return this.post<AnpAnnouncement>('/anp/announcements', data); }
  async registerWitness(data: AnpWitnessRegister): Promise<AnpWitness> { return this.post<AnpWitness>('/anp/witnesses', data); }

  async verify(data: AnpVerificationVerify): Promise<AnpVerification> { return this.post<AnpVerification>('/anp/verifications', data); }

  // === GBP (Google Business Profile) ===
  async listGbpAccounts(): Promise<GbpAccount[]> { return this.get<GbpAccount[]>('/gbp/accounts'); }
  async createGbpAccount(data: GbpAccountCreate): Promise<GbpAccount> { return this.post<GbpAccount>('/gbp/accounts', data); }
  async syncGbpLocations(accountId: string): Promise<{ bpps: Bpp[] }> { return this.post<{ bpps: Bpp[] }>(`/gbp/accounts/${encodeURIComponent(accountId)}/sync`, {}); }

  // === Health ===
  async health(): Promise<{ status: string }> { return this.get<{ status: string }>('/health'); }
}
