"""TypeScript and Python type definitions for the BeckN Protocol SDK."""

from __future__ import annotations
from dataclasses import dataclass, field
from typing import Any, Optional
from enum import Enum


class OrderState(str, Enum):
    PENDING = "pending"
    ACCEPTED = "accepted"
    IN_FULFILLMENT = "in_fulfillment"
    COMPLETED = "completed"
    CANCELLED = "cancelled"


class SubscriptionStatus(str, Enum):
    TRIALING = "trialing"
    ACTIVE = "active"
    PAST_DUE = "past_due"
    CANCELED = "canceled"
    UNPAID = "unpaid"


class SubscriberType(str, Enum):
    BAP = "bap"
    BPP = "bpp"


class TaskStatus(str, Enum):
    SUBMITTED = "submitted"
    WORKING = "working"
    INPUT_REQUIRED = "input_required"
    COMPLETED = "completed"
    FAILED = "failed"
    CANCELED = "canceled"


class TransportType(str, Enum):
    STDIO = "stdio"
    STREAMABLE_HTTP = "streamable_http"


class MessageRole(str, Enum):
    USER = "user"
    AGENT = "agent"
    SYSTEM = "system"


@dataclass
class Bap:
    id: str
    name: str
    did: Optional[str] = None
    endpoint: Optional[str] = None
    public_key: Optional[str] = None
    active: bool = True
    capabilities: list[str] = field(default_factory=list)
    country: Optional[str] = None
    city: Optional[str] = None
    region: Optional[str] = None
    lat: Optional[float] = None
    lon: Optional[float] = None
    company_id: Optional[str] = None


@dataclass
class BapCreate:
    id: str
    name: str
    did: Optional[str] = None
    endpoint: Optional[str] = None
    public_key: Optional[str] = None
    active: bool = True
    capabilities: list[str] = field(default_factory=list)
    country: Optional[str] = None
    city: Optional[str] = None
    region: Optional[str] = None
    lat: Optional[float] = None
    lon: Optional[float] = None
    company_id: Optional[str] = None


@dataclass
class BapUpdate:
    name: Optional[str] = None
    endpoint: Optional[str] = None
    public_key: Optional[str] = None
    active: Optional[bool] = None
    capabilities: Optional[list[str]] = None
    country: Optional[str] = None
    city: Optional[str] = None
    region: Optional[str] = None
    lat: Optional[float] = None
    lon: Optional[float] = None
    company_id: Optional[str] = None


@dataclass
class Bpp:
    id: str
    name: str
    did: Optional[str] = None
    endpoint: Optional[str] = None
    public_key: Optional[str] = None
    active: bool = True
    capabilities: list[str] = field(default_factory=list)
    country: Optional[str] = None
    city: Optional[str] = None
    symbol: Optional[str] = None
    currency: Optional[str] = None
    region: Optional[str] = None
    lat: Optional[float] = None
    lon: Optional[float] = None
    company_id: Optional[str] = None


@dataclass
class BppCreate:
    id: str
    name: str
    did: Optional[str] = None
    endpoint: Optional[str] = None
    public_key: Optional[str] = None
    active: bool = True
    capabilities: list[str] = field(default_factory=list)
    country: Optional[str] = None
    city: Optional[str] = None
    symbol: Optional[str] = None
    currency: Optional[str] = None
    region: Optional[str] = None
    lat: Optional[float] = None
    lon: Optional[float] = None
    company_id: Optional[str] = None


@dataclass
class BppUpdate:
    name: Optional[str] = None
    endpoint: Optional[str] = None
    public_key: Optional[str] = None
    active: Optional[bool] = None
    capabilities: Optional[list[str]] = None
    country: Optional[str] = None
    city: Optional[str] = None
    region: Optional[str] = None
    lat: Optional[float] = None
    lon: Optional[float] = None
    company_id: Optional[str] = None


@dataclass
class Order:
    id: str
    order_state: Optional[OrderState] = None
    provider_id: Optional[str] = None
    bpp_id: Optional[str] = None
    bap_id: Optional[str] = None
    transaction_id: Optional[str] = None
    billing: Optional[dict[str, Any]] = None
    fulfillments: list[Any] = field(default_factory=list)
    quote: Optional[dict[str, Any]] = None
    payment: Optional[dict[str, Any]] = None
    items: list[Any] = field(default_factory=list)
    created_at: Optional[str] = None
    updated_at: Optional[str] = None


@dataclass
class OrderCreate:
    id: str
    order_state: Optional[OrderState] = None
    provider_id: Optional[str] = None
    bpp_id: Optional[str] = None
    bap_id: Optional[str] = None
    transaction_id: Optional[str] = None
    billing: Optional[dict[str, Any]] = None
    fulfillments: list[Any] = field(default_factory=list)
    quote: Optional[dict[str, Any]] = None
    payment: Optional[dict[str, Any]] = None
    items: list[Any] = field(default_factory=list)


@dataclass
class OrderUpdate:
    order_state: Optional[OrderState] = None
    provider_id: Optional[str] = None
    billing: Optional[dict[str, Any]] = None
    fulfillments: Optional[list[Any]] = None
    quote: Optional[dict[str, Any]] = None
    payment: Optional[dict[str, Any]] = None


@dataclass
class Item:
    id: str
    provider_id: Optional[str] = None
    descriptor: Optional[dict[str, Any]] = None
    price: Optional[dict[str, Any]] = None
    category_id: Optional[str] = None
    location_id: Optional[str] = None


@dataclass
class ItemCreate:
    provider_id: Optional[str] = None
    descriptor: Optional[dict[str, Any]] = None
    price: Optional[dict[str, Any]] = None
    category_id: Optional[str] = None
    location_id: Optional[str] = None


@dataclass
class Provider:
    id: str
    descriptor: Optional[dict[str, Any]] = None
    locations: list[Any] = field(default_factory=list)
    categories: list[Any] = field(default_factory=list)
    items: list[Any] = field(default_factory=list)


@dataclass
class ProviderCreate:
    id: str
    descriptor: Optional[dict[str, Any]] = None
    locations: list[Any] = field(default_factory=list)
    categories: list[Any] = field(default_factory=list)
    items: list[Any] = field(default_factory=list)


@dataclass
class Fulfillment:
    id: str
    order_id: Optional[str] = None
    fulfillment_type: Optional[str] = None
    start: Optional[dict[str, Any]] = None
    end: Optional[dict[str, Any]] = None
    tracking: bool = False
    tracking_id: Optional[str] = None
    instructions: Optional[dict[str, Any]] = None


@dataclass
class FulfillmentCreate:
    order_id: Optional[str] = None
    fulfillment_type: Optional[str] = None
    start: Optional[dict[str, Any]] = None
    end: Optional[dict[str, Any]] = None
    tracking: bool = False
    tracking_id: Optional[str] = None
    instructions: Optional[dict[str, Any]] = None


@dataclass
class Subscription:
    id: str
    subscriber_id: str
    subscriber_type: SubscriberType
    plan_id: Optional[str] = None
    status: SubscriptionStatus = SubscriptionStatus.TRIALING
    trial_ends_at: Optional[str] = None
    current_period_start: Optional[str] = None
    current_period_end: Optional[str] = None
    cancel_at_period_end: bool = False
    metadata: Optional[dict[str, Any]] = None


@dataclass
class SubscriptionCreate:
    id: str
    subscriber_id: str
    subscriber_type: SubscriberType
    plan_id: Optional[str] = None
    status: SubscriptionStatus = SubscriptionStatus.TRIALING
    trial_ends_at: Optional[str] = None
    current_period_start: Optional[str] = None
    current_period_end: Optional[str] = None
    cancel_at_period_end: bool = False
    metadata: Optional[dict[str, Any]] = None
    activate: bool = False


@dataclass
class ApiKey:
    id: str = ""
    description: Optional[str] = None
    owner: Optional[str] = None
    scopes: list[str] = field(default_factory=list)
    active: bool = True
    expires_at: Optional[str] = None
    last_used_at: Optional[str] = None


@dataclass
class ApiKeyCreate:
    description: Optional[str] = None
    owner: Optional[str] = None
    scopes: list[str] = field(default_factory=list)
    expires_at: Optional[str] = None
    active: bool = True
    metadata: Optional[dict[str, Any]] = None


@dataclass
class ApiKeyResponse:
    id: str
    secret: Optional[str] = None
    description: Optional[str] = None
    owner: Optional[str] = None
    scopes: list[str] = field(default_factory=list)
    active: bool = True
    expires_at: Optional[str] = None


@dataclass
class ApiKeyVerifyRequest:
    secret: str


@dataclass
class ApiKeyVerifyResponse:
    valid: bool
    owner: Optional[str] = None
    scopes: Optional[list[str]] = None
    expires_at: Optional[str] = None


@dataclass
class ErrorResponse:
    error: Optional[str] = None
    message: Optional[str] = None


@dataclass
class Company:
    id: str
    name: str
    domain: Optional[str] = None
    country: Optional[str] = None
    active: bool = True
    metadata: Optional[dict[str, Any]] = None


@dataclass
class CompanyCreate:
    name: str
    domain: Optional[str] = None
    country: Optional[str] = None
    metadata: Optional[dict[str, Any]] = None


@dataclass
class GeoLocation:
    lat: float
    lng: float
    country: str
    region: Optional[str] = None
    city: Optional[str] = None


@dataclass
class GeoDnsResult:
    hostname: str
    endpoints: list[str]
    location: GeoLocation
    latency_ms: Optional[int] = None


@dataclass
class GeoDiscoveryConfig:
    timeout: int = 10000
    max_results: int = 10


# A2A Types
@dataclass
class AgentCard:
    agent_id: str
    name: str
    url: str
    description: Optional[str] = None
    version: str = "1.0.0"
    capabilities: Optional[dict[str, Any]] = None
    skills: list[Any] = field(default_factory=list)
    default_input_modes: list[str] = field(default_factory=lambda: ["text"])
    default_output_modes: list[str] = field(default_factory=lambda: ["text"])
    authentication: Optional[dict[str, Any]] = None
    country: Optional[str] = None


@dataclass
class AgentCardRegister:
    agent_id: str
    name: str
    url: str
    description: Optional[str] = None
    version: str = "1.0.0"
    capabilities: Optional[dict[str, Any]] = None
    skills: list[Any] = field(default_factory=list)
    default_input_modes: list[str] = field(default_factory=lambda: ["text"])
    default_output_modes: list[str] = field(default_factory=lambda: ["text"])
    authentication: Optional[dict[str, Any]] = None
    country: Optional[str] = None


@dataclass
class A2ATask:
    task_id: str
    context_id: str
    status: Optional[TaskStatus] = None
    history: list[Any] = field(default_factory=list)
    artifacts: list[Any] = field(default_factory=list)


@dataclass
class A2ATaskCreate:
    task_id: str
    context_id: str
    status: Optional[TaskStatus] = None
    history: list[Any] = field(default_factory=list)
    artifacts: list[Any] = field(default_factory=list)


@dataclass
class A2ATaskMessage:
    message_id: str
    role: MessageRole
    parts: list[Any]
    context_id: Optional[str] = None
    task_id: Optional[str] = None


@dataclass
class A2ATaskMessageSend:
    message_id: str
    role: MessageRole
    parts: list[Any]
    context_id: Optional[str] = None
    task_id: Optional[str] = None


@dataclass
class A2AArtifact:
    artifact_id: str
    name: Optional[str] = None
    description: Optional[str] = None
    task_id: Optional[str] = None
    parts: list[Any] = field(default_factory=list)


@dataclass
class A2AArtifactCreate:
    artifact_id: str
    task_id: Optional[str] = None
    name: Optional[str] = None
    description: Optional[str] = None
    parts: list[Any] = field(default_factory=list)


# MCP Types
@dataclass
class McpTool:
    name: str
    description: Optional[str] = None
    input_schema: Optional[dict[str, Any]] = None
    handler: Optional[str] = None


@dataclass
class McpToolCreate:
    name: str
    description: Optional[str] = None
    input_schema: Optional[dict[str, Any]] = None
    handler: Optional[str] = None


@dataclass
class McpResource:
    name: str
    uri: Optional[str] = None
    mime_type: Optional[str] = None


@dataclass
class McpResourceCreate:
    name: str
    uri: Optional[str] = None
    mime_type: Optional[str] = None
    text: Optional[str] = None


@dataclass
class McpPrompt:
    name: str
    description: Optional[str] = None


@dataclass
class McpPromptCreate:
    name: str
    description: Optional[str] = None
    arguments: Optional[list[Any]] = None


@dataclass
class McpClientType:
    client_info: Optional[dict[str, Any]] = None
    capabilities: Optional[dict[str, Any]] = None
    transport: Optional[TransportType] = None


@dataclass
class McpClientRegister:
    client_info: Optional[dict[str, Any]] = None
    capabilities: Optional[dict[str, Any]] = None
    transport: Optional[TransportType] = None


# ACP Types
@dataclass
class AcpIssuer:
    issuer_id: str
    name: Optional[str] = None
    authorization_endpoint: Optional[str] = None
    token_endpoint: Optional[str] = None
    jwks_uri: Optional[str] = None


@dataclass
class AcpIssuerRegister:
    issuer_id: str
    name: Optional[str] = None
    authorization_endpoint: Optional[str] = None
    token_endpoint: Optional[str] = None
    jwks_uri: Optional[str] = None


@dataclass
class AcpToken:
    token_value: str
    token_type: str = "Bearer"
    scope: Optional[str] = None
    subject: Optional[str] = None
    audience: Optional[str] = None
    revoked: bool = False


@dataclass
class AcpTokenIssue:
    token_value: str
    token_type: str = "Bearer"
    scope: Optional[str] = None
    expires_at: Optional[str] = None
    credential_issuer_id: Optional[str] = None
    subject: Optional[str] = None
    audience: Optional[str] = None


@dataclass
class AcpPresentation:
    presentation_id: str
    holder_did: str
    issuer_id: str
    claims: dict[str, Any]
    verified: bool = False


@dataclass
class AcpPresentationSubmit:
    presentation_id: str
    holder_did: str
    issuer_id: str
    claims: dict[str, Any]
    signature: Optional[str] = None


@dataclass
class AcpAccessPolicy:
    policy_id: str
    action: str
    resource: str
    scope_required: Optional[str] = None


@dataclass
class AcpAccessPolicyCreate:
    policy_id: str
    action: str
    resource: str
    scope_required: Optional[str] = None
    audience: Optional[str] = None


# ANP Types
@dataclass
class AnpAnnouncement:
    announcement_id: str
    did: str
    did_document_hash: Optional[str] = None
    service_endpoint: Optional[str] = None
    ttl: int = 3600


@dataclass
class AnpAnnouncementCreate:
    announcement_id: str
    did: str
    did_document_hash: Optional[str] = None
    service_endpoint: Optional[str] = None
    timestamp: Optional[str] = None
    ttl: int = 3600
    proof: Optional[dict[str, Any]] = None


@dataclass
class AnpWitness:
    witness_id: str
    did: Optional[str] = None
    endpoint: Optional[str] = None
    protocols: list[str] = field(default_factory=list)


@dataclass
class AnpWitnessRegister:
    witness_id: str
    did: Optional[str] = None
    endpoint: Optional[str] = None
    protocols: list[str] = field(default_factory=list)
    announcements: Optional[dict[str, Any]] = None


@dataclass
class AnpVerification:
    proof_id: str
    did: str
    verification_method: str
    valid: bool = False


@dataclass
class AnpVerificationVerify:
    proof_id: str
    did: str
    verification_method: str


@dataclass
class BeckNClientConfig:
    base_url: str = "http://localhost:4000/v1"
    api_key: Optional[str] = None
    timeout: int = 30000
