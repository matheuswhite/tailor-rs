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

#[test]
fn test_new_binary_package() {
    let test_dir = setup_test_dir("hello");
    let test_path = &test_dir.path;

    // Run: tailor new hello
    let output = Command::new(get_tailor_binary())
        .arg("new")
        .arg(test_path)
        .output()
        .expect("Failed to execute tailor");

    // Verify command succeeded
    assert!(
        output.status.success(),
        "Command failed with stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );

    // Verify output message
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        stdout.contains("Creating") && stdout.contains("binary"),
        "Output should contain 'Creating' and 'binary' messages"
    );

    // Verify directory structure
    assert!(test_path.exists(), "Package directory was not created");
    assert!(
        test_path.join("src").exists(),
        "src directory was not created"
    );
    assert!(
        test_path.join("include").exists(),
        "include directory was not created"
    );
    assert!(
        test_path.join("src/main.c").exists(),
        "src/main.c was not created"
    );
    assert!(
        test_path.join("Tailor.toml").exists(),
        "Tailor.toml was not created"
    );

    // Verify main.c content matches template exactly
    let main_c_content =
        fs::read_to_string(test_path.join("src/main.c")).expect("Failed to read src/main.c");

    let expected_main_c =
        "#include <stdio.h>\n\nint main() {\n  printf(\"Hello, World!\\n\");\n\n  return 0;\n}\n";
    assert_eq!(
        main_c_content, expected_main_c,
        "main.c content should match template exactly"
    );

    // Verify Tailor.toml content matches template exactly
    let manifest_content =
        fs::read_to_string(test_path.join("Tailor.toml")).expect("Failed to read Tailor.toml");

    let expected_manifest =
        "name = \"hello\"\nversion = \"0.1.0\"\nedition = \"2026.1\"\n\n[dependencies]\n";
    assert_eq!(
        manifest_content, expected_manifest,
        "Tailor.toml content should match template exactly"
    );
}

#[test]
fn test_new_binary_package_with_bin_flag() {
    let test_dir = setup_test_dir("hello_bin");
    let test_path = &test_dir.path;

    // Run: tailor new --bin hello_bin
    let output = Command::new(get_tailor_binary())
        .arg("new")
        .arg("--bin")
        .arg(test_path)
        .output()
        .expect("Failed to execute tailor");

    // Verify command succeeded
    assert!(
        output.status.success(),
        "Command failed with stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );

    // Verify output message
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        stdout.contains("Creating") && stdout.contains("binary"),
        "Output should contain 'Creating' and 'binary' messages"
    );

    // Verify directory structure (same as default binary package)
    assert!(test_path.exists(), "Package directory was not created");
    assert!(
        test_path.join("src").exists(),
        "src directory was not created"
    );
    assert!(
        test_path.join("include").exists(),
        "include directory was not created"
    );
    assert!(
        test_path.join("src/main.c").exists(),
        "src/main.c was not created"
    );
    assert!(
        test_path.join("Tailor.toml").exists(),
        "Tailor.toml was not created"
    );

    // Verify main.c content matches template exactly
    let main_c_content =
        fs::read_to_string(test_path.join("src/main.c")).expect("Failed to read src/main.c");

    let expected_main_c =
        "#include <stdio.h>\n\nint main() {\n  printf(\"Hello, World!\\n\");\n\n  return 0;\n}\n";
    assert_eq!(
        main_c_content, expected_main_c,
        "main.c content should match template exactly"
    );

    // Verify Tailor.toml content matches template exactly
    let manifest_content =
        fs::read_to_string(test_path.join("Tailor.toml")).expect("Failed to read Tailor.toml");

    let expected_manifest =
        "name = \"hello_bin\"\nversion = \"0.1.0\"\nedition = \"2026.1\"\n\n[dependencies]\n";
    assert_eq!(
        manifest_content, expected_manifest,
        "Tailor.toml content should match template exactly"
    );
}

#[test]
fn test_new_binary_package_with_existing_folder() {
    let test_dir = setup_test_dir("hello_existing_bin");
    let test_path = &test_dir.path;

    // Setup: Create the directory first
    fs::create_dir_all(test_path).expect("Failed to create test directory");

    // Run: tailor new hello_existing_bin (should fail)
    let output = Command::new(get_tailor_binary())
        .arg("new")
        .arg(test_path)
        .output()
        .expect("Failed to execute tailor");

    // Verify command failed with non-zero exit code
    assert!(
        !output.status.success(),
        "Command should fail (non-zero exit code) when directory already exists. Exit code: {:?}",
        output.status.code()
    );

    // Verify error message is displayed in stderr
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.contains("already exists"),
        "Error message should mention that destination already exists in stderr. Got stderr: {}",
        stderr
    );
}

#[test]
fn test_new_library_package() {
    let test_dir = setup_test_dir("mylib");
    let test_path = &test_dir.path;

    // Run: tailor new --lib mylib
    let output = Command::new(get_tailor_binary())
        .arg("new")
        .arg("--lib")
        .arg(test_path)
        .output()
        .expect("Failed to execute tailor");

    // Verify command succeeded
    assert!(
        output.status.success(),
        "Command failed with stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );

    // Verify output message
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        stdout.contains("Creating") && stdout.contains("library"),
        "Output should contain 'Creating' and 'library' messages"
    );

    // Verify directory structure
    assert!(test_path.exists(), "Package directory was not created");
    assert!(
        test_path.join("src").exists(),
        "src directory was not created"
    );
    assert!(
        test_path.join("include/mylib").exists(),
        "include/mylib directory was not created"
    );
    assert!(
        test_path.join("src/mylib.c").exists(),
        "src/mylib.c was not created"
    );
    assert!(
        test_path.join("include/mylib/mylib.h").exists(),
        "include/mylib/mylib.h was not created"
    );
    assert!(
        test_path.join("Tailor.toml").exists(),
        "Tailor.toml was not created"
    );

    // Verify library .c file content matches template exactly
    let lib_c_content =
        fs::read_to_string(test_path.join("src/mylib.c")).expect("Failed to read src/mylib.c");

    let expected_lib_c = "#include \"mylib/mylib.h\"\n#include <stdio.h>\n\nvoid mylib() { printf(\"Hello from the mylib library!\\n\"); }\n";
    assert_eq!(
        lib_c_content, expected_lib_c,
        "mylib.c content should match template exactly"
    );

    // Verify header file content matches template exactly
    let lib_h_content = fs::read_to_string(test_path.join("include/mylib/mylib.h"))
        .expect("Failed to read include/mylib/mylib.h");

    let expected_lib_h =
        "#ifndef MYLIB_H\n#define MYLIB_H\n\nvoid mylib();\n\n#endif /* MYLIB_H */\n";
    assert_eq!(
        lib_h_content, expected_lib_h,
        "mylib.h content should match template exactly"
    );

    // Verify Tailor.toml content matches template exactly
    let manifest_content =
        fs::read_to_string(test_path.join("Tailor.toml")).expect("Failed to read Tailor.toml");

    let expected_manifest = "name = \"mylib\"\nversion = \"0.1.0\"\nedition = \"2026.1\"\ntype = \"lib\"\n\n[dependencies]\n";
    assert_eq!(
        manifest_content, expected_manifest,
        "Tailor.toml content should match template exactly"
    );
}

#[test]
fn test_new_library_package_with_existing_folder() {
    let test_dir = setup_test_dir("mylib_existing");
    let test_path = &test_dir.path;

    // Setup: Create the directory first
    fs::create_dir_all(test_path).expect("Failed to create test directory");

    // Run: tailor new --lib mylib_existing (should fail)
    let output = Command::new(get_tailor_binary())
        .arg("new")
        .arg("--lib")
        .arg(test_path)
        .output()
        .expect("Failed to execute tailor");

    // Verify command failed with non-zero exit code
    assert!(
        !output.status.success(),
        "Command should fail (non-zero exit code) when directory already exists. Exit code: {:?}",
        output.status.code()
    );

    // Verify error message is displayed in stderr
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.contains("already exists"),
        "Error message should mention that destination already exists in stderr. Got stderr: {}",
        stderr
    );
}
