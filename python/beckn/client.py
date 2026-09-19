"""Client module for the BeckN Protocol SDK."""

from __future__ import annotations

import json
import os
from typing import Any, Optional, TypeVar
from dataclasses import asdict

import httpx

from .types import (
    Bap, BapCreate, BapUpdate, Bpp, BppCreate, BppUpdate,
    Order, OrderCreate, OrderUpdate, Item, ItemCreate,
    Provider, ProviderCreate, Fulfillment, FulfillmentCreate,
    Subscription, SubscriptionCreate,
    ApiKey, ApiKeyCreate, ApiKeyResponse, ApiKeyVerifyRequest, ApiKeyVerifyResponse,
    ErrorResponse, GeoDnsResult, GeoLocation,
    AgentCard, AgentCardRegister, A2ATask, A2ATaskCreate,
    A2ATaskMessage, A2ATaskMessageSend, A2AArtifact, A2AArtifactCreate,
    McpTool, McpToolCreate, McpResource, McpResourceCreate,
    McpPrompt, McpPromptCreate, McpClientType, McpClientRegister,
    AcpIssuer, AcpIssuerRegister, AcpToken, AcpTokenIssue,
    AcpPresentation, AcpPresentationSubmit, AcpAccessPolicy, AcpAccessPolicyCreate,
    AnpAnnouncement, AnpAnnouncementCreate, AnpWitness, AnpWitnessRegister,
    AnpVerification, AnpVerificationVerify,
    Company, CompanyCreate, BeckNClientConfig,
)

T = TypeVar('T')


class BeckNError(Exception):
    def __init__(self, message: str, status: int = 500, response: Optional[ErrorResponse] = None):
        super().__init__(message)
        self.status = status
        self.response = response


def _to_dict(obj: Any) -> dict[str, Any]:
    if isinstance(obj, dict):
        return obj
    if hasattr(obj, '__dict__'):
        return {k: v for k, v in obj.__dict__.items() if v is not None}
    return asdict(obj)


def _from_dict(cls: type, data: dict[str, Any]) -> Any:
    if hasattr(cls, '__dataclass_fields__'):
        field_names = set(cls.__dataclass_fields__.keys())
        filtered = {k: v for k, v in data.items() if k in field_names}
        return cls(**filtered)
    return data


class BeckNClient:
    def __init__(self, config: Optional[BeckNClientConfig] = None):
        if config is None:
            config = BeckNClientConfig()
        self.base_url = config.base_url or os.environ.get('BECKN_API_URL', 'https://api.beckn.network/v1')
        self.api_key = config.api_key or os.environ.get('BECKN_API_KEY')
        self.timeout = config.timeout or 30
        self._client = httpx.Client(timeout=self.timeout, base_url=self.base_url)
        self._async_client = httpx.AsyncClient(timeout=self.timeout, base_url=self.base_url)

    def _headers(self) -> dict[str, str]:
        h = {'Content-Type': 'application/json', 'Accept': 'application/json'}
        if self.api_key:
            h['x-api-key'] = self.api_key
        return h

    def _request(self, method: str, path: str, body: Optional[dict] = None) -> dict[str, Any]:
        kwargs = {'headers': self._headers()}
        if body is not None:
            kwargs['content'] = json.dumps(body)
        resp = self._client.request(method, path, **kwargs)
        if not resp.is_success:
            try:
                err = resp.json()
            except Exception:
                err = {'message': resp.text}
            raise BeckNError(err.get('message', f'HTTP {resp.status_code}'), resp.status_code, _from_dict(ErrorResponse, err))
        if resp.status_code == 204:
            return None
        return resp.json()

    # === BeckN Orders ===
    def list_orders(self) -> list[Order]:
        return [Order(**o) for o in self._request('GET', '/orders')]

    def create_order(self, data: OrderCreate) -> Order:
        return Order(**self._request('POST', '/orders', _to_dict(data)))

    def get_order(self, order_id: str) -> Order:
        return Order(**self._request('GET', f'/orders/{order_id}'))

    def update_order(self, order_id: str, data: OrderUpdate) -> Order:
        return Order(**self._request('PATCH', f'/orders/{order_id}', _to_dict(data)))

    def delete_order(self, order_id: str) -> None:
        self._request('DELETE', f'/orders/{order_id}')

    # === BAPs ===
    def list_baps(self) -> list[Bap]:
        return [Bap(**b) for b in self._request('GET', '/baps')]

    def list_baps_by_company(self, company_id: str) -> list[Bap]:
        return [Bap(**b) for b in self._request('GET', f'/baps?company_id={company_id}')]

    def create_bap(self, data: BapCreate) -> Bap:
        return Bap(**self._request('POST', '/baps', _to_dict(data)))

    def get_bap(self, bap_id: str) -> Bap:
        return Bap(**self._request('GET', f'/baps/{bap_id}'))

    def update_bap(self, bap_id: str, data: BapUpdate) -> Bap:
        return Bap(**self._request('PATCH', f'/baps/{bap_id}', _to_dict(data)))

    # === BPPs ===
    def list_bpps(self) -> list[Bpp]:
        return [Bpp(**b) for b in self._request('GET', '/bpps')]

    def list_bpps_by_company(self, company_id: str) -> list[Bpp]:
        return [Bpp(**b) for b in self._request('GET', f'/bpps?company_id={company_id}')]

    def create_bpp(self, data: BppCreate) -> Bpp:
        return Bpp(**self._request('POST', '/bpps', _to_dict(data)))

    def get_bpp(self, bpp_id: str) -> Bpp:
        return Bpp(**self._request('GET', f'/bpps/{bpp_id}'))

    def update_bpp(self, bpp_id: str, data: BppUpdate) -> Bpp:
        return Bpp(**self._request('PATCH', f'/bpps/{bpp_id}', _to_dict(data)))

    # === Items ===
    def list_items(self) -> list[Item]:
        return [Item(**i) for i in self._request('GET', '/items')]

    def create_item(self, data: ItemCreate) -> Item:
        return Item(**self._request('POST', '/items', _to_dict(data)))

    def get_item(self, item_id: str) -> Item:
        return Item(**self._request('GET', f'/items/{item_id}'))

    # === Providers ===
    def list_providers(self) -> list[Provider]:
        return [Provider(**p) for p in self._request('GET', '/providers')]

    def create_provider(self, data: ProviderCreate) -> Provider:
        return Provider(**self._request('POST', '/providers', _to_dict(data)))

    def get_provider(self, provider_id: str) -> Provider:
        return Provider(**self._request('GET', f'/providers/{provider_id}'))

    def delete_provider(self, provider_id: str) -> None:
        self._request('DELETE', f'/providers/{provider_id}')

    # === Fulfillments ===
    def list_fulfillments(self) -> list[Fulfillment]:
        return [Fulfillment(**f) for f in self._request('GET', '/fulfillments')]

    def create_fulfillment(self, data: FulfillmentCreate) -> Fulfillment:
        return Fulfillment(**self._request('POST', '/fulfillments', _to_dict(data)))

    def track_fulfillment(self, tracking_id: str) -> list[Fulfillment]:
        return [Fulfillment(**f) for f in self._request('POST', '/fulfillments/track', {'tracking_id': tracking_id})]

    # === Subscriptions ===
    def list_subscriptions(self) -> list[Subscription]:
        return [Subscription(**s) for s in self._request('GET', '/subscriptions')]

    def create_subscription(self, data: SubscriptionCreate) -> Subscription:
        return Subscription(**self._request('POST', '/subscriptions', _to_dict(data)))

    def get_subscription(self, sub_id: str) -> Subscription:
        return Subscription(**self._request('GET', f'/subscriptions/{sub_id}'))

    def cancel_subscription(self, sub_id: str) -> Subscription:
        return Subscription(**self._request('POST', f'/subscriptions/{sub_id}/cancel', {}))

    def renew_subscription(self, sub_id: str) -> Subscription:
        return Subscription(**self._request('POST', f'/subscriptions/{sub_id}/renew', {}))

    # === API Keys ===
    def list_api_keys(self) -> list[ApiKey]:
        return [ApiKey(**k) for k in self._request('GET', '/api-keys')]

    def create_api_key(self, data: ApiKeyCreate) -> ApiKeyResponse:
        return ApiKeyResponse(**self._request('POST', '/api-keys', _to_dict(data)))

    def get_api_key(self, key_id: str) -> ApiKey:
        return ApiKey(**self._request('GET', f'/api-keys/{key_id}'))

    def verify_api_key(self, secret: str) -> ApiKeyVerifyResponse:
        resp = self._request('POST', '/api-keys/verify', {'secret': secret})
        return ApiKeyVerifyResponse(**resp)

    def revoke_api_key(self, key_id: str) -> ApiKey:
        return ApiKey(**self._request('POST', f'/api-keys/{key_id}/revoke', {}))

    def rotate_api_key(self, key_id: str) -> ApiKeyResponse:
        return ApiKeyResponse(**self._request('POST', f'/api-keys/{key_id}/rotate', {}))

    def list_api_keys_by_owner(self, owner_id: str) -> list[ApiKey]:
        return [ApiKey(**k) for k in self._request('GET', f'/api-keys/owner/{owner_id}')]

    # === GeoDNS Discovery ===
    def discover_nearest_bap(self, country: Optional[str] = None, city: Optional[str] = None) -> GeoDnsResult:
        from urllib.parse import urlencode
        params = []
        if country: params.append(f'country={country}')
        if city: params.append(f'city={city}')
        query = urlencode({'country': country or '', 'city': city or ''})
        resp = self._request('GET', f'/geodns/baps?{query}')
        loc = GeoLocation(**resp['location'])
        return GeoDnsResult(**resp, location=loc)

    def discover_nearest_bpp(self, country: Optional[str] = None, city: Optional[str] = None) -> GeoDnsResult:
        from urllib.parse import urlencode
        query = urlencode({'country': country or '', 'city': city or ''})
        resp = self._request('GET', f'/geodns/bpps?{query}')
        loc = GeoLocation(**resp['location'])
        return GeoDnsResult(**resp, location=loc)

    def discover_marketplace(self, lat: float, lng: float, radius_km: int = 50, limit: int = 10) -> list[GeoDnsResult]:
        resp = self._request('GET', f'/geodns/marketplaces?lat={lat}&lng={lng}&radius_km={radius_km}&limit={limit}')
        return [GeoDnsResult(**r, location=GeoLocation(**r['location'])) for r in resp]

    # === A2A ===
    def list_agent_cards(self) -> list[AgentCard]:
        return [AgentCard(**a) for a in self._request('GET', '/a2a/agent-cards')]

    def register_agent_card(self, data: AgentCardRegister) -> AgentCard:
        return AgentCard(**self._request('POST', '/a2a/agent-cards', _to_dict(data)))

    def get_agent_card(self, agent_id: str) -> AgentCard:
        return AgentCard(**self._request('GET', f'/a2a/agent-cards/{agent_id}'))

    # === MCP ===
    def list_tools(self) -> list[McpTool]:
        return [McpTool(**t) for t in self._request('GET', '/mcp/tools')]

    def create_tool(self, data: McpToolCreate) -> McpTool:
        return McpTool(**self._request('POST', '/mcp/tools', _to_dict(data)))

    # === ACP ===
    def list_issuers(self) -> list[AcpIssuer]:
        return [AcpIssuer(**i) for i in self._request('GET', '/acp/issuers')]

    def register_issuer(self, data: AcpIssuerRegister) -> AcpIssuer:
        return AcpIssuer(**self._request('POST', '/acp/issuers', _to_dict(data)))

    def issue_token(self, data: AcpTokenIssue) -> AcpToken:
        return AcpToken(**self._request('POST', '/acp/tokens', _to_dict(data)))

    # === ANP ===
    def list_announcements(self) -> list[AnpAnnouncement]:
        return [AnpAnnouncement(**a) for a in self._request('GET', '/anp/announcements')]

    def announce(self, data: AnpAnnouncementCreate) -> AnpAnnouncement:
        return AnpAnnouncement(**self._request('POST', '/anp/announcements', _to_dict(data)))

    # === Companies (B2B Multi-Tenant) ===
    def list_companies(self) -> list[Company]:
        return [Company(**c) for c in self._request('GET', '/companies')]

    def create_company(self, data: CompanyCreate) -> Company:
        return Company(**self._request('POST', '/companies', _to_dict(data)))

    def get_company(self, company_id: str) -> Company:
        return Company(**self._request('GET', f'/companies/{company_id}'))

    def list_companies_by_domain(self, domain: str) -> list[Company]:
        return [Company(**c) for c in self._request('GET', f'/companies?domain={domain}')]

    # === GBP (Google Business Profile) ===
    def list_gbp_accounts(self) -> list[Any]:
        return self._request('GET', '/gbp/accounts')

    def create_gbp_account(self, data: dict[str, Any]) -> Any:
        return self._request('POST', '/gbp/accounts', data)

    def sync_gbp_locations(self, account_id: str) -> dict[str, Any]:
        return self._request('POST', f'/gbp/accounts/{account_id}/sync', {})

    # === Health ===
    def health(self) -> dict[str, Any]:
        return self._request('GET', '/health')
