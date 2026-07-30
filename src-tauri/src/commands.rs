use crate::llm::{route_chat, ChatEvent, LlmProvider};
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
    #[serde(rename = "toolCalls", skip_serializing_if = "Option::is_none")]
    pub tool_calls: Option<Vec<ToolCallSummary>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolCallSummary {
    pub name: String,
    pub args: HashMap<String, String>,
    pub result: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProviderConfig {
    pub provider: String,
    #[serde(rename = "apiKey")]
    pub api_key: String,
    pub model: String,
}

const MAX_TOOL_ITERATIONS: u8 = 5;

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

fn parse_tool_arguments(raw: &str) -> HashMap<String, String> {
    let Ok(serde_json::Value::Object(map)) = serde_json::from_str::<serde_json::Value>(raw) else {
        return HashMap::new();
    };
    map.into_iter()
        .map(|(k, v)| (k, v.as_str().map(str::to_string).unwrap_or_else(|| v.to_string())))
        .collect()
}

/// Runs the model in a loop: the model can request tool calls, we actually execute
/// them via `tools::run_tool`, feed the real result back, and let the model react —
/// instead of the model just claiming in text that it did something.
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

    let llm_provider = LlmProvider::from_str(&provider.provider);

    let mut events: Vec<ChatEvent> = history
        .iter()
        .filter(|m| m.role == "user" || m.role == "assistant")
        .map(|m| {
            if m.role == "user" {
                ChatEvent::User(m.content.clone())
            } else {
                ChatEvent::Assistant(m.content.clone())
            }
        })
        .collect();

    let tool_schemas = tools::tool_schemas();
    let mut executed: Vec<ToolCallSummary> = Vec::new();

    for _ in 0..MAX_TOOL_ITERATIONS {
        let outcome = route_chat(
            llm_provider,
            &system_prompt,
            &events,
            &tool_schemas,
            &provider.api_key,
            &provider.model,
        )
        .await
        .map_err(|e| e.to_string())?;

        if outcome.tool_calls.is_empty() {
            let content = outcome.message.unwrap_or_default();
            return Ok(ChatMessage {
                id: uuid::Uuid::new_v4().to_string(),
                role: "assistant".to_string(),
                content,
                created_at: chrono::Utc::now().timestamp_millis(),
                tool_calls: if executed.is_empty() { None } else { Some(executed) },
            });
        }

        for call in outcome.tool_calls {
            let args = parse_tool_arguments(&call.arguments);
            let result = tools::run_tool(&call.name, &args).unwrap_or_else(|e| format!("Error: {e}"));
            let _ = state.memory.log_maintenance(&call.name, &result);

            events.push(ChatEvent::ToolCall {
                id: call.id.clone(),
                name: call.name.clone(),
                arguments: call.arguments.clone(),
            });
            events.push(ChatEvent::ToolResult {
                id: call.id,
                name: call.name.clone(),
                content: result.clone(),
            });
            executed.push(ToolCallSummary { name: call.name, args, result });
        }
    }

    Ok(ChatMessage {
        id: uuid::Uuid::new_v4().to_string(),
        role: "assistant".to_string(),
        content: "I ran out of steps chaining tool calls — but check above, I probably did what you asked."
            .to_string(),
        created_at: chrono::Utc::now().timestamp_millis(),
        tool_calls: if executed.is_empty() { None } else { Some(executed) },
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
pub fn save_personality(state: State<AppState>, personality: String) -> Result<(), String> {
    state.memory.set_setting("personality", &personality).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn load_personality(state: State<AppState>) -> Result<Option<String>, String> {
    state.memory.get_setting("personality").map_err(|e| e.to_string())
}

#[tauri::command]
pub fn list_alerts(state: State<AppState>) -> Result<Vec<Alert>, String> {
    state.memory.list_alerts(50).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn acknowledge_alert(state: State<AppState>, id: i64) -> Result<(), String> {
    state.memory.acknowledge_alert(id).map_err(|e| e.to_string())
}
