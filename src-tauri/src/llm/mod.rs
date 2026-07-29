mod gemini;
mod grok;
mod mistral;
mod openai;

use anyhow::Result;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatTurn {
    pub role: String,
    pub content: String,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum LlmProvider {
    Openai,
    Mistral,
    Gemini,
    Grok,
}

#[async_trait::async_trait]
pub trait LlmClient {
    async fn chat(
        &self,
        system_prompt: &str,
        history: &[ChatTurn],
        api_key: &str,
        model: &str,
    ) -> Result<String>;
}

pub fn client_for(provider: LlmProvider) -> Box<dyn LlmClient + Send + Sync> {
    match provider {
        LlmProvider::Openai => Box::new(openai::OpenAiClient),
        LlmProvider::Mistral => Box::new(mistral::MistralClient),
        LlmProvider::Gemini => Box::new(gemini::GeminiClient),
        LlmProvider::Grok => Box::new(grok::GrokClient),
    }
}

pub async fn route_chat(
    provider: LlmProvider,
    system_prompt: &str,
    history: &[ChatTurn],
    api_key: &str,
    model: &str,
) -> Result<String> {
    if api_key.trim().is_empty() {
        anyhow::bail!(
            "No API key configured for {:?}. Add one in Settings before chatting.",
            provider
        );
    }
    let client = client_for(provider);
    client.chat(system_prompt, history, api_key, model).await
}
