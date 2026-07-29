use super::{ChatTurn, LlmClient};
use anyhow::{bail, Result};
use serde_json::json;

pub struct GeminiClient;

#[async_trait::async_trait]
impl LlmClient for GeminiClient {
    async fn chat(
        &self,
        system_prompt: &str,
        history: &[ChatTurn],
        api_key: &str,
        model: &str,
    ) -> Result<String> {
        let url = format!(
            "https://generativelanguage.googleapis.com/v1beta/models/{model}:generateContent?key={api_key}"
        );

        let contents: Vec<_> = history
            .iter()
            .map(|turn| {
                let role = if turn.role == "assistant" {
                    "model"
                } else {
                    "user"
                };
                json!({ "role": role, "parts": [{ "text": turn.content }] })
            })
            .collect();

        let client = reqwest::Client::new();
        let resp = client
            .post(&url)
            .json(&json!({
                "systemInstruction": { "parts": [{ "text": system_prompt }] },
                "contents": contents,
                "generationConfig": { "temperature": 0.8 },
            }))
            .send()
            .await?;

        if !resp.status().is_success() {
            let status = resp.status();
            let body = resp.text().await.unwrap_or_default();
            bail!("LLM request failed ({status}): {body}");
        }

        let body: serde_json::Value = resp.json().await?;
        let content = body["candidates"][0]["content"]["parts"][0]["text"]
            .as_str()
            .unwrap_or("(empty response)")
            .to_string();
        Ok(content)
    }
}
