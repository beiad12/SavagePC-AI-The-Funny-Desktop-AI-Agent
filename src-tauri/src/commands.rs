use crate::llm::{route_chat, ChatTurn, LlmProvider};
use crate::memory::{Alert, Memory};
use crate::personality::{build_system_prompt, Personality};
use crate::telemetry::{Telemetry, TelemetryCollector};
use crate::tools;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use tauri::State;

pub struct AppState {
    pub telemetry: Arc<Mutex<TelemetryCollector>>,
    pub memory: Arc<Memory>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatMessage {
    pub id: String,
    pub role: String,
    pub content: String,
    #[serde(rename = "createdAt")]
    pub created_at: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProviderConfig {
    pub provider: String,
    #[serde(rename = "apiKey")]
    pub api_key: String,
    pub model: String,
}

#[tauri::command]
pub fn get_telemetry(state: State<AppState>) -> Result<Telemetry, String> {
    let mut collector = state.telemetry.lock().map_err(|e| e.to_string())?;
    Ok(collector.collect())
}

#[tauri::command]
pub fn list_tools() -> Vec<tools::ToolDefinition> {
    tools::list_tools()
}

#[tauri::command]
pub fn run_tool(
    state: State<AppState>,
    name: String,
    args: Option<HashMap<String, String>>,
) -> Result<String, String> {
    let result = tools::run_tool(&name, &args.unwrap_or_default()).map_err(|e| e.to_string())?;
    let _ = state.memory.log_maintenance(&name, &result);
    Ok(result)
}

#[tauri::command]
pub async fn send_chat_message(
    state: State<'_, AppState>,
    history: Vec<ChatMessage>,
    personality: String,
    provider: ProviderConfig,
    language: String,
) -> Result<ChatMessage, String> {
    let telemetry = {
        let mut collector = state.telemetry.lock().map_err(|e| e.to_string())?;
        collector.collect()
    };
    let telemetry_json = serde_json::to_string_pretty(&telemetry).map_err(|e| e.to_string())?;
    let system_prompt = build_system_prompt(Personality::from_str(&personality), &telemetry_json, &language);

    let llm_provider = match provider.provider.as_str() {
        "mistral" => LlmProvider::Mistral,
        "gemini" => LlmProvider::Gemini,
        "grok" => LlmProvider::Grok,
        _ => LlmProvider::Openai,
    };

    let turns: Vec<ChatTurn> = history
        .iter()
        .filter(|m| m.role == "user" || m.role == "assistant")
        .map(|m| ChatTurn {
            role: m.role.clone(),
            content: m.content.clone(),
        })
        .collect();

    let reply = route_chat(
        llm_provider,
        &system_prompt,
        &turns,
        &provider.api_key,
        &provider.model,
    )
    .await
    .map_err(|e| e.to_string())?;

    Ok(ChatMessage {
        id: uuid::Uuid::new_v4().to_string(),
        role: "assistant".to_string(),
        content: reply,
        created_at: chrono::Utc::now().timestamp_millis(),
    })
}

#[tauri::command]
pub fn save_provider_config(state: State<AppState>, config: ProviderConfig) -> Result<(), String> {
    let json = serde_json::to_string(&config).map_err(|e| e.to_string())?;
    state
        .memory
        .set_setting("provider_config", &json)
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub fn load_provider_config(state: State<AppState>) -> Result<Option<ProviderConfig>, String> {
    let raw = state
        .memory
        .get_setting("provider_config")
        .map_err(|e| e.to_string())?;
    match raw {
        Some(json) => serde_json::from_str(&json)
            .map(Some)
            .map_err(|e| e.to_string()),
        None => Ok(None),
    }
}

#[tauri::command]
pub fn save_language(state: State<AppState>, language: String) -> Result<(), String> {
    state.memory.set_setting("language", &language).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn load_language(state: State<AppState>) -> Result<Option<String>, String> {
    state.memory.get_setting("language").map_err(|e| e.to_string())
}

#[tauri::command]
pub fn list_alerts(state: State<AppState>) -> Result<Vec<Alert>, String> {
    state.memory.list_alerts(50).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn acknowledge_alert(state: State<AppState>, id: i64) -> Result<(), String> {
    state.memory.acknowledge_alert(id).map_err(|e| e.to_string())
}
