use std::ffi::OsString;
use std::env::{
    // current_exe,
    args_os
};
use std::os::unix::ffi::OsStringExt;
// use std::path::PathBuf;
use clap::{
    value_parser,
    Arg,
    ArgAction,
    Command
};
use anyhow::Result;
use std::process::{
    Command as ProcessCommand, Stdio,
    // exit
};

fn main() -> Result<(), String> {

    // discards the first element, consumes 2nd (the subcommand)
    let mut args = args_os();
    let sub_cmd = args.nth(1);
    let Some(sub_cmd) = sub_cmd else {
        return Err(String::from("Argument required | try `sigrs --help`"));
    };

    match sub_cmd.to_string_lossy().as_ref() {
        // continue
        "set-config-path" => {},
        _ => {
            // pipe everything to sigrs_function
            let mut oargs = vec![sub_cmd];
            oargs.append(&mut args.collect::<Vec<OsString>>());
            let res = ProcessCommand::new("sigrs_function")
                .args(oargs)
                .stdout(Stdio::inherit())
                .stderr(Stdio::inherit())
                .status()
                .map_err(|e| e.to_string())?;
            if !res.success() {
                return Err(String::from(""));
            }
            return Ok(());
        }
    }

    let m = command().get_matches();
    let Some(("set-config-path", sub_m)) = m.subcommand() else {
        panic!("unreachable");
    };

    // cfg_path is the path to set
    let Some(cfg_path) = sub_m.get_one::<OsString>("path") else {
        return Err(String::from("--path required"));
    };

    // call `sigrs_functionality get-bin-path`, read output from stdout
    let bin_path_res = ProcessCommand::new("sigrs_function")
        .arg("get-bin-path")
        .output()
        .map_err(|e| e.to_string())?;
    // println!(
    //     "DISTRIBUTOR | sigrs_function stdout: {}",
    //     String::from_utf8_lossy(&bin_path_res.stdout)
    // );
    // println!(
    //     "DISTRIBUTOR | sigrs_function stderr: {}",
    //     String::from_utf8_lossy(&bin_path_res.stderr)
    // );
    if !bin_path_res.stderr.is_empty() {
        return Err(format!(
            "Error getting bin path: {:?}",
            String::from_utf8_lossy(&bin_path_res.stderr)
        ));
    }
    if bin_path_res.stdout.is_empty() {
        return Err(String::from("No response from `sigrs_function get-bin-path`"));
    }

    let bin_path = OsString::from_vec(bin_path_res.stdout);

    let mut cmd = ProcessCommand::new("sigrs_modifier");
    cmd.arg("--bin-path")
        .arg(bin_path)
        .arg("--new-cfg-path")
        .arg(cfg_path);

    if sub_m.get_flag("no-generate") {
        cmd.arg("--no-generate");
    }

    let mod_res = cmd
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .status()
        .map_err(|e| e.to_string())?;
    if !mod_res.success() {
        return Err(String::new());
    }

    // println!("DISTRIBUTOR | sigrs_modifier stdout: {}", String::from_utf8_lossy(&mod_res.stdout));
    // println!("DISTRIBUTOR | sigrs_modifier stderr: {}", String::from_utf8_lossy(&mod_res.stderr));
    // if !mod_res.stderr.is_empty() {
    //     return Err(format!("Error calling sigrs_modifier: {:?}", bin_path_res.stderr));
    // }
    // if mod_res.stdout.is_empty() {
    //     return Err(String::from("No response from sigrs_modifier"));
    // }

    Ok(())
}

fn command() -> Command {
    Command::new("sigrs")
        .about("sigrs")
        .color(clap::ColorChoice::Always)
        .subcommand(
            Command::new("set-config-path")
                .arg_required_else_help(true)
                .arg(
                    Arg::new("path")
                        .short('p')
                        .required(true)
                        .require_equals(true)
                        .value_parser(value_parser!(OsString))
                )
                .arg(
                    Arg::new("no-generate")
                        .long("no-generate")
                        .action(ArgAction::SetTrue)
                )
        )
}
