use crate::{absolute_path::AbsolutePath, manifest::package_type::PackageType, mode::Mode};

pub struct CMake;

impl CMake {
    pub fn build(mode: Mode, path: &AbsolutePath, defines: Vec<String>) -> Result<(), String> {
        match mode {
            Mode::Debug => {
                let path = path.inner().join("build").join("debug");

                let status = std::process::Command::new("cmake")
                    .arg("-DCMAKE_BUILD_TYPE=Debug")
                    .args(defines)
                    .arg("-B")
                    .arg(&path)
                    .arg("-S")
                    .arg(&path)
                    .status()
                    .map_err(|e| format!("Failed to gen cmake in debug mode: {}", e))?;
                if !status.success() {
                    return Err(format!(
                        "CMake configuration failed in debug mode with status: {}",
                        status
                    ));
                }

                let status = std::process::Command::new("cmake")
                    .arg("--build")
                    .arg(&path)
                    .status()
                    .map_err(|e| format!("Failed to build in debug mode: {}", e))?;
                if !status.success() {
                    return Err(format!(
                        "CMake build failed in debug mode with status: {}",
                        status
                    ));
                }

                Ok(())
            }
            Mode::Release => {
                let path = path.inner().join("build").join("release");

                let status = std::process::Command::new("cmake")
                    .arg("-DCMAKE_BUILD_TYPE=Release")
                    .args(defines)
                    .arg("-B")
                    .arg(&path)
                    .arg("-S")
                    .arg(&path)
                    .status()
                    .map_err(|e| format!("Failed to gen cmake in release mode: {}", e))?;
                if !status.success() {
                    return Err(format!(
                        "CMake configuration failed in release mode with status: {}",
                        status
                    ));
                }

                let status = std::process::Command::new("cmake")
                    .arg("--build")
                    .arg(&path)
                    .status()
                    .map_err(|e| format!("Failed to build in release mode: {}", e))?;
                if !status.success() {
                    return Err(format!(
                        "CMake build failed in release mode with status: {}",
                        status
                    ));
                }

                Ok(())
            }
        }
    }

    pub fn run(mode: Mode, path: &AbsolutePath) -> Result<(), String> {
        match mode {
            Mode::Debug => {
                let status = std::process::Command::new(
                    path.inner().join("build").join("debug").join("app"),
                )
                .status()
                .map_err(|e| format!("Failed to run in debug mode: {}", e))?;
                if !status.success() {
                    return Err(format!(
                        "Failed to run in debug mode with status: {}",
                        status
                    ));
                }

                Ok(())
            }
            Mode::Release => {
                let status = std::process::Command::new(
                    path.inner().join("build").join("release").join("app"),
                )
                .status()
                .map_err(|e| format!("Failed to run in release mode: {}", e))?;
                if !status.success() {
                    return Err(format!(
                        "Failed to run in release mode with status: {}",
                        status
                    ));
                }

                Ok(())
            }
        }
    }

    pub fn create_cmake_lists(
        pkg_type: PackageType,
        sources: Vec<String>,
        includes: Vec<String>,
        path: AbsolutePath,
    ) -> Result<(), String> {
        match pkg_type {
            PackageType::Binary => Self::create_binary_cmake_lists(sources, includes, path),
            PackageType::Library => Self::create_library_cmake_lists(sources, includes, path),
        }
    }

    pub fn write_tailor_lock(path: AbsolutePath, content: String) -> Result<(), String> {
        let dest = path.inner().join("Tailor.lock");

        std::fs::write(&dest, content).map_err(|e| format!("Failed to write {:?}: {}", dest, e))
    }

    pub fn needs_recreate(path: AbsolutePath, manifest: String) -> bool {
        let cmake_lists_exists = std::fs::metadata(path.inner().join("CMakeLists.txt")).is_ok();
        let tailor_lock_exists = std::fs::metadata(path.inner().join("Tailor.lock")).is_ok();

        if !cmake_lists_exists {
            return true;
        }

        if !tailor_lock_exists {
            return true;
        }

        let tailor_lock_content = std::fs::read_to_string(path.inner().join("Tailor.lock"));
        if tailor_lock_content.is_err() {
            return true;
        }

        if tailor_lock_content.unwrap() != manifest {
            return true;
        }

        false
    }

    fn create_binary_cmake_lists(
        sources: Vec<String>,
        includes: Vec<String>,
        path: AbsolutePath,
    ) -> Result<(), String> {
        let cmake_content = bin::CMAKE_LISTS
            .replace("$pkg_name", "app")
            .replace("$sources", &sources.join(" "))
            .replace("$include", &includes.join(" "));

        std::fs::write(path.inner().join("CMakeLists.txt"), cmake_content)
            .map_err(|e| format!("Failed to write CMakeLists.txt: {}", e))?;

        Ok(())
    }

    fn create_library_cmake_lists(
        sources: Vec<String>,
        includes: Vec<String>,
        path: AbsolutePath,
    ) -> Result<(), String> {
        let cmake_content = lib::CMAKE_LISTS
            .replace("$pkg_name", "app")
            .replace("$sources", &sources.join(" "))
            .replace("$include", &includes.join(" "));

        std::fs::write(path.inner().join("CMakeLists.txt"), cmake_content)
            .map_err(|e| format!("Failed to write CMakeLists.txt: {}", e))?;

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
