use std::path::PathBuf;

use crate::{
    cmake,
    commands::command::Command,
    dependency_manager::resolve_dependencies,
    fmt::{info, success},
    mode::Mode,
    package::{Package, PackageType},
};

#[derive(Default)]
pub struct BuildPkg {
    path: PathBuf,
    mode: Mode,
}

impl BuildPkg {
    #[allow(unused)]
    pub fn path(&self) -> &PathBuf {
        &self.path
    }

    fn create_cmake_lists(
        &self,
        content: &str,
        sources_ext: Vec<String>,
        includes_ext: Vec<String>,
    ) -> Result<Package, String> {
        let abs_path = self
            .path
            .canonicalize()
            .map_err(|e| format!("fail to canonicalize path: {}", e))?;

        if !abs_path.join("Tailor.toml").exists() {
            return Err("Tailor.toml file does not exist".to_string());
        }

        let pkg = Package::from_file(&abs_path.join("Tailor.toml"))?;

        let sources = pkg
            .sources()
            .iter()
            .map(|s| abs_path.join(s).to_string_lossy().to_string())
            .collect::<Vec<String>>();
        let sources = [sources_ext, sources].concat();
        let includes = pkg
            .includes()
            .iter()
            .map(|s| abs_path.join(s).to_string_lossy().to_string())
            .collect::<Vec<String>>();
        let includes = [includes_ext, includes].concat();

        if let Ok(tailor_cache) = std::fs::read(
            abs_path
                .join("build")
                .join(self.mode.to_string())
                .join("TailorCache"),
        ) {
            if tailor_cache == pkg.hash() {
                return Ok(pkg);
            } else {
                println!(
                    "{} CMakeLists for package `{}` in {} mode",
                    info("Updating"),
                    pkg.name(),
                    self.mode.to_string()
                );
            }
        } else {
            println!(
                "{} CMakeLists.txt for package `{}` in {} mode",
                success("Creating"),
                pkg.name(),
                self.mode.to_string()
            );
        }

        let cmake_content = content
            .replace("$pkg_name", &pkg.name())
            .replace("$sources", &sources.join(" "))
            .replace("$include", &includes.join(" "));
        std::fs::create_dir_all(abs_path.join("build").join(self.mode.to_string()))
            .map_err(|e| format!("Failed to create build directory: {}", e))?;
        std::fs::write(
            abs_path
                .join("build")
                .join(self.mode.to_string())
                .join("CMakeLists.txt"),
            cmake_content,
        )
        .map_err(|e| format!("Failed to write CMakeLists.txt: {}", e))?;

        println!(
            "{} CMake for `{}` in {} mode",
            info("Generating"),
            pkg.name(),
            self.mode.to_string()
        );

        cmake::gen_cmake(&self.mode, &self.path)?;

        std::fs::write(
            abs_path
                .join("build")
                .join(self.mode.to_string())
                .join("TailorCache"),
            pkg.hash(),
        )
        .map_err(|e| format!("Failed to write TailorCache: {}", e))?;

        Ok(pkg)
    }
}

impl Command for BuildPkg {
    fn parse_args(&mut self, args: &[String]) -> Option<()>
    where
        Self: Sized,
    {
        if args.is_empty() || args[0] != "build" {
            return None;
        }

        match args.len() {
            1 => {
                self.mode = Mode::Debug;
                self.path = std::env::current_dir().ok()?;

                Some(())
            }
            2 => {
                match args[1].as_str().try_into() {
                    Ok(mode) => {
                        self.mode = mode;
                        self.path = std::env::current_dir().ok()?;
                    }
                    Err(_) => {
                        self.mode = Mode::Debug;
                        self.path = PathBuf::from(&args[1]);
                    }
                }

                Some(())
            }
            3 => {
                let mode = match args[1].as_str().try_into() {
                    Ok(mode) => mode,
                    Err(_) => return None,
                };

                self.mode = mode;
                self.path = PathBuf::from(&args[2]);

                Some(())
            }
            _ => None,
        }
    }

    fn execute(&self) -> Result<(), String> {
        let mode_name = self.mode.to_string();
        let pkg = Package::from_file(&self.path.join("Tailor.toml"))?;

        let (sources_ext, include_ext) = resolve_dependencies(&pkg, &self.path)?;

        let pkg = match pkg.pkg_type() {
            PackageType::Binary => {
                self.create_cmake_lists(bin::CMAKE_LISTS, sources_ext, include_ext)?
            }
            PackageType::Library => {
                self.create_cmake_lists(lib::CMAKE_LISTS, sources_ext, include_ext)?
            }
        };

        println!(
            "{} `{}` in {} mode",
            success("Building"),
            pkg.name(),
            mode_name
        );

        cmake::build(&self.mode, &self.path)?;

        Ok(())
    }
}

mod bin {
    pub const CMAKE_LISTS: &str = "cmake_minimum_required(VERSION 3.10)
project($pkg_name C)
set(CMAKE_C_STANDARD 99)
file(GLOB src_files $sources)
add_executable($pkg_name ${src_files})
target_include_directories($pkg_name PRIVATE $include)
if (CMAKE_BUILD_TYPE STREQUAL \"Debug\")
  target_compile_definitions($pkg_name PRIVATE DEBUG)
else()
  target_compile_definitions($pkg_name PRIVATE RELEASE)
endif()
";
}

mod lib {
    pub const CMAKE_LISTS: &str = "cmake_minimum_required(VERSION 3.10)
project($pkg_name C)
set(CMAKE_C_STANDARD 99)
file(GLOB src_files $sources)
add_library($pkg_name STATIC ${src_files})
target_include_directories($pkg_name PRIVATE $include)
if (CMAKE_BUILD_TYPE STREQUAL \"Debug\")
  target_compile_definitions($pkg_name PRIVATE DEBUG)
else()
  target_compile_definitions($pkg_name PRIVATE RELEASE)
endif()
";
}
