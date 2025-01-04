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
use crate::signing::php::{
    composer_json_has_authors,
    composer_json_is_signable,
};

#[cfg(test)]
use crate::signing::signpath::SignPath;


#[test]
fn test_noauthors_hasauthors_composer_json() {

    // No authors assert false
    let test_buf = PathBuf::from_str("./TestComposerJsonNoAuthorsHasAuthors.json").unwrap();
    let mut test_sign_path = SignPath::ComposerJson {
        path: test_buf.clone(),
        file: None,
        file_content: None
    };
    let cfg_type = ConfigFileType::ComposerJson(ConfigTemplate::NoAuthors);

    let file_name = test_buf.file_name().unwrap();
    let file_path = PathBuf::from(file_name);
    let mut test_file = gen_test_config(cfg_type, &file_path).unwrap();

    let res = composer_json_has_authors(&mut test_sign_path, test_file);

    assert!(!res, "Composer_json_has_authors returned true on package.json with no contributors field");
    assert!(test_sign_path.get_file().is_none());

    delete_test_file(&test_buf).unwrap();
}

#[test]
fn test_noauthors_issignable_composer_json() {

    // No authors assert false
    let test_buf = PathBuf::from_str("./TestComposerJsonNoAuthorsSignable.json").unwrap();
    let mut test_sign_path = SignPath::ComposerJson {
        path: test_buf.clone(),
        file: None,
        file_content: None
    };
    let cfg_type = ConfigFileType::ComposerJson(ConfigTemplate::NoAuthors);

    let file_name = test_buf.file_name().unwrap();
    let file_path = PathBuf::from(file_name);
    let mut test_file = gen_test_config(cfg_type, &file_path).unwrap();

    let res = composer_json_is_signable(&mut test_sign_path, test_file);

    assert!(!res, "Composer_json_is_signable returned true on package.json with no contributors field");
    assert!(test_sign_path.get_file().is_none());

    delete_test_file(&test_buf).unwrap();
}

#[test]
fn test_emptyauthors_hasauthors_composer_json() {

    // No authors assert false
    let test_buf = PathBuf::from_str("./TestComposerJsonEmptyAuthorsHasAuthors.json").unwrap();
    let mut test_sign_path = SignPath::ComposerJson {
        path: test_buf.clone(),
        file: None,
        file_content: None
    };
    let cfg_type = ConfigFileType::ComposerJson(ConfigTemplate::EmptyAuthors);

    let file_name = test_buf.file_name().unwrap();
    let file_path = PathBuf::from(file_name);
    let mut test_file = gen_test_config(cfg_type, &file_path).unwrap();

    let res = composer_json_has_authors(&mut test_sign_path, test_file);

    assert!(!res, "Composer_json_has_authors returned true on package.json with empty contributors field");
    assert!(test_sign_path.get_file().is_none());

    delete_test_file(&test_buf).unwrap();
}


#[test]
fn test_emptyauthors_issignable_composer_json() {

    // No authors assert false
    let test_buf = PathBuf::from_str("./TestComposerJsonEmptyAuthorsSignable.json").unwrap();
    let mut test_sign_path = SignPath::ComposerJson {
        path: test_buf.clone(),
        file: None,
                file_content: None
    };
    let cfg_type = ConfigFileType::ComposerJson(ConfigTemplate::EmptyAuthors);

    let file_name = test_buf.file_name().unwrap();
    let file_path = PathBuf::from(file_name);
    let mut test_file = gen_test_config(cfg_type, &file_path).unwrap();

    let res = composer_json_is_signable(&mut test_sign_path, test_file);

    assert!(res, "Composer_json_is_signable returned false on package.json with empty contributors field");
    assert!(test_sign_path.get_file().is_some());

    delete_test_file(&test_buf).unwrap();
}


#[test]
fn test_withauthors_hasauthors_composer_json() {

    // No authors assert false
    let test_buf = PathBuf::from_str("./TestComposerJsonWIthAuthorsHasAuthors.json").unwrap();
    let mut test_sign_path = SignPath::ComposerJson {
        path: test_buf.clone(),
        file: None,
                file_content: None
    };
    let cfg_type = ConfigFileType::ComposerJson(ConfigTemplate::WithAuthors);

    let file_name = test_buf.file_name().unwrap();
    let file_path = PathBuf::from(file_name);
    let mut test_file = gen_test_config(cfg_type, &file_path).unwrap();

    let res = composer_json_has_authors(&mut test_sign_path, test_file);

    assert!(res, "Composer_json_has_authors returned false on package.json with 1+ contributor entry");
    assert!(test_sign_path.get_file().is_some());

    delete_test_file(&test_buf).unwrap();
}


#[test]
fn test_withauthors_issignable_composer_json() {

    // No authors assert false
    let test_buf = PathBuf::from_str("./TestComposerJsonWIthAuthorsSignable.json").unwrap();
    let mut test_sign_path = SignPath::ComposerJson {
        path: test_buf.clone(),
        file: None,
                file_content: None
    };
    let cfg_type = ConfigFileType::ComposerJson(ConfigTemplate::WithAuthors);

    let file_name = test_buf.file_name().unwrap();
    let file_path = PathBuf::from(file_name);
    let mut test_file = gen_test_config(cfg_type, &file_path).unwrap();

    let res = composer_json_is_signable(&mut test_sign_path, test_file);

    assert!(res, "Composer_json_is_signable returned false on package.json with 1+ contributor entry");
    assert!(test_sign_path.get_file().is_some());

    delete_test_file(&test_buf).unwrap();
}




