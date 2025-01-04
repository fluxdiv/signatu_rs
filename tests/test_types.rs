use std::ops::Deref;
use core::panic;
use std::process::Command;

// Used across tests
pub const BIN_NAME: &'static str = "sigrs";

pub fn test<'a, ARGS>(params: TestParams<'a, ARGS>) 
where
    ARGS: Arguments<'a>
{
    
    let output = Command::new(BIN_NAME)
        .arg(params.args.get_cmd())
        .args(params.args.into_args())
        .output();

    println!("=============================");
    println!("Ctx   : {}", params.ctx);

    match params.args {
        ArgType::ShouldPass(_) => {
            println!("EXPECT: Pass");
            match output {
                // OS success && app success, intended
                Ok(out) if out.status.success() => {
                    println!("RESULT: Pass");
                    if params.verbose {
                        let stdout = String::from_utf8_lossy(&out.stdout);
                        let stderr = String::from_utf8_lossy(&out.stderr);
                        println!("stdout: {}", stdout);
                        println!("stderr: {}", stderr);
                    }
                },
                Ok(out) => {
                    // OS success but app fail
                    println!("RESULT: Fail");
                    let mut msg_buf = String::new();
                    if params.verbose {
                        let stdout = String::from_utf8_lossy(&out.stdout);
                        let stderr = String::from_utf8_lossy(&out.stderr);
                        msg_buf.push_str(&format!("stdout: {}", stdout));
                        msg_buf.push_str(&format!("stderr: {}", stderr));
                    }
                    panic!("{}", msg_buf);
                },
                Err(e) => {
                    // OS fail
                    println!("RESULT: OS Fail");
                    let mut msg_buf = String::from("");
                    if params.verbose {
                        msg_buf.push_str(&format!("Error : {:?}\n", e));
                    }
                    panic!("{}", msg_buf);
                }
            }
        },
        ArgType::ShouldFail(_) => {
            println!("EXPECT: Fail");

            match output {
                Ok(out) if out.status.success() => {
                    println!("RESULT: Pass");
                    let mut msg_buf = String::new();
                    if params.verbose {
                        let stdout = String::from_utf8_lossy(&out.stdout);
                        let stderr = String::from_utf8_lossy(&out.stderr);
                        msg_buf.push_str(&format!("stdout: {}", stdout));
                        msg_buf.push_str(&format!("stderr: {}", stderr));
                    }
                    panic!("{}", msg_buf);
                },
                Ok(out) => {
                    // OS success but app fail, intended
                    println!("RESULT: Fail");
                    if params.verbose {
                        let stdout = String::from_utf8_lossy(&out.stdout);
                        let stderr = String::from_utf8_lossy(&out.stderr);
                        println!("stdout: {}", stdout);
                        println!("stderr: {}", stderr);
                    }
                },
                Err(e) => {
                    // OS fail should panic
                    println!("RESULT: OS Fail");
                    let mut msg_buf = String::from("");
                    if params.verbose {
                        msg_buf.push_str(&format!("Error : {:?}\n", e));
                    }
                    panic!("{}", msg_buf);
                }
            }
        }
    }
}


// Type accepted by every testing function
pub struct TestParams<'at, T: Arguments<'at>> {
    /// Context to print on test completion
    pub ctx: String,
    /// Verbosity of output during cargo test -- --nocapture
    /// `true` print everything | `false` print nothing
    /// ### Start with `false`, toggle to `true` when a test fails to get info
    pub verbose: bool,
    /// Arguments for test command
    pub args: ArgType<'at, T>
}


// T are types trait that has "into_args" function
pub enum ArgType<'at, T: Arguments<'at>> {
    ShouldPass(&'at T),
    ShouldFail(&'at T)
}

impl<'at, T: Arguments<'at>> Deref for ArgType<'at, T> {
    type Target = T;
    /// Extract & take ownership of inner argument value
    fn deref(&self) -> &Self::Target {
        match self {
            Self::ShouldPass(val) | Self::ShouldFail(val) => val
        }
    }
}

pub trait Arguments<'a> {
    const CMD: &'static [&'static str];

    fn into_args(&self) -> Vec<&'a str>;

    // Default only 1 command name
    fn get_cmd(&self) -> &'static str {
        Self::CMD[0]
    }
}

// and now can do stuff like
// test_list_find(ArgType::ShouldPass(some_ListArgs));
// within test function, I assert pass/fail

// pub fn example_test_list_fn<'a>(args: ArgType<'a, ListArgs<'a>>) {
//     let z = match args {
//         ArgType::ShouldPass(list_args) => {
//             0 // Can assert the command run succeeds here
//         },
//         ArgType::ShouldFail(list_args) => {
//             0 // Can assert the command run fails here
//         }
//     };
// }

// ========================= Updating Identities
/// Arguments for "--update" (updating an existing identity)
pub struct UpdateArgs<'a> {
    /// ["--config-path", "./Test.sigrs"]
    config_path: Option<[&'a str; 2]>,
    /// ["--identity", "some key"]
    identity: Option<[&'a str; 2]>,
    /// Vec<
    /// ["--change-identity", "newid"],
    /// ["--au", "uname to add"], ["--ae", "email to add"],
    /// ["--ru", "removing username"], ["--re", "removing email"],
    /// ["--remove-all-emails"], ["--remove-all-usernames"]>
    args: Option<Vec<[&'a str; 2]>>
}

impl<'a> Arguments<'a> for UpdateArgs<'a> {
    const CMD: &'static [&'static str] = &["--update"];

    fn into_args(&self) -> Vec<&'a str> {
        let mut args: Vec<&'a str> = Vec::new();

        if let Some(config_args) = self.config_path {
            for arg in config_args {
                args.push(arg);
            }
        }

        if let Some(identity_args) = self.identity {
            for arg in identity_args {
                args.push(arg);
            }
        }

        if let Some(add_args) = &self.args {
            for arg in add_args.as_flattened() {
                args.push(*arg);
            }
        }
        args
    }
}


// ========================= Adding Identities

/// Arguments for "add-new" (adding a new identity)
pub struct AddArgs<'a> {
    /// ["--config-path", "./Test.sigrs"]
    pub config_path: Option<[&'a str; 2]>,
    /// ["--identity", "some key"]
    pub identity: Option<[&'a str; 2]>,
    /// Vec<["-U", "uname1"], ["-E", "x@y.com"]>
    pub args: Option<Vec<[&'a str; 2]>>
}

impl<'a> Arguments<'a> for AddArgs<'a> {
    const CMD: &'static [&'static str] = &["add-new"];

    // NIT: Deduplicate this for AddArgs & UpdateArgs since they have same structure
    fn into_args(&self) -> Vec<&'a str> {
        let mut args: Vec<&'a str> = Vec::new();

        if let Some(config_args) = self.config_path {
            for arg in config_args {
                args.push(arg);
            }
        }

        if let Some(identity_args) = self.identity {
            for arg in identity_args {
                args.push(arg);
            }
        }

        if let Some(add_args) = &self.args {
            for arg in add_args.as_flattened() {
                args.push(*arg);
            }
        }
        args
    }
}


// ======================================= Listing Identities

// Type: list-all, list-by-id, list-find
pub enum ListArgs<'a> {
    All {
        config_path: Option<&'a str>,
        verbose: bool,
    },
    ByID {
        config_path: Option<&'a str>,
        verbose: bool,
        id: Option<&'a str>,
    },
    Find {
        config_path: Option<&'a str>,
        verbose: bool,
        args: Option<Vec<[&'a str; 2]>>
    }
}

// impl<'a> ListArgs<'a> {
//     const ALL: &'static str = "list-all";
//     const BY_ID: &'static str = "list-by-id";
//     const FIND: &'static str = "list-find";
// }

impl<'a> Arguments<'a> for ListArgs<'a> {
    const CMD: &'static [&'static str] = &[
        "list-all",
        "list-find",
        "list-by-id",
    ];

    fn get_cmd(&self) -> &'static str {
        match self {
            &Self::All {..} => Self::CMD[0],
            &Self::Find {..} => Self::CMD[1],
            &Self::ByID {..} => Self::CMD[2]
        }
    }

    fn into_args(&self) -> Vec<&'a str> {

        let mut arg_list: Vec<&'a str> = vec![];

        match self {
            Self::All { config_path, verbose } => {
                if let Some(c) = config_path {
                    arg_list.extend(&["--config-path", c]);
                }
                if *verbose {
                    arg_list.push("--verbose");
                }
                arg_list
            },
            Self::ByID { config_path, verbose, id } => {
                if let Some(c) = config_path {
                    arg_list.extend(&["--config-path", c]);
                }
                if *verbose {
                    arg_list.push("--verbose");
                }
                if let Some(i) = id {
                    arg_list.extend(&["--identity", i]);
                }
                arg_list
            },
            Self::Find { config_path, verbose, args } => {
                if let Some(c) = config_path {
                    arg_list.extend(&["--config-path", c]);
                }
                if *verbose {
                    arg_list.push("--verbose");
                }
                if let Some(a) = args {
                    for arg in a.as_flattened() {
                        arg_list.push(*arg);
                    }
                }
                arg_list
            }
        }
    }
}

