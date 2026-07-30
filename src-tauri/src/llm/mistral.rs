use super::openai::openai_compatible_chat;
use super::{ChatEvent, ChatOutcome, LlmClient};
use anyhow::Result;

pub struct MistralClient;

#[async_trait::async_trait]
impl LlmClient for MistralClient {
    async fn chat(
        &self,
        system_prompt: &str,
        history: &[ChatEvent],
        tools: &[serde_json::Value],
        api_key: &str,
        model: &str,
    ) -> Result<ChatOutcome> {
        openai_compatible_chat(
            "https://api.mistral.ai/v1/chat/completions",
            api_key,
            model,
            system_prompt,
            history,
            tools,
        )
        .await
    }
}
