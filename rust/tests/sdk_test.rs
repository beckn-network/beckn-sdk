use beckn_sdk::{Client, ClientConfig, BeckNError};
use beckn_sdk::{OrderCreate, BapCreate, BppCreate, ApiKeyCreate, AgentCardRegister,
                McpToolCreate, AcpTokenIssue, AnpAnnouncementCreate};
use mockito::Server;

fn default_config(url: String) -> ClientConfig {
    ClientConfig::default().with_base_url(url)
}

#[tokio::test]
async fn test_create_order() {
    let mut server = Server::new_async().await;
    server.mock("POST", "/orders")
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(r#"{"id":"order-123","order_state":"pending"}"#)
        .create_async()
        .await;

    let client = Client::new(default_config(server.url()));
    let order = client.create_order(&OrderCreate {
        id: "order-123".to_string(),
        order_state: Some("pending".to_string()),
        provider_id: None,
        bpp_id: None,
        bap_id: None,
        transaction_id: None,
    }).await.unwrap();

    assert_eq!(order.id, "order-123");
}

#[tokio::test]
async fn test_get_order() {
    let mut server = Server::new_async().await;
    server.mock("GET", "/orders/order-123")
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(r#"{"id":"order-123","order_state":"accepted"}"#)
        .create_async()
        .await;

    let client = Client::new(default_config(server.url()));
    let order = client.get_order("order-123").await.unwrap();
    assert_eq!(order.id, "order-123");
}

#[tokio::test]
async fn test_create_bap() {
    let mut server = Server::new_async().await;
    server.mock("POST", "/baps")
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(r#"{"id":"bap_001","name":"Test BAP","country":"IND"}"#)
        .create_async()
        .await;

    let client = Client::new(default_config(server.url()));
    let bap = client.create_bap(&BapCreate {
        id: "bap_001".to_string(),
        name: "Test BAP".to_string(),
        did: None,
        endpoint: None,
        public_key: None,
        active: Some(true),
        capabilities: vec![],
        country: Some("IND".to_string()),
        city: None,
        region: None,
        lat: None,
        lon: None,
    }).await.unwrap();

    assert_eq!(bap.id, "bap_001");
    assert_eq!(bap.country.as_deref(), Some("IND"));
}

#[tokio::test]
async fn test_create_bpp() {
    let mut server = Server::new_async().await;
    server.mock("POST", "/bpps")
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(r#"{"id":"bpp_001","name":"Test BPP","currency":"INR"}"#)
        .create_async()
        .await;

    let client = Client::new(default_config(server.url()));
    let bpp = client.create_bpp(&BppCreate {
        id: "bpp_001".to_string(),
        name: "Test BPP".to_string(),
        did: None,
        endpoint: None,
        public_key: None,
        active: Some(true),
        capabilities: vec![],
        country: Some("IND".to_string()),
        city: None,
        symbol: Some("₹".to_string()),
        currency: Some("INR".to_string()),
        region: None,
        lat: None,
        lon: None,
    }).await.unwrap();

    assert_eq!(bpp.id, "bpp_001");
    assert_eq!(bpp.currency.as_deref(), Some("INR"));
}

#[tokio::test]
async fn test_create_api_key() {
    let mut server = Server::new_async().await;
    server.mock("POST", "/api-keys")
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(r#"{"id":"key_001","secret":"bk_secret_123","owner":"bap_001"}"#)
        .create_async()
        .await;

    let client = Client::new(default_config(server.url()));
    let key = client.create_api_key(&ApiKeyCreate {
        description: Some("Test key".to_string()),
        owner: Some("bap_001".to_string()),
        scopes: Some(vec!["read".to_string()]),
        expires_at: None,
    }).await.unwrap();

    assert_eq!(key.id, "key_001");
    assert!(key.secret.is_some());
    assert_eq!(key.secret.unwrap(), "bk_secret_123");
}

#[tokio::test]
async fn test_verify_api_key() {
    let mut server = Server::new_async().await;
    server.mock("POST", "/api-keys/verify")
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(r#"{"valid":true,"owner":"bap_001","scopes":["read"]}"#)
        .create_async()
        .await;

    let client = Client::new(default_config(server.url()));
    let result = client.verify_api_key("bk_secret_123").await.unwrap();
    assert!(result.valid);
    assert_eq!(result.owner.as_deref(), Some("bap_001"));
}

#[tokio::test]
async fn test_list_orders() {
    let mut server = Server::new_async().await;
    server.mock("GET", "/orders")
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(r#"[{"id":"order-1"},{"id":"order-2"}]"#)
        .create_async()
        .await;

    let client = Client::new(default_config(server.url()));
    let orders = client.list_orders().await.unwrap();
    assert_eq!(orders.len(), 2);
    assert_eq!(orders[0].id, "order-1");
}

#[tokio::test]
async fn test_list_baps() {
    let mut server = Server::new_async().await;
    server.mock("GET", "/baps")
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(r#"[{"id":"bap_001","name":"Test BAP"}]"#)
        .create_async()
        .await;

    let client = Client::new(default_config(server.url()));
    let baps = client.list_baps().await.unwrap();
    assert_eq!(baps.len(), 1);
    assert_eq!(baps[0].id, "bap_001");
}

#[tokio::test]
async fn test_list_bpps() {
    let mut server = Server::new_async().await;
    server.mock("GET", "/bpps")
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(r#"[{"id":"bpp_001","name":"Test BPP"}]"#)
        .create_async()
        .await;

    let client = Client::new(default_config(server.url()));
    let bpps = client.list_bpps().await.unwrap();
    assert_eq!(bpps.len(), 1);
    assert_eq!(bpps[0].id, "bpp_001");
}

#[tokio::test]
async fn test_list_items() {
    let mut server = Server::new_async().await;
    server.mock("GET", "/items")
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(r#"[{"id":"item_001","name":"Test Item"}]"#)
        .create_async()
        .await;

    let client = Client::new(default_config(server.url()));
    let items = client.list_items().await.unwrap();
    assert_eq!(items.len(), 1);
    assert_eq!(items[0].id, "item_001");
}

#[tokio::test]
async fn test_list_providers() {
    let mut server = Server::new_async().await;
    server.mock("GET", "/providers")
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(r#"[{"id":"prov_001","name":"Test Provider"}]"#)
        .create_async()
        .await;

    let client = Client::new(default_config(server.url()));
    let providers = client.list_providers().await.unwrap();
    assert_eq!(providers.len(), 1);
    assert_eq!(providers[0].id, "prov_001");
}

#[tokio::test]
async fn test_list_subscriptions() {
    let mut server = Server::new_async().await;
    server.mock("GET", "/subscriptions")
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(r#"[{"id":"sub_001","subscriber_id":"bap_001","subscriber_type":"bap"}]"#)
        .create_async()
        .await;

    let client = Client::new(default_config(server.url()));
    let subs = client.list_subscriptions().await.unwrap();
    assert_eq!(subs.len(), 1);
    assert_eq!(subs[0].id, "sub_001");
}

#[tokio::test]
async fn test_list_fulfillments() {
    let mut server = Server::new_async().await;
    server.mock("GET", "/fulfillments")
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(r#"[{"id":"ful_001","tracking_id":"track_001"}]"#)
        .create_async()
        .await;

    let client = Client::new(default_config(server.url()));
    let fulfillments = client.list_fulfillments().await.unwrap();
    assert_eq!(fulfillments.len(), 1);
    assert_eq!(fulfillments[0].id, "ful_001");
}

#[tokio::test]
async fn test_list_tools() {
    let mut server = Server::new_async().await;
    server.mock("GET", "/mcp/tools")
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(r#"[{"name":"search_hotels","description":"Search for hotels"}]"#)
        .create_async()
        .await;

    let client = Client::new(default_config(server.url()));
    let tools = client.list_tools().await.unwrap();
    assert_eq!(tools.len(), 1);
    assert_eq!(tools[0].name, "search_hotels");
}

#[tokio::test]
async fn test_mcp_create_tool() {
    let mut server = Server::new_async().await;
    server.mock("POST", "/mcp/tools")
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(r#"{"name":"my_tool","handler":"MyMod.func"}"#)
        .create_async()
        .await;

    let client = Client::new(default_config(server.url()));
    let tool = client.create_tool(&McpToolCreate {
        name: "my_tool".to_string(),
        handler: Some("MyMod.func".to_string()),
        description: None,
        input_schema: None,
    }).await.unwrap();
    assert_eq!(tool.name, "my_tool");
}

#[tokio::test]
async fn test_list_agent_cards() {
    let mut server = Server::new_async().await;
    server.mock("GET", "/a2a/agents")
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(r#"[{"agent_id":"agent_001","name":"Test Agent","url":"https://agent.example.com"}]"#)
        .create_async()
        .await;

    let client = Client::new(default_config(server.url()));
    let agents = client.list_agent_cards().await.unwrap();
    assert_eq!(agents.len(), 1);
    assert_eq!(agents[0].agent_id, "agent_001");
}

#[tokio::test]
async fn test_a2a_register_agent() {
    let mut server = Server::new_async().await;
    server.mock("POST", "/a2a/agents")
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(r#"{"agent_id":"agent_001","name":"Test Agent","url":"https://agent.example.com"}"#)
        .create_async()
        .await;

    let client = Client::new(default_config(server.url()));
    let agent = client.register_agent_card(&AgentCardRegister {
        agent_id: "agent_001".to_string(),
        name: "Test Agent".to_string(),
        url: "https://agent.example.com".to_string(),
        description: None,
    }).await.unwrap();
    assert_eq!(agent.agent_id, "agent_001");
}

#[tokio::test]
async fn test_register_issuer() {
    let mut server = Server::new_async().await;
    server.mock("POST", "/acp/issuers")
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(r#"{"issuer_id":"issuer_001","name":"Test Issuer"}"#)
        .create_async()
        .await;

    let client = Client::new(default_config(server.url()));
    let issuer = client.register_issuer(&beckn_sdk::AcpIssuerRegister {
        issuer_id: "issuer_001".to_string(),
        name: Some("Test Issuer".to_string()),
        jwks_uri: None,
    }).await.unwrap();

    assert_eq!(issuer.issuer_id, "issuer_001");
}

#[tokio::test]
async fn test_acp_issue_token() {
    let mut server = Server::new_async().await;
    server.mock("POST", "/acp/tokens")
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(r#"{"token_value":"tok_123","token_type":"Bearer"}"#)
        .create_async()
        .await;

    let client = Client::new(default_config(server.url()));
    let token = client.issue_token(&AcpTokenIssue {
        token_value: "tok_123".to_string(),
        token_type: Some("Bearer".to_string()),
        scope: None,
        expires_at: None,
        credential_issuer_id: None,
        subject: None,
        audience: None,
    }).await.unwrap();
    assert_eq!(token.token_value, "tok_123");
}

#[tokio::test]
async fn test_anp_announce() {
    let mut server = Server::new_async().await;
    server.mock("POST", "/anp/announcements")
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(r#"{"announcement_id":"ann_001","did":"did:key:abc"}"#)
        .create_async()
        .await;

    let client = Client::new(default_config(server.url()));
    let ann = client.announce(&AnpAnnouncementCreate {
        announcement_id: "ann_001".to_string(),
        did: "did:key:abc".to_string(),
        did_document_hash: None,
        service_endpoint: None,
        timestamp: None,
        ttl: Some(3600),
        proof: None,
    }).await.unwrap();
    assert_eq!(ann.announcement_id, "ann_001");
    assert_eq!(ann.did, "did:key:abc");
}

#[tokio::test]
async fn test_geodns_discovery() {
    let mut server = Server::new_async().await;
    let mock = server.mock("GET", "/geodns/baps")
        .match_query(mockito::Matcher::Any)
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(r#"{"hostname":"dns.beckn.network","endpoints":["https://api.beckn.network"],"country":"IN","city":"BLR","region":"IN-KA"}"#)
        .create_async()
        .await;

    let client = Client::new(default_config(server.url()));
    let result = client.discover_nearest_bap(Some("IN"), Some("BLR")).await.unwrap();
    assert_eq!(result.hostname, "dns.beckn.network");
    assert_eq!(result.country, "IN");
    assert!(result.endpoints.len() > 0);
    mock.assert_async().await;
}

#[tokio::test]
async fn test_error_handling() {
    let mut server = Server::new_async().await;
    server.mock("GET", "/orders/nonexistent")
        .with_status(404)
        .with_header("content-type", "application/json")
        .with_body(r#"{"error":"Not found","message":"Order not found"}"#)
        .create_async()
        .await;

    let client = Client::new(default_config(server.url()));
    let result = client.get_order("nonexistent").await;
    assert!(result.is_err());
    match result.unwrap_err() {
        BeckNError::Http { status, .. } => assert_eq!(status, 404),
        _ => panic!("expected Http error"),
    }
}
