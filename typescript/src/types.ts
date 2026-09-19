export type OrderState = 'pending' | 'accepted' | 'in_fulfillment' | 'completed' | 'cancelled';
export type SubscriptionStatus = 'trialing' | 'active' | 'past_due' | 'canceled' | 'unpaid';
export type SubscriberType = 'bap' | 'bpp';
export type TaskStatus = 'submitted' | 'working' | 'input_required' | 'completed' | 'failed' | 'canceled';
export type TransportType = 'stdio' | 'streamable_http';
export type MessageRole = 'user' | 'agent' | 'system';

// BeckN Types
export interface Company { id: string; name: string; domain?: string; country?: string; active?: boolean; metadata?: Record<string, any>; created_at?: string; updated_at?: string }
export interface CompanyCreate { name: string; domain?: string; country?: string; metadata?: Record<string, any> }
export interface CompanyUpdate { name?: string; domain?: string; country?: string; active?: boolean; metadata?: Record<string, any> }

export interface GbpAccount { id: string; email: string; account_name?: string; expires_at?: string; created_at?: string; updated_at?: string }
export interface GbpAccountCreate { email: string; account_name?: string; refresh_token?: string }
export interface Bap { id: string; name: string; did?: string; endpoint?: string; public_key?: string; active?: boolean; capabilities?: string[]; country?: string; city?: string; region?: string; lat?: number; lon?: number; company_id?: string }
export interface BapCreate { id: string; name: string; did?: string; endpoint?: string; public_key?: string; active?: boolean; capabilities?: string[]; country?: string; city?: string; region?: string; lat?: number; lon?: number; company_id?: string }
export interface BapUpdate { name?: string; endpoint?: string; public_key?: string; active?: boolean; capabilities?: string[]; country?: string; city?: string; region?: string; lat?: number; lon?: number; company_id?: string }
export interface BapUpdate { name?: string; endpoint?: string; public_key?: string; active?: boolean; capabilities?: string[]; country?: string; city?: string; region?: string; lat?: number; lon?: number; company_id?: string }

export interface Bpp { id: string; name: string; did?: string; endpoint?: string; public_key?: string; active?: boolean; capabilities?: string[]; country?: string; city?: string; symbol?: string; currency?: string; region?: string; lat?: number; lon?: number; company_id?: string }
export interface BppCreate { id: string; name: string; did?: string; endpoint?: string; public_key?: string; active?: boolean; capabilities?: string[]; country?: string; city?: string; symbol?: string; currency?: string; region?: string; lat?: number; lon?: number; company_id?: string }
export interface BppUpdate { name?: string; endpoint?: string; public_key?: string; active?: boolean; capabilities?: string[]; country?: string; city?: string; region?: string; lat?: number; lon?: number; company_id?: string }

export interface Order { id: string; order_state?: OrderState; provider_id?: string; bpp_id?: string; bap_id?: string; transaction_id?: string; billing?: Record<string, any>; fulfillments?: any[]; quote?: Record<string, any>; payment?: Record<string, any>; items?: any[]; created_at?: string; updated_at?: string }
export interface OrderCreate { id: string; order_state?: OrderState; provider_id?: string; bpp_id?: string; bap_id?: string; transaction_id?: string; billing?: Record<string, any>; fulfillments?: any[]; quote?: Record<string, any>; payment?: Record<string, any>; items?: any[] }
export interface OrderUpdate { order_state?: OrderState; provider_id?: string; billing?: Record<string, any>; fulfillments?: any[]; quote?: Record<string, any>; payment?: Record<string, any> }

export interface Item { id: string; provider_id?: string; descriptor?: Record<string, any>; price?: Record<string, any>; category_id?: string; location_id?: string; created_at?: string; updated_at?: string }
export interface ItemCreate { provider_id?: string; descriptor?: Record<string, any>; price?: Record<string, any>; category_id?: string; location_id?: string }

export interface Provider { id: string; descriptor?: Record<string, any>; locations?: any[]; categories?: any[]; items?: any[]; created_at?: string; updated_at?: string }
export interface ProviderCreate { id: string; descriptor?: Record<string, any>; locations?: any[]; categories?: any[]; items?: any[] }

export interface Fulfillment { id: string; order_id?: string; fulfillment_type?: string; start?: Record<string, any>; end?: Record<string, any>; tracking?: boolean; tracking_id?: string; instructions?: Record<string, any>; created_at?: string; updated_at?: string }
export interface FulfillmentCreate { order_id?: string; fulfillment_type?: string; start?: Record<string, any>; end?: Record<string, any>; tracking?: boolean; tracking_id?: string; instructions?: Record<string, any> }

export interface Subscription { id: string; subscriber_id: string; subscriber_type: SubscriberType; plan_id?: string; status?: SubscriptionStatus; trial_ends_at?: string; current_period_start?: string; current_period_end?: string; cancel_at_period_end?: boolean; metadata?: Record<string, any>; created_at?: string; updated_at?: string }
export interface SubscriptionCreate { id: string; subscriber_id: string; subscriber_type: SubscriberType; plan_id?: string; status?: SubscriptionStatus; trial_ends_at?: string; current_period_start?: string; current_period_end?: string; cancel_at_period_end?: boolean; metadata?: Record<string, any>; activate?: boolean }

export interface ApiKey { id: string; description?: string; owner?: string; scopes?: string[]; active?: boolean; expires_at?: string; last_used_at?: string; metadata?: Record<string, any>; created_at?: string; updated_at?: string }
export interface ApiKeyCreate { description?: string; owner?: string; scopes?: string[]; expires_at?: string; active?: boolean; metadata?: Record<string, any> }
export interface ApiKeyResponse extends ApiKey { secret?: string }
export interface ApiKeyVerifyRequest { secret: string }
export interface ApiKeyVerifyResponse { valid: boolean; owner?: string; scopes?: string[]; expires_at?: string }

export interface ErrorResponse { error?: string; message?: string }

export type AnyBeckN = Bap | Bpp | Order | Item | Provider | Fulfillment | Subscription | ApiKey;

// GeoDNS Discovery
export interface GeoLocation { lat: number; lng: number; country: string; region?: string; city?: string }
export interface GeoDnsResult { hostname: string; endpoints: string[]; location: GeoLocation; latency_ms?: number }
export interface GeoDiscoveryConfig { timeout?: number; maxResults?: number }

// A2A Types
export interface AgentCard { id?: string; agent_id: string; name: string; description?: string; url: string; version?: string; capabilities?: Record<string, any>; skills?: any[]; default_input_modes?: string[]; default_output_modes?: string[]; authentication?: Record<string, any>; country?: string; region?: string; created_at?: string; updated_at?: string }
export interface AgentCardRegister { agent_id: string; name: string; description?: string; url: string; version?: string; capabilities?: Record<string, any>; skills?: any[]; default_input_modes?: string[]; default_output_modes?: string[]; authentication?: Record<string, any>; country?: string; region?: string }

export interface A2ATask { id?: string; task_id: string; context_id: string; status?: TaskStatus; history?: any[]; artifacts?: any[]; metadata?: Record<string, any>; created_at?: string; updated_at?: string }
export interface A2ATaskCreate { task_id: string; context_id: string; status?: TaskStatus; history?: any[]; artifacts?: any[]; metadata?: Record<string, any> }

export interface A2ATaskMessage { id?: string; message_id: string; context_id?: string; task_id?: string; role: MessageRole; parts: any[]; metadata?: Record<string, any>; created_at?: string; updated_at?: string }
export interface A2ATaskMessageSend { message_id: string; context_id?: string; task_id?: string; role: MessageRole; parts: any[]; metadata?: Record<string, any> }

export interface A2AArtifact { id?: string; artifact_id: string; task_id?: string; name?: string; description?: string; parts?: any[]; metadata?: Record<string, any>; created_at?: string; updated_at?: string }
export interface A2AArtifactCreate { artifact_id: string; task_id?: string; name?: string; description?: string; parts?: any[]; metadata?: Record<string, any> }

// MCP Types
export interface McpTool { id?: string; name: string; description?: string; input_schema?: Record<string, any>; annotations?: Record<string, any>; handler?: string; created_at?: string; updated_at?: string }
export interface McpToolCreate { name: string; description?: string; input_schema?: Record<string, any>; annotations?: Record<string, any>; handler?: string }

export interface McpResource { id?: string; name: string; description?: string; uri?: string; mime_type?: string; text?: string; created_at?: string; updated_at?: string }
export interface McpResourceCreate { name: string; description?: string; uri?: string; mime_type?: string; text?: string }

export interface McpPrompt { id?: string; name: string; description?: string; arguments?: any[]; messages?: any[]; created_at?: string; updated_at?: string }
export interface McpPromptCreate { name: string; description?: string; arguments?: any[]; messages?: any[] }

export interface McpClient { id?: string; client_info?: Record<string, any>; capabilities?: Record<string, any>; transport?: TransportType; created_at?: string; updated_at?: string }
export interface McpClientRegister { client_info?: Record<string, any>; capabilities?: Record<string, any>; transport?: TransportType }

// ACP Types
export interface AcpIssuer { id?: string; issuer_id: string; name?: string; authorization_endpoint?: string; token_endpoint?: string; jwks_uri?: string; revocation_endpoint?: string; introspection_endpoint?: string; userinfo_endpoint?: string; scopes?: string[]; claims?: Record<string, any>; created_at?: string; updated_at?: string }
export interface AcpIssuerRegister { issuer_id: string; name?: string; authorization_endpoint?: string; token_endpoint?: string; jwks_uri?: string; revocation_endpoint?: string; introspection_endpoint?: string; userinfo_endpoint?: string; scopes?: string[]; claims?: Record<string, any> }

export interface AcpToken { id?: string; token_value: string; token_type?: string; scope?: string; expires_at?: string; credential_issuer_id?: string; subject?: string; audience?: string; revoked?: boolean; created_at?: string; updated_at?: string }
export interface AcpTokenIssue { id?: string; token_value: string; token_type?: string; scope?: string; expires_at?: string; credential_issuer_id?: string; subject?: string; audience?: string }

export interface AcpPresentation { id?: string; presentation_id: string; holder_did: string; issuer_id: string; claims: Record<string, any>; signature?: string; verified?: boolean; created_at?: string; updated_at?: string }
export interface AcpPresentationSubmit { presentation_id: string; holder_did: string; issuer_id: string; claims: Record<string, any>; signature?: string }

export interface AcpAccessPolicy { id?: string; policy_id: string; action: string; resource: string; scope_required?: string; audience?: string; created_at?: string; updated_at?: string }
export interface AcpAccessPolicyCreate { policy_id: string; action: string; resource: string; scope_required?: string; audience?: string }

// ANP Types
export interface AnpAnnouncement { id?: string; announcement_id: string; did: string; did_document_hash?: string; service_endpoint?: string; timestamp?: string; ttl?: number; proof?: Record<string, any>; created_at?: string; updated_at?: string }
export interface AnpAnnouncementCreate { announcement_id: string; did: string; did_document_hash?: string; service_endpoint?: string; timestamp?: string; ttl?: number; proof?: Record<string, any> }

export interface AnpWitness { id?: string; witness_id: string; did?: string; endpoint?: string; protocols?: string[]; announcements?: Record<string, any>; created_at?: string; updated_at?: string }
export interface AnpWitnessRegister { witness_id: string; did?: string; endpoint?: string; protocols?: string[]; announcements?: Record<string, any> }

export interface AnpVerification { id?: string; proof_id: string; did: string; verification_method: string; valid?: boolean; created_at?: string; updated_at?: string }
export interface AnpVerificationVerify { proof_id: string; did: string; verification_method: string }

export interface BeckNClientConfig { baseUrl?: string; apiKey?: string; timeout?: number }
