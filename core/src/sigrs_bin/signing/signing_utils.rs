use std::{
    io::{
        Read,
        Seek,
        SeekFrom,
    },
    fs::File,
    path::PathBuf,
    time::{
        Duration,
        UNIX_EPOCH
    }
};

/// If fc (file content) is Some, that is returned
/// `else`
/// If f (file handle) is Some, that is read & returned
/// `else`
/// path (to config file) is opened, read, & returned
pub fn extract_file_content(
    path: &PathBuf,
    f: Option<&mut File>,
    fc: Option<&mut String>
) -> Result<String, String> {
    match (fc, f) {
        (Some(content), Some(_handle)) => {
            Ok::<String, String>(content.to_owned())
        },
        (None, Some(handle)) => {
            let mut c = String::new();

            handle
                .seek(SeekFrom::Start(0))
                .map_err(|e| e.to_string())?;
            handle
                .read_to_string(&mut c)
                .map_err(|e| e.to_string())?;

            Ok::<String, String>(c.to_owned())
        },
        _ => {
            let mut handle = File::options()
                .read(true)
                .write(true)
                .open(&path)
                .map_err(|e| format!("Problem opening '{:?}': {:?}", path, e))?;

            let mut c = String::new();
            handle
                .seek(SeekFrom::Start(0))
                .map_err(|e| e.to_string())?;
            handle
                .read_to_string(&mut c)
                .map_err(|e| e.to_string())?;

            Ok(c.to_owned())
        }
    }
}

/// Generates temp file name, returns path to temp file in same dir as `path`
pub fn generate_temp_path(path: &PathBuf) -> Result<PathBuf, String> {
    let curr_dir = path.parent().ok_or(String::from("Cannot get paths parent"))?;

    if !curr_dir.is_dir() {
        return Err(String::from("Unable to get path's parent directory"));
    }

    // lets dance
    let now = UNIX_EPOCH.elapsed().unwrap_or(Duration::MAX).as_secs();
    let ptr = &true as *const bool;
    let seed = u64::try_from((ptr as i64).wrapping_abs()).unwrap() ^ now;
    // min. max char in file name on most (all?) OS is 255
    let mut tmp_path = PathBuf::with_capacity((u8::MAX - 6).into());
    tmp_path.push(curr_dir);
    tmp_path.push(format!("sigrs{seed:#x}"));
    Ok(tmp_path)
}
