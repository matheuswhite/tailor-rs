use crate::{
    external_tool::compile_commands::CompileCommandEntry,
    fmt::{Progress, success},
    manifest::package_type::PackageType,
    mode::Mode,
    package::Package,
};
use std::{path::Path, process::Command};

pub struct Compiler {
    compiler: String,
    pkg_full_name: String,
}

impl Compiler {
    pub fn new(compiler: &str, pkg_full_name: String) -> Self {
        Self {
            compiler: compiler.to_string(),
            pkg_full_name,
        }
    }

    fn get_object_path(source: &str, build_path: &Path) -> Result<String, String> {
        let source_path = Path::new(source);
        let file_stem = source_path
            .file_stem()
            .ok_or_else(|| "invalid source file".to_string())?
            .to_string_lossy();
        let object_path = build_path.join(format!("{}.o", file_stem));
        Ok(object_path.to_string_lossy().to_string())
    }

    pub fn build(
        &self,
        mode: Mode,
        build_path: &Path,
        package: Package,
        pkg_type: PackageType,
        defines: Vec<String>,
    ) -> Result<(), String> {
        let mut dependencies = package.dependencies().to_vec();
        dependencies.push(package.manifest().clone());

        let mut object_list = vec![];
        let mut compile_command_entries = vec![];

        let mut progress = Progress::new("Building", dependencies.len());
        for dependency in dependencies {
            let message = format!(
                "{} {} v{}",
                success("Compiling"),
                dependency.name(),
                dependency.version()
            );

            for source in dependency.sources() {
                let object_path = Self::get_object_path(&source, build_path)?;

                object_list.push(object_path.clone());

                let include_list = dependency
                    .includes()
                    .iter()
                    .map(|inc| format!("-I{}", inc))
                    .collect::<Vec<_>>()
                    .join(" ");

                let define_list = defines.join(" ");
                let opt_level = match mode {
                    Mode::Debug => "-Og",
                    Mode::Release => "-Os",
                };
                let compile_cmd = format!(
                    "{} -c {} {} {} {} -o {}",
                    self.compiler, source, opt_level, define_list, include_list, object_path
                );

                let source_path = Path::new(&source);
                let source_dir = source_path.parent().unwrap_or_else(|| Path::new("."));
                let source_file = source_path
                    .file_name()
                    .unwrap_or_else(|| std::ffi::OsStr::new(""))
                    .into();
                let arguments = vec![
                    self.compiler.clone(),
                    "-c".to_string(),
                    source.clone(),
                    opt_level.to_string(),
                    define_list,
                    include_list,
                    "-o".to_string(),
                    object_path.clone(),
                ];
                let compile_command_entry =
                    CompileCommandEntry::new(source_dir.to_owned(), arguments, source_file);
                compile_command_entries.push(compile_command_entry);

                let status = Command::new("sh")
                    .arg("-c")
                    .arg(&compile_cmd)
                    .status()
                    .map_err(|e| {
                        format!("failed to execute compile command `{}`: {}", compile_cmd, e)
                    })?;
                if !status.success() {
                    return Err(format!(
                        "compilation failed for dependency source: {}",
                        source
                    ));
                }
            }

            progress.next(&message);
        }

        let compile_commands_json = serde_json::to_string_pretty(&compile_command_entries)
            .map_err(|e| format!("failed to serialize compile commands: {}", e))?;
        std::fs::write(
            build_path.join("compile_commands.json"),
            compile_commands_json,
        )
        .map_err(|e| format!("failed to write compile_commands.json: {}", e))?;

        let link_cmd = match pkg_type {
            PackageType::Binary => format!(
                "{} {} -o {}",
                self.compiler,
                object_list.join(" "),
                build_path.join(&self.pkg_full_name).to_string_lossy()
            ),
            PackageType::Library => format!(
                "{} -shared {} -o {}",
                self.compiler,
                object_list.join(" "),
                build_path
                    .join(format!("lib{}.so", self.pkg_full_name))
                    .to_string_lossy()
            ),
        };

        let status = std::process::Command::new("sh")
            .arg("-c")
            .arg(link_cmd)
            .status()
            .map_err(|e| format!("failed to execute link command: {}", e))?;
        if !status.success() {
            return Err("linking failed".to_string());
        }

        progress.finish();

        Ok(())
    }
}
