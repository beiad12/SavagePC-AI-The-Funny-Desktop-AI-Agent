use super::{ChatTurn, LlmClient};
use anyhow::{bail, Result};
use serde_json::json;

pub struct OpenAiClient;

#[async_trait::async_trait]
impl LlmClient for OpenAiClient {
    async fn chat(
        &self,
        system_prompt: &str,
        history: &[ChatTurn],
        api_key: &str,
        model: &str,
    ) -> Result<String> {
        openai_compatible_chat(
            "https://api.openai.com/v1/chat/completions",
            api_key,
            model,
            system_prompt,
            history,
        )
        .await
    }
}

/// Shared implementation for OpenAI-compatible chat completion APIs
/// (OpenAI, Mistral, and Grok all speak this dialect).
pub async fn openai_compatible_chat(
    url: &str,
    api_key: &str,
    model: &str,
    system_prompt: &str,
    history: &[ChatTurn],
) -> Result<String> {
    let mut messages = vec![json!({ "role": "system", "content": system_prompt })];
    for turn in history {
        messages.push(json!({ "role": turn.role, "content": turn.content }));
    }

    let client = reqwest::Client::new();
    let resp = client
        .post(url)
        .bearer_auth(api_key)
        .json(&json!({
            "model": model,
            "messages": messages,
            "temperature": 0.8,
        }))
        .send()
        .await?;

    if !resp.status().is_success() {
        let status = resp.status();
        let body = resp.text().await.unwrap_or_default();
        bail!("LLM request failed ({status}): {body}");
    }

    let body: serde_json::Value = resp.json().await?;
    let content = body["choices"][0]["message"]["content"]
        .as_str()
        .unwrap_or("(empty response)")
        .to_string();
    Ok(content)
}
