use std::fs::File;
use clap::ArgMatches;
use anyhow::Result;

use memmap2::Mmap;
use crate::identity::*;
use crate::utils::extract_config_path;


pub const DOUBLE_QUOTE_BITS: u8 = 34u8;
pub const NEW_LINE_BITS: u8 = 10u8;
pub const POUND_BITS: u8 = 35u8;
// pub const RS_DELIMITER_BITS: u8 = 0b00011110;
pub const K_BITS: u8 = 75u8;
pub const U_BITS: u8 = 85u8;
pub const E_BITS: u8 = 69u8;

/// Get Mmap from `--config-path` or dirs default
pub unsafe fn get_memmap(args: &ArgMatches) -> Result<Mmap, String> {
    let config_path = extract_config_path(args)?;

    {
        // scoped to immediately drop File handle after creating memmap
        let handle = File::open(config_path)
            .map_err(|e| e.to_string())?;

        unsafe {
            Mmap::map(&handle)
                .map_err(|e| String::from("Problem getting memory map"))
        }
    }
}

pub fn process_las<'a>(memmap: &'a [u8]) -> LocalAuthorStoragePerf<'a> {

    let mut key_buf = Vec::<&[u8]>::new();
    let mut las = LocalAuthorStoragePerf::new(&memmap);
    let mut identity_buf = IdentityPerf::new();
    let mut s = 0;

    loop {
        // find next newline in memmap_las
        if let Some(newline_pos) = memmap[s..]
            .iter()
            .position(|&b| b == NEW_LINE_BITS) {

            // get line slice of memmap
            let line_slice = &memmap[s.. s + newline_pos + 1];

            // update starting position for next line
            s += newline_pos + 1;

            // match on first byte to determine line type
            match &line_slice[0..1] {
                &[POUND_BITS] => {
                    // comment line ignore
                    continue;
                },
                &[K_BITS] => {
                    // handle key line
                    let mut cursor = 3;
                    while line_slice[cursor] != DOUBLE_QUOTE_BITS {
                        cursor += 1;
                    }
                    key_buf.push(&line_slice[3..cursor]);
                    // write keybuf
                    // key_buf.write(&line_slice[3..cursor]).unwrap();
                    continue;
                },
                &[U_BITS] => {
                    // parse usernames directly from line_slice
                    let mut cursor = 2;
                    let line_len = line_slice.len();
                    while cursor < line_len && line_slice[cursor] != NEW_LINE_BITS {
                        // move to opening double quote
                        while line_slice[cursor] != DOUBLE_QUOTE_BITS {
                            cursor += 1;
                        }

                        // skip opening double quote
                        cursor += 1;
                        let start = cursor;

                        // move to closing double quote
                        while line_slice[cursor] != DOUBLE_QUOTE_BITS {
                            cursor += 1;
                        }

                        // slice containing username
                        identity_buf.add_username(&line_slice[start..cursor]);
                        // skip closing double quote
                        cursor += 1;
                    }
                    // identity_buf has all usernames
                    continue;
                },
                &[E_BITS] => {
                    // handle emails line
                    let mut cursor = 2;
                    let line_len = line_slice.len();
                    while cursor < line_len && line_slice[cursor] != NEW_LINE_BITS {
                        // move to opening "
                        while line_slice[cursor] != DOUBLE_QUOTE_BITS {
                            cursor += 1;
                        }
                        // skip opening
                        cursor += 1;
                        let start = cursor;
                        // move to closing "
                        while line_slice[cursor] != DOUBLE_QUOTE_BITS {
                            cursor += 1;
                        }

                        // add email slice
                        identity_buf.add_email(&line_slice[start..cursor]);
                        // skip closing double quote
                        cursor += 1;
                    }

                    // after emails are done, the identity is finished, store in LAS
                    if let Some(key_slice) = key_buf.pop() {
                        las.add_identity(key_slice, &identity_buf).unwrap();
                    }

                    // clear identity buf
                    identity_buf = IdentityPerf::new();

                },
                _ => {
                    unimplemented!();
                }
            }
        } else {
            break;
        }
    };

    las
}

