use std::fs::File;
use crate::signing::signpath::SignPath;
use crate::signing::signing_utils::{extract_file_content, generate_temp_path};
use std::io::{
    self, Read, Seek, SeekFrom, Write
};
use std::path::PathBuf;
use serde_json::{Value, json};

pub fn composer_json_is_signable(sign_path: &mut SignPath, mut file: File) -> bool {

    let mut contents = String::new();
    let Ok(_) = file.seek(SeekFrom::Start(0)) else {
        eprintln!("Problem seeking start of file");
        return false;
    };
    let Ok(_) = file.read_to_string(&mut contents) else {
        eprintln!("Problem reading file to string");
        return false;
    };

    let parsed: Value = match serde_json::from_str(&contents) {
        Ok(json) => json,
        Err(e) => {
            eprintln!("Problem parsing JSON: {:?}", e);
            return false;
        }
    };

    // If authors field is present, is signable
    if let Some(_a) = parsed.get("authors") {
        sign_path.add_file(file, contents);
        return true;
    };

    false
}

pub fn composer_json_has_authors(sign_path: &mut SignPath, mut file: File) -> bool {

    let mut contents = String::new();
    let Ok(_) = file.seek(SeekFrom::Start(0)) else {
        eprintln!("Problem seeking start of file");
        return false;
    };
    let Ok(_) = file.read_to_string(&mut contents) else {
        eprintln!("Problem reading file to string");
        return false;
    };

    let parsed: Value = match serde_json::from_str(&contents) {
        Ok(json) => json,
        Err(e) => {
            eprintln!("Problem parsing JSON: {:?}", e);
            return false;
        }
    };

    // If authors field is present && has entries
    if let Some(a) = parsed.get("authors") {
        match a {
            Value::Array(a_list) => {
                if !a_list.is_empty() {
                    sign_path.add_file(file, contents);
                    return true;
                }
            },
            _ => {
                eprintln!("Authors is not a valid list");
                return false;
            }
        }
    };

    false
}

pub fn sign_composer_json(
    path: &mut PathBuf,
    f: &mut Option<File>,
    fc: &mut Option<String>,
    uname: &[u8],
    email: &[u8]
) -> Result<String, String> {

    let file_contents = extract_file_content(&path, f.as_mut(), fc.as_mut())?;

    let mut parsed: Value = match serde_json::from_str(&file_contents) {
        Ok(json) => json,
        Err(e) => {
            return Err(format!("Problem parsing JSON: {:?}", e));
        }
    };

    let Some(config_map) = parsed.as_object_mut() else {
        return Err(String::from("Composer JSON not valid"));
    };

    let authors_field = config_map
        .entry("authors")
        .or_insert_with(|| Value::Array(vec![]));

    let name = String::from_utf8(uname.to_vec())
        .map_err(|e| e.to_string())?;
    let em = String::from_utf8(email.to_vec())
        .map_err(|e| e.to_string())?;

    // composer.json authors field must be an array
    if let Value::Array(authors) = authors_field {
        // {"name": name, "email": email}
        let entry = json!({
            "name": name,
            "email": em,
        });
        authors.push(entry);
    } else {
        return Err(String::from("Invalid composer.json format"));
    }

    let mut tmp_file: Result<File, String> = Err(String::from("Cannot create tmp file"));
    let mut tmp_path = PathBuf::new();
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

    match serde_json::to_string(&parsed) {
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

// {
//   "name": "example-project",
//   "description": "An example PHP project",
//   "version": "1.0.0",
//   "authors": [
//     {
//       "name": "Jane Doe",
//       "email": "jane.doe@example.com",
//       "role": "Developer"
//     },
//     {
//       "name": "John Smith",
//       "email": "john.smith@example.com",
//       "role": "Maintainer"
//     }
//   ],
//   "require": {
//     "php": ">=7.4",
//     "monolog/monolog": "^2.0"
//   }
// }
//
