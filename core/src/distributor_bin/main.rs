use std::ffi::OsString;
use std::env::{
    // current_exe,
    args_os
};
use std::os::unix::ffi::OsStringExt;
// use std::path::PathBuf;
use clap::{
    value_parser,
    // builder::NonEmptyStringValueParser,
    Arg,
    // ArgAction,
    // ArgGroup,
    Command
};
use anyhow::Result;
use std::process::{
    Command as ProcessCommand,
    // exit
};

fn main() -> Result<(), String> {

    // includes bin name at idx 0
    let matches = command()
        .try_get_matches_from(args_os())
        .map_err(|e| e.to_string())?;

    // If no "set-config-path", forward everything to the functionality binary
    let Some(cfg_path) = matches.get_one::<OsString>("set-config-path") else {
        // pipe everything to `sigrs_functionality` except arg[0]
        // skip binary name `sigrs`
        let args: Vec<OsString> = std::env::args_os().skip(1).collect();
        
        let res = ProcessCommand::new("sigrs_function")
            .args(args)
            .output()
            .map_err(|e| e.to_string())?;

        let stdout = String::from_utf8_lossy(&res.stdout);
        let stderr = String::from_utf8_lossy(&res.stderr);
        println!("sigrs_function stdout: {:?}", stdout);
        println!();
        println!("sigrs_function stderr: {:?}", stderr);

        return Ok(());
    };

    // cfg_path is the path to set

    // 1) call `sigrs_functionality get-bin-path`, read output from stdout
    let bin_path_res = ProcessCommand::new("sigrs_function")
        .arg("get-bin-path")
        .output()
        .map_err(|e| e.to_string())?;
    println!("DISTRIBUTOR | sigrs_function stdout: {}", String::from_utf8_lossy(&bin_path_res.stdout));
    println!("DISTRIBUTOR | sigrs_function stderr: {}", String::from_utf8_lossy(&bin_path_res.stderr));
    // if !bin_path_res.stderr.is_empty() {
    //     return Err(format!("Error getting bin path: {:?}", bin_path_res.stderr));
    // }
    // if bin_path_res.stdout.is_empty() {
    //     return Err(String::from("No response when getting bin path"));
    // }

    // bin_path_res.stdout contains bytes of path
    // let bin_path = PathBuf::from(OsString::from_vec(bin_path_res.stdout));
    let bin_path = OsString::from_vec(bin_path_res.stdout);

    // now I have the bin path, I need to call "sigrs_modifier" with the 
    // new cfg_path, plus the path to the binary
    // - path_to_binary gets modified by
    // - appending cfg_path.as_bytes() to the end of it

    let mod_res = ProcessCommand::new("sigrs_modifier")
        .arg("--bin-path")
        .arg(bin_path)
        .arg("--new-cfg-path")
        .arg(cfg_path)
        .output()
        .map_err(|e| e.to_string())?;

    println!("DISTRIBUTOR | sigrs_modifier stdout: {}", String::from_utf8_lossy(&mod_res.stdout));
    println!("DISTRIBUTOR | sigrs_modifier stderr: {}", String::from_utf8_lossy(&mod_res.stderr));
    // if !mod_res.stderr.is_empty() {
    //     return Err(format!("Error calling sigrs_modifier: {:?}", bin_path_res.stderr));
    // }
    // if mod_res.stdout.is_empty() {
    //     return Err(String::from("No response from sigrs_modifier"));
    // }


    // if let Some(cfg_path) = args.get_one::<OsString>("set-config-path") {
    //     // cfg_path is the path
    //     // extract new config path
    //     //
    //     // call `sigrs_function get-bin-path`
    //     // read stdout, parse into OsString
    //     //
    //     // call `sigrs_modifier set-config-path --bin="bin.exe" --new="xxx.sigrs"`
    //     Ok(())
    // }
    Ok(())

}

fn command() -> Command {
    Command::new("sigrs")
        .about("sigrs")
        .arg(
            Arg::new("set-config-path")
                .long("set-config-path")
                .alias("scp")
                .value_parser(value_parser!(OsString))
        )
}
