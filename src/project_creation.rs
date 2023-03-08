extern crate reqwest;

use crate::disk::Disk;
use crate::error::TailorErr;
use crate::message::Message;
use crate::progress_bar::ProgressBar;
use crate::remote_repo::Github;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

const VERSION: &'static str = "v0.1.0";

pub struct ProjectBuilder {
    project_name: String,
    overwrite: bool,
    git: bool,
    repository: Github,
}

#[derive(Deserialize, Serialize)]
struct ProjectConfigFile {
    project: ProjectOptions,
    rust: RustOptions,
    zbus: Option<ZbusOptions>,
}

#[derive(Deserialize, Serialize)]
struct ProjectOptions {
    name: String,
    arch: String,
}

#[derive(Deserialize, Serialize)]
struct RustOptions {
    entry_name: String,
    n_tasks: u16,
}

#[derive(Deserialize, Serialize)]
struct ZbusOptions {
    channels: Vec<ZbusOptionChannel>,
}

#[derive(Deserialize, Serialize)]
struct ZbusOptionChannel {
    name: String,
    item_type: String,
    observers: Option<Vec<String>>,
}

impl ProjectBuilder {
    const ARCH: &'static str = "thumbv7em-none-eabi";

    pub fn new(project_name: &str) -> Self {
        ProjectBuilder {
            project_name: project_name.to_string(),
            overwrite: false,
            git: false,
            repository: Github::new("matheuswhite", "hat-rs-template", Disk::new(project_name)),
        }
    }

    pub fn enable_overwrite(&mut self) {
        self.overwrite = true;
    }

    pub fn enable_git(&mut self) {
        self.git = true;
    }

    pub async fn build(&self) -> Result<(), TailorErr> {
        if self.prepering_dir().is_err() {
            return Err(TailorErr::PreperingDirFail);
        }

        if self.create_dirs().is_err() {
            return Err(TailorErr::CreateDirsFail);
        }

        if self.download_bridge_files().await.is_err() {
            return Err(TailorErr::DownloadFilesFail);
        }

        if self.gen_rust_files().await.is_err() {
            return Err(TailorErr::RustFileGenerationFail);
        }

        if self.gen_c_files().await.is_err() {
            return Err(TailorErr::CFileGenerationFail);
        }

        if self.git {
            if self.init_git_repo().is_err() {
                return Err(TailorErr::GitInitError);
            }
        }

        if self.create_hat_toml().is_err() {
            return Err(TailorErr::CreateHatTomlFail);
        }

        Message::ok("Finished", &format!("{} @ {}", self.project_name, VERSION)).print();
        Ok(())
    }

    fn prepering_dir(&self) -> anyhow::Result<()> {
        if !dir_exist(&self.project_name) {
            std::fs::create_dir(&self.project_name)?;
        } else if !dir_is_empty(&self.project_name) {
            if self.overwrite {
                Message::warning(&format!(
                    "Directory \"{}\" is not empty, but the overwrite flag was enabled",
                    &self.project_name
                ))
                .print();

                clear_dir_content(&self.project_name);

                Message::ok("Cleared", &format!("{} is now clean", &self.project_name)).print();
            } else {
                Message::fail(&format!(
                    "Directory \"{}\" is not an empty directory",
                    &self.project_name
                ))
                .print();

                return Err(anyhow::Error::msg("Empty Dir"));
            }
        }

        Ok(())
    }

    fn create_dirs(&self) -> anyhow::Result<()> {
        let dirs = ["rust", "rust/bridge", "rust/src", "src"];
        let mut progress_bar = ProgressBar::new("Creating", dirs.len(), true);
        progress_bar.print();

        for dir in dirs {
            std::fs::create_dir(PathBuf::from(self.project_name.clone() + "/" + dir))?;
            progress_bar.next();
        }

        Message::ok("Created", &dirs.join(", ")).print();

        Ok(())
    }

    async fn download_bridge_files(&self) -> anyhow::Result<()> {
        let files = [
            "hat_bridge.c",
            "zbus_bridge.c",
            "rustlib_bridge.c",
            "bridge.h",
        ];

        let mut progress_bar = ProgressBar::new("Downloading", files.len(), true);
        progress_bar.print();

        for file in files {
            self.repository
                .get_n_store("bridge/", "rust/bridge/", file)
                .await?;
            progress_bar.next();
        }
        Message::ok("Downloaded", &files.join(", ")).print();

        Ok(())
    }

    async fn gen_rust_files(&self) -> anyhow::Result<()> {
        let files = [("Cargo.toml", "rust/"), ("lib.rs", "rust/src/")];
        let mut progress_bar = ProgressBar::new("Generating", files.len() + 1, true);
        progress_bar.print();

        for (file, dst) in files {
            self.repository.get_n_store("rust/", dst, file).await?;
            progress_bar.next();
        }

        let cmake_content = self.repository.get("rust/CMakeLists.txt").await?;
        let cmake_content = cmake_content.replace("@ARCH@", ProjectBuilder::ARCH);
        self.repository
            .disk()
            .create_store("rust/CMakeLists.txt", cmake_content)?;
        progress_bar.next();

        Message::ok("Generated", "rust files generated").print();

        Ok(())
    }

    async fn gen_c_files(&self) -> anyhow::Result<()> {
        let files = [("main.c", "src/"), ("prj.conf", "")];
        let mut progress_bar = ProgressBar::new("Generating", files.len() + 1, true);
        progress_bar.print();

        for (file, dst) in files {
            self.repository.get_n_store("c/", dst, file).await?;
            progress_bar.next();
        }

        let cmake_content = self.repository.get("c/CMakeLists.txt").await?;
        let cmake_content = cmake_content.replace("@PROJECT_NAME@", &self.project_name);
        self.repository
            .disk()
            .create_store("CMakeLists.txt", cmake_content)?;
        progress_bar.next();

        Message::ok("Generated", "c files generated").print();

        Ok(())
    }

    fn init_git_repo(&self) -> anyhow::Result<()> {
        let mut progress_bar = ProgressBar::new("Creating", 2, true);

        let output = std::process::Command::new("git")
            .args(["init", "-b", "main", &self.project_name])
            .output()?;
        progress_bar.next();

        let dot_gitignore = "rust/target\nrust/Cargo.lock\nbuild/\n".to_string();
        self.repository
            .disk()
            .create_store(".gitignore", dot_gitignore)?;
        progress_bar.next();

        Message::ok("Created", "git/ and .gitignore created").print();

        assert_eq!(output.status.code(), Some(0));

        Ok(())
    }

    fn create_hat_toml(&self) -> anyhow::Result<()> {
        let config = ProjectConfigFile {
            project: ProjectOptions {
                name: self.project_name.to_string(),
                arch: ProjectBuilder::ARCH.to_string(),
            },
            rust: RustOptions {
                entry_name: "main_task".to_string(),
                n_tasks: 1,
            },
            zbus: None,
        };

        let content = toml::to_string(&config)?;

        self.repository
            .disk()
            .create_store("Tailor.toml", content)?;

        Message::ok("Saved", "Project options saved at Tailor.toml").print();

        Ok(())
    }
}

fn dir_exist(path: &str) -> bool {
    PathBuf::from(path).is_dir()
}

fn dir_is_empty(path: &str) -> bool {
    PathBuf::from(path)
        .read_dir()
        .expect(&format!("Cannot read dir {path}"))
        .next()
        .is_none()
}

fn clear_dir_content(path: &str) {
    std::fs::remove_dir_all(path).expect(&format!("Cannot remove dirs inside {path}"));
    std::fs::create_dir(path).expect(&format!("Cannot create dir {path}"));
}
