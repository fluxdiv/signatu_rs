use clap::ArgMatches;
use anyhow::Result;
use crate::memmap::*;

// UNSAFE WARNING
// See `memmap.rs` for explanations for why these functions are unsafe
//
// ========== LIST (list locally stored authors)
// == Lists all authors in local author storage
// sigrs list-all
// == List details about an identity in LAS
// sigrs list-by-id --identity "some-key"
// == List all authors with a given username, email, or both
// sigrs list-find --username "fluxdiv"
// sigrs list-find --email "xxx@email.com"
// = Prints all containing 1 or more of the options provided
// sigrs list-find --username "fluxdiv" --email "x@e.com y@e.com"

pub unsafe fn handle_list(args: &ArgMatches) -> Result<(), String> {

    let memmap_las = get_memmap(args)?;
    let las = process_las(&memmap_las);
    las.pretty_print_all(args.get_flag("verbose"));

    Ok(())
}

pub unsafe fn handle_list_by_id(args: &ArgMatches) -> Result<(), String> {

    let memmap_las = get_memmap(args)?;
    let las = process_las(&memmap_las);
    let id_key = args.get_one::<String>("id").unwrap();

    las.pretty_print_id(id_key, args.get_flag("verbose"))
}


pub unsafe fn handle_list_find(args: &ArgMatches) -> Result<(), String> {

    let usernames: Vec<String> = args
        .get_many::<String>("usernames")
        .map(|vals| vals.cloned().collect())
        .unwrap_or_else(Vec::new);

    // Extract the "emails list if available
    let emails: Vec<String> = args
        .get_many::<String>("emails")
        .map(|vals| vals.cloned().collect())
        .unwrap_or_else(Vec::new);

    if usernames.len() + emails.len() <= 0 {
        return Err(String::from("At least 1 username or email must be provided"));
    }

    let memmap_las = get_memmap(args)?;
    let las = process_las(&memmap_las);
    // print out any entry that contains one of the emails or usernames
    las.print_any_match(&usernames, &emails);

    Ok(())
}
