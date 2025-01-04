#[cfg(test)]
use std::{
    path::PathBuf,
    str::FromStr
};

#[cfg(test)]
use crate::signing::signing_tests::test_utils::{
    ConfigFileType, ConfigTemplate, gen_test_config, delete_test_file
};

#[cfg(test)]
use crate::signing::rust::{
    cargo_has_authors,
    cargo_is_signable
};

#[cfg(test)]
use crate::signing::signpath::SignPath;

#[test]
fn test_noauthors_hasauthors() {

    // No authors assert false
    let test_buf = PathBuf::from_str("./TestCargoTomlNoAuthorsHasAuthors.toml").unwrap();
    let mut test_sign_path = SignPath::CargoToml {
        // path shouldn't matter
        path: test_buf.clone(),
        file: None,
        file_content: None
    };
    let cfg_type = ConfigFileType::CargoToml(ConfigTemplate::NoAuthors);

    let file_name = test_buf.file_name().unwrap();
    let file_path = PathBuf::from(file_name);
    let mut test_file = gen_test_config(cfg_type, &file_path).unwrap();

    let res = cargo_has_authors(&mut test_sign_path, test_file);

    assert!(!res, "cargo_has_authors returned true on Cargo.toml with no authors field");
    assert!(test_sign_path.get_file().is_none());

    delete_test_file(&test_buf).unwrap();
}

#[test]
fn test_noauthors_issignable() {

    let test_buf = PathBuf::from_str("./TestCargoTomlNoAuthorsSignable.toml").unwrap();
    let mut test_sign_path = SignPath::CargoToml {
        // path shouldn't matter
        path: test_buf.clone(),
        file: None,
        file_content: None
    };
    let cfg_type = ConfigFileType::CargoToml(ConfigTemplate::NoAuthors);

    let file_name = test_buf.file_name().unwrap();
    let file_path = PathBuf::from(file_name);
    let mut test_file = gen_test_config(cfg_type, &file_path).unwrap();

    let res = cargo_is_signable(&mut test_sign_path, test_file);

    assert!(!res, "cargo_is_signable returned true on Cargo.toml with no authors field");
    assert!(test_sign_path.get_file().is_none());

    delete_test_file(&test_buf).unwrap();
}



#[test]
fn test_emptyauthors_hasauthors() {
    let test_buf = PathBuf::from_str("./TestCargoTomlEmptyAuthorsHasAuthors.toml").unwrap();
    let mut test_sign_path = SignPath::CargoToml {
        // path shouldn't matter
        path: test_buf.clone(),
        file: None,
        file_content: None
    };
    let cfg_type = ConfigFileType::CargoToml(ConfigTemplate::EmptyAuthors);

    let file_name = test_buf.file_name().unwrap();
    let file_path = PathBuf::from(file_name);
    let mut test_file = gen_test_config(cfg_type, &file_path).unwrap();

    let res = cargo_has_authors(&mut test_sign_path, test_file);

    assert!(!res, "cargo_has_authors returned true on Cargo.toml with empty authors field");
    assert!(test_sign_path.get_file().is_none());

    delete_test_file(&test_buf).unwrap();
}

#[test]
fn test_emptyauthors_issignable() {
    let test_buf = PathBuf::from_str("./TestCargoTomlEmptyAuthorsSignable.toml").unwrap();
    let mut test_sign_path = SignPath::CargoToml {
        path: test_buf.clone(),
        file: None,
        file_content: None
    };
    let cfg_type = ConfigFileType::CargoToml(ConfigTemplate::EmptyAuthors);


    let file_name = test_buf.file_name().unwrap();
    let file_path = PathBuf::from(file_name);
    let mut test_file = gen_test_config(cfg_type, &file_path).unwrap();

    let res = cargo_is_signable(&mut test_sign_path, test_file);

    assert!(res, "cargo_is_signable returned false on Cargo.toml with empty authors field");
    assert!(test_sign_path.get_file().is_some());

    delete_test_file(&test_buf).unwrap();
}

#[test]
fn test_oneauthor_hasauthors() {
    let test_buf = PathBuf::from_str("./TestCargoTomlOneAuthorHasAuthors.toml").unwrap();
    let mut test_sign_path = SignPath::CargoToml {
        path: test_buf.clone(),
        file: None,
        file_content: None
    };
    let cfg_type = ConfigFileType::CargoToml(ConfigTemplate::WithAuthors);
    let file_name = test_buf.file_name().unwrap();
    let file_path = PathBuf::from(file_name);
    let mut test_file = gen_test_config(cfg_type, &file_path).unwrap();

    let res = cargo_has_authors(&mut test_sign_path, test_file);

    assert!(res, "cargo_has_authors returned false on Cargo.toml with non-empty authors field");
    assert!(test_sign_path.get_file().is_some());

    delete_test_file(&test_buf).unwrap();
}

#[test]
fn test_oneauthor_issignable() {
    let test_buf = PathBuf::from_str("./TestCargoTomlOneAuthorsSignable.toml").unwrap();
    let mut test_sign_path = SignPath::CargoToml {
        path: test_buf.clone(),
        file: None,
        file_content: None
    };
    let cfg_type = ConfigFileType::CargoToml(ConfigTemplate::WithAuthors);

    let file_name = test_buf.file_name().unwrap();
    let file_path = PathBuf::from(file_name);
    let mut test_file = gen_test_config(cfg_type, &file_path).unwrap();

    let res = cargo_is_signable(&mut test_sign_path, test_file);

    assert!(res, "cargo_is_signable returned false on Cargo.toml with non-empty authors field");
    assert!(test_sign_path.get_file().is_some());

    delete_test_file(&test_buf).unwrap();

}



