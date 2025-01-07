use std::{
    // env::current_dir,
    // ffi::{OsStr, OsString},
    // path::{Path, PathBuf},
    process::Command,
    // thread::sleep,
    // time::Duration
};
mod list_tests;
use list_tests::{
    test_list_by_id, test_list_find, test_list_all
};
mod las_modifications_tests;
use las_modifications_tests::{
    test_add_new,
    test_updates
};
const TEST_CONFIG_BASE_PATH: &'static str = "./tests/TestConfigBase.sigrs";

/// Helper to install latest version of binary
fn install_latest_binary() {
    let status = Command::new("cargo")
        .arg("install")
        .arg("--path")
        .arg(".")
        .status()
        .expect("Failed to install binary");

    assert!(status.success(), "Failed to install latest version of the binary");
}

#[test]
fn xyz() {
    let out = Command::new("sigrs")
        .arg("list-all")
        .arg("--help")
        .output()
        .unwrap();
    let stdout = String::from_utf8_lossy(&out.stdout);
    let stderr = String::from_utf8_lossy(&out.stderr);

    assert!(false, "stdout: {:?}  |  stderr: {:?}", stdout, stderr);
}

/// Creates a copy of `/tests/TestConfigBase.sigrs` at `into_path`
/// Returns `into_path` if successful
/// ex: `into_path: "./tests/ListTestCopy.sigrs"`
fn clone_test_config<'p>(into_path: &'p str) -> &'p str {
    let status = Command::new("cp")
        .arg(TEST_CONFIG_BASE_PATH)
        .arg(into_path)
        .status()
        .expect(&format!("OS | Failed to copy {} into {}", TEST_CONFIG_BASE_PATH, into_path));

    assert!(status.success(), "Failed");

    into_path
}

/// Deletes file at `path`
fn delete_test_config_clone(path: &str) {
    let status = Command::new("rm")
        .arg(path)
        .status()
        .expect(&format!("OS | Failed to delete {}", path));
    assert!(status.success(), "Failed to remove test config");
}

// =======================================================
// =================================== Test groups
// =======================================================

// build && install latest binary, runs before other tests
#[test]
fn test_binstall() {
    install_latest_binary();
}

#[test]
fn test_group_list() {
    let config = clone_test_config("./tests/ListTests.sigrs");
    test_list_all(config);
    test_list_by_id(config);
    test_list_find(config);
    delete_test_config_clone(config);
}

// test local storage modifications
#[test]
fn test_las_modifications() {
    let config = clone_test_config("./tests/LASMods.sigrs");
    test_add_new(config);
    test_updates(config);
    // test updates
    // test deletes
    delete_test_config_clone(config);
}

// test signing


