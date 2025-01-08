use clap::ArgMatches;
use anyhow::Result;
use std::io::{
    Read, Seek, SeekFrom, Write
};
use std::fs::File;
use std::os::unix::ffi::OsStringExt;
use std::os::unix::fs::FileExt;
use std::os::unix::ffi::OsStrExt;
use std::ffi::OsString;
use std::str::FromStr;

/// If `--config-path` flag passed, uses that value
/// Else, reads config-path appended to binary (set at build time to default config dir)
/// example default unix: `/home/alice/.config/sigrs/config.sigrs`
pub fn extract_config_path(args: &ArgMatches) -> Result<OsString, String> {

    match args.get_one::<String>("config-path") {
        Some(path) => {
            let os = OsString::from_str(path.as_str()).map_err(|e| e.to_string())?;
            Ok(os)
        },
        None => {
            let path = get_config_path()?;
            Ok(path)
        }
    }
}

/// Returns the config path appended to the end of this binary
pub fn get_config_path() -> Result<OsString, String> {
    let Ok(bin_path) = std::env::current_exe() else {
        return Err(String::from("Unable to get current bin path"));
    };

    let mut bin_handle = File::options()
        .read(true).write(false)
        .open(bin_path)
        .map_err(|e| e.to_string())?;

    let eof_idx = bin_handle.seek(SeekFrom::End(-1)).map_err(|e| e.to_string())?;

    // recursively ignore whitepsace
    fn seek_data_end(
        handle: &File,
        trash: &mut [u8; 1],
        idx: u64
    ) -> Result<u64, String> {
        handle.read_exact_at(trash, idx).map_err(|e| e.to_string())?;
        // danger
        match trash.get(0) {
            Some(byte) if byte.is_ascii_hexdigit() => Ok(idx),
            Some(_) => seek_data_end(handle, trash, idx - 1),
            None => Err(String::from("EOF err"))
        }
    }

    let mut trash_buf = [0u8; 1];
    let data_end_idx = seek_data_end(&bin_handle, &mut trash_buf, eof_idx)?;

    let mut path_len_buf = [0u8; 8];
    let _ = bin_handle
        .read_exact_at(&mut path_len_buf, data_end_idx - 7)
        .map_err(|e| e.to_string())?;

    let path_len_str = std::str::from_utf8(&path_len_buf)
        .map_err(|e| e.to_string())?;

    let path_len_int = i32::from_str_radix(path_len_str, 16)
        .map_err(|e| e.to_string())?;

    let path_start_idx = bin_handle
        .seek(SeekFrom::Start(data_end_idx - (7 + (path_len_int.abs() as u64))))
        .map_err(|e| e.to_string())?;

    let mut path_name: Vec<u8> = Vec::with_capacity(path_len_int.abs() as usize);
    let mut path_handle = bin_handle.take((path_len_int.abs() as u64));
    path_handle.read_to_end(&mut path_name)
        .map_err(|e| e.to_string())?;

    let path = OsString::from_vec(path_name);

    Ok(path)
}


/// Writes the name of this binary to stdout / on error to stderr
pub fn print_bin_path() -> Result<(), String> {
    match std::env::current_exe() {
        Ok(path) => {
            // I believe these will be u8/u16 depending on OS without
            // needing conditional compilation 
            let path_bytes = path.as_os_str().as_bytes();
            let stdout = std::io::stdout();
            let mut lock = stdout.lock();
            lock.write_all(path_bytes)
                .map_err(|e| e.to_string())?;

            Ok(())
        },
        Err(e) => {
            // write err to stderr as bytes
            let stderr = std::io::stderr();
            let mut lock = stderr.lock();
            let err = e.to_string();

            lock.write_all(err.as_bytes())
                .map_err(|e| e.to_string())?;

            Ok(())
        }
    }
}
