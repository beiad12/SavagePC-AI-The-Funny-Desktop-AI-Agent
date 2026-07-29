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
