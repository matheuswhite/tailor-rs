#![deny(warnings)]

use crate::common::{
    check_dir::CheckDir, tailor_user::CheckOutput, tailor_user::TailorUser, test_dir::TestDir,
};

mod common;

#[test]
fn test_build_binary_debug() {
    let test_dir = TestDir::new("build_bin_debug");
    let test_path = test_dir.path();
    let user = TailorUser;

    user.new_binary(Some(test_path), false, None)
        .assert_success(&["Creating", "binary"]);

    user.build(None, Some("--debug"), Some(test_path))
        .assert_success(&["Finished", "debug"]);

    let build_dir = CheckDir::from(test_path).join("build").join("debug");
    build_dir.assert();
    build_dir.file("main.o").assert_exists();
    build_dir.file("build_bin_debug@0.1.0").assert_exists();
    build_dir.file("compile_commands.json").assert_exists();
}

#[test]
fn test_build_binary_no_flag() {
    let test_dir = TestDir::new("build_bin_no_flag");
    let test_path = test_dir.path();
    let user = TailorUser;

    user.new_binary(Some(test_path), false, None)
        .assert_success(&["Creating", "binary"]);

    user.build(None, None, Some(test_path))
        .assert_success(&["Finished", "debug"]);

    let build_dir = CheckDir::from(test_path).join("build").join("debug");
    build_dir.assert();
    build_dir.file("main.o").assert_exists();
    build_dir.file("build_bin_no_flag@0.1.0").assert_exists();
    build_dir.file("compile_commands.json").assert_exists();
}

#[test]
fn test_build_binary_release() {
    let test_dir = TestDir::new("build_bin_release");
    let test_path = test_dir.path();
    let user = TailorUser;

    user.new_binary(Some(test_path), false, None)
        .assert_success(&["Creating", "binary"]);

    user.build(None, Some("--release"), Some(test_path))
        .assert_success(&["Finished", "release"]);

    let build_dir = CheckDir::from(test_path).join("build").join("release");
    build_dir.assert();
    build_dir.file("main.o").assert_exists();
    build_dir.file("build_bin_release@0.1.0").assert_exists();
    build_dir.file("compile_commands.json").assert_exists();
}

#[test]
fn test_build_library_debug() {
    let test_dir = TestDir::new("build_lib_debug");
    let test_path = test_dir.path();
    let user = TailorUser;

    user.new_library(Some(test_path))
        .assert_success(&["Creating", "library"]);

    user.build(None, Some("--debug"), Some(test_path))
        .assert_success(&["Finished", "debug"]);

    let build_dir = CheckDir::from(test_path).join("build").join("debug");
    build_dir.assert();
    build_dir.file("build_lib_debug.o").assert_exists();
    build_dir
        .file("libbuild_lib_debug@0.1.0.so")
        .assert_exists();
    build_dir.file("compile_commands.json").assert_exists();
}

#[test]
fn test_build_library_no_flag() {
    let test_dir = TestDir::new("build_lib_no_flag");
    let test_path = test_dir.path();
    let user = TailorUser;

    user.new_library(Some(test_path))
        .assert_success(&["Creating", "library"]);

    user.build(None, None, Some(test_path))
        .assert_success(&["Finished", "debug"]);

    let build_dir = CheckDir::from(test_path).join("build").join("debug");
    build_dir.assert();
    build_dir.file("build_lib_no_flag.o").assert_exists();
    build_dir
        .file("libbuild_lib_no_flag@0.1.0.so")
        .assert_exists();
    build_dir.file("compile_commands.json").assert_exists();
}

#[test]
fn test_build_library_release() {
    let test_dir = TestDir::new("build_lib_release");
    let test_path = test_dir.path();
    let user = TailorUser;

    user.new_library(Some(test_path))
        .assert_success(&["Creating", "library"]);

    user.build(None, Some("--release"), Some(test_path))
        .assert_success(&["Finished", "release"]);

    let build_dir = CheckDir::from(test_path).join("build").join("release");
    build_dir.assert();
    build_dir.file("build_lib_release.o").assert_exists();
    build_dir
        .file("libbuild_lib_release@0.1.0.so")
        .assert_exists();
    build_dir.file("compile_commands.json").assert_exists();
}

#[test]
fn test_build_inside_folder() {
    let test_dir = TestDir::new("build_inside_folder");
    let test_path = test_dir.path();
    let user = TailorUser;

    user.new_binary(Some(test_path), false, None)
        .assert_success(&["Creating", "binary"]);

    user.build(Some(test_path), None, None)
        .assert_success(&["Finished", "debug"]);

    let build_dir = CheckDir::from(test_path).join("build").join("debug");
    build_dir.assert();
    build_dir.file("main.o").assert_exists();
    build_dir.file("build_inside_folder@0.1.0").assert_exists();
    build_dir.file("compile_commands.json").assert_exists();
}

#[test]
fn test_build_without_manifest() {
    let test_dir = TestDir::new("build_without_manifest");
    let test_path = test_dir.path();
    let user = TailorUser;

    std::fs::create_dir_all(test_path).expect("Failed to create test directory");

    user.build(None, None, Some(test_path))
        .assert_failure(&["fail to read Tailor.toml"]);
}

#[test]
fn test_build_with_errors() {
    let test_dir = TestDir::new("build_with_errors");
    let test_path = test_dir.path();
    let user = TailorUser;

    user.new_binary(Some(test_path), false, None)
        .assert_success(&["Creating", "binary"]);

    let main_c_path = test_path.join("src").join("main.c");
    std::fs::write(
        &main_c_path,
        "#include <stdio.h>\n\nint main() { return; }\n",
    )
    .expect("Failed to write invalid main.c");

    user.build(None, None, Some(test_path))
        .assert_failure(&["compilation failed"]);
}
