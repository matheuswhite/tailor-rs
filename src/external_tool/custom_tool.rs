use std::process::Command;

use crate::{
    absolute_path::AbsolutePath,
    external_tool::registry::Registry,
    fmt::success,
    manifest::{Manifest, dependency::Dependency},
    mode::Mode,
    storage::Storage,
};

pub struct CustomTool;

impl CustomTool {
    pub fn build(
        mode: Mode,
        path: &AbsolutePath,
        tool: Dependency,
        sources: Vec<String>,
        includes: Vec<String>,
        registry: &Registry,
    ) -> Result<(), String> {
        let options = tool.options().to_vec();
        let manifest = Storage::download(tool, registry)?;
        let manifest_name = manifest.name();
        let Manifest::Tool { script, .. } = manifest else {
            return Err("Expected a tool manifest".to_string());
        };

        // $script build $mode "$path" "$sources" "$includes" "$options"
        println!(
            "{} `{}` in {} mode",
            success("Building"),
            manifest_name,
            mode
        );

        let status = Command::new(script)
            .arg("build")
            .arg(mode.to_string())
            .arg(path.inner())
            .arg(sources.join(","))
            .arg(includes.join(","))
            .arg(
                options
                    .into_iter()
                    .map(|opt| opt.to_cli_option())
                    .collect::<Vec<String>>()
                    .join(","),
            )
            .status()
            .map_err(|e| format!("Failed to build in {} mode: {}", mode, e))?;
        if !status.success() {
            return Err(format!(
                "Custom tool build failed in {} mode with status: {}",
                mode, status
            ));
        }

        Ok(())
    }

    pub fn run(
        mode: Mode,
        path: &AbsolutePath,
        tool: Dependency,
        registry: &Registry,
    ) -> Result<(), String> {
        let options = tool.options().to_vec();
        let manifest = Storage::download(tool, registry)?;
        let manifest_name = manifest.name();
        let Manifest::Tool { script, .. } = manifest else {
            return Err("Expected a tool manifest".to_string());
        };

        // $script run $mode "$path" "$options"
        println!(
            "{} `{}` in {} mode",
            success("Running"),
            manifest_name,
            mode
        );

        let status = Command::new(script)
            .arg("run")
            .arg(mode.to_string())
            .arg(path.inner())
            .arg(
                options
                    .into_iter()
                    .map(|opt| opt.to_cli_option())
                    .collect::<Vec<String>>()
                    .join(","),
            )
            .status()
            .map_err(|e| format!("Failed to run in {} mode: {}", mode, e))?;
        if !status.success() {
            return Err(format!(
                "Custom tool run failed in {} mode with status: {}",
                mode, status
            ));
        }

        Ok(())
    }
}
