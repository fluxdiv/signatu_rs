// #![allow(unused_imports)]
use std::ffi::OsString;
use std::env::current_exe;
use std::io::{self, Write};
use std::os::unix::ffi::{OsStrExt, OsStringExt};
use clap::{
    value_parser,
    builder::NonEmptyStringValueParser,
    Arg,
    ArgAction,
    ArgGroup,
    Command
};
use anyhow::Result;

mod identity;

mod las;
use las::list::{handle_list, handle_list_find, handle_list_by_id};
use las::storage::{handle_update, handle_add_new};
use las::delete::handle_delete;

mod signing;
use signing::handle::handle_sign;

mod utils;
use utils::*;

mod memmap;

// Here I need to read myself (std::env::current_exe()),
// and parse the last bytes to get the config path


fn main() -> Result<(), String> {

    let matches = sigrs_command().get_matches();

    match matches.subcommand() {
        // =============== General
        Some(("get-bin-path", _)) => {
            match std::env::current_exe() {
                Ok(path) => {
                    // write path to stdout as bytes
                    // I believe these will be u8/u16 depending on OS without
                    // needing conditional compilation 
                    let path_bytes = path.as_os_str().as_bytes();
                    let stdout = io::stdout();
                    let mut lock = stdout.lock();
                    lock.write_all(path_bytes)
                        .map_err(|e| e.to_string())?;

                    return Ok(());
                },
                Err(e) => {
                    // write err to stderr as bytes
                    let stderr = io::stderr();
                    let mut lock = stderr.lock();
                    let err = e.to_string();

                    lock.write_all(err.as_bytes())
                        .map_err(|e| e.to_string())?;

                    return Ok(());
                }
            }
        },
        Some(("las", _sub_matches)) => {
            println!("{}", LAS_HELP);
        },
        // =============== Signing
        Some(("sign", sub_matches)) => {
            unsafe {
                handle_sign(sub_matches)?;
            }
        },
        // =============== Storage
        Some(("add-new", sub_matches)) => {
            handle_add_new(sub_matches)?;
        },
        // TODO Default username/email 
        Some(("update", sub_matches)) => {
            handle_update(sub_matches)?;
        },
        Some(("delete", sub_matches)) => {
            handle_delete(sub_matches)?;
        },
        // ============== Listing
        Some(("list-all", sub_matches)) => {
            unsafe {
                handle_list(sub_matches)?;
            }
        },
        Some(("list-by-id", sub_matches)) => {
            unsafe {
                handle_list_by_id(sub_matches)?;
            }
        },
        Some(("list-find", sub_matches)) => {
            unsafe {
                handle_list_find(sub_matches)?;
            }
        }
        _ => unreachable!()
    }

    Ok(())
}



pub fn sigrs_command() -> Command {

    Command::new("sigrs_function")
        .about("Primary functionality binary for signatu_rs. You probably didn't mean to execute this directly. Instead use `sigrs --help`")
        // .subcommand_required(true)
        .arg_required_else_help(true)
        // ================================= Misc
        .subcommand(
            Command::new("get-bin-path")
                .long_flag("get-bin-path")
        )
        .subcommand(
            Command::new("las")
                // .about("Explanation of LAS / Local Author Storage")
                .long_flag("las")
                .long_flag_aliases(["LAS", "local-author-storage"])
                .aliases(["LAS", "local-author-storage"])
                .long_about(LAS_HELP)
        )
        // Needs work, 
        // add run-time modifiable const embeded in binary via include_bytes probably,
        // That way it won't need to be specified every time
        // .subcommand(
        //     Command::new("generate-config")
        //         .about("Can be used to specify a location for `config.sigrs` to be stored. Run `sigrs generate-config --help` for more info")
        //         .long_about(GENERATE_CONFIG_ABOUT)
        //         .arg_required_else_help(true)
        //         .arg(
        //             Arg::new("path")
        //                 .required(true)
        //                 .help("Absolute path for `config.sigrs` to be stored")
        //         )
        // )
        .subcommand(
            Command::new("sign")
                .about("Sign your credentials to a configuration file(s)")
                .long_about("Explain the 3 parts of this:")
                // Choose an identity to sign with
                // Choose which directories to look for configurations
                // Choose what kind of configs to look for (if-signable, cargo.toml ..
                .arg(
                    Arg::new("config-path")
                        .long("config-path")
                        .help("...")
                )
                .arg(
                    Arg::new("identity")
                        .help("Identity to use when signing a configuration file(s)")
                        .long_help("Info about how default is used etc.")
                        .long("identity")
                        .alias("id")
                )
                .arg(
                    Arg::new("username")
                        .help("Sign with a custom, one-off username not stored in LAS")
                        .help("Use custom username? Is this needed?")
                        .long_help("Yes, shouldn't force user to use LAS")
                        // features like sign all configs in working dir etc.
                        .long_help("Because the signing features alone are useful")
                )
                .arg(
                    Arg::new("email")
                        .help("Sign with a custom, one-off email not stored in LAS")
                        .help("see 'username' above")
                )
                .arg(
                    Arg::new("file").short('f')
                        .help("Relative or absolute path to configuration file(s) to sign")
                        .action(ArgAction::Append)
                        .value_parser(value_parser!(OsString))
                        // NEEDS to allow OsStr as file name etc. for Windows users
                )
                // =========================== Directory matching
                .arg(
                    Arg::new("delimiter").long("delimiter")
                        .help("Specify a custom delimiter to use in flags like `dir-with-depth`. Might be useful if you have unusual characters in a path. Defaults to a double-colon `::` if not provided.")
                        .default_value("::")
                )
                .arg(
                    Arg::new("dir").short('d')
                        .help("Relative or absolute path to directory(s) to search. Will recursively search all sub directories. To provide a max depth, see `--dir-with-depth`")
                        .action(ArgAction::Append)
                        .value_parser(value_parser!(OsString))
                )
                .arg(
                    Arg::new("dir-with-depth")
                        .long("dir-with-depth")
                        .help("Relative or absolute path to directory(s) to search and a maximum depth of sub-directories to check.\n A depth of 0 will search only the directory provided.\nDepth must be between 0-255.\nEx: `--dir-with-depth 2::some_dir`")
                        .long_help("Use `::` as delimiter unless you provided a custom delimiter like `--delimiter=\"|-|\"")
                        .action(ArgAction::Append)
                        .value_name("DEPTH>::<DIR PATH")
                        // .value_names(["DIR PATH", "DEPTH"])
                        // .num_args(2)
                        // .value_parser(value_parser!(OsString))
                )
                .arg(
                    Arg::new("working-dir")
                        .help("Search the current working directory, but not subdirectories within it")
                        .action(ArgAction::SetTrue)
                )
                .arg(
                    Arg::new("working-dir-recursive")
                        .help("Search current working directory && all sub-directories recursively. Optionally provide an integer value to be the maximum depth. If no value is passed, a default of `-1` is used, which is equivalent to 'search ALL sub-directories recursively'\nEx: `--working-dir-recursive` - Search ALL sub-dirs recursively\n`--working-dir-recursive=-1` - Search ALL sub-dirs recursively (same as passing no value)\n`--working-dir-recursive=0` - Search only working dir (same as `--working-dir` non-recursive)\n`--working-dir-recursive=2` - Search sub-dirs max depth 2 (`=` equal sign REQUIRED)")
                        .value_parser(value_parser!(i8).range(-1..))
                        .num_args(0..=1)
                        // Require equals sign `--working-dir-recursive=2`
                        .require_equals(true)
                        .default_missing_value("-1")
                )

                .group(
                    ArgGroup::new("directory-filtering")
                        .args(&["working-dir", "working-dir-recursive"])
                        .multiple(false)
                        .required(false)
                )

                // =========================== File matching
                .arg(
                    Arg::new("only-include")
                        .help("Only include these types of configuration files. All others will be ignored. Cannot be used in conjunction with `only-exclude`\nBy default, ALL configuration types will be considered if `only-include` or `only-exclude` is not used.\nRun --help for how to use this parameter")
                        .long_help("Examples using default delimiter `::`, replace with custom delimiter if using --delimiter\nSingle | `--only-include Cargo.toml`\nMultiple | `--only-include Cargo.toml::package.json::setup.py`\nPossible values (casing DOES matter): < `Cargo.toml`, `package.json`, `pyproject.toml`, `setup.py`, `setup.cfg` >")
                        
                )
                .arg(
                    Arg::new("only-exclude")
                        .help("Only exclude these types of configuration files. All others will be included. Cannot be used in conjunction with `only-include`\nBy default, ALL configuration types will be considered if `only-include` or `only-exclude` is not used.")
                        .long_help("Examples using default delimiter `::`, replace with custom delimiter if using --delimiter\nSingle | `--only-exclude Cargo.toml`\nMultiple | `--only-exclude Cargo.toml::package.json::setup.py`\nPossible values (casing DOES matter): < `Cargo.toml`, `package.json`, `pyproject.toml`, `setup.py`, `setup.cfg` >")
                )
                .group(
                    ArgGroup::new("only-include-exclude")
                        .args(&["only-include", "only-exclude"])
                        .multiple(false)
                        .required(false)
                )
                .arg(
                    Arg::new("if-signable")
                        .help("Only sign matching configuration files if they already have authors")
                        .long_help("After finding configuration files that match your parameters, sigrs will check each of these files to see if it already has an authors field (or equivalent, depending on the type of configuration file) present, and your signature will only be added to the files that do.\n\nA config file with an empty authors list (but with an authors field present) WILL be signed.\n\nIf you only want to append your signature to an authors field if it already has 1+ authors within it, use `if-has-signatures` instead")
                        .action(ArgAction::SetTrue)
                )
                .arg(
                    Arg::new("if-has-signatures")
                        .help("Only sign matching configuration files if they already have authors")
                        .long_help("After finding configuration files that match your parameters, sigrs will check each of these files to see if it already has other authors listed, and your signature will only be added to the files that do.\n\nA config with an empty authors list (but with an authors field present) will NOT be signed.")
                        .action(ArgAction::SetTrue)
                )
                .group(
                    ArgGroup::new("existing-authors")
                        .args(&["if-signable", "if-has-signatures"])
                        .multiple(false)
                        .required(false)
                )

            // give users an option to specify --Cargo.toml or config type
            // --if-has-authors  | Only sign matching configs if it has authors already
            // --cargo-workspace | Only sign main Cargo.toml for matching workspace

            // .group(
            //     // Only 1 of "username" or "username-index" can be used
            //     ArgGroup::new("signing-username")
            //         .args(&["username", "username-index"])
            //         .multiple(false)
            //         .required(false)
            // )
        )
        // ===================================================== LOCAL STORAGE MODS
        .subcommand(
            Command::new("add-new")
                .about("Create a new identity and save it in Local Author Storage")
                .arg(
                    Arg::new("config-path")
                        .long("config-path")
                        .help("Absolute path to `config.sigrs`. Only needed if you provided a custom `--config-path` when running `sigrs` for the first time")
                        .value_parser(NonEmptyStringValueParser::new()),
                )
                .arg(
                    Arg::new("identity")
                        .long("identity")
                        // remove required if/when I add interactivity
                        .required(true)
                        .requires("usernames")
                        .requires("emails")
                        .value_parser(NonEmptyStringValueParser::new()),
                )
                .arg(
                    Arg::new("usernames")
                        .short('U')
                        .help("Usernames to add to created identity")
                        .long_help("Usernames to include in created identity. To include multiple usernames, prefix each username with `-U`.\nEx: `sigrs add-new -U \"some name\" -U \"bob\"` ")
                        .action(ArgAction::Append)
                )
                .arg(
                    Arg::new("emails")
                        .short('E')
                        .help("Emails to add to created identity")
                        .long_help("Emails to add to created identity. To include multiple emails, prefix each email with `-E`\nEx: `sigrs add-new --identity \"some_id\" -E \"x@y.com\" -E \"y@x.com\"`")
                        .action(ArgAction::Append)
                )

        )
        .subcommand(
            Command::new("update")
                .about("Update an existing identity stored in LAS")
                .arg_required_else_help(true)
                .arg(
                    Arg::new("config-path")
                        .long("config-path")
                        .help("Absolute path to `config.sigrs`. Only needed if you provided a custom `--config-path` when running `sigrs` for the first time")
                        .value_parser(NonEmptyStringValueParser::new()),
                )
                .arg(
                    Arg::new("identity")
                        .long("identity")
                        .required(true)
                        .value_parser(NonEmptyStringValueParser::new()),
                )
                .arg(
                    Arg::new("change-identity")
                        .long("change-identity")
                        .help("Change the identity lookup provided in --identity to this identity\nEx: `sigrs update --identity \"old_id\" --change-identity \"new_identity\"`")
                )
                .arg(
                    Arg::new("add-username")
                        .long("add-username")
                        .alias("au")
                        .long_help("Usernames to add to this identity. To include multiple usernames, prefix each username with `--au`.\nEx: `sigrs update-- identity \"x\" --au \"bob\" --au \"rob\"` ")
                        .action(ArgAction::Append)
                )
                .arg(
                    Arg::new("add-email")
                        .long("add-email")
                        .alias("ae")
                        .long_help("Emails to add to this identity. To include multiple emails, prefix each email with `--ae`.\nEx: `sigrs update --identity \"x\" --ae \"x@y.com\" --ae \"y@x.com\"` ")
                        .action(ArgAction::Append)
                )
                .arg(
                    Arg::new("remove-username")
                        .long("remove-username")
                        .alias("ru")
                        .long_help("Usernames to remove from this identity. To include multiple usernames, prefix each username with `--ru`.\nEx: `sigrs update --identity \"x\" --ru \"bob\" --ru \"rob\"` ")
                        .action(ArgAction::Append)
                )
                .arg(
                    Arg::new("remove-email")
                        .long("remove-email")
                        .alias("re")
                        .long_help("Emails to remove from this identity. To include multiple emails, prefix each email with `--re`.\nEx: `sigrs update --identity \"x\" --re \"x@y.com\" --re \"y@x.com\"` ")
                        .action(ArgAction::Append)
                )
                .arg(
                    Arg::new("remove-all-emails")
                        .long("remove-all-emails")
                        .long_help("Remove all emails from this identity.\nIf called with `add-email`, all emails will be cleared before adding the new ones provided.")
                        .action(ArgAction::SetTrue)
                )
                .arg(
                    Arg::new("remove-all-usernames")
                        .long("remove-all-usernames")
                        .long_help("Remove all usernames from this identity.\nIf called with `add-username`, all usernames will be cleared before adding the new ones provided.")
                        .action(ArgAction::SetTrue)
                )
                .group(
                    // Only 1 removal method can be given for each
                    ArgGroup::new("removal-usernames")
                        .args(&["remove-username", "remove-all-usernames"])
                        .multiple(false)
                        .required(false)
                )
                .group(
                    ArgGroup::new("removal-emails")
                        .args(&["remove-email", "remove-all-emails"])
                        .multiple(false)
                        .required(false)
                )
        )
        // ===================================================== NEW LIST
        // TODO: Add option for JSON output quality of life
        .subcommand(
            Command::new("list-all")
                .about("List all identities stored in LAS")
                .arg(
                    Arg::new("config-path")
                        .long("config-path")
                        .help("Absolute path to `config.sigrs`. Only needed if you provided a custom `--config-path` when running `sigrs` for the first time")
                        .value_parser(NonEmptyStringValueParser::new()),
                )
                .arg(
                    Arg::new("verbose")
                        .long("verbose")
                        .action(clap::ArgAction::SetTrue)
                )
        )
        .subcommand(
            Command::new("list-by-id")
                .about("List details about a specific identity in LAS")
                .arg_required_else_help(true)
                .arg(
                    Arg::new("config-path")
                        .long("config-path")
                        .help("Absolute path to `config.sigrs`. Only needed if you provided a custom `--config-path` when running `sigrs` for the first time")
                        .value_parser(NonEmptyStringValueParser::new()),
                )
                .arg(
                    Arg::new("verbose")
                        .long("verbose")
                        .action(clap::ArgAction::SetTrue)
                )
                .arg(
                    Arg::new("id")
                        .required(true)
                        .long("identity")
                        .help("The identity you want to lookup in LAS")
                        .value_parser(NonEmptyStringValueParser::new()),
                )
        )
        .subcommand(
            Command::new("list-find")
                .about("List details about all identities in LAS containing 1 or more of the provided arguments")
                .arg_required_else_help(true)
                .arg(
                    Arg::new("config-path")
                        .long("config-path")
                        .help("Absolute path to `config.sigrs`. Only needed if you provided a custom `--config-path` when running `sigrs` for the first time")
                        .value_parser(NonEmptyStringValueParser::new()),
                )
                .arg(
                    Arg::new("verbose")
                        .long("verbose")
                        .action(clap::ArgAction::SetTrue)
                )
                .arg(
                    Arg::new("usernames")
                        // .long("usernames")
                        .short('U')
                        .help("Usernames to include in search")
                        .long_help("Usernames to include in search. To include multiple usernames, prefix each username with `-U`.\nEx: `sigrs list-find -U \"some name\" -U \"bob\"` ")
                        .action(ArgAction::Append)
                )
                .arg(
                    Arg::new("emails")
                        .short('E')
                        .help("Emails to include in search")
                        .long_help("Emails to include in search. To include multiple emails, prefix each email with `-E`.\nEx: `sigrs list-find -E \"x@y.com\" -E \"y@x.com\"` ")
                        .action(ArgAction::Append)
                )
                .group(
                    // This group makes at least 1 of usernames or emails required
                    ArgGroup::new("identifiers")
                        .args(&["usernames", "emails"])
                        .multiple(true)
                        .required(true)
                )
        )
        .subcommand(
            Command::new("delete")
                .about("Delete an entire Identity from local storage")
                .arg_required_else_help(true)
                .arg(
                    Arg::new("config-path")
                        .long("config-path")
                        .help("Absolute path to `config.sigrs`. Only needed if you provided a custom `--config-path` when running `sigrs` for the first time")
                        .value_parser(NonEmptyStringValueParser::new()),
                )
                .arg(
                    Arg::new("identity")
                        .required(true)
                        .long("identity")
                        .help("Identity to delete")
                        .value_parser(NonEmptyStringValueParser::new()),
                )
        )
}


const LAS_HELP: &str = "
Local Author Storage (LAS) is a locally stored configuration of different 'identities' for you to use with sigrs. Each 'identity' is stored under a unique 'key' or 'id'. Each identity can include 1 or more 'username's and 'email's, that you can choose from when using sigrs to add your author information to a project.

Your LAS file will be stored in a `config.sigrs` file, typically located ___.

Because `config.sigrs` uses a custom schema, it is highly recommended that you DO NOT manually edit your `config.sigrs`. If you want to edit/change something, use `sigrs update` etc.

If your `config.sigrs` file becomes 'corrupted' (incorrect formatting) through manual changes, the sigrs tool will not function correctly. You can 'reset' to a blank `config.sigrs` via running `sigrs reset-config`.
";

const GENERATE_CONFIG_ABOUT: &str = r"
You can run `sigrs --generate-config` to generate a `config.sigrs` to be stored at a location of your choosing.

You do not need to run this command, as a `config.sigrs` will be generated the first time you use any other command if one does not exist.

By default, all sigrs commands will assume that your `config.sigrs` exists at the following locatiions (this is also where your `config.sigrs` will be generated upon first use, unless you specify otherwise:

Linux
Value: `$XDG_CONFIG_HOME` or `$HOME/.config`
Example: `/home/alice/.config`

macOS
Value: `$HOME/Library/Application Support`
Example: `/Users/Alice/Library/Application Support`

Windows
Value: `{FOLDERID_RoamingAppData}`
Example: `C:\Users\Alice\AppData\Roaming`

If you choose to put your `config.sigrs` in a different location, you will NEED to specify that location every time you run a `config.sigrs` command.
";

