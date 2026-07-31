mod maintenance;
mod power;
mod system_info;

use anyhow::{bail, Result};
use serde::Serialize;
use std::collections::HashMap;
use tauri::AppHandle;

#[derive(Debug, Clone, Serialize)]
pub struct ToolDefinition {
    pub name: &'static str,
    pub description: &'static str,
    pub category: &'static str,
    pub dangerous: bool,
}

pub fn list_tools() -> Vec<ToolDefinition> {
    vec![
        ToolDefinition {
            name: "empty_recycle_bin",
            description: "Empty the recycle bin / trash",
            category: "maintenance",
            dangerous: true,
        },
        ToolDefinition {
            name: "delete_temp_files",
            description: "Delete files in the OS temporary directory",
            category: "maintenance",
            dangerous: true,
        },
        ToolDefinition {
            name: "clear_browser_cache",
            description: "Clear cached data for common browsers",
            category: "maintenance",
            dangerous: true,
        },
        ToolDefinition {
            name: "analyze_large_folders",
            description: "Quick report of the largest top-level folders under the user's home directory",
            category: "maintenance",
            dangerous: false,
        },
        ToolDefinition {
            name: "find_large_files",
            description: "Deep scan of the user's home directory for the individual largest files on disk \
                (not just folders). Slower than analyze_large_folders but gives concrete file paths worth \
                deleting or moving. Use this for 'find large files' / 'why is my disk full' / 'why is my PC \
                slow' style questions, alongside the live CPU/RAM telemetry you already have.",
            category: "maintenance",
            dangerous: false,
        },
        ToolDefinition {
            name: "open_task_manager",
            description: "Open the OS task/activity manager",
            category: "performance",
            dangerous: false,
        },
        ToolDefinition {
            name: "open_settings",
            description: "Open the OS settings app",
            category: "performance",
            dangerous: false,
        },
        ToolDefinition {
            name: "kill_process",
            description: "Terminate a process by name (requires 'process_name' arg)",
            category: "performance",
            dangerous: true,
        },
        ToolDefinition {
            name: "launch_application",
            description: "Open ANY path with its OS default handler: an application/.exe, a document, a URL, \
                or a folder path (e.g. 'C:\\Users\\MEHDI' opens it in File Explorer). This is the tool to call \
                whenever the user says 'open <path>'.",
            category: "performance",
            dangerous: false,
        },
        ToolDefinition {
            name: "close_path",
            description: "Close any open File Explorer / Finder window that is showing the given folder path. \
                This is the tool to call whenever the user says 'close <path>' after you opened it.",
            category: "performance",
            dangerous: false,
        },
        ToolDefinition {
            name: "restart_pc",
            description: "Restart the computer",
            category: "power",
            dangerous: true,
        },
        ToolDefinition {
            name: "shutdown_pc",
            description: "Shut down the computer",
            category: "power",
            dangerous: true,
        },
        ToolDefinition {
            name: "sleep_pc",
            description: "Put the computer to sleep",
            category: "power",
            dangerous: false,
        },
    ]
}

/// OpenAI-style function-calling schemas for every tool, so an LLM can actually invoke
/// them instead of just describing what it would do. Gemini's function-declaration
/// format is compatible enough with this subset to reuse directly.
pub fn tool_schemas() -> Vec<serde_json::Value> {
    list_tools()
        .into_iter()
        .map(|t| {
            let parameters = match t.name {
                "kill_process" => serde_json::json!({
                    "type": "object",
                    "properties": {
                        "process_name": {
                            "type": "string",
                            "description": "Exact process name to terminate, e.g. chrome.exe or Discord.exe"
                        }
                    },
                    "required": ["process_name"]
                }),
                "launch_application" => serde_json::json!({
                    "type": "object",
                    "properties": {
                        "path": { "type": "string", "description": "Path, URL, or command to open" }
                    },
                    "required": ["path"]
                }),
                "close_path" => serde_json::json!({
                    "type": "object",
                    "properties": {
                        "path": { "type": "string", "description": "Folder path whose open window(s) should be closed" }
                    },
                    "required": ["path"]
                }),
                _ => serde_json::json!({ "type": "object", "properties": {} }),
            };
            let description = if t.dangerous {
                format!(
                    "{} Destructive action — the app itself shows the user a confirm/cancel button before this \
                     actually runs, so call it directly when it's the right thing to do; you do not need to ask \
                     permission in chat text first.",
                    t.description
                )
            } else {
                t.description.to_string()
            };
            serde_json::json!({
                "type": "function",
                "function": {
                    "name": t.name,
                    "description": description,
                    "parameters": parameters,
                }
            })
        })
        .collect()
}

pub fn is_dangerous(name: &str) -> bool {
    list_tools().iter().any(|t| t.name == name && t.dangerous)
}

pub fn description_for(name: &str) -> String {
    list_tools()
        .into_iter()
        .find(|t| t.name == name)
        .map(|t| t.description.to_string())
        .unwrap_or_default()
}

pub fn run_tool(name: &str, args: &HashMap<String, String>, app: &AppHandle) -> Result<String> {
    match name {
        "empty_recycle_bin" => maintenance::empty_recycle_bin(),
        "delete_temp_files" => maintenance::delete_temp_files(),
        "clear_browser_cache" => maintenance::clear_browser_cache(),
        "analyze_large_folders" => system_info::analyze_large_folders(),
        "find_large_files" => system_info::find_large_files(app),
        "open_task_manager" => power::open_task_manager(),
        "open_settings" => power::open_settings(),
        "kill_process" => {
            let process_name = args
                .get("process_name")
                .ok_or_else(|| anyhow::anyhow!("missing 'process_name' argument"))?;
            power::kill_process(process_name)
        }
        "launch_application" => {
            let path = args
                .get("path")
                .ok_or_else(|| anyhow::anyhow!("missing 'path' argument"))?;
            power::launch_application(path)
        }
        "close_path" => {
            let path = args
                .get("path")
                .ok_or_else(|| anyhow::anyhow!("missing 'path' argument"))?;
            power::close_path(path)
        }
        "restart_pc" => power::restart_pc(),
        "shutdown_pc" => power::shutdown_pc(),
        "sleep_pc" => power::sleep_pc(),
        other => bail!("Unknown tool: {other}"),
    }
}
