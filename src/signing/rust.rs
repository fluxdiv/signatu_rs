use std::fs::File;
use crate::signing::signing_utils::{
    extract_file_content,
    generate_temp_path
};
use crate::signing::signpath::SignPath;
use std::io::{
    self, Read, Seek, SeekFrom, Write
};
use std::path::PathBuf;
use toml::Value;

pub fn cargo_is_signable(sign_path: &mut SignPath, mut file: File) -> bool {

    let mut contents = String::new();
    let Ok(_) = file.seek(SeekFrom::Start(0)) else {
        eprintln!("Problem seeking start of file");
        return false;
    };
    let Ok(_) = file.read_to_string(&mut contents) else {
        eprintln!("Problem reading file to string");
        return false;
    };

    let Ok(parsed) = contents.parse::<toml::Value>() else {
        return false;
    };

    let mut is_signable = false;

    fn search_authors(value: &toml::Value, is_signable: &mut bool) {

        if let Some(authors_field) = value.get("authors") {
            if authors_field.as_array().is_some() {
                *is_signable = true;
            }
        }
        match value {
            toml::Value::Table(table) => {
                for val in table.values() {
                    search_authors(val, is_signable);
                }
            },
            toml::Value::Array(array) => {
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

/// Checks if cargo.toml has authors field && it has at least 1 author already
pub fn cargo_has_authors(sign_path: &mut SignPath, mut file: File) -> bool {

    let mut contents = String::new();
    let Ok(_) = file.seek(SeekFrom::Start(0)) else {
        eprintln!("Problem seeking start of file");
        return false;
    };
    let Ok(_) = file.read_to_string(&mut contents) else {
        eprintln!("Problem reading file to string");
        return false;
    };

    let Ok(parsed) = contents.parse::<toml::Value>() else {
        return false;
    };

    let mut has_authors = false;

    fn search_authors(value: &toml::Value, has_authors: &mut bool) {

        if let Some(authors_field) = value.get("authors") {
            if let Some(authors_vec) = authors_field.as_array() {
                if !authors_vec.is_empty() {
                    *has_authors = true;
                }
            }
        }
        match value {
            toml::Value::Table(table) => {
                for val in table.values() {
                    search_authors(val, has_authors);
                }
            },
            toml::Value::Array(array) => {
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


pub fn sign_cargo_toml(
    path: &mut PathBuf,
    f: &mut Option<File>,
    fc: &mut Option<String>,
    uname: &[u8],
    email: &[u8]
) -> Result<String, String> {

    let file_contents = extract_file_content(&path, f.as_mut(), fc.as_mut())?;

    // Parse toml toml value
    let mut parsed = file_contents.parse::<toml::Value>().map_err(|e| e.to_string())?;

    // Find authors field if it exists. If it exists,
    // append uname/email to it
    // otherwise create authors field
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

    if let Some(package_section) = parsed.get_mut("package") {
        append_author(package_section, uname, email)?;
    } else if let Some(workspace_section) = parsed.get_mut("workspace.package") {
        append_author(workspace_section, uname, email)?;
    } else {
        return Err(String::from("Unable to parse Cargo.toml"));
    }

    // Create temp file
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
            // write data to the temp file
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
