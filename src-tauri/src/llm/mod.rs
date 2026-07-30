mod gemini;
mod grok;
mod mistral;
mod openai;

use anyhow::Result;
use serde::{Deserialize, Serialize};

/// One turn of conversation, including tool-call round trips, sent to a provider.
#[derive(Debug, Clone)]
pub enum ChatEvent {
    User(String),
    Assistant(String),
    /// The model asked to call a tool.
    ToolCall { id: String, name: String, arguments: String },
    /// The result we got back from actually running that tool.
    ToolResult { id: String, name: String, content: String },
}

#[derive(Debug, Clone)]
pub struct RequestedToolCall {
    pub id: String,
    pub name: String,
    /// Raw JSON object string, e.g. `{"process_name":"chrome.exe"}`.
    pub arguments: String,
}

#[derive(Debug, Clone, Default)]
pub struct ChatOutcome {
    pub message: Option<String>,
    pub tool_calls: Vec<RequestedToolCall>,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum LlmProvider {
    Openai,
    Mistral,
    Gemini,
    Grok,
}

impl LlmProvider {
    pub fn from_str(s: &str) -> Self {
        match s {
            "mistral" => LlmProvider::Mistral,
            "gemini" => LlmProvider::Gemini,
            "grok" => LlmProvider::Grok,
            _ => LlmProvider::Openai,
        }
    }
}

#[async_trait::async_trait]
pub trait LlmClient {
    async fn chat(
        &self,
        system_prompt: &str,
        history: &[ChatEvent],
        tools: &[serde_json::Value],
        api_key: &str,
        model: &str,
    ) -> Result<ChatOutcome>;
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
    history: &[ChatEvent],
    tools: &[serde_json::Value],
    api_key: &str,
    model: &str,
) -> Result<ChatOutcome> {
    if api_key.trim().is_empty() {
        anyhow::bail!(
            "No API key configured for {:?}. Add one in Settings before chatting.",
            provider
        );
    }
    let client = client_for(provider);
    client.chat(system_prompt, history, tools, api_key, model).await
}

/// Single-shot text generation, no tools, no multi-turn history — used for short
/// AI-generated proactive notifications rather than a chat exchange.
pub async fn generate_short_text(
    provider: LlmProvider,
    system_prompt: &str,
    user_message: &str,
    api_key: &str,
    model: &str,
) -> Result<String> {
    let events = vec![ChatEvent::User(user_message.to_string())];
    let outcome = route_chat(provider, system_prompt, &events, &[], api_key, model).await?;
    Ok(outcome.message.unwrap_or_default())
}
