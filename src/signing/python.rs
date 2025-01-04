use std::fs::File;
use std::io::{
    self, Read, Seek, SeekFrom, Write
};
use std::path::PathBuf;
use toml::Value;
use crate::signing::signpath::SignPath;
use crate::signing::signing_utils::{
    extract_file_content,
    generate_temp_path
};

/// Checks if authors field is present within a pyproject_toml
/// Does not require the authors field to have any entries
pub fn pyproject_toml_is_signable(sign_path: &mut SignPath, mut file: File) -> bool {

    let mut contents = String::new();
    let Ok(_) = file.seek(SeekFrom::Start(0)) else {
        eprintln!("Problem seeking start of file");
        return false;
    };
    let Ok(_) = file.read_to_string(&mut contents) else {
        eprintln!("Problem reading file to string");
        return false;
    };

    let Ok(parsed) = contents.parse::<Value>() else {
        return false;
    };
    let mut is_signable = false;

    fn search_authors(value: &Value, is_signable: &mut bool) {

        if let Some(authors_field) = value.get("authors") {
            if authors_field.as_array().is_some() {
                *is_signable = true;
            }
        }
        match value {
            Value::Table(table) => {
                for val in table.values() {
                    search_authors(val, is_signable);
                }
            },
            Value::Array(array) => {
                for val in array {
                    search_authors(val, is_signable);
                }
            },
            _ => {} // ignore non collections
        }
    }

    search_authors(&parsed, &mut is_signable);

    if is_signable {
        sign_path.add_file(file, contents);
        true
    } else {
        false
    }
}

pub fn pyproject_toml_has_authors(sign_path: &mut SignPath, mut file: File) -> bool {

    let mut contents = String::new();
    let Ok(_) = file.seek(SeekFrom::Start(0)) else {
        eprintln!("Problem seeking start of file");
        return false;
    };
    let Ok(_) = file.read_to_string(&mut contents) else {
        eprintln!("Problem reading file to string");
        return false;
    };

    let Ok(parsed) = contents.parse::<Value>() else {
        return false;
    };

    let mut has_authors = false;

    fn search_authors(value: &Value, has_authors: &mut bool) {

        if let Some(authors_field) = value.get("authors") {
            if let Some(authors_vec) = authors_field.as_array() {
                if !authors_vec.is_empty() {
                    *has_authors = true;
                }
            }
        }
        match value {
            Value::Table(table) => {
                for val in table.values() {
                    search_authors(val, has_authors);
                }
            },
            Value::Array(array) => {
                for val in array {
                    search_authors(val, has_authors);
                }
            },
            _ => {} // ignore non collections
        }
    }

    search_authors(&parsed, &mut has_authors);

    if has_authors {
        sign_path.add_file(file, contents);
        true
    } else {
        false
    }
}


// ex `pyproject.toml`
// [tool.poetry]
// name = "example-project"
// version = "0.1.0"
// description = "An example Python project"
// authors = [
//     "Jane Doe <jane.doe@example.com>",
//     "John Smith <john.smith@example.com>",
//     "Alice Johnson <alice.johnson@example.com>"
// ]
pub fn sign_pyproject_toml(
    path: &mut PathBuf,
    f: &mut Option<File>,
    fc: &mut Option<String>,
    uname: &[u8],
    email: &[u8]
) -> Result<String, String> {

    let file_contents = extract_file_content(&path, f.as_mut(), fc.as_mut())?;

    let mut parsed = file_contents.parse::<toml::Value>().map_err(|e| e.to_string())?;

    fn append_author(
        section: &mut Value,
        name: &[u8],
        email: &[u8]
    ) -> Result<(), String> {
        if let Value::Table(table) = section {
            // get authors field Value or create one
            let authors_field = table
                .entry("authors")
                .or_insert_with(|| Value::Array(vec![]));
            
            // parsing
            if let Value::Array(authors_vec) = authors_field {
                let n = String::from_utf8(name.to_vec())
                    .map_err(|e| e.to_string())?;
                let e = String::from_utf8(email.to_vec())
                    .map_err(|e| e.to_string())?;
                // name <e@x.com>
                let entry = format!("{} <{}>", n, e);
                authors_vec.push(Value::String(entry));
            }
        }
        Ok(())
    }

    if let Some(tool_poetry) = parsed.get_mut("tool.poetry") {
        append_author(tool_poetry, uname, email)?;
    } else {
        return Err(String::from("Unable to parse pyproject.toml"));
    }

    let mut tmp_file: Result<File, String> = Err(String::from("Cannot create tmp file"));
    let mut tmp_path = PathBuf::new();
    // cutting corners make better
    for _ in 0..200 {
        let tmp_path_try = generate_temp_path(path)?;

        let r = File::options()
            .read(true).write(true)
            .create_new(true)
            .open(&tmp_path_try);

        match r {
            Ok(f) => {
                tmp_file = Ok(f);
                tmp_path = tmp_path_try;
            },
            Err(e) => {
                if e.kind() == io::ErrorKind::AlreadyExists {
                    continue;
                }
                if let Err(old) = &tmp_file {
                    tmp_file = Err(format!("{}: {}", old, e));
                }
            }
        }
    };

    let mut tmp_file_up = tmp_file.map_err(|e| e)?;

    // write updated to tmp file
    match toml::to_string(&parsed) {
        Err(e) => return Err(e.to_string()),
        Ok(s) => {
            // write tmp file
            tmp_file_up.set_len(0).map_err(|e| e.to_string())?;
            tmp_file_up.seek(SeekFrom::Start(0)).map_err(|e| e.to_string())?;
            tmp_file_up.write_all(s.as_bytes()).map_err(|e| e.to_string())?;
            // fsync() the temp file
            tmp_file_up.sync_data().map_err(|e| e.to_string())?;
            // rename the temp file to the appropriate name
            std::fs::rename(tmp_path, &mut *path).map_err(|e| e.to_string())?;
            // fsync() the containing directory
            if let Some(parent_dir) = path.parent() {
                let dir_file = File::open(parent_dir).map_err(|e| e.to_string())?;
                dir_file.sync_data().map_err(|e| e.to_string())?;
            } else {
                return Err(String::from("Cannot get parent path"));
            }
        }
    }
    
    Ok(format!("{} successfully updated", path.to_str().unwrap_or("Config")))
}


// ===============================================================================

// Will not be supporting `setup.py` since there is no agreed upon standard
// for defining multiple authors. If this changes in the future I can add support

// pub fn setup_py_is_signable(sign_path: &mut SignPath, mut file: File) -> bool {
//     false
// }
//
// pub fn setup_py_has_authors(sign_path: &mut SignPath, mut file: File) -> bool {
//     false
// }
//
// pub fn sign_setup_py(f: &mut File, uname: &[u8], email: &[u8]) -> Result<String, String> {
//     Ok(String::new())
// }
// // ================= `setup.py`
// // from setuptools import setup
// // setup(
// //     name="example_project",
// //     version="0.1.0",
// //     description="An example Python project",
// //     author="Jane Doe, John Smith, Alice Pete",
// //     author_email="j.doe@example.com, j.smith@example.com, a.pete@example.com",
// // )
