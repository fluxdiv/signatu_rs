#![allow(non_snake_case, dead_code)]
use anyhow::Result;
use std::collections::BTreeMap;
use std::fmt::Debug;

// UNSAFE WARNING
// This data type holds file-backed memorymaps which are inherently unsafe
//
// The reason it works is because whenever this is used, the file's are
// only read-only access
//
// -- memmapped files are fine when reading only (list-all etc.)
// -- as long as nothing else on the host OS accesses/modifies the file while reading
// -- list-all etc. should be fast executing and only need this access
// ---- for a very short period of time, so very short period of time in between start
// ---- and finish of a list-all call
// HOWEVER
// - This WILL cause UD if there's a write to the file while I hold a reference
// --- to the memmapped file
// - So I need to ensure that all references are dropped before writing
#[derive(Clone, Debug)]
pub struct IdentityPerf<'LASMemoryMap> {
    pub usernames: Vec<&'LASMemoryMap [u8]>,
    pub emails: Vec<&'LASMemoryMap [u8]>
}

impl<'LASMemoryMap> IdentityPerf<'LASMemoryMap> {
    // GAT maybe?
    // type RefType<'a> = &'a T where T: 'a;

    pub fn new() -> Self {
        Self {
            usernames: Vec::new(),
            emails: Vec::new()
        }
    }

    pub fn add_username(&mut self, username: &'LASMemoryMap [u8]) -> () {
        self.usernames.push(username);
    }

    pub fn add_email(&mut self, email: &'LASMemoryMap [u8]) -> () {
        self.emails.push(email);
    }
}


#[derive(Debug)]
pub struct LocalAuthorStoragePerf<'LASMemoryMap> {
    pub memmap: &'LASMemoryMap [u8],
    // Boxed Identity contains only references to Memmap, not re-allocating
    // each portion of Memmap on heap
    pub identities: BTreeMap<&'LASMemoryMap [u8], Box<IdentityPerf<'LASMemoryMap>>>
}

impl<'LASMemoryMap> LocalAuthorStoragePerf<'LASMemoryMap> {
    pub fn new(memmap: &'LASMemoryMap [u8]) -> Self {
        Self {
            memmap,
            identities: BTreeMap::new()
        }
    }

    pub fn add_identity(
        &mut self,
        key: &'LASMemoryMap [u8], 
        identity: &IdentityPerf<'LASMemoryMap>
    ) -> Result<(), String> {
        if self.identities.contains_key(key) {
            Err(String::from("Key already exists"))
        } else {
            // Allocating IdentityPerf on heap via box, but allocation
            // only contains references to Memmap
            self.identities.insert(key, Box::new(identity.clone()));
            Ok(())
        }
    }

    pub fn lookup_id(&self, key: &String) -> Option<(&&[u8], &Box<IdentityPerf>)> {
        self.identities.get_key_value(key.as_bytes())
    }

    pub fn print_any_match(&self, usernames: &Vec<String>, emails: &Vec<String>) {

        let mut count = 0;

        for (k, v) in &self.identities {
            // if v.usernames contains any value in emails print
            for uname in usernames.iter() {
                if v.usernames.contains(&uname.as_bytes()) {
                    self.pretty_print(k, v, true);
                    count += 1;
                }
            }
            // If v.emails contains any email in emails
            // print. TODO fix dupes
            for email in emails.iter() {
                if v.emails.contains(&email.as_bytes()) {
                    self.pretty_print(k, v, true);
                    count += 1;
                }
            }
        }

        if count == 0 {
            println!("No matching identities found");
        }
    }

    pub fn pretty_print_all(&self, verbose: bool) {
        for (k, v) in &self.identities {
            self.pretty_print(k, v, verbose);
        }
    }

    pub fn pretty_print_id(&self, key: &String, verbose: bool) -> Result<(), String> {
        if let Some(identity) = self.lookup_id(key) {
            self.pretty_print(identity.0, identity.1, verbose);
            Ok(())
        } else {
            Err(format!("Identity \"{}\" not found", key))
        }
    }

    pub fn pretty_print(&self, k: &[u8], v: &Box<IdentityPerf>, verbose: bool) {
        if verbose == true {
            // Print identity key
            println!("================================");
            println!("Identity: \"{}\"", String::from_utf8_lossy(k));
            println!("--------------------------------");

            // Print usernames
            println!("Usernames:");
            println!("  Default: \"{}\"", String::from_utf8_lossy(v.usernames[0]));
            // if v.usernames.len() > 1 {
            println!("  All:");
            for uname in &v.usernames {
                println!("    - \"{}\"", String::from_utf8_lossy(uname));
            }

            println!("\nEmails:");
            println!("  Default: \"{}\"", String::from_utf8_lossy(v.emails[0]));
            println!("  All:");
            for email in &v.emails {
                println!("    - \"{}\"", String::from_utf8_lossy(email));
            }

            println!("================================\n");
        } else {
            let key = format!("Identity: \"{}\"", String::from_utf8_lossy(k));
            println!("{}", key);

            // Identity: some name
            // Usernames: | Default: "fluxdiv" | "name2" "name3"
            let mut usernames = format!("| Default: \"{}\" | ", String::from_utf8_lossy(v.usernames[0]));
            for uname in &v.usernames[1..] {
                usernames.push_str("\"");
                usernames.push_str(&String::from_utf8_lossy(uname));
                usernames.push_str("\"");
            }
            println!("Usernames: {}", usernames);

            let mut emails = format!("| Default: \"{}\" | ", String::from_utf8_lossy(v.emails[0]));
            for email in &v.emails[1..] {
                emails.push_str("\"");
                emails.push_str(&String::from_utf8_lossy(email));
                emails.push_str("\"");
            }
            println!("Emails: {} \n", emails);

            println!();
        }
    }
}
