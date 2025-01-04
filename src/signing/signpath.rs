use std::{
    cmp::{Ord, Ordering},
    ffi::OsStr,
    fs::File,
    path::{Path, PathBuf},
    io::{Read, Seek, SeekFrom},
};

use super::{
    rust::{
        cargo_is_signable,
        cargo_has_authors,
        sign_cargo_toml,
    },
    python::{ 
        sign_pyproject_toml,
        pyproject_toml_is_signable,
        pyproject_toml_has_authors,
    }, 
    jsts::{
        sign_package_json,
        package_json_is_signable,
        package_json_has_authors
    },
    php::{
        sign_composer_json,
        composer_json_is_signable,
        composer_json_has_authors
    },
};

pub enum SignPath {
    // https://docs.rs/cargo-util-schemas/0.7.0/cargo_util_schemas/manifest/struct.TomlPackage.html#structfield.authors
    // ------------------- Rust
    CargoToml {
        path: PathBuf,
        file: Option<File>,
        file_content: Option<String>
    },
    // ------------------- JS / TS
    PackageJson {
        path: PathBuf,
        file: Option<File>,
        file_content: Option<String>
    },
    // ------------------- PHP
    ComposerJson {
        path: PathBuf,
        file: Option<File>,
        file_content: Option<String>
    },

    // ------------------- Python
    // `pyproject.toml`
    PyProjectToml {
        path: PathBuf,
        file: Option<File>,
        file_content: Option<String>
    },
    // Will not be supporting setup.py since there is no agreed upon standard
    // for defining multiple authors. Can add support in the future if possible,
    // but note that this has been stale since 2009, see
    // https://github.com/python/cpython/issues/51241
    // `setup.py`
    // SetupPy {
    //     path: PathBuf,
    //     file: Option<File>,
    //     file_content: Option<String>
    // }
}


impl SignPath {

    /// Create a new SignPath, does not open File handle to path,
    /// `self.file` = None & self.file_content = None
    pub fn new(fpath: PathBuf) -> Result<SignPath, String> {
        if fpath.is_dir() {
            return Err(String::from("Expected file path, received dir path"));
        }

        let Some(file_name): Option<&OsStr> = fpath.file_name() else {
            return Err(String::from("Invalid file name"));
        };
        // cannot convert from OsStr because Windows paths/strings are utf-16
        // actually this shouldn't matter until I'm allowing custom config types,
        // since Cargo.toml etc can all be converted to valid utf-8
        let ret: Self = match file_name.to_str() {
            Some("Cargo.toml") => {
                Self::CargoToml {
                    path: fpath,
                    file: None,
                    file_content: None
                }
            },
            Some("package.json") => {
                Self::PackageJson {
                    path: fpath,
                    file: None,
                    file_content: None
                }
            },
            Some("composer.json") => {
                Self::ComposerJson {
                    path: fpath,
                    file: None,
                    file_content: None
                }
            },
            Some("pyproject.toml") => {
                Self::PyProjectToml {
                    path: fpath,
                    file: None,
                    file_content: None
                }
            },
            Some(_) | None => panic!("TODO: Handle nonmatching config file names? or err")
        };

        Ok(ret)
    }

    pub fn get_path<'s>(&'s self) -> &'s Path {
        let (path, _maybe_file) = match self {
            Self::CargoToml { path, file, .. }
            | Self::PackageJson { path, file, .. }
            | Self::ComposerJson { path, file, .. }
            | Self::PyProjectToml { path, file, .. } => (path, file)
        };
        path
    }

    pub fn get_file<'s>(&'s self) -> &'s Option<File> {
        let (_path, maybe_file) = match self {
            Self::CargoToml { path, file, .. }
            | Self::PackageJson { path, file, .. }
            | Self::ComposerJson { path, file, .. }
            | Self::PyProjectToml { path, file, .. } => (path, file)
        };
        maybe_file
    }

    /// Add file handle && file contents to file
    pub fn add_file(&mut self, f: File, fc: String) {
        let (_path, maybe_file, maybe_file_content) = match self {
            Self::CargoToml { path, file, file_content }
            | Self::PackageJson { path, file, file_content }
            | Self::ComposerJson { path, file, file_content}
            | Self::PyProjectToml { path, file, file_content } => (path, file, file_content)
        };

        if maybe_file.is_none() {
            *maybe_file = Some(f);
        }

        if maybe_file_content.is_none() {
            *maybe_file_content = Some(fc);
        }
    }

    /// Opens file @ self.path && adds it to self.file: Some(File)
    /// Does nothing if self.file is already Some(f)
    /// Errors on problem opening handle
    pub fn open_file_handle<'s>(&'s mut self) -> Result<&'s mut Self, String> {
        let (path, maybe_file, maybe_file_content) = match self {
            Self::CargoToml { path, file, file_content }
            | Self::PackageJson { path, file, file_content }
            | Self::ComposerJson { path, file, file_content}
            | Self::PyProjectToml { path, file, file_content } => (path, file, file_content)
        };

        if maybe_file.is_some() {
            return Ok(self);
        }

        let mut f = File::options()
            .read(true)
            .write(true)
            .open(&mut *path)
            .map_err(|e| format!("Problem opening '{:?}': {:?}", path, e))?;

        let mut contents = String::new();
        let Ok(_) = f.seek(SeekFrom::Start(0)) else {
            return Err(String::from("Problem seeking start of file"));
        };
        let Ok(_) = f.read_to_string(&mut contents) else {
            return Err(String::from("Problem reading file"));
        };

        *maybe_file = Some(f);
        *maybe_file_content = Some(contents);

        Ok(self)
    }


    /// Signs this SignPath's path with a Username_bytes && Author_bytes
    /// ! Sign cannot require self.file to exist, because it wont if user
    /// didn't use --is-signable filters etc.
    pub fn sign(&mut self, uname: &[u8], email: &[u8]) -> Result<String, String> {
        match self {
            // Each of these methods should return an Ok("file_x was signed") msg
            Self::CargoToml { path, file, file_content } => {
                // sign_cargo_toml(file, uname, email)
                sign_cargo_toml(path, file, file_content, uname, email)
            },

            Self::PackageJson { path, file, file_content } => {
                sign_package_json(path, file, file_content, uname, email)
            },

            Self::ComposerJson { path, file, file_content } => {
                sign_composer_json(path, file, file_content, uname, email)
            },

            Self::PyProjectToml { path, file, file_content } => {
                sign_pyproject_toml(path, file, file_content, uname, email)
            },
        }

    }

    /// Checks if self same variant as other
    pub fn same_variant_as(&self, other: &Self) -> bool {
        matches!( 
            (self, other),
            (Self::CargoToml {..}, Self::CargoToml {..}) |
            (Self::PackageJson {..}, Self::PackageJson {..}) |
            (Self::ComposerJson {..}, Self::ComposerJson {..}) |
            (Self::PyProjectToml {..}, Self::PyProjectToml {..})
        )
    }

    /// If sign_path is signable, 
    /// --- appends File handle to self.file
    /// --- appends file content to self.file_content
    /// --- returns true
    /// else
    /// --- returns false
    pub fn is_signable(&mut self) -> bool {

        // Read file to check if signable
        // If it is signable, assign self.file = Some(opened file) && return true
        // Otherwise, drop file && return false
        let Ok(mut file) = File::options()
            .read(true).write(true)
            .open(self.get_path()) else 
        {
            return false;
        };

        // TODO: Fix so that I use existing self.file if it exists
        // then do same for has_signatures
        // Needed in future if I have filters other than is-signable/has-signatures which need read
        // access to the file

        // Read file to check if signable (field present)
        // if they are going to return true,
        // this adds file && file content to self
        match self {
            Self::CargoToml {..} => {
                cargo_is_signable(self, file)
            },

            Self::PackageJson {..} => {
                package_json_is_signable(self, file)
            },

            Self::ComposerJson {..} => {
                composer_json_is_signable(self, file)
            },

            Self::PyProjectToml { .. } => {
                pyproject_toml_is_signable(self, file)
            },
        }
    }

    /// If sign_path has signatures
    /// --- appends File handle to self.file
    /// --- appends file content to self.file_content
    /// --- returns true
    /// else 
    /// -- returns false
    pub fn has_signatures(&mut self) -> bool {

        let Ok(mut file) = File::options()
            .read(true).write(true)
            .open(self.get_path()) else {
            return false;
        };

        // Read file to check if signable && has 1+ authors already
        // if they are going to return true,
        // this adds file && file content to self
        match self {
            Self::CargoToml {..} => {
                cargo_has_authors(self, file)
            },

            Self::PackageJson {..} => {
                package_json_has_authors(self, file)
            },

            Self::ComposerJson {..} => {
                composer_json_has_authors(self, file)
            },

            Self::PyProjectToml { .. } => {
                pyproject_toml_has_authors(self, file)
            },
        }

    }
}

// ========================== Comparison / ordering
impl<'s> PartialEq for SignPath
{
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            // equal if variant && path are equal
            (
                SignPath::CargoToml { path, .. },
                SignPath::CargoToml { path: other_path, ..},
            )
            | (
                SignPath::PackageJson { path, .. },
                SignPath::PackageJson { path: other_path, ..},
            )
            | (
                SignPath::ComposerJson { path, .. },
                SignPath::ComposerJson { path: other_path, ..},
            )
            | (
                SignPath::PyProjectToml { path, .. },
                SignPath::PyProjectToml { path: other_path, .. }
            ) => {
                path.eq(other_path)
            },
            _ => false,
        }
    }
}


impl<'s> Eq for SignPath {}

impl<'s> PartialOrd for SignPath
{
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<'s> Ord for SignPath
{
    fn cmp(&self, other: &Self) -> Ordering {
        // basic get order closure
        let get_var_order = |variant: &SignPath| match variant {
            SignPath::CargoToml { .. } => 0,
            SignPath::PackageJson { .. } => 1,
            SignPath::ComposerJson { .. } => 2,
            SignPath::PyProjectToml { .. } => 3,
        };

        let self_var_order = get_var_order(self);
        let other_var_order = get_var_order(other);

        // compare variant then path
        self_var_order
            .cmp(&other_var_order)
            .then_with(|| {
                match (self, other) {
                    (
                        SignPath::CargoToml { path, .. },
                        SignPath::CargoToml { path: other_path, .. },
                    )
                    | (
                        SignPath::PackageJson { path, .. },
                        SignPath::PackageJson { path: other_path, .. },
                    )
                    | (
                        SignPath::ComposerJson { path, .. },
                        SignPath::ComposerJson { path: other_path, .. },
                    )
                    | (
                        SignPath::PyProjectToml { path, .. },
                        SignPath::PyProjectToml { path: other_path, .. },
                    ) => {
                        path.cmp(other_path)
                    },
                    // variants already checked
                    _ => Ordering::Equal,
                }
            })
    }
}

