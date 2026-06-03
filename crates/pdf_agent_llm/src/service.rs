use async_trait::async_trait;
use pdf_agent_core::error::{Error, Result};
use pdf_agent_core::providers::traits::{LlmRequest, LlmService};
use reqwest::header::{HeaderMap, HeaderValue, AUTHORIZATION, CONTENT_TYPE};
use serde_json::{json, Value};

pub struct OpenAiLlmService {
    api_key: String,
    base_url: String,
    model_name: String,
    client: reqwest::Client,
}

impl OpenAiLlmService {
    pub fn new(api_key: String, base_url: String, model_name: String) -> Self {
        Self {
            api_key,
            base_url,
            model_name,
            client: reqwest::Client::new(),
        }
    }
}

#[async_trait]
impl LlmService for OpenAiLlmService {
    async fn complete(&self, request: LlmRequest) -> Result<String> {
        let url = format!("{}/chat/completions", self.base_url.trim_end_matches('/'));
        
        let mut messages = Vec::new();
        if let Some(system) = request.system_prompt {
            messages.push(json!({
                "role": "system",
                "content": system
            }));
        }
        messages.push(json!({
            "role": "user",
            "content": request.user_prompt
        }));

        let mut body = json!({
            "model": self.model_name,
            "messages": messages,
        });

        if let Some(temp) = request.temperature {
            body["temperature"] = json!(temp);
        }
        if let Some(tokens) = request.max_tokens {
            body["max_tokens"] = json!(tokens);
        }
        if request.json_mode {
            body["response_format"] = json!({ "type": "json_object" });
        }

        let mut headers = HeaderMap::new();
        headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));
        headers.insert(
            AUTHORIZATION,
            HeaderValue::from_str(&format!("Bearer {}", self.api_key))
                .map_err(|e| Error::Llm(format!("Invalid auth header: {}", e)))?,
        );

        let res = self.client.post(&url)
            .headers(headers)
            .json(&body)
            .send()
            .await
            .map_err(|e| Error::Llm(format!("HTTP request failed: {}", e)))?;

        if !res.status().is_success() {
            let status = res.status();
            let err_text = res.text().await.unwrap_or_default();
            return Err(Error::Llm(format!("API error response ({}): {}", status, err_text)));
        }

        let json_resp: Value = res.json().await
            .map_err(|e| Error::Llm(format!("Failed to parse response JSON: {}", e)))?;

        let text = json_resp["choices"][0]["message"]["content"]
            .as_str()
            .ok_or_else(|| Error::Llm("Missing content in completion response".to_string()))?;

        Ok(text.to_string())
    }

    async fn complete_json(&self, request: LlmRequest) -> Result<Value> {
        let mut req = request;
        req.json_mode = true;
        let text = self.complete(req).await?;
        let val: Value = serde_json::from_str(&text)
            .map_err(|e| Error::Llm(format!("Failed to parse LLM text response as JSON: {}", e)))?;
        Ok(val)
    }
}

pub struct MockLlmService;

#[async_trait]
impl LlmService for MockLlmService {
    async fn complete(&self, request: LlmRequest) -> Result<String> {
        if request.json_mode {
            Ok(r#"{"patched_markdown": "This is a mocked patched result."}"#.to_string())
        } else {
            Ok("This is a mocked LLM completion response.".to_string())
        }
    }

    async fn complete_json(&self, request: LlmRequest) -> Result<Value> {
        let text = self.complete(request).await?;
        let val = serde_json::from_str(&text).unwrap();
        Ok(val)
    }
}
