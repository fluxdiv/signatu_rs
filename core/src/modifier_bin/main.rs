use std::ffi::OsString;
use std::io::{
    self,
    // Read,
    Seek,
    SeekFrom,
    Write
};
use std::fs::File;
use std::os::unix::ffi::{
    OsStrExt,
    // OsStringExt
};
use std::os::unix::fs::FileExt;
use clap::{
    value_parser,
    Arg,
    Command
};
use anyhow::Result;

/// Writes err to StdErr & propagates Err(String),
/// or returns Ok(Value)
fn pipe_err<T, E: ToString>(
    res: Result<T, E>,
    errlock: &mut dyn Write,
) -> Result<T, String> {
    match res {
        Ok(v) => Ok(v),
        Err(e) => {
            let es = e.to_string();
            if let Err(write_err) = errlock.write_all(es.as_bytes()) {
                return Err(write_err.to_string());
            }
            Err(es)
        }
    }
}

// Now I need to figure out what to do on the first call, when the binary will
// not have anything appended to it
fn main() -> Result<(), String> {
    let matches = command().get_matches();

    let Some(bin_path) = matches.get_one::<OsString>("bin-path") else {
        return Err(String::from("--bin-path required"));
    };

    let Some(new_cfg_path) = matches.get_one::<OsString>("new-cfg-path") else {
        return Err(String::from("--new-cfg-path required"));
    };

    let mut msg = OsString::from("MODIFIER |  bin_path: ");
    msg.push(bin_path);
    msg.push(" | new_cfg_path: ");
    msg.push(new_cfg_path);
    let b = msg.as_bytes();
    let stdout = io::stdout();
    let mut lock = stdout.lock();
    lock.write_all(b)
        .map_err(|e| e.to_string())?;

    // open handle to binary
    // read last 8 bytes of file, which is lenght of path
    // seek to start of path
    // calculate length of new cfg path into hex
    // remove old path & length
    // write new cfg path and length
    let stderr = io::stderr();
    let mut errlock = stderr.lock();

    let mut bin_handle = pipe_err(
        File::options().read(true).write(true).open(bin_path),
        &mut errlock
    )?;

    // read & remove old path & length
    // end index of file
    let file_end_idx = pipe_err(
        bin_handle.seek(SeekFrom::End(-1)),
        &mut errlock
    )?;

    fn seek_data_end(
        stderr: &mut dyn Write,
        handle: &File,
        trash: &mut [u8; 1],
        idx: u64
    ) -> Result<u64, String> {
        // read exactly 1 byte, can assume if 0 that the byte was 0 &
        // initial trash was overwritten since this will fail otherwise
        let _ = pipe_err(
            handle.read_exact_at(trash, idx),
            stderr
        )?;
        // - possible danger of recursing the entire binary if no hex digit found
        match trash.get(0) {
            Some(byte) if byte.is_ascii_hexdigit() => Ok(idx),
            Some(_) => seek_data_end(stderr, handle, trash, idx - 1),
            None => Err(String::from("EOF err"))
        }
    }

    let mut trash_buf = [0u8; 1];
    let data_end_idx = pipe_err(
        seek_data_end(&mut errlock, &bin_handle, &mut trash_buf, file_end_idx),
        &mut errlock
    )?;

    // get path length (last 8 bytes of file)
    let mut path_len_buf = [0u8; 8];

    // -7 not -8
    let _bytes_read = pipe_err(
        bin_handle.read_exact_at(&mut path_len_buf, data_end_idx - 7),
        &mut errlock
    )?;

    // assert_eq!(bytes_read, 8);
    // convert path_len to numerical
    let path_len_str = pipe_err(
        std::str::from_utf8(&path_len_buf),
        &mut errlock
    )?;
    
    // dbg!(path_len_str);
    let path_len_int = pipe_err(
        i32::from_str_radix(path_len_str, 16),
        &mut errlock
    )?;

    // have to account for actual data_end_idx in calculating seek pos
    let path_start_idx = pipe_err(
        bin_handle.seek(
            SeekFrom::Start(data_end_idx - (7 + path_len_int.abs() as u64))
        ),
        &mut errlock
    )?;

    // update file len
    // calculate (new cfg path).len + (new cfg path).len to hex
    let new_cfg_path_bytes = new_cfg_path.as_bytes();
    let new_cfg_path_len = pipe_err(
        u32::try_from(new_cfg_path_bytes.len()),
        &mut errlock
    )?;
    let new_cfg_path_len_hexstr = format!("{:08X}", new_cfg_path_len);
    let hexstr_bytes = new_cfg_path_len_hexstr.as_bytes();
    let hexstr_bytes_len = pipe_err(
        u32::try_from(hexstr_bytes.len()),
        &mut errlock
    )?;

    // total length of (new_cfg_path) + (new_cfg_path len) to hex
    let total_new_path_len = (new_cfg_path_len + hexstr_bytes_len) as u64;

    // update file length to contents + new cfg path len
    let new_file_len = path_start_idx + total_new_path_len;
    let _ = pipe_err(
        bin_handle.set_len(new_file_len),
        &mut errlock
    )?;

    // append new cfg_path & len to file
    let append_data = [new_cfg_path_bytes, hexstr_bytes].concat();
    let _ = pipe_err(
        bin_handle.write_all(&append_data),
        &mut errlock
    )?;

    // try calling this
    // --bin-path is the file that gets updated, use a.md
    // --new-cfg-path is the path that gets appended to --bin-path, use whatever
    
    // --bin-path "/home/user/signatu_rs/src/modifier_bin/a.md"
    // --new-cfg-path "/some/path.sigrs"

    Ok(())
}

fn command() -> Command {
    Command::new("sigrs_modifier")
        .about("You probably didn't mean to clal this directly. Use `sigrs --help`")
        .arg_required_else_help(true)
        .arg(
            Arg::new("bin-path")
                .required(true)
                .long("bin-path")
                .value_parser(value_parser!(OsString))
        )
        .arg(
            Arg::new("new-cfg-path")
                .required(true)
                .long("new-cfg-path")
                .value_parser(value_parser!(OsString))
        )
}
