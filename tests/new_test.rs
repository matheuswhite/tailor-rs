use crate::common::{
    check_dir::CheckDir,
    tailor_user::{CheckOutput, TailorUser},
    test_dir::TestDir,
};

mod common;

#[test]
fn test_new_binary_package() {
    let test_dir = TestDir::new("hello");
    let test_path = test_dir.path();
    let user = TailorUser;

    user.new_binary(Some(test_path), false)
        .assert_success(&["Creating", "binary"]);

    let test_path = CheckDir::from(test_path);
    test_path.assert();

    test_path.join("src").assert();
    test_path.join("include").assert();

    let expected_main_c =
        "#include <stdio.h>\n\nint main() {\n  printf(\"Hello, World!\\n\");\n\n  return 0;\n}\n";
    test_path.join("src").file("main.c").assert(expected_main_c);

    let expected_manifest =
        "name = \"hello\"\nversion = \"0.1.0\"\nedition = \"2026.1\"\n\n[dependencies]\n";
    test_path.file("Tailor.toml").assert(expected_manifest);
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

    let expected_src_c = "#include \"mylib/mylib.h\"\n#include <stdio.h>\n\nvoid mylib() { printf(\"Hello from the mylib library!\\n\"); }\n";
    test_path.join("src").file("mylib.c").assert(expected_src_c);

    let expected_include_h =
        "#ifndef MYLIB_H\n#define MYLIB_H\n\nvoid mylib();\n\n#endif /* MYLIB_H */\n";
    test_path
        .join("include")
        .join("mylib")
        .file("mylib.h")
        .assert(expected_include_h);

    let expected_manifest = "name = \"mylib\"\nversion = \"0.1.0\"\nedition = \"2026.1\"\ntype = \"lib\"\n\n[dependencies]\n";
    test_path.file("Tailor.toml").assert(expected_manifest);
}

#[test]
fn test_new_binary_package_with_bin_flag() {
    let test_dir = TestDir::new("hello");
    let test_path = test_dir.path();
    let user = TailorUser;

    user.new_binary(Some(test_path), true)
        .assert_success(&["Creating", "binary"]);

    let test_path = CheckDir::from(test_path);
    test_path.assert();

    test_path.join("src").assert();
    test_path.join("include").assert();

    let expected_main_c =
        "#include <stdio.h>\n\nint main() {\n  printf(\"Hello, World!\\n\");\n\n  return 0;\n}\n";
    test_path.join("src").file("main.c").assert(expected_main_c);

    let expected_manifest =
        "name = \"hello\"\nversion = \"0.1.0\"\nedition = \"2026.1\"\n\n[dependencies]\n";
    test_path.file("Tailor.toml").assert(expected_manifest);
}

#[test]
fn test_new_binary_package_with_existing_folder() {
    let test_dir = TestDir::new("hello");
    let test_path = test_dir.path();
    let user = TailorUser;

    std::fs::create_dir_all(test_path).expect("Failed to create test directory");

    user.new_binary(Some(test_path), false)
        .assert_failure(&["already exists"]);
}

#[test]
fn test_new_binary_without_path() {
    let user = TailorUser;

    user.new_binary(None, false)
        .assert_failure(&["Too few arguments"]);
}
