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
use crate::signing::python::{
    pyproject_toml_has_authors,
    pyproject_toml_is_signable,
};

#[cfg(test)]
use crate::signing::signpath::SignPath;

// ========================================= pyproject
#[test]
fn test_noauthors_hasauthors_pyproject() {

    // No authors assert false
    let test_buf = PathBuf::from_str("./TestPyProjectTomlNoAuthorsHasAuthors.toml").unwrap();
    let mut test_sign_path = SignPath::PyProjectToml {
        // path shouldn't matter
        path: test_buf.clone(),
        file: None,
        file_content: None
    };
    let cfg_type = ConfigFileType::PyProjectToml(ConfigTemplate::NoAuthors);

    let file_name = test_buf.file_name().unwrap();
    let file_path = PathBuf::from(file_name);
    let mut test_file = gen_test_config(cfg_type, &file_path).unwrap();

    let res = pyproject_toml_has_authors(&mut test_sign_path, test_file);

    assert!(!res, "pyproject_toml_has_authors returned true on PyProject.toml with no authors field");
    assert!(test_sign_path.get_file().is_none());

    delete_test_file(&test_buf).unwrap();
}

#[test]
fn test_noauthors_issignable_pyproject() {

    let test_buf = PathBuf::from_str("./TestPyProjectTomlNoAuthorsSignable.toml").unwrap();
    let mut test_sign_path = SignPath::PyProjectToml {
        // path shouldn't matter
        path: test_buf.clone(),
        file: None,
        file_content: None
    };
    let cfg_type = ConfigFileType::PyProjectToml(ConfigTemplate::NoAuthors);

    let file_name = test_buf.file_name().unwrap();
    let file_path = PathBuf::from(file_name);
    let mut test_file = gen_test_config(cfg_type, &file_path).unwrap();

    let res = pyproject_toml_is_signable(&mut test_sign_path, test_file);

    assert!(!res, "pyproject_toml_is_signable returned true on PyProject.toml with no authors field");
    assert!(test_sign_path.get_file().is_none());

    delete_test_file(&test_buf).unwrap();
}


#[test]
fn test_emptyauthors_hasauthors_pyproject() {
    let test_buf = PathBuf::from_str("./TestPyProjectTomlEmptyAuthorsHasAuthors.toml").unwrap();
    let mut test_sign_path = SignPath::PyProjectToml {
        // path shouldn't matter
        path: test_buf.clone(),
        file: None,
        file_content: None
    };
    let cfg_type = ConfigFileType::PyProjectToml(ConfigTemplate::EmptyAuthors);

    let file_name = test_buf.file_name().unwrap();
    let file_path = PathBuf::from(file_name);
    let mut test_file = gen_test_config(cfg_type, &file_path).unwrap();

    let res = pyproject_toml_has_authors(&mut test_sign_path, test_file);

    assert!(!res, "pyproject_toml_has_authors returned true on PyProject.toml with empty authors field");
    assert!(test_sign_path.get_file().is_none());

    delete_test_file(&test_buf).unwrap();
}

#[test]
fn test_emptyauthors_issignable_pyproject() {
    let test_buf = PathBuf::from_str("./TestPyProjectTomlEmptyAuthorsSignable.toml").unwrap();
    let mut test_sign_path = SignPath::PyProjectToml {
        path: test_buf.clone(),
        file: None,
        file_content: None
    };
    let cfg_type = ConfigFileType::PyProjectToml(ConfigTemplate::EmptyAuthors);


    let file_name = test_buf.file_name().unwrap();
    let file_path = PathBuf::from(file_name);
    let mut test_file = gen_test_config(cfg_type, &file_path).unwrap();

    let res = pyproject_toml_is_signable(&mut test_sign_path, test_file);

    assert!(res, "pyproject_toml_is_signable returned false on PyProject.toml with empty authors field");
    assert!(test_sign_path.get_file().is_some());

    delete_test_file(&test_buf).unwrap();
}

#[test]
fn test_oneauthor_hasauthors_pyproject() {
    let test_buf = PathBuf::from_str("./TestPyProjectTomlOneAuthorHasAuthors.toml").unwrap();
    let mut test_sign_path = SignPath::PyProjectToml {
        path: test_buf.clone(),
        file: None,
        file_content: None
    };
    let cfg_type = ConfigFileType::PyProjectToml(ConfigTemplate::WithAuthors);
    let file_name = test_buf.file_name().unwrap();
    let file_path = PathBuf::from(file_name);
    let mut test_file = gen_test_config(cfg_type, &file_path).unwrap();

    let res = pyproject_toml_has_authors(&mut test_sign_path, test_file);

    assert!(res, "pyproject_toml_has_authors returned false on PyProject.toml with non-empty authors field");
    assert!(test_sign_path.get_file().is_some());

    delete_test_file(&test_buf).unwrap();
}

#[test]
fn test_oneauthor_issignable_pyproject() {
    let test_buf = PathBuf::from_str("./TestPyProjectTomlOneAuthorsSignable.toml").unwrap();
    let mut test_sign_path = SignPath::PyProjectToml {
        path: test_buf.clone(),
        file: None,
        file_content: None
    };
    let cfg_type = ConfigFileType::PyProjectToml(ConfigTemplate::WithAuthors);

    let file_name = test_buf.file_name().unwrap();
    let file_path = PathBuf::from(file_name);
    let mut test_file = gen_test_config(cfg_type, &file_path).unwrap();

    let res = pyproject_toml_is_signable(&mut test_sign_path, test_file);

    assert!(res, "pyproject_toml_is_signable returned false on PyProject.toml with non-empty authors field");
    assert!(test_sign_path.get_file().is_some());

    delete_test_file(&test_buf).unwrap();

}


// Return if/when setup.cfg added
// #[test]
// fn test_noauthors_hasauthors_setup_cfg() {
//
//     // No authors assert false
//     let test_buf = PathBuf::from_str("./TestSetupCfgNoAuthorsHasAuthors.cfg").unwrap();
//     let mut test_sign_path = SignPath::SetupCfg {
//         path: test_buf.clone(),
//         file: None,
//                 file_content: None
//     };
//     // NoAuthors hits "metadata found, no author/author email" which is good
//     let cfg_type = ConfigFileType::SetupCfg(ConfigTemplate::NoAuthors);
//     // Empty authors hits "both are empty" which is good
//     // let cfg_type = ConfigFileType::SetupCfg(ConfigTemplate::EmptyAuthors);
//     // WithAuthors prints each email/author, good
//     // let cfg_type = ConfigFileType::SetupCfg(ConfigTemplate::WithAuthors);
//     // Prints no metadata section found, good
//     // let cfg_type = ConfigFileType::SetupCfg(ConfigTemplate::Unique(String::new()));
//
//     let file_name = test_buf.file_name().unwrap();
//     let file_path = PathBuf::from(file_name);
//     let mut test_file = gen_test_config(cfg_type, &file_path).unwrap();
//
//     let res = setup_cfg_has_authors(&mut test_sign_path, test_file);
//
//     assert!(!res, "setup_cfg_has_authors returned true on setup.cfg with no authors field");
//     assert!(test_sign_path.get_file().is_none());
//
//     delete_test_file(&test_buf).unwrap();
// }
//
// #[test]
// fn test_noauthors_issignable_setup_cfg() {
//
//     let test_buf = PathBuf::from_str("./TestSetupCfgNoAuthorsSignable.cfg").unwrap();
//     let mut test_sign_path = SignPath::SetupCfg {
//         // path shouldn't matter
//         path: test_buf.clone(),
//         file: None,
//                 file_content: None
//     };
//     // NoAuthors hits "metadata found, no author/author email" which is good
//     let cfg_type = ConfigFileType::SetupCfg(ConfigTemplate::NoAuthors);
//     // Empty authors hits "both are empty" which is good
//     // let cfg_type = ConfigFileType::SetupCfg(ConfigTemplate::EmptyAuthors);
//     // WithAuthors prints each email/author, good
//     // let cfg_type = ConfigFileType::SetupCfg(ConfigTemplate::WithAuthors);
//     // Prints no metadata section found, good
//     // let cfg_type = ConfigFileType::SetupCfg(ConfigTemplate::Unique(String::new()));
//
//     let file_name = test_buf.file_name().unwrap();
//     let file_path = PathBuf::from(file_name);
//     let mut test_file = gen_test_config(cfg_type, &file_path).unwrap();
//
//     let res = setup_cfg_is_signable(&mut test_sign_path, test_file);
//
//     assert!(!res, "setup_cfg_is_signable returned true on setup.cfg with no authors field");
//     assert!(test_sign_path.get_file().is_none());
//
//     delete_test_file(&test_buf).unwrap();
// }
//
// #[test]
// fn test_emptyauthors_hasauthors_setup_cfg() {
//
//     let test_buf = PathBuf::from_str("./TestSetupCfgEmptyAuthorsHasAuthors.cfg").unwrap();
//     let mut test_sign_path = SignPath::SetupCfg {
//         path: test_buf.clone(),
//         file: None,
//                 file_content: None
//     };
//     let cfg_type = ConfigFileType::SetupCfg(ConfigTemplate::EmptyAuthors);
//
//     let file_name = test_buf.file_name().unwrap();
//     let file_path = PathBuf::from(file_name);
//     let mut test_file = gen_test_config(cfg_type, &file_path).unwrap();
//
//     let res = setup_cfg_has_authors(&mut test_sign_path, test_file);
//
//     assert!(!res, "setup_cfg_is_signable returned true on setup.cfg with empty authors field");
//     assert!(test_sign_path.get_file().is_none());
//
//     delete_test_file(&test_buf).unwrap();
// }
//
//
// #[test]
// fn test_emptyauthors_issignable_setup_cfg() {
//
//     let test_buf = PathBuf::from_str("./TestSetupCfgEmptyAuthorsIsSignable.cfg").unwrap();
//     let mut test_sign_path = SignPath::SetupCfg {
//         path: test_buf.clone(),
//         file: None,
//                 file_content: None
//     };
//     let cfg_type = ConfigFileType::SetupCfg(ConfigTemplate::EmptyAuthors);
//
//     let file_name = test_buf.file_name().unwrap();
//     let file_path = PathBuf::from(file_name);
//     let mut test_file = gen_test_config(cfg_type, &file_path).unwrap();
//
//     let res = setup_cfg_is_signable(&mut test_sign_path, test_file);
//
//     assert!(res, "setup_cfg_is_signable returned false on setup.cfg with empty authors field");
//     assert!(test_sign_path.get_file().is_some());
//
//     delete_test_file(&test_buf).unwrap();
// }
//
// #[test]
// fn test_withauthors_hasauthors_setup_cfg() {
//
//     let test_buf = PathBuf::from_str("./TestSetupCfgWithAuthorsHasAuthors.cfg").unwrap();
//     let mut test_sign_path = SignPath::SetupCfg {
//         path: test_buf.clone(),
//         file: None,
//                 file_content: None
//     };
//     let cfg_type = ConfigFileType::SetupCfg(ConfigTemplate::WithAuthors);
//
//     let file_name = test_buf.file_name().unwrap();
//     let file_path = PathBuf::from(file_name);
//     let mut test_file = gen_test_config(cfg_type, &file_path).unwrap();
//
//     let res = setup_cfg_has_authors(&mut test_sign_path, test_file);
//
//     assert!(res, "setup_cfg_has_authors returned false on setup.cfg with 1+ authors");
//     assert!(test_sign_path.get_file().is_some());
//
//     delete_test_file(&test_buf).unwrap();
// }
//
// #[test]
// fn test_withauthors_issignable_setup_cfg() {
//
//     let test_buf = PathBuf::from_str("./TestSetupCfgWithAuthorsIsSignable.cfg").unwrap();
//     let mut test_sign_path = SignPath::SetupCfg {
//         path: test_buf.clone(),
//         file: None,
//                 file_content: None
//     };
//     let cfg_type = ConfigFileType::SetupCfg(ConfigTemplate::WithAuthors);
//
//     let file_name = test_buf.file_name().unwrap();
//     let file_path = PathBuf::from(file_name);
//     let mut test_file = gen_test_config(cfg_type, &file_path).unwrap();
//
//     let res = setup_cfg_is_signable(&mut test_sign_path, test_file);
//
//     assert!(res, "setup_cfg_is_signable returned false on setup.cfg with 1+ authors");
//     assert!(test_sign_path.get_file().is_some());
//
//     delete_test_file(&test_buf).unwrap();
// }
//
//
// #[test]
// fn test_nometadata_hasauthors_setup_cfg() {
//
//     let test_buf = PathBuf::from_str("./TestSetupCfgNoMetadataHasAuthors.cfg").unwrap();
//     let mut test_sign_path = SignPath::SetupCfg {
//         path: test_buf.clone(),
//         file: None,
//                 file_content: None
//     };
//     let cfg_type = ConfigFileType::SetupCfg(ConfigTemplate::Unique(String::new()));
//
//     let file_name = test_buf.file_name().unwrap();
//     let file_path = PathBuf::from(file_name);
//     let mut test_file = gen_test_config(cfg_type, &file_path).unwrap();
//
//     let res = setup_cfg_has_authors(&mut test_sign_path, test_file);
//
//     assert!(!res, "setup_cfg_has_authors returned true on setup.cfg with no metadata section");
//     assert!(test_sign_path.get_file().is_none());
//
//     delete_test_file(&test_buf).unwrap();
// }
//
//
// #[test]
// fn test_nometadata_issignable_setup_cfg() {
//
//     let test_buf = PathBuf::from_str("./TestSetupCfgNoMetadataIsSignable.cfg").unwrap();
//     let mut test_sign_path = SignPath::SetupCfg {
//         path: test_buf.clone(),
//         file: None,
//                 file_content: None
//     };
//     let cfg_type = ConfigFileType::SetupCfg(ConfigTemplate::Unique(String::new()));
//
//     let file_name = test_buf.file_name().unwrap();
//     let file_path = PathBuf::from(file_name);
//     let mut test_file = gen_test_config(cfg_type, &file_path).unwrap();
//
//     let res = setup_cfg_is_signable(&mut test_sign_path, test_file);
//
//     assert!(!res, "setup_cfg_is_signable returned true on setup.cfg with no metadata section");
//     assert!(test_sign_path.get_file().is_none());
//
//     delete_test_file(&test_buf).unwrap();
// }


