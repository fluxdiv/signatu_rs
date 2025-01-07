use std::{
    ffi::OsString,
    path::PathBuf,
};
use super::signpath::SignPath;

pub enum SignableFilter {
    IfSignable,
    IfHasSignatures
}

pub enum ConfigTypeFilter {
    Include(Vec<OsString>),
    Exclude(Vec<OsString>)
}

// Is dynamic dispatch really worth it here?
// no probably not, but this was good practice
pub trait FileFilterApply {
    fn apply_filters<'a>(&'a mut self, filters: Vec<Box<dyn FileFilter>>) -> Self;
}

impl FileFilterApply for Vec<SignPath> {
    fn apply_filters<'a>(&'a mut self, filters: Vec<Box<dyn FileFilter>>) -> Self {
        let mut replacement = Vec::<SignPath>::new();

        for mut sign_path in self.drain(..) {
            if filters.iter().all(|filter| filter.matches(&mut sign_path)) {
                replacement.push(sign_path);
            }
        }
        replacement
    }
}

pub trait FileFilterSort {
    /// Sort a vec of FileFilters such that filters which require opening/reading
    /// files come after those that dont
    fn sort_by_access<'a>(&'a mut self) -> &'a mut Self;
}

impl FileFilterSort for Vec<Box<dyn FileFilter>> {
    fn sort_by_access<'a>(&'a mut self) -> &'a mut Self {
        self.sort_by(|a, b| a.get_ord().cmp(&b.get_ord()));
        self
    }
}

pub trait FileFilter {
    /// Helper for sorting. Filters that require file access > those that dont
    fn get_ord(&self) -> u8;

    /// Checks whether or not the sign_path matches this filter
    /// If FileFilter is a type that requires file access, it may
    /// mutate the SignPath by updating SignPath.file = Some(File),
    /// but only if the matches call will return true
    fn matches(&self, sign_path: &mut SignPath) -> bool;
}

impl FileFilter for ConfigTypeFilter {

    /// `self` here [doesn't require file access, returns 0
    fn get_ord(&self) -> u8 {
        0u8
    }

    fn matches(&self, sign_path: &mut SignPath) -> bool {
        // sign_path variant will determine each of these
        // idea 1)
        // convert every OsString in cfg_types into a SignPath (w/ no file),
        // and compare that way (easiest)
        // Short circuit when filter isn't matching
        match self {
            Self::Exclude(cfg_types) => {
                // If sign_path matches any of cfg_types, return false
                for cfg_type in cfg_types {
                    // If cfg_type is same variant, return false
                    if SignPath::new(PathBuf::from(cfg_type))
                        .is_ok_and(|sp| sp.same_variant_as(sign_path)) 
                    {
                        // `sign_path` matches one of the exclude cfg_paths,
                        // short circuit false
                        return false;
                    }
                }
                // `sign_path` config type not included within exclude cfg_paths 
                true
            },
            Self::Include(cfg_types) => {
                if cfg_types.iter()
                    .filter_map(|cfg| SignPath::new(PathBuf::from(cfg)).ok())
                    .any(|sp| sp.same_variant_as(sign_path)) 
                {
                    // `sign_path` matches one of the included cfg_paths
                    // short circuit true
                    return true;
                }
                // `sign_path` not included in include cfg_paths
                false
            }
        }
    }
}

impl FileFilter for SignableFilter {

    /// this filter requires file access, returns 255
    fn get_ord(&self) -> u8 {
        255u8
    }

    fn matches(&self, sign_path: &mut SignPath) -> bool {
        // Read file instead of memmap, need it either way
        // but hmm,
        // If I go with normal File
        // I will read the file (read&&write access), check for signable/signatures
        //
        // If it's going to return true, I can leave the file handle open &&
        // set it to SignPath.file
        //
        // If it's going to return false, I can drop the file handle && return
        // false
        //
        // This works, and if future filters require read/write access,
        // I can check sign_path.file for some before opening file
        match self {
            Self::IfSignable => {
                sign_path.is_signable()
            },
            Self::IfHasSignatures => {
                sign_path.has_signatures()

            }
        }
    }
}


