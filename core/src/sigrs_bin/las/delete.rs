use clap::ArgMatches;
use std::{
    fs::File,
    io::{Read, Seek, SeekFrom, Write},
};
use crate::utils::extract_config_path;

/// Delete an entire identity
pub fn handle_delete(args: &ArgMatches) -> Result<(), String> {

    let identity = args.get_one::<String>("identity")
        .ok_or_else(|| String::from("--identity required"))?;

    let config_path = extract_config_path(args)?;

    // Get file handle, do not create if didn't exist - nothing ot update
    let mut config_handle = File::options()
        .read(true)
        .write(true)
        .open(&config_path)
        .map_err(|e| format!("Problem opening config file: {:?}", e))?;

    let mut file_content = String::new();
    config_handle.read_to_string(&mut file_content).unwrap();
    assert!(!file_content.is_empty());

    // Find matching key, delete that line && following 2 lines
    let mut line_iter = file_content.lines();
    let mut new_file_content = String::new();
    let mut found = false;

    while let Some(line) = line_iter.next() {
        // Skip comment line
        if line.starts_with("#") {
            new_file_content.push_str(line);
            new_file_content.push('\n');
            continue;
        }

        // If key line, check identity matches
        if line.starts_with("K:") {
            // unwrappable unless file format wrong
            let uname_line = line_iter.next().unwrap();
            let email_line = line_iter.next().unwrap();

            // If identity doesn't match add identity & continue
            let id_key = line.trim_matches(&['K', ':', '"']);
            if id_key != identity {
                new_file_content.push_str(line);
                new_file_content.push('\n');
                new_file_content.push_str(uname_line);
                new_file_content.push('\n');
                new_file_content.push_str(email_line);
                new_file_content.push('\n');
                continue;
            }

            // Identity matches, simple don't add it to new_file content & continue
            found = true;
            continue;
        }
    }

    if !found {
        return Err(String::from("Identity does not exist"));
    }

    // Update file
    config_handle.set_len(0)
        .map_err(|e| format!("Cannot alter file size: {:?}", e))?;
    config_handle.seek(SeekFrom::Start(0))
        .map_err(|e| format!("Cannot seek start of file: {:?}", e))?;
    config_handle.write_all(new_file_content.as_bytes())
        .map_err(|e| format!("Cannot update file: {:?}", e))?;

    Ok(())
}
