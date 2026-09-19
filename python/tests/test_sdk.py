"""Tests for the BeckN Python SDK."""

import os
import pytest
from unittest.mock import patch, MagicMock

os.environ.setdefault('BECKN_API_KEY', 'bk_test_key')

from beckn import (
    BeckNClient, BeckNError, BeckNClientConfig,
    Order, OrderCreate, Bap, BapCreate, Bpp, BppCreate, ApiKeyCreate, ApiKeyResponse,
    AgentCard, AgentCardRegister, A2ATask, A2ATaskMessage,
    McpTool, McpToolCreate, McpClientType, McpClientRegister,
    AcpIssuer, AcpIssuerRegister, AcpToken, AcpTokenIssue,
    AnpAnnouncement, AnpAnnouncementCreate,
)


def _mock_response(data, status=200):
    m = MagicMock()
    m.is_success = 200 <= status < 300
    m.status_code = status
    m.json.return_value = data
    m.text = '{}' if isinstance(data, dict) else '[]'
    return m


@pytest.fixture
def client():
    return BeckNClient(BeckNClientConfig(api_key='bk_test'))


class TestOrders:
    def test_create_order(self, client):
        resp_data = {'id': 'order-123', 'order_state': 'pending'}
        with patch.object(client._client, 'request', return_value=_mock_response(resp_data)):
            order = client.create_order(OrderCreate(id='order-123', order_state='pending'))
            assert order.id == 'order-123'

    def test_get_order(self, client):
        resp_data = {'id': 'order-123', 'order_state': 'accepted'}
        with patch.object(client._client, 'request', return_value=_mock_response(resp_data)):
            order = client.get_order('order-123')
            assert order.id == 'order-123'

    def test_list_orders(self, client):
        resp_data = [{'id': 'order-1'}, {'id': 'order-2'}]
        with patch.object(client._client, 'request', return_value=_mock_response(resp_data)):
            orders = client.list_orders()
            assert len(orders) == 2

    def test_delete_order(self, client):
        with patch.object(client._client, 'request', return_value=_mock_response(None, 204)):
            client.delete_order('order-123')


class TestBAPs:
    def test_create_bap(self, client):
        resp_data = {'id': 'bap_001', 'name': 'Test BAP', 'country': 'IND'}
        with patch.object(client._client, 'request', return_value=_mock_response(resp_data)):
            bap = client.create_bap(BapCreate(id='bap_001', name='Test BAP', country='IND'))
            assert bap.id == 'bap_001'
            assert bap.country == 'IND'

    def test_get_bap(self, client):
        resp_data = {'id': 'bap_001', 'name': 'Test BAP'}
        with patch.object(client._client, 'request', return_value=_mock_response(resp_data)):
            bap = client.get_bap('bap_001')
            assert bap.id == 'bap_001'


class TestBPPs:
    def test_create_bpp(self, client):
        resp_data = {'id': 'bpp_001', 'name': 'Test BPP', 'country': 'IND', 'currency': 'INR'}
        with patch.object(client._client, 'request', return_value=_mock_response(resp_data)):
            bpp = client.create_bpp(BppCreate(id='bpp_001', name='Test BPP', country='IND', currency='INR'))
            assert bpp.id == 'bpp_001'
            assert bpp.currency == 'INR'

    def test_get_bpp(self, client):
        resp_data = {'id': 'bpp_001', 'name': 'Test BPP'}
        with patch.object(client._client, 'request', return_value=_mock_response(resp_data)):
            bpp = client.get_bpp('bpp_001')
            assert bpp.id == 'bpp_001'


class TestApiKeys:
    def test_create_api_key(self, client):
        resp_data = {'id': 'key_001', 'secret': 'bk_secret_123', 'owner': 'bap_001'}
        with patch.object(client._client, 'request', return_value=_mock_response(resp_data)):
            key = client.create_api_key(ApiKeyCreate(description='Test key', owner='bap_001'))
            assert key.id == 'key_001'
            assert key.secret == 'bk_secret_123'

    def test_verify_api_key(self, client):
        resp_data = {'valid': True, 'owner': 'bap_001', 'scopes': ['read']}
        with patch.object(client._client, 'request', return_value=_mock_response(resp_data)):
            result = client.verify_api_key('bk_secret_123')
            assert result.valid is True
            assert result.owner == 'bap_001'


class TestSubscriptions:
    def test_create_subscription(self, client):
        from beckn import SubscriptionCreate, SubscriberType
        resp_data = {'id': 'sub_001', 'subscriber_id': 'bap_001', 'subscriber_type': 'bap'}
        with patch.object(client._client, 'request', return_value=_mock_response(resp_data)):
            sub = client.create_subscription(SubscriptionCreate(id='sub_001', subscriber_id='bap_001', subscriber_type=SubscriberType.BAP))
            assert sub.id == 'sub_001'


class TestA2A:
    def test_register_agent_card(self, client):
        resp_data = {'agent_id': 'agent_001', 'name': 'Test Agent', 'url': 'https://agent.example.com'}
        with patch.object(client._client, 'request', return_value=_mock_response(resp_data)):
            agent = client.register_agent_card(AgentCardRegister(agent_id='agent_001', name='Test Agent', url='https://agent.example.com'))
            assert agent.agent_id == 'agent_001'

    def test_list_agent_cards(self, client):
        resp_data = [{'agent_id': 'agent_001', 'name': 'Test Agent', 'url': 'https://agent.example.com'}]
        with patch.object(client._client, 'request', return_value=_mock_response(resp_data)):
            agents = client.list_agent_cards()
            assert len(agents) == 1


class TestMCP:
    def test_list_tools(self, client):
        resp_data = [{'name': 'search_hotels', 'description': 'Search hotels'}]
        with patch.object(client._client, 'request', return_value=_mock_response(resp_data)):
            tools = client.list_tools()
            assert len(tools) == 1
            assert tools[0].name == 'search_hotels'

    def test_create_tool(self, client):
        resp_data = {'name': 'my_tool', 'handler': 'MyMod.func'}
        with patch.object(client._client, 'request', return_value=_mock_response(resp_data)):
            tool = client.create_tool(McpToolCreate(name='my_tool', handler='MyMod.func'))
            assert tool.name == 'my_tool'


class TestACP:
    def test_list_issuers(self, client):
        resp_data = [{'issuer_id': 'issuer_001', 'name': 'Test Issuer'}]
        with patch.object(client._client, 'request', return_value=_mock_response(resp_data)):
            issuers = client.list_issuers()
            assert len(issuers) == 1
            assert issuers[0].issuer_id == 'issuer_001'

    def test_issue_token(self, client):
        resp_data = {'token_value': 'tok_123', 'subject': 'tenant:abc'}
        with patch.object(client._client, 'request', return_value=_mock_response(resp_data)):
            token = client.issue_token(AcpTokenIssue(token_value='tok_123', subject='tenant:abc'))
            assert token.token_value == 'tok_123'


class TestANP:
    def test_announce(self, client):
        resp_data = {'announcement_id': 'ann_001', 'did': 'did:key:abc'}
        with patch.object(client._client, 'request', return_value=_mock_response(resp_data)):
            ann = client.announce(AnpAnnouncementCreate(announcement_id='ann_001', did='did:key:abc'))
            assert ann.announcement_id == 'ann_001'

    def test_list_announcements(self, client):
        resp_data = [{'announcement_id': 'ann_001', 'did': 'did:key:abc'}]
        with patch.object(client._client, 'request', return_value=_mock_response(resp_data)):
            announcements = client.list_announcements()
            assert len(announcements) == 1


class TestErrorHandling:
    def test_raises_beckn_error(self, client):
        with patch.object(client._client, 'request', return_value=_mock_response({'message': 'Not found'}, 404)):
            with pytest.raises(BeckNError) as exc_info:
                client.get_order('nonexistent')
            assert exc_info.value.status == 404
