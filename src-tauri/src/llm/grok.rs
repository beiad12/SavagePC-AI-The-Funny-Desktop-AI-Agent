use super::openai::openai_compatible_chat;
use super::{ChatTurn, LlmClient};
use anyhow::Result;

pub struct GrokClient;

#[async_trait::async_trait]
impl LlmClient for GrokClient {
    async fn chat(
        &self,
        system_prompt: &str,
        history: &[ChatTurn],
        api_key: &str,
        model: &str,
    ) -> Result<String> {
        openai_compatible_chat(
            "https://api.x.ai/v1/chat/completions",
            api_key,
            model,
            system_prompt,
            history,
        )
        .await
    }
}
