use std::fs::File;
use crate::signing::signpath::SignPath;
use std::io::{
    Read, Seek, SeekFrom
};

// -------------------------------------------------
// ------------ gemspec implementation incomplete
// -------------------------------------------------

pub fn _gemspec_is_signable(sign_path: &mut SignPath, mut file: File) -> bool {
    let mut contents = String::new();
    let Ok(_) = file.seek(SeekFrom::Start(0)) else {
        eprintln!("Problem seeking start of file");
        return false;
    };

    let Ok(_) = file.read_to_string(&mut contents) else {
        eprintln!("Problem reading file to string");
        return false;
    };

    if let Some(author_start) = contents.find("spec.authors") {
        // problem, must make sure [ comes immediately after = following spec.authors
        let Some(author_arr_start) = contents[author_start..].find('[') else {
            eprintln!("Invalid gemspec format: No opening bracket after spec.authors");
            return false;
        };
        let Some(_author_arr_end) = contents[author_arr_start..].find(']') else {
            eprintln!("Invalid gemspec format: No closing bracket after spec.authors");
            return false;
        };
        // Found valid spec.authors field, move on
    } else {
        // No spec.authors found, not signable
        return false;
    }

    if let Some(email_start) = contents.find("spec.email") {
        let Some(e_arr_start) = contents[email_start..].find('[') else {
            eprintln!("Invalid gemspec format: No opening bracket after spec.email");
            return false;
        };
        let Some(_e_arr_end) = contents[e_arr_start..].find(']') else {
            eprintln!("Invalid gemspec format: No closing bracket after spec.email");
            return false;
        };
        // author.authors and spec.email found is_signable true
        sign_path.add_file(file, contents);
        return true;
    };

    false
}

pub fn _gemspec_has_authors(sign_path: &mut SignPath, mut file: File) -> bool {

    let mut contents = String::new();
    let Ok(_) = file.seek(SeekFrom::Start(0)) else {
        eprintln!("Problem seeking start of file");
        return false;
    };
    let Ok(_) = file.read_to_string(&mut contents) else {
        eprintln!("Problem reading file to string");
        return false;
    };


    if let Some(author_start) = contents.find("spec.authors") {
        // problem, must make sure [ comes immediately after = following spec.authors
        let Some(authors_start) = contents[author_start..].find('[') else {
            eprintln!("Invalid gemspec format: No opening bracket after spec.authors");
            return false;
        };
        let Some(_authors_end) = contents[authors_start..].find(']') else {
            eprintln!("Invalid gemspec format: No closing bracket after spec.authors");
            return false;
        };

        // STOPPED HERE

    } else {
        // No spec.authors found, not signable
        return false;
    }

    if let Some(email_start) = contents.find("spec.email") {
        let Some(e_arr_start) = contents[email_start..].find('[') else {
            eprintln!("Invalid gemspec format: No opening bracket after spec.email");
            return false;
        };
        let Some(_e_arr_end) = contents[e_arr_start..].find(']') else {
            eprintln!("Invalid gemspec format: No closing bracket after spec.email");
            return false;
        };
        // author.authors and spec.email found is_signable true
        sign_path.add_file(file, contents);
        return true;
    };

    false
}
