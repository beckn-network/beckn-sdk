import { test, describe, beforeEach, afterEach } from 'node:test';
import assert from 'node:assert/strict';
import { BeckNClient, BeckNError } from '../src/index.js';

function mockFetch(status: number, data: unknown) {
  return (_url: string | URL | Request, _opts?: RequestInit) =>
    Promise.resolve({
      ok: status >= 200 && status < 300,
      status,
      statusText: status === 200 ? 'OK' : 'Error',
      json: () => Promise.resolve(data),
      text: () => Promise.resolve(typeof data === 'string' ? data : JSON.stringify(data)),
    } as Response);
}

describe('BeckN TypeScript SDK', () => {
  let client: BeckNClient;
  let originalFetch: typeof globalThis.fetch;

  beforeEach(() => {
    client = new BeckNClient({ apiKey: 'bk_test_key', baseUrl: 'http://localhost:4000/v1' });
    originalFetch = globalThis.fetch;
  });

  afterEach(() => { globalThis.fetch = originalFetch; });

  describe('Orders', () => {
    test('createOrder', async () => {
      globalThis.fetch = mockFetch(200, { id: 'order-123', order_state: 'pending' });
      const order = await client.createOrder({ id: 'order-123', order_state: 'pending' });
      assert.equal(order.id, 'order-123');
      assert.equal(order.order_state, 'pending');
    });

    test('getOrder', async () => {
      globalThis.fetch = mockFetch(200, { id: 'order-123', order_state: 'accepted' });
      const order = await client.getOrder('order-123');
      assert.equal(order.id, 'order-123');
    });

    test('listOrders', async () => {
      globalThis.fetch = mockFetch(200, [{ id: 'order-1' }, { id: 'order-2' }]);
      const orders = await client.listOrders();
      assert.equal(orders.length, 2);
    });
  });

  describe('BAPs', () => {
    test('createBap', async () => {
      globalThis.fetch = mockFetch(200, { id: 'bap_001', name: 'Test BAP', country: 'IND' });
      const bap = await client.createBap({ id: 'bap_001', name: 'Test BAP', country: 'IND' });
      assert.equal(bap.id, 'bap_001');
      assert.equal(bap.country, 'IND');
    });

    test('getBap', async () => {
      globalThis.fetch = mockFetch(200, { id: 'bap_001', name: 'Test BAP' });
      const bap = await client.getBap('bap_001');
      assert.equal(bap.id, 'bap_001');
    });
  });

  describe('API Keys', () => {
    test('createApiKey', async () => {
      globalThis.fetch = mockFetch(200, { id: 'key_001', secret: 'bk_secret_123', owner: 'bap_001' });
      const key = await client.createApiKey({ owner: 'bap_001' });
      assert.equal(key.id, 'key_001');
      assert.ok(key.secret);
    });

    test('verifyApiKey', async () => {
      globalThis.fetch = mockFetch(200, { valid: true, owner: 'bap_001', scopes: ['read'] });
      const result = await client.verifyApiKey('bk_secret_123');
      assert.equal(result.valid, true);
      assert.equal(result.owner, 'bap_001');
    });
  });

  describe('A2A', () => {
    test('listAgentCards', async () => {
      globalThis.fetch = mockFetch(200, [{ agent_id: 'agent_001', name: 'Test Agent' }]);
      const agents = await client.listAgentCards();
      assert.equal(agents.length, 1);
      assert.equal(agents[0].agent_id, 'agent_001');
    });

    test('registerAgentCard', async () => {
      globalThis.fetch = mockFetch(200, { agent_id: 'agent_001', name: 'Test Agent' });
      const agent = await client.registerAgentCard({ agent_id: 'agent_001', name: 'Test Agent', url: 'https://agent.example.com' });
      assert.equal(agent.agent_id, 'agent_001');
    });
  });

  describe('MCP', () => {
    test('listTools', async () => {
      globalThis.fetch = mockFetch(200, [{ name: 'search_hotels', description: 'Search for hotels' }]);
      const tools = await client.listTools();
      assert.equal(tools.length, 1);
      assert.equal(tools[0].name, 'search_hotels');
    });

    test('createTool', async () => {
      globalThis.fetch = mockFetch(200, { name: 'my_tool', handler: 'MyMod.func' });
      const tool = await client.createTool({ name: 'my_tool', handler: 'MyMod.func' });
      assert.equal(tool.name, 'my_tool');
    });
  });

  describe('ACP', () => {
    test('listIssuers', async () => {
      globalThis.fetch = mockFetch(200, [{ issuer_id: 'issuer_001', name: 'Test Issuer' }]);
      const issuers = await client.listIssuers();
      assert.equal(issuers.length, 1);
      assert.equal(issuers[0].issuer_id, 'issuer_001');
    });
  });

  describe('ANP', () => {
    test('listAnnouncements', async () => {
      globalThis.fetch = mockFetch(200, [{ announcement_id: 'ann_001', did: 'did:key:abc' }]);
      const announcements = await client.listAnnouncements();
      assert.equal(announcements.length, 1);
      assert.equal(announcements[0].announcement_id, 'ann_001');
    });
  });

  describe('Error handling', () => {
    test('throws BeckNError on non-OK response', async () => {
      globalThis.fetch = mockFetch(404, { error: 'Not found', message: 'Resource not found' });
      await assert.rejects(
        () => client.getOrder('nonexistent'),
        (err: unknown) => {
          assert.ok(err instanceof BeckNError);
          assert.equal(err.status, 404);
          return true;
        }
      );
    });
  });
});
