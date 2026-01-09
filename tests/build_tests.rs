use std::fs;
use std::path::PathBuf;
use std::process::Command;

/// Helper function to get the path to the tailor binary built by Cargo for tests.
fn get_tailor_binary() -> PathBuf {
    PathBuf::from(env!("CARGO_BIN_EXE_tailor"))
}

struct TestDir {
    path: PathBuf,
}

impl Drop for TestDir {
    fn drop(&mut self) {
        if self.path.exists() {
            fs::remove_dir_all(&self.path).ok();
        }
    }
}

/// Setup: Creates and returns test directory path.
/// Teardown is automatic via `Drop`, even if the test panics.
fn setup_test_dir(name: &str) -> TestDir {
    let test_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join(format!("tests/{}", name));
    if test_dir.exists() {
        fs::remove_dir_all(&test_dir).ok();
    }
    TestDir { path: test_dir }
}

// ============================================================================
// Build Package Tests - Binary
// ============================================================================

#[test]
fn test_build_binary_package() {
    let test_dir = setup_test_dir("build_hello");
    let test_path = &test_dir.path;

    // Setup: Create a new binary package
    let output = Command::new(get_tailor_binary())
        .arg("new")
        .arg(test_path)
        .output()
        .expect("Failed to execute tailor new");
    assert!(output.status.success());

    // Run: tailor build
    let output = Command::new(get_tailor_binary())
        .arg("build")
        .arg(test_path)
        .output()
        .expect("Failed to execute tailor build");

    assert!(
        output.status.success(),
        "Build command failed with stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );

    // Verify CMakeLists.txt was created and has correct content
    let cmake_path = test_path.join("build/debug/CMakeLists.txt");
    assert!(
        cmake_path.exists(),
        "CMakeLists.txt should be created in build/debug"
    );

    let cmake_content = fs::read_to_string(&cmake_path).expect("Failed to read CMakeLists.txt");

    // Get absolute paths for sources and includes
    let src_path = test_path.join("src/*.c").to_string_lossy().to_string();
    let include_path = test_path.join("include/").to_string_lossy().to_string();

    let expected_cmake = format!(
        "cmake_minimum_required(VERSION 3.10)
project(app C)
set(CMAKE_C_STANDARD 99)
file(GLOB src_files {})
add_executable(app ${{src_files}})
target_include_directories(app PRIVATE {})
if (CMAKE_BUILD_TYPE STREQUAL \"Debug\")
  target_compile_definitions(app PRIVATE DEBUG)
else()
  target_compile_definitions(app PRIVATE RELEASE)
endif()
",
        src_path, include_path
    );

    assert_eq!(
        cmake_content, expected_cmake,
        "CMakeLists.txt content does not match expected"
    );

    // Verify binary was created
    let binary_path = test_path.join("build/debug/app");
    assert!(
        binary_path.exists() || test_path.join("build/debug/app.exe").exists(),
        "Binary executable should be created"
    );

    // Run the binary and check output
    if binary_path.exists() {
        let binary_output = Command::new(&binary_path)
            .output()
            .expect("Failed to run binary");
        let stdout = String::from_utf8_lossy(&binary_output.stdout);
        assert!(
            stdout.contains("Hello, World!"),
            "Binary should output 'Hello, World!'"
        );
    }
}

#[test]
fn test_build_binary_package_with_debug_flag() {
    let test_dir = setup_test_dir("build_hello_debug");
    let test_path = &test_dir.path;

    // Setup: Create a new binary package
    let output = Command::new(get_tailor_binary())
        .arg("new")
        .arg(test_path)
        .output()
        .expect("Failed to execute tailor new");
    assert!(output.status.success());

    // Run: tailor build --debug
    let output = Command::new(get_tailor_binary())
        .arg("build")
        .arg("--debug")
        .arg(test_path)
        .output()
        .expect("Failed to execute tailor build");

    assert!(
        output.status.success(),
        "Build command failed with stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );

    // Verify CMakeLists.txt was created in debug folder
    let cmake_path = test_path.join("build/debug/CMakeLists.txt");
    assert!(
        cmake_path.exists(),
        "CMakeLists.txt should be created in build/debug"
    );

    let cmake_content = fs::read_to_string(&cmake_path).expect("Failed to read CMakeLists.txt");

    // Get absolute paths for sources and includes
    let src_path = test_path.join("src/*.c").to_string_lossy().to_string();
    let include_path = test_path.join("include/").to_string_lossy().to_string();

    let expected_cmake = format!(
        "cmake_minimum_required(VERSION 3.10)
project(app C)
set(CMAKE_C_STANDARD 99)
file(GLOB src_files {})
add_executable(app ${{src_files}})
target_include_directories(app PRIVATE {})
if (CMAKE_BUILD_TYPE STREQUAL \"Debug\")
  target_compile_definitions(app PRIVATE DEBUG)
else()
  target_compile_definitions(app PRIVATE RELEASE)
endif()
",
        src_path, include_path
    );

    assert_eq!(
        cmake_content, expected_cmake,
        "CMakeLists.txt content does not match expected"
    );

    // Verify binary was created
    let binary_path = test_path.join("build/debug/app");
    assert!(
        binary_path.exists() || test_path.join("build/debug/app.exe").exists(),
        "Binary executable should be created in debug mode"
    );

    // Run the binary and check output
    if binary_path.exists() {
        let binary_output = Command::new(&binary_path)
            .output()
            .expect("Failed to run binary");
        let stdout = String::from_utf8_lossy(&binary_output.stdout);
        assert!(
            stdout.contains("Hello, World!"),
            "Binary should output 'Hello, World!'"
        );
    }
}

#[test]
fn test_build_binary_package_with_release_flag() {
    let test_dir = setup_test_dir("build_hello_release");
    let test_path = &test_dir.path;

    // Setup: Create a new binary package
    let output = Command::new(get_tailor_binary())
        .arg("new")
        .arg(test_path)
        .output()
        .expect("Failed to execute tailor new");
    assert!(output.status.success());

    // Run: tailor build --release
    let output = Command::new(get_tailor_binary())
        .arg("build")
        .arg("--release")
        .arg(test_path)
        .output()
        .expect("Failed to execute tailor build");

    assert!(
        output.status.success(),
        "Build command failed with stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );

    // Verify CMakeLists.txt was created in release folder
    let cmake_path = test_path.join("build/release/CMakeLists.txt");
    assert!(
        cmake_path.exists(),
        "CMakeLists.txt should be created in build/release"
    );

    let cmake_content = fs::read_to_string(&cmake_path).expect("Failed to read CMakeLists.txt");

    // Get absolute paths for sources and includes
    let src_path = test_path.join("src/*.c").to_string_lossy().to_string();
    let include_path = test_path.join("include/").to_string_lossy().to_string();

    let expected_cmake = format!(
        "cmake_minimum_required(VERSION 3.10)
project(app C)
set(CMAKE_C_STANDARD 99)
file(GLOB src_files {})
add_executable(app ${{src_files}})
target_include_directories(app PRIVATE {})
if (CMAKE_BUILD_TYPE STREQUAL \"Debug\")
  target_compile_definitions(app PRIVATE DEBUG)
else()
  target_compile_definitions(app PRIVATE RELEASE)
endif()
",
        src_path, include_path
    );

    assert_eq!(
        cmake_content, expected_cmake,
        "CMakeLists.txt content does not match expected"
    );

    // Verify binary was created in release folder
    let binary_path = test_path.join("build/release/app");
    assert!(
        binary_path.exists() || test_path.join("build/release/app.exe").exists(),
        "Binary executable should be created in release mode"
    );

    // Run the binary and check output
    if binary_path.exists() {
        let binary_output = Command::new(&binary_path)
            .output()
            .expect("Failed to run binary");
        let stdout = String::from_utf8_lossy(&binary_output.stdout);
        assert!(
            stdout.contains("Hello, World!"),
            "Binary should output 'Hello, World!'"
        );
    }
}

// ============================================================================
// Build Package Tests - Library
// ============================================================================

#[test]
fn test_build_library_package() {
    let test_dir = setup_test_dir("build_mylib");
    let test_path = &test_dir.path;

    // Setup: Create a new library package
    let output = Command::new(get_tailor_binary())
        .arg("new")
        .arg("--lib")
        .arg(test_path)
        .output()
        .expect("Failed to execute tailor new");
    assert!(output.status.success());

    // Run: tailor build
    let output = Command::new(get_tailor_binary())
        .arg("build")
        .arg(test_path)
        .output()
        .expect("Failed to execute tailor build");

    assert!(
        output.status.success(),
        "Build command failed with stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );

    // Verify CMakeLists.txt was created and has correct content for library
    let cmake_path = test_path.join("build/debug/CMakeLists.txt");
    assert!(
        cmake_path.exists(),
        "CMakeLists.txt should be created in build/debug"
    );

    let cmake_content = fs::read_to_string(&cmake_path).expect("Failed to read CMakeLists.txt");

    // Get absolute paths for sources and includes
    let src_path = test_path.join("src/*.c").to_string_lossy().to_string();
    let include_path = test_path.join("include/").to_string_lossy().to_string();

    let expected_cmake = format!(
        "cmake_minimum_required(VERSION 3.10)
project(app C)
set(CMAKE_C_STANDARD 99)
file(GLOB src_files {})
add_library(app STATIC ${{src_files}})
target_include_directories(app PRIVATE {})
if (CMAKE_BUILD_TYPE STREQUAL \"Debug\")
  target_compile_definitions(app PRIVATE DEBUG)
else()
  target_compile_definitions(app PRIVATE RELEASE)
endif()
",
        src_path, include_path
    );

    assert_eq!(
        cmake_content, expected_cmake,
        "CMakeLists.txt content does not match expected"
    );

    // Verify static library was created
    let lib_path = test_path.join("build/debug/libapp.a");
    let lib_path_alt = test_path.join("build/debug/app.lib");
    assert!(
        lib_path.exists() || lib_path_alt.exists(),
        "Static library should be created (.a or .lib)"
    );
}

#[test]
fn test_build_library_package_with_debug_flag() {
    let test_dir = setup_test_dir("build_mylib_debug");
    let test_path = &test_dir.path;

    // Setup: Create a new library package
    let output = Command::new(get_tailor_binary())
        .arg("new")
        .arg("--lib")
        .arg(test_path)
        .output()
        .expect("Failed to execute tailor new");
    assert!(output.status.success());

    // Run: tailor build --debug
    let output = Command::new(get_tailor_binary())
        .arg("build")
        .arg("--debug")
        .arg(test_path)
        .output()
        .expect("Failed to execute tailor build");

    assert!(
        output.status.success(),
        "Build command failed with stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );

    // Verify CMakeLists.txt was created in debug folder
    let cmake_path = test_path.join("build/debug/CMakeLists.txt");
    assert!(
        cmake_path.exists(),
        "CMakeLists.txt should be created in build/debug"
    );

    let cmake_content = fs::read_to_string(&cmake_path).expect("Failed to read CMakeLists.txt");

    // Get absolute paths for sources and includes
    let src_path = test_path.join("src/*.c").to_string_lossy().to_string();
    let include_path = test_path.join("include/").to_string_lossy().to_string();

    let expected_cmake = format!(
        "cmake_minimum_required(VERSION 3.10)
project(app C)
set(CMAKE_C_STANDARD 99)
file(GLOB src_files {})
add_library(app STATIC ${{src_files}})
target_include_directories(app PRIVATE {})
if (CMAKE_BUILD_TYPE STREQUAL \"Debug\")
  target_compile_definitions(app PRIVATE DEBUG)
else()
  target_compile_definitions(app PRIVATE RELEASE)
endif()
",
        src_path, include_path
    );

    assert_eq!(
        cmake_content, expected_cmake,
        "CMakeLists.txt content does not match expected"
    );

    // Verify static library was created
    let lib_path = test_path.join("build/debug/libapp.a");
    let lib_path_alt = test_path.join("build/debug/app.lib");
    assert!(
        lib_path.exists() || lib_path_alt.exists(),
        "Static library should be created in debug mode"
    );
}

#[test]
fn test_build_library_package_with_release_flag() {
    let test_dir = setup_test_dir("build_mylib_release");
    let test_path = &test_dir.path;

    // Setup: Create a new library package
    let output = Command::new(get_tailor_binary())
        .arg("new")
        .arg("--lib")
        .arg(test_path)
        .output()
        .expect("Failed to execute tailor new");
    assert!(output.status.success());

    // Run: tailor build --release
    let output = Command::new(get_tailor_binary())
        .arg("build")
        .arg("--release")
        .arg(test_path)
        .output()
        .expect("Failed to execute tailor build");

    assert!(
        output.status.success(),
        "Build command failed with stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );

    // Verify CMakeLists.txt was created in release folder
    let cmake_path = test_path.join("build/release/CMakeLists.txt");
    assert!(
        cmake_path.exists(),
        "CMakeLists.txt should be created in build/release"
    );

    let cmake_content = fs::read_to_string(&cmake_path).expect("Failed to read CMakeLists.txt");

    // Get absolute paths for sources and includes
    let src_path = test_path.join("src/*.c").to_string_lossy().to_string();
    let include_path = test_path.join("include/").to_string_lossy().to_string();

    let expected_cmake = format!(
        "cmake_minimum_required(VERSION 3.10)
project(app C)
set(CMAKE_C_STANDARD 99)
file(GLOB src_files {})
add_library(app STATIC ${{src_files}})
target_include_directories(app PRIVATE {})
if (CMAKE_BUILD_TYPE STREQUAL \"Debug\")
  target_compile_definitions(app PRIVATE DEBUG)
else()
  target_compile_definitions(app PRIVATE RELEASE)
endif()
",
        src_path, include_path
    );

    assert_eq!(
        cmake_content, expected_cmake,
        "CMakeLists.txt content does not match expected"
    );

    // Verify static library was created in release folder
    let lib_path = test_path.join("build/release/libapp.a");
    let lib_path_alt = test_path.join("build/release/app.lib");
    assert!(
        lib_path.exists() || lib_path_alt.exists(),
        "Static library should be created in release mode"
    );
}
