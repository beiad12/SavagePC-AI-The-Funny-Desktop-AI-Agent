mod maintenance;
mod power;
mod system_info;

use anyhow::{bail, Result};
use serde::Serialize;
use std::collections::HashMap;

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
            description: "Report the largest folders under the user's home directory",
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
            description: "Launch an application by path or command (requires 'path' arg)",
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
                        "path": { "type": "string", "description": "Path or command to launch" }
                    },
                    "required": ["path"]
                }),
                _ => serde_json::json!({ "type": "object", "properties": {} }),
            };
            let description = if t.dangerous {
                format!(
                    "{} Destructive action — only call this after the user has explicitly confirmed \
                     in the conversation (a plain 'yes'/'go ahead'/'واخا' etc. to your own confirmation \
                     question counts).",
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

pub fn run_tool(name: &str, args: &HashMap<String, String>) -> Result<String> {
    match name {
        "empty_recycle_bin" => maintenance::empty_recycle_bin(),
        "delete_temp_files" => maintenance::delete_temp_files(),
        "clear_browser_cache" => maintenance::clear_browser_cache(),
        "analyze_large_folders" => system_info::analyze_large_folders(),
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
        "restart_pc" => power::restart_pc(),
        "shutdown_pc" => power::shutdown_pc(),
        "sleep_pc" => power::sleep_pc(),
        other => bail!("Unknown tool: {other}"),
    }
}
