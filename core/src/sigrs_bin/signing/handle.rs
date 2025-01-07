use std::{
    collections::BTreeSet,
    env::current_dir,
    ffi::OsString,
    fs::read_dir,
    ops::Deref,
    path::{Path, PathBuf},
};
use clap::ArgMatches;
use crate::memmap::{get_memmap, process_las};
use super::{
    signpath::SignPath,
    filters:: {
        ConfigTypeFilter,
        FileFilter,
        FileFilterApply,
        FileFilterSort,
        SignableFilter
    }, 
};

/// Returns list of file paths that are `depth` directories deep within `dir`
/// `depth: 0` returns only files within `dir`
/// `depth: 1` returns files within `dir` + files 1 sub-dir deep
/// Errors reading directories are currently propagated
/// - Should they be ignored? Printed? return a struct of { paths: Vec<>, errors: Vec<String> } ?
fn visit_depth(
    path: &Path,
    depth: u8,
) -> Result<Vec<PathBuf>, String> {

    let mut files: Vec<PathBuf> = vec![];

    // if path is file add to return else
    // add files in path & recurse if depth > 0
    if path.is_dir() {
        for entry in read_dir(path).map_err(|e| e.to_string())? {
            let entry = entry.map_err(|e| e.to_string())?;
            let entry_path = entry.path();
            if !entry_path.is_dir() {
                files.push(entry_path.to_path_buf());
            } else {
                // only recurse on dir if not end depth
                if depth > 0 {
                    let i = visit_depth(&path, depth - 1)?;
                    files.extend(i);
                }
            }
        }
    } else {
        files.push(path.to_path_buf());
    }
    Ok(files)
}

// Entry point, exports handle_sign
// each language/config type (python etc.) in own file
// UNSAFE: uses memorymap to read from LAS if needed, see `memmap.rs` for info
pub unsafe fn handle_sign(args: &ArgMatches) -> Result<(), String> {

    // Path 1)
    // TODO No identity passed, go through prompt
    let Some(identity_key) = args.get_one::<String>("identity") else {
        return Err(String::from("Interactivity coming soon"));
    };

    // Path 3.2)
    // If both --email & --username are provided, don't need to access LAS
    let maybe_email = args.get_one::<String>("email");
    let maybe_uname = args.get_one::<String>("username");

    if let (Some(custom_email), Some(custom_uname)) = (maybe_email, maybe_uname) {
        // Both custom values are provided, don't need LAS, just go sign
        return do_signing(custom_uname.as_bytes(), custom_email.as_bytes(), args);
    }

    // Path 3.1 && 2) Both of these paths need LAS lookup
    // If either --email or --uname passed, use that custom value + LAS default
    // If neither passed, use default value for both
    // Handles --config-path
    let memmap_las = get_memmap(args)?;
    let las = process_las(&memmap_las);
    let id_storage_entry = las.lookup_id(identity_key)
        .ok_or_else(|| String::from("Identity does not exist"))?;
    let identity = id_storage_entry.1.deref();

    // Use --username if passed otherwise look up default
    let username = match maybe_uname {
        Some(e) => {
            Ok::<Vec<u8>, String>(e.as_bytes().to_vec())
        },
        None => {
            // Look up from LAS
            let x = identity.usernames.get(0)
                .ok_or_else(|| String::from("Identity does not contain any usernames"))?;
            Ok(x.to_vec())
        }
    }?;
    
    // Use --email if passed otherwise look up default
    let email = match maybe_email {
        Some(e) => {
            Ok::<Vec<u8>, String>(e.as_bytes().to_vec())
        },
        None => {
            // Look up from LAS
            let x = identity.emails.get(0)
                .ok_or_else(|| String::from("Identity does not contain any emails"))?;
            Ok(x.to_vec())
        }
    }?;

    // Have username && email now go sign
    do_signing(username, email, args)
}

fn do_signing<T>(username: T, email: T, args: &ArgMatches) -> Result<(), String> 
where
    T: Into<Vec<u8>>,
{
    // Failure in creating 1 signing path shouldn't fail all,
    // errors should be handled (just log?) individually
    let uname_bytes: Vec<u8> = username.into();
    let email_bytes: Vec<u8> = email.into();

    let (mut paths, errs) = get_signing_paths(args);

    // Just logging errors getting signing paths for now
    for e in errs.into_iter() {
        eprintln!("{e}");
    }

    paths
        .iter_mut()
        .map(|path| {
            path.open_file_handle()
                .and_then(|p| p.sign(uname_bytes.as_slice(), email_bytes.as_slice()))
        })
        .for_each(|res| {
            match res {
                Ok(r) => println!("{r}"),
                Err(e) => eprintln!("{e}")
            }
        });

    Ok(())

}

// Use args to find && return all paths that need to be signed
/// Returns (Vec<SignPaths to sign>  ,  Vec<Error strings creating SignPaths>)
fn get_signing_paths<'s>(args: &'s ArgMatches) -> (Vec<SignPath>, Vec<String>) {

    let mut path_vals: BTreeSet<SignPath> = BTreeSet::new();
    let mut err_vals: Vec<String> = Vec::new();

    // First, get all hardcoded --file paths passed
    if let Some(hf) = args.get_many::<OsString>("file")
        .map(|files| files.collect::<Vec<&OsString>>()) 
    {
        for f in hf {
            let v = SignPath::new(PathBuf::from(f));
            match v {
                Ok(sp) => {
                    let _ = path_vals.insert(sp);
                },
                Err(e) => {
                    err_vals.push(e);
                }
            }
        }
    }

    // let the filtering begin
    // =================  Step 1)  Handle which directories are to be searched
    // basic dirs passed via --dir
    if let Some(dirs) = args.get_many::<OsString>("dir")
        .map(|dirs| dirs.collect::<Vec<&OsString>>()) 
    {
        // for each dir, recursively get all files (--dir) has no depth limit
        for dir in dirs {
            let p_bufs = visit_depth(Path::new(&dir), u8::MAX).unwrap();
            for pb in p_bufs {
                let v = SignPath::new(pb);
                match v {
                    Ok(sp) => {
                        let _ = path_vals.insert(sp);
                    },
                    Err(e) => {
                        err_vals.push(e);
                    }
                }
            }
        }
    }

    // search dirs-with-depth
    if let Some(dwds) = args.get_many::<String>("dir-with-depth")
        .map(|dirs| dirs.cloned().collect::<Vec<String>>()) 
    {
        // get delimiter
        let delimiter = args.get_one::<&String>("delimiter").unwrap();
        // for each, split by delimiter, then use visit_depth
        for dwd in dwds {
            // split each dir-with-depth by delimiter
            let (depth, dir) = parse_dir_with_depth(&dwd, &delimiter).unwrap();

            // use visit_depth to get files && append to file_paths
            let p_bufs = visit_depth(Path::new(&dir), depth).unwrap();
            for pb in p_bufs {
                let v = SignPath::new(pb);
                match v {
                    Ok(sp) => {
                        let _ = path_vals.insert(sp);
                    },
                    Err(e) => {
                        err_vals.push(e);
                    }
                }
            }
        }
    }

    // if including current working directory (but not recursive sub-dirs)
    if args.get_flag("working-dir") {
        let working_dir = current_dir();
        match working_dir {
            Ok(wd) => {
                let p_bufs = visit_depth(wd.as_path(), 0u8).unwrap();
                for pb in p_bufs.into_iter() {
                    let v = SignPath::new(pb);
                    match v {
                        Ok(sp) => {
                            let _ = path_vals.insert(sp);
                        },
                        Err(e) => {
                            err_vals.push(e);
                        }
                    }
                }
            },
            Err(e) => {
                err_vals.push(format!("{:?}", e));
            }
        };
    }

    // working-dir-recursive has default missing value -1 if no depth level provided
    if let Some(depth) = args.get_one::<i8>("working-dir-recursive") {

        let working_dir = current_dir();
        // If -1 (which is default) recurse all, 
        // else recurse user provided depth level
        match working_dir {
            Ok(wd) => {
                // hack to safely cast to u8
                let depth = if depth <= &-1i8 { u8::MAX } else { depth.unsigned_abs() };
                let p_bufs = visit_depth(wd.as_path(), depth).unwrap();
                for pb in p_bufs {
                    let v = SignPath::new(pb);
                    match v {
                        Ok(sp) => {
                            let _ = path_vals.insert(sp);
                        },
                        Err(e) => {
                            err_vals.push(e);
                        }
                    }
                }
            },
            Err(e) => {
                err_vals.push(format!("{:?}", e));
            }
        };
    }

    // ====================== Ok that's enough directory features for now
    // At this point, path_vals contains ALL paths within directories
    // that matched user's directory filters
    // Step 2) Extract/parse all file filters

    let mut filters: Vec<Box<dyn FileFilter>> = Vec::new();
    
    // These are mutually exclusive
    if let Some(only_include) = args.get_one::<&String>("only-include") {
        let delimiter = args.get_one::<String>("delimiter").unwrap();
        let include_types = parse_only_include_exclude(only_include, delimiter);
        filters.push(Box::new(ConfigTypeFilter::Include(include_types)));
    } else if let Some(only_exclude) = args.get_one::<&String>("only-exclude") {
        let delimiter = args.get_one::<String>("delimiter").unwrap();
        let exclude_types = parse_only_include_exclude(only_exclude, delimiter);
        filters.push(Box::new(ConfigTypeFilter::Exclude(exclude_types)));
    }

    // Mutually exclusive
    if args.get_flag("if-signable") {
        filters.push(Box::new(SignableFilter::IfSignable));
    } else if args.get_flag("if-has-signatures") {
        filters.push(Box::new(SignableFilter::IfHasSignatures));
    }

    // All file filters have been parsed/extracted, apply them to 
    // all the paths that matched directory filters
    // ---
    // Sort filters by access requirements
    // filters requiring file reads come last, so they are only checked if nessecary
    filters.sort_by_access();

    let mut path_vals_vec = path_vals.into_iter().collect::<Vec<SignPath>>();
    path_vals_vec.apply_filters(filters);

    // At this point, entries have been deduped, filters have been applied
    // and path_vals should contain all matching SignPaths,
    // and err_vals should contain err strings for problems creating SignPaths
    (path_vals_vec, err_vals)
}

/// parsng for --only-exclude and --only-include
fn parse_only_include_exclude(value: &str, delimiter: &str) -> Vec<OsString> {
    // split using delimiter nice
    value.split(delimiter)
        .map(|v| OsString::from(v))
        .collect::<Vec<OsString>>()
}

/// parsing for --dir-with-depth 3::/some_dir/
fn parse_dir_with_depth(value: &str, delimiter: &str) -> Result<(u8, OsString), String> {
    if let Some((depth_str, dir)) = value.split_once(delimiter) {
        match depth_str.parse::<u8>() {
            Ok(depth) => Ok((depth, OsString::from(dir))),
            Err(_) => Err(format!("Invalid depth value: '{}'", depth_str)),
        }
    } else {
        Err(format!(
            "Value must be in the format <depth>{}<dir>. Example: 3{}some_dir",
            delimiter, delimiter
        ))
    }
}


// workspace ignore
//
// ========== SIGN (add an entry to a cargo.toml etc)
// == `sigrs sign`
// ==== `--identity "x"` / `-id "x"`    : The lookup key in storage structure "some-key"
// ==== `--file "x"` / `-f "x"`         : The file to add the signature to "/path/to/Cargo.toml"
// ==== `--dir_recursive "x"`           : If provided, recursively searches "x" for files of type
//                                      : Cargo.toml / package.json etc. and adds signature to them
//                                      : Note: This probably needs an --ignore param too,
//                                      : or a `--file_type "Cargo.toml"` to only sign Cargo.toml's &&
//                                      : ignore package.json's etc.
// - When user calls `sign`:
// - If usernames && emails both have only 1 entry for `--identity "some-key"` =>
// --- `sigrs sign --identity "some-key"` will use those values (defaults)
// - If usernames || emails have over 1 entry =>
// --- The "default" value will be stored at index 0, so user
// ------ can call `sigrs sign --identity "some-key" --default` to use defaults @ idx 0
// --- If user calls `sigrs sign --identity "some-key"` without --default,
// ------ the user gets prompted to select a username, then select an email
// MUST
// sigrs sign --identity "key"
//    no --email or no --username   =  use default of both
//    automatically uses default (index 0)
//
// GREAT - Shouldn't force user to user LAS, signing features are useful even without it
// sigrs sign --id "key" --email "custom_email@x.com" --username "custom"
//    Uses a one-off email/username passed in as the argument
// ===============================================================
// Initial 3 path splits:
//
// 1) `sigrs sign`
//    No identity key passed, prompt user for identity, username, email
//
// 2) `sigrs sign --id "some key"`
//    Only identity passed, user default username/email (index 0)
//
// 3.1) `sigrs sign --id "key" --email "x@y.com"
//      Uses default username for "key", and one-off email that user passed in
//
// 3.2) `sigrs sign --email "x@y.com" --username "hi"`
//      Uses one-off email & username, doesn't need to access LAS
//
// `sigrs sign ... --file "./Cargo.toml"`
//     Direct path to 1+ configuration files
// 
//
// ===== Finished ========================
// ===== File filtering =======
// [ A, B ] --  A is arg parsing, B is actually filtering logic
// [ X, X ] --file               | direct paths
// [ X, X ] --only-include       | only include specific types of config files
// [ X, X ] --only-exclude       | only exclude specific types of config files
// [ X, X ] --if-signable        | only include signable files
// [ X, X ] --if-has-signatures  | only include files already signed by others
//
// ===== Directory Filtering
// All finished


//
// PROBLEM:
// the "if-signable" filters have to read the file,
// and I'll then be opening another file handle after already having one
// this is not optimal
//
//
// IDEAS:
// 1) Assume that a read during checks for if-signable is fast enough, leave it as is
//
// 2)
// Perhaps another way would be to open file handles on each SignPath BEFORE filtering
// them, if the file doesn't match a filter, I drop the file handle at that point
//
// either way I'm going to have to open all files that matched directory filters
//
// HOWEVER
//
// With option 1, I can do this:
// - Make the FileFilter types sortable, where all other filters come before 
//   if-signable (or any other filters I create that require reading file contents)
//
// - Then I sort the FileFilters before using them on apply_filters
//
// - This way, I will at least be filtering out Files that don't match all other filters
//   first before reading the file
//
// -!!- However, if I do end up adding more filters that need to read file content,
//      then at that point I'll have to figure something out so I'm not re-reading 
//      the same files over and over
//
// -!!- Accepting this trade off for now
//
// === Options for finding files to sign ===
//
// MUST HAVE
// --file "./Path/To/Cargo.toml"    | absolute path to file(s) to sign
// --file "./Cargo.toml"           | should also accept relative right?
//                                 | yes because this is most straight forward usage
//
// SECTION 2)  Choosing which directories to search
//
// --working-dir
// Search top level current working directory, non-recursive
//
// --working-dir-recursive
// Search current working directory and every directory within it
//
// NEEDS WORK
// --working-dir-only --include "Cargo.toml" --include "package.json"
//                    --include-path "/path/to/cargo.toml"
//
// Recursively searches current working dir (dir that `sigrs sign` was called from)
// and finds config files matching `--include` args && signs them
// --include  | includes all configs of the given type ("Cargo.toml" etc.)
// --include-path | includes only the path 
//
//
// --working-dir-all --exclude "Cargo.toml" --exclude-path "/path/to/package.json"
//
// Recursively searches working dir && signs all config files EXCEPT `--exclude` args
//
//
// GOOD
// --only-within 3 --include "Cargo.toml" --include "Package.json"
// Finds & signs only `--included` config types within 3 of working dir
//
// --all-within 3 --exclude "Cargo.toml" --exclude "package.json"
// Finds & signs all config types within 3, excluding the `--excluded`
//
// GOOD
// --nearest "Cargo.toml" --max-distance 3
// Finds & signs the nearest Cargo.toml config file (within 3)
