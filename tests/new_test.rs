#![deny(warnings)]

use crate::common::{
    check_dir::CheckDir,
    tailor_user::{CheckOutput, TailorUser},
    test_dir::TestDir,
};

mod common;

#[cfg(windows)]
const NEW_LINE: &str = "\r\n";

#[cfg(not(windows))]
const NEW_LINE: &str = "\n";

#[test]
fn test_new_binary_package() {
    let test_dir = TestDir::new("hello");
    let test_path = test_dir.path();
    let user = TailorUser;

    user.new_binary(Some(test_path), false, None)
        .assert_success(&["Creating", "binary"]);

    let test_path = CheckDir::from(test_path);
    test_path.assert();

    test_path.join("src").assert();
    test_path.join("include").assert();

    let expected_main_c =
        "#include <stdio.h>\n\nint main() {\n  printf(\"Hello, World!\\n\");\n\n  return 0;\n}\n"
            .replace("\n", NEW_LINE);
    test_path
        .join("src")
        .file("main.c")
        .assert(&expected_main_c);

    let expected_manifest =
        "name = \"hello\"\nversion = \"0.1.0\"\nedition = \"2026.1\"\n\n[dependencies]\n"
            .replace("\n", NEW_LINE);
    test_path.file("Tailor.toml").assert(&expected_manifest);
}

#[test]
fn test_new_library_package() {
    let test_dir = TestDir::new("mylib");
    let test_path = test_dir.path();
    let user = TailorUser;

    user.new_library(Some(test_path))
        .assert_success(&["Creating", "library"]);

    let test_path = CheckDir::from(test_path);
    test_path.assert();

    test_path.join("src").assert();
    test_path.join("include").assert();
    test_path.join("include").join("mylib").assert();

    let expected_src_c = "#include \"mylib/mylib.h\"\n#include <stdio.h>\n\nvoid mylib() { printf(\"Hello from the mylib library!\\n\"); }\n"
        .replace("\n", NEW_LINE);
    test_path
        .join("src")
        .file("mylib.c")
        .assert(&expected_src_c);

    let expected_include_h =
        "#ifndef MYLIB_H\n#define MYLIB_H\n\nvoid mylib();\n\n#endif /* MYLIB_H */\n"
            .replace("\n", NEW_LINE);
    test_path
        .join("include")
        .join("mylib")
        .file("mylib.h")
        .assert(&expected_include_h);

    let expected_manifest = "name = \"mylib\"\nversion = \"0.1.0\"\nedition = \"2026.1\"\ntype = \"lib\"\n\n[dependencies]\n"
        .replace("\n", NEW_LINE);
    test_path.file("Tailor.toml").assert(&expected_manifest);
}

#[test]
fn test_new_binary_package_with_bin_flag() {
    let test_dir = TestDir::new("hello_bin_flag");
    let test_path = test_dir.path();
    let user = TailorUser;

    user.new_binary(Some(test_path), true, None)
        .assert_success(&["Creating", "binary"]);

    let test_path = CheckDir::from(test_path);
    test_path.assert();

    test_path.join("src").assert();
    test_path.join("include").assert();

    let expected_main_c =
        "#include <stdio.h>\n\nint main() {\n  printf(\"Hello, World!\\n\");\n\n  return 0;\n}\n"
            .replace("\n", NEW_LINE);
    test_path
        .join("src")
        .file("main.c")
        .assert(&expected_main_c);

    let expected_manifest =
        "name = \"hello_bin_flag\"\nversion = \"0.1.0\"\nedition = \"2026.1\"\n\n[dependencies]\n"
            .replace("\n", NEW_LINE);
    test_path.file("Tailor.toml").assert(&expected_manifest);
}

#[test]
fn test_new_binary_package_with_existing_folder() {
    let test_dir = TestDir::new("hello_again");
    let test_path = test_dir.path();
    let user = TailorUser;

    std::fs::create_dir_all(test_path).expect("Failed to create test directory");

    user.new_binary(Some(test_path), false, None)
        .assert_failure(&["already exists"]);
}

#[test]
fn test_new_binary_without_path() {
    let user = TailorUser;

    user.new_binary(None, false, None)
        .assert_failure(&["error: the following required arguments were not provided"]);
}

#[test]
fn test_new_binary_with_many_arguments() {
    let test_dir = TestDir::new("hello_many");
    let test_path = test_dir.path();
    let user = TailorUser;

    user.new_binary(Some(test_path), true, Some("world"))
        .assert_failure(&["error: unexpected argument 'world' found"]);
}

#[test]
fn test_no_args() {
    let user = TailorUser;

    user.no_args()
        .assert_failure(&["A C package manager inspired by Rust's Cargo"]);
}
