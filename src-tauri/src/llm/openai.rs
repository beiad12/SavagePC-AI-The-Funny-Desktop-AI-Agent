use super::{ChatEvent, ChatOutcome, LlmClient, RequestedToolCall};
use anyhow::{bail, Result};
use serde_json::json;

pub struct OpenAiClient;

#[async_trait::async_trait]
impl LlmClient for OpenAiClient {
    async fn chat(
        &self,
        system_prompt: &str,
        history: &[ChatEvent],
        tools: &[serde_json::Value],
        api_key: &str,
        model: &str,
    ) -> Result<ChatOutcome> {
        openai_compatible_chat(
            "https://api.openai.com/v1/chat/completions",
            api_key,
            model,
            system_prompt,
            history,
            tools,
        )
        .await
    }
}

/// Shared implementation for OpenAI-compatible chat completion APIs with function
/// calling (OpenAI, Mistral, and Grok all speak this dialect).
pub async fn openai_compatible_chat(
    url: &str,
    api_key: &str,
    model: &str,
    system_prompt: &str,
    history: &[ChatEvent],
    tools: &[serde_json::Value],
) -> Result<ChatOutcome> {
    let mut messages = vec![json!({ "role": "system", "content": system_prompt })];
    for event in history {
        match event {
            ChatEvent::User(text) => messages.push(json!({ "role": "user", "content": text })),
            ChatEvent::Assistant(text) => messages.push(json!({ "role": "assistant", "content": text })),
            ChatEvent::ToolCall { id, name, arguments } => messages.push(json!({
                "role": "assistant",
                "content": null,
                "tool_calls": [{
                    "id": id,
                    "type": "function",
                    "function": { "name": name, "arguments": arguments }
                }]
            })),
            ChatEvent::ToolResult { id, content, .. } => messages.push(json!({
                "role": "tool",
                "tool_call_id": id,
                "content": content,
            })),
        }
    }

    let mut body = json!({
        "model": model,
        "messages": messages,
        "temperature": 0.8,
    });
    if !tools.is_empty() {
        body["tools"] = json!(tools);
        body["tool_choice"] = json!("auto");
    }

    let client = reqwest::Client::new();
    let resp = client.post(url).bearer_auth(api_key).json(&body).send().await?;

    if !resp.status().is_success() {
        let status = resp.status();
        let body = resp.text().await.unwrap_or_default();
        bail!("LLM request failed ({status}): {body}");
    }

    let body: serde_json::Value = resp.json().await?;
    let message = &body["choices"][0]["message"];

    if let Some(calls) = message["tool_calls"].as_array() {
        let tool_calls = calls
            .iter()
            .filter_map(|c| {
                Some(RequestedToolCall {
                    id: c["id"].as_str()?.to_string(),
                    name: c["function"]["name"].as_str()?.to_string(),
                    arguments: c["function"]["arguments"].as_str().unwrap_or("{}").to_string(),
                })
            })
            .collect();
        return Ok(ChatOutcome { message: None, tool_calls });
    }

    let content = message["content"].as_str().unwrap_or("(empty response)").to_string();
    Ok(ChatOutcome { message: Some(content), tool_calls: vec![] })
}
