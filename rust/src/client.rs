use crate::models::*;
use reqwest::header::{HeaderMap, HeaderValue, CONTENT_TYPE, ACCEPT};
use serde_json::Value;
use std::time::Duration;

fn parse_error(status: u16, body: &str) -> BeckNError {
    let msg = if let Ok(json) = serde_json::from_str::<ErrorResponse>(body) {
        json.message.unwrap_or_else(|| format!("HTTP {status}"))
    } else {
        body.to_string()
    };
    BeckNError::Http { status, message: msg }
}

pub struct Client {
    base_url: String,
    api_key: Option<String>,
    timeout: Duration,
}

impl Client {
    pub fn new(config: ClientConfig) -> Self {
        let base_url = config.base_url;
        let api_key = config.api_key;
        Self {
            base_url,
            api_key,
            timeout: Duration::from_secs(config.timeout_seconds),
        }
    }

    pub fn from_env() -> Self {
        Self::new(ClientConfig::default())
    }

    fn headers(&self) -> HeaderMap {
        let mut h = HeaderMap::new();
        h.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));
        h.insert(ACCEPT, HeaderValue::from_static("application/json"));
        if let Some(key) = &self.api_key {
            h.insert("x-api-key", HeaderValue::from_str(key).unwrap());
        }
        h
    }

    async fn request(&self, method: &str, path: &str, body: Option<&Value>) -> Result<Value, BeckNError> {
        let mut req = reqwest::Client::builder()
            .timeout(self.timeout)
            .build()
            .map_err(|e| BeckNError::Other(e.to_string()))?
            .request(match method {
                "GET" => reqwest::Method::GET,
                "POST" => reqwest::Method::POST,
                "PATCH" => reqwest::Method::PATCH,
                "DELETE" => reqwest::Method::DELETE,
                _ => reqwest::Method::GET,
            }, format!("{}{}", self.base_url, path))
            .headers(self.headers());

        if let Some(b) = body {
            req = req.json(b);
        }

        let resp = req.send().await?;
        let status = resp.status().as_u16();
        let text = resp.text().await?;

        if status >= 400 {
            return Err(parse_error(status, &text));
        }

        if status == 204 {
            return Ok(Value::Null);
        }

        Ok(serde_json::from_str(&text)?)
    }

    // === BAP ===
    pub async fn list_baps(&self) -> Result<Vec<Bap>, BeckNError> {
        let data = self.request("GET", "/baps", None).await?;
        Ok(serde_json::from_value(data)?)
    }
    pub async fn create_bap(&self, data: &BapCreate) -> Result<Bap, BeckNError> {
        let resp = self.request("POST", "/baps", Some(&serde_json::to_value(data)?)).await?;
        Ok(serde_json::from_value(resp)?)
    }
    pub async fn get_bap(&self, id: &str) -> Result<Bap, BeckNError> {
        let resp = self.request("GET", &format!("/baps/{}", id), None).await?;
        Ok(serde_json::from_value(resp)?)
    }

    // === BPP ===
    pub async fn list_bpps(&self) -> Result<Vec<Bpp>, BeckNError> {
        let data = self.request("GET", "/bpps", None).await?;
        Ok(serde_json::from_value(data)?)
    }
    pub async fn create_bpp(&self, data: &BppCreate) -> Result<Bpp, BeckNError> {
        let resp = self.request("POST", "/bpps", Some(&serde_json::to_value(data)?)).await?;
        Ok(serde_json::from_value(resp)?)
    }
    pub async fn get_bpp(&self, id: &str) -> Result<Bpp, BeckNError> {
        let resp = self.request("GET", &format!("/bpps/{}", id), None).await?;
        Ok(serde_json::from_value(resp)?)
    }

    // === Order ===
    pub async fn list_orders(&self) -> Result<Vec<Order>, BeckNError> {
        let data = self.request("GET", "/orders", None).await?;
        Ok(serde_json::from_value(data)?)
    }
    pub async fn create_order(&self, data: &OrderCreate) -> Result<Order, BeckNError> {
        let resp = self.request("POST", "/orders", Some(&serde_json::to_value(data)?)).await?;
        Ok(serde_json::from_value(resp)?)
    }
    pub async fn get_order(&self, id: &str) -> Result<Order, BeckNError> {
        let resp = self.request("GET", &format!("/orders/{}", id), None).await?;
        Ok(serde_json::from_value(resp)?)
    }
    pub async fn update_order(&self, id: &str, data: &Order) -> Result<Order, BeckNError> {
        let resp = self.request("PATCH", &format!("/orders/{}", id), Some(&serde_json::to_value(data)?)).await?;
        Ok(serde_json::from_value(resp)?)
    }
    pub async fn delete_order(&self, id: &str) -> Result<(), BeckNError> {
        self.request("DELETE", &format!("/orders/{}", id), None).await?;
        Ok(())
    }

    // === Item ===
    pub async fn list_items(&self) -> Result<Vec<Item>, BeckNError> {
        let data = self.request("GET", "/items", None).await?;
        Ok(serde_json::from_value(data)?)
    }
    pub async fn create_item(&self, data: &Item) -> Result<Item, BeckNError> {
        let resp = self.request("POST", "/items", Some(&serde_json::to_value(data)?)).await?;
        Ok(serde_json::from_value(resp)?)
    }
    pub async fn get_item(&self, id: &str) -> Result<Item, BeckNError> {
        let resp = self.request("GET", &format!("/items/{}", id), None).await?;
        Ok(serde_json::from_value(resp)?)
    }

    // === Provider ===
    pub async fn list_providers(&self) -> Result<Vec<Provider>, BeckNError> {
        let data = self.request("GET", "/providers", None).await?;
        Ok(serde_json::from_value(data)?)
    }
    pub async fn create_provider(&self, data: &Provider) -> Result<Provider, BeckNError> {
        let resp = self.request("POST", "/providers", Some(&serde_json::to_value(data)?)).await?;
        Ok(serde_json::from_value(resp)?)
    }
    pub async fn get_provider(&self, id: &str) -> Result<Provider, BeckNError> {
        let resp = self.request("GET", &format!("/providers/{}", id), None).await?;
        Ok(serde_json::from_value(resp)?)
    }
    pub async fn delete_provider(&self, id: &str) -> Result<(), BeckNError> {
        self.request("DELETE", &format!("/providers/{}", id), None).await?;
        Ok(())
    }

    // === Fulfillment ===
    pub async fn list_fulfillments(&self) -> Result<Vec<Fulfillment>, BeckNError> {
        let data = self.request("GET", "/fulfillments", None).await?;
        Ok(serde_json::from_value(data)?)
    }
    pub async fn create_fulfillment(&self, data: &FulfillmentCreate) -> Result<Fulfillment, BeckNError> {
        let resp = self.request("POST", "/fulfillments", Some(&serde_json::to_value(data)?)).await?;
        Ok(serde_json::from_value(resp)?)
    }
    pub async fn track_fulfillment(&self, tracking_id: &str) -> Result<Vec<Fulfillment>, BeckNError> {
        let data = serde_json::json!({ "tracking_id": tracking_id });
        let resp = self.request("POST", "/fulfillments/track", Some(&data)).await?;
        Ok(serde_json::from_value(resp)?)
    }

    // === Subscription ===
    pub async fn list_subscriptions(&self) -> Result<Vec<Subscription>, BeckNError> {
        let data = self.request("GET", "/subscriptions", None).await?;
        Ok(serde_json::from_value(data)?)
    }
    pub async fn create_subscription(&self, data: &Subscription) -> Result<Subscription, BeckNError> {
        let resp = self.request("POST", "/subscriptions", Some(&serde_json::to_value(data)?)).await?;
        Ok(serde_json::from_value(resp)?)
    }
    pub async fn get_subscription(&self, id: &str) -> Result<Subscription, BeckNError> {
        let resp = self.request("GET", &format!("/subscriptions/{}", id), None).await?;
        Ok(serde_json::from_value(resp)?)
    }
    pub async fn cancel_subscription(&self, id: &str) -> Result<Subscription, BeckNError> {
        let resp = self.request("POST", &format!("/subscriptions/{}/cancel", id), Some(&serde_json::json!({}))).await?;
        Ok(serde_json::from_value(resp)?)
    }
    pub async fn renew_subscription(&self, id: &str) -> Result<Subscription, BeckNError> {
        let resp = self.request("POST", &format!("/subscriptions/{}/renew", id), Some(&serde_json::json!({}))).await?;
        Ok(serde_json::from_value(resp)?)
    }

    // === ApiKey ===
    pub async fn list_api_keys(&self) -> Result<Vec<ApiKey>, BeckNError> {
        let data = self.request("GET", "/api-keys", None).await?;
        Ok(serde_json::from_value(data)?)
    }
    pub async fn create_api_key(&self, data: &ApiKeyCreate) -> Result<ApiKeyResponse, BeckNError> {
        let resp = self.request("POST", "/api-keys", Some(&serde_json::to_value(data)?)).await?;
        Ok(serde_json::from_value(resp)?)
    }
    pub async fn get_api_key(&self, id: &str) -> Result<ApiKey, BeckNError> {
        let resp = self.request("GET", &format!("/api-keys/{}", id), None).await?;
        Ok(serde_json::from_value(resp)?)
    }
    pub async fn verify_api_key(&self, secret: &str) -> Result<ApiKeyVerifyResponse, BeckNError> {
        let data = serde_json::json!({ "secret": secret });
        let resp = self.request("POST", "/api-keys/verify", Some(&data)).await?;
        Ok(serde_json::from_value(resp)?)
    }
    pub async fn revoke_api_key(&self, id: &str) -> Result<ApiKey, BeckNError> {
        let resp = self.request("POST", &format!("/api-keys/{}/revoke", id), Some(&serde_json::json!({}))).await?;
        Ok(serde_json::from_value(resp)?)
    }
    pub async fn rotate_api_key(&self, id: &str) -> Result<ApiKeyResponse, BeckNError> {
        let resp = self.request("POST", &format!("/api-keys/{}/rotate", id), Some(&serde_json::json!({}))).await?;
        Ok(serde_json::from_value(resp)?)
    }
    pub async fn api_keys_by_owner(&self, owner_id: &str) -> Result<Vec<ApiKey>, BeckNError> {
        let data = self.request("GET", &format!("/api-keys/owner/{}", owner_id), None).await?;
        Ok(serde_json::from_value(data)?)
    }

    // === GeoDNS Discovery ===
    pub async fn discover_nearest_bap(&self, country: Option<&str>, _city: Option<&str>) -> Result<GeoDnsResult, BeckNError> {
        let mut path = String::from("/geodns/baps");
        if let Some(c) = country {
            path.push_str(&format!("?country={}", c));
        }
        let resp = self.request("GET", &path, None).await?;
        Ok(serde_json::from_value(resp)?)
    }

    pub async fn discover_nearest_bpp(&self, country: Option<&str>, _city: Option<&str>) -> Result<GeoDnsResult, BeckNError> {
        let mut path = String::from("/geodns/bpps");
        if let Some(c) = country {
            path.push_str(&format!("?country={}", c));
        }
        let resp = self.request("GET", &path, None).await?;
        Ok(serde_json::from_value(resp)?)
    }

    pub async fn discover_marketplace(&self, lat: f64, lng: f64, radius_km: u32, limit: u32) -> Result<Vec<GeoDnsResult>, BeckNError> {
        let resp = self.request("GET", &format!("/geodns/marketplaces?lat={}&lng={}&radius_km={}&limit={}", lat, lng, radius_km, limit), None).await?;
        Ok(serde_json::from_value(resp)?)
    }

    // === A2A ===
    pub async fn register_agent_card(&self, data: &AgentCardRegister) -> Result<AgentCard, BeckNError> {
        let resp = self.request("POST", "/a2a/agents", Some(&serde_json::to_value(data)?)).await?;
        Ok(serde_json::from_value(resp)?)
    }
    pub async fn list_agent_cards(&self) -> Result<Vec<AgentCard>, BeckNError> {
        let data = self.request("GET", "/a2a/agents", None).await?;
        Ok(serde_json::from_value(data)?)
    }
    pub async fn get_agent_card(&self, id: &str) -> Result<AgentCard, BeckNError> {
        let resp = self.request("GET", &format!("/a2a/agents/{}", id), None).await?;
        Ok(serde_json::from_value(resp)?)
    }
    pub async fn create_task(&self, data: &A2ATaskCreate) -> Result<A2ATask, BeckNError> {
        let resp = self.request("POST", "/a2a/tasks", Some(&serde_json::to_value(data)?)).await?;
        Ok(serde_json::from_value(resp)?)
    }
    pub async fn list_tasks(&self) -> Result<Vec<A2ATask>, BeckNError> {
        let data = self.request("GET", "/a2a/tasks", None).await?;
        Ok(serde_json::from_value(data)?)
    }
    pub async fn get_task(&self, id: &str) -> Result<A2ATask, BeckNError> {
        let resp = self.request("GET", &format!("/a2a/tasks/{}", id), None).await?;
        Ok(serde_json::from_value(resp)?)
    }
    pub async fn send_message(&self, data: &Message) -> Result<Message, BeckNError> {
        let resp = self.request("POST", "/a2a/messages", Some(&serde_json::to_value(data)?)).await?;
        Ok(serde_json::from_value(resp)?)
    }
    pub async fn list_messages(&self) -> Result<Vec<Message>, BeckNError> {
        let data = self.request("GET", "/a2a/messages", None).await?;
        Ok(serde_json::from_value(data)?)
    }
    pub async fn create_artifact(&self, data: &A2AArtifact) -> Result<Artifact, BeckNError> {
        let resp = self.request("POST", "/a2a/artifacts", Some(&serde_json::to_value(data)?)).await?;
        Ok(serde_json::from_value(resp)?)
    }
    pub async fn list_artifacts(&self) -> Result<Vec<Artifact>, BeckNError> {
        let data = self.request("GET", "/a2a/artifacts", None).await?;
        Ok(serde_json::from_value(data)?)
    }

    // === MCP ===
    pub async fn create_tool(&self, data: &McpToolCreate) -> Result<McpTool, BeckNError> {
        let resp = self.request("POST", "/mcp/tools", Some(&serde_json::to_value(data)?)).await?;
        Ok(serde_json::from_value(resp)?)
    }
    pub async fn list_tools(&self) -> Result<Vec<McpTool>, BeckNError> {
        let data = self.request("GET", "/mcp/tools", None).await?;
        Ok(serde_json::from_value(data)?)
    }
    pub async fn create_resource(&self, data: &McpResourceCreate) -> Result<McpResource, BeckNError> {
        let resp = self.request("POST", "/mcp/resources", Some(&serde_json::to_value(data)?)).await?;
        Ok(serde_json::from_value(resp)?)
    }
    pub async fn list_resources(&self) -> Result<Vec<McpResource>, BeckNError> {
        let data = self.request("GET", "/mcp/resources", None).await?;
        Ok(serde_json::from_value(data)?)
    }
    pub async fn create_prompt(&self, data: &McpPromptCreate) -> Result<McpPrompt, BeckNError> {
        let resp = self.request("POST", "/mcp/prompts", Some(&serde_json::to_value(data)?)).await?;
        Ok(serde_json::from_value(resp)?)
    }
    pub async fn list_prompts(&self) -> Result<Vec<McpPrompt>, BeckNError> {
        let data = self.request("GET", "/mcp/prompts", None).await?;
        Ok(serde_json::from_value(data)?)
    }
    pub async fn register_client(&self, data: &McpClientRegister) -> Result<McpClientInfo, BeckNError> {
        let resp = self.request("POST", "/mcp/clients", Some(&serde_json::to_value(data)?)).await?;
        Ok(serde_json::from_value(resp)?)
    }
    pub async fn list_clients(&self) -> Result<Vec<McpClientInfo>, BeckNError> {
        let data = self.request("GET", "/mcp/clients", None).await?;
        Ok(serde_json::from_value(data)?)
    }

    // === ACP ===
    pub async fn register_issuer(&self, data: &AcpIssuerRegister) -> Result<AcpIssuer, BeckNError> {
        let resp = self.request("POST", "/acp/issuers", Some(&serde_json::to_value(data)?)).await?;
        Ok(serde_json::from_value(resp)?)
    }
    pub async fn list_issuers(&self) -> Result<Vec<AcpIssuer>, BeckNError> {
        let data = self.request("GET", "/acp/issuers", None).await?;
        Ok(serde_json::from_value(data)?)
    }
    pub async fn get_issuer(&self, id: &str) -> Result<AcpIssuer, BeckNError> {
        let resp = self.request("GET", &format!("/acp/issuers/{}", id), None).await?;
        Ok(serde_json::from_value(resp)?)
    }
    pub async fn issue_token(&self, data: &AcpTokenIssue) -> Result<AcpToken, BeckNError> {
        let resp = self.request("POST", "/acp/tokens", Some(&serde_json::to_value(data)?)).await?;
        Ok(serde_json::from_value(resp)?)
    }
    pub async fn list_tokens(&self) -> Result<Vec<AcpToken>, BeckNError> {
        let data = self.request("GET", "/acp/tokens", None).await?;
        Ok(serde_json::from_value(data)?)
    }
    pub async fn introspect_token(&self, token: &str) -> Result<AcpToken, BeckNError> {
        let data = serde_json::json!({ "token": token });
        let resp = self.request("POST", "/acp/tokens/introspect", Some(&data)).await?;
        Ok(serde_json::from_value(resp)?)
    }
    pub async fn submit_presentation(&self, data: &AcpPresentationSubmit) -> Result<AcpPresentation, BeckNError> {
        let resp = self.request("POST", "/acp/presentations", Some(&serde_json::to_value(data)?)).await?;
        Ok(serde_json::from_value(resp)?)
    }
    pub async fn list_presentations(&self) -> Result<Vec<AcpPresentation>, BeckNError> {
        let data = self.request("GET", "/acp/presentations", None).await?;
        Ok(serde_json::from_value(data)?)
    }
    pub async fn create_policy(&self, data: &AcpAccessPolicyCreate) -> Result<AcpAccessPolicy, BeckNError> {
        let resp = self.request("POST", "/acp/policies", Some(&serde_json::to_value(data)?)).await?;
        Ok(serde_json::from_value(resp)?)
    }
    pub async fn list_policies(&self) -> Result<Vec<AcpAccessPolicy>, BeckNError> {
        let data = self.request("GET", "/acp/policies", None).await?;
        Ok(serde_json::from_value(data)?)
    }

    // === ANP ===
    pub async fn announce(&self, data: &AnpAnnouncementCreate) -> Result<AnpAnnouncement, BeckNError> {
        let resp = self.request("POST", "/anp/announcements", Some(&serde_json::to_value(data)?)).await?;
        Ok(serde_json::from_value(resp)?)
    }
    pub async fn list_announcements(&self) -> Result<Vec<AnpAnnouncement>, BeckNError> {
        let data = self.request("GET", "/anp/announcements", None).await?;
        Ok(serde_json::from_value(data)?)
    }
    pub async fn get_announcement(&self, id: &str) -> Result<AnpAnnouncement, BeckNError> {
        let resp = self.request("GET", &format!("/anp/announcements/{}", id), None).await?;
        Ok(serde_json::from_value(resp)?)
    }
    pub async fn register_witness(&self, data: &AnpWitnessRegister) -> Result<AnpWitness, BeckNError> {
        let resp = self.request("POST", "/anp/witnesses", Some(&serde_json::to_value(data)?)).await?;
        Ok(serde_json::from_value(resp)?)
    }
    pub async fn list_witnesses(&self) -> Result<Vec<AnpWitness>, BeckNError> {
        let data = self.request("GET", "/anp/witnesses", None).await?;
        Ok(serde_json::from_value(data)?)
    }
    pub async fn get_witness(&self, id: &str) -> Result<AnpWitness, BeckNError> {
        let resp = self.request("GET", &format!("/anp/witnesses/{}", id), None).await?;
        Ok(serde_json::from_value(resp)?)
    }
    pub async fn verify(&self, data: &AnpVerificationVerify) -> Result<AnpVerification, BeckNError> {
        let resp = self.request("POST", "/anp/verifications", Some(&serde_json::to_value(data)?)).await?;
        Ok(serde_json::from_value(resp)?)
    }
    pub async fn list_verifications(&self) -> Result<Vec<AnpVerification>, BeckNError> {
        let data = self.request("GET", "/anp/verifications", None).await?;
        Ok(serde_json::from_value(data)?)
    }
}
