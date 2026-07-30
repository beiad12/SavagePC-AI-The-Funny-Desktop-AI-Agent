use super::{ChatEvent, ChatOutcome, LlmClient, RequestedToolCall};
use anyhow::{bail, Result};
use serde_json::json;

pub struct GeminiClient;

#[async_trait::async_trait]
impl LlmClient for GeminiClient {
    async fn chat(
        &self,
        system_prompt: &str,
        history: &[ChatEvent],
        tools: &[serde_json::Value],
        api_key: &str,
        model: &str,
    ) -> Result<ChatOutcome> {
        let url = format!(
            "https://generativelanguage.googleapis.com/v1beta/models/{model}:generateContent?key={api_key}"
        );

        let mut contents = Vec::new();
        for event in history {
            match event {
                ChatEvent::User(text) => contents.push(json!({ "role": "user", "parts": [{ "text": text }] })),
                ChatEvent::Assistant(text) => contents.push(json!({ "role": "model", "parts": [{ "text": text }] })),
                ChatEvent::ToolCall { name, arguments, .. } => {
                    let args: serde_json::Value =
                        serde_json::from_str(arguments).unwrap_or(serde_json::Value::Object(Default::default()));
                    contents.push(json!({
                        "role": "model",
                        "parts": [{ "functionCall": { "name": name, "args": args } }]
                    }));
                }
                ChatEvent::ToolResult { name, content, .. } => contents.push(json!({
                    "role": "function",
                    "parts": [{ "functionResponse": { "name": name, "response": { "content": content } } }]
                })),
            }
        }

        let mut body = json!({
            "systemInstruction": { "parts": [{ "text": system_prompt }] },
            "contents": contents,
            "generationConfig": { "temperature": 0.8 },
        });
        if !tools.is_empty() {
            let declarations: Vec<serde_json::Value> = tools
                .iter()
                .filter_map(|t| {
                    let f = &t["function"];
                    Some(json!({
                        "name": f["name"].as_str()?,
                        "description": f["description"].as_str().unwrap_or(""),
                        "parameters": f["parameters"],
                    }))
                })
                .collect();
            body["tools"] = json!([{ "functionDeclarations": declarations }]);
        }

        let client = reqwest::Client::new();
        let resp = client.post(&url).json(&body).send().await?;

        if !resp.status().is_success() {
            let status = resp.status();
            let body = resp.text().await.unwrap_or_default();
            bail!("LLM request failed ({status}): {body}");
        }

        let body: serde_json::Value = resp.json().await?;
        let empty = Vec::new();
        let parts = body["candidates"][0]["content"]["parts"].as_array().unwrap_or(&empty);

        let mut tool_calls = Vec::new();
        let mut text = String::new();
        for part in parts {
            if let Some(call) = part.get("functionCall") {
                if let Some(name) = call["name"].as_str() {
                    tool_calls.push(RequestedToolCall {
                        id: uuid::Uuid::new_v4().to_string(),
                        name: name.to_string(),
                        arguments: call["args"].to_string(),
                    });
                }
            } else if let Some(t) = part["text"].as_str() {
                text.push_str(t);
            }
        }

        if !tool_calls.is_empty() {
            return Ok(ChatOutcome { message: None, tool_calls });
        }
        if text.is_empty() {
            text.push_str("(empty response)");
        }
        Ok(ChatOutcome { message: Some(text), tool_calls: vec![] })
    }
}
