#![allow(dead_code)]
use std::path::PathBuf;

// ----------------------------------------------------------------------------
#[derive(Debug)]
pub enum Error {
    Logging,
    InvalidArgument {
        arg: String,
    },
    InvalidPath,
    InvalidDate,
    InvalidTime,
    InvalidPhotoId,
    InvalidCString,
    InvalidLocation,
    InvalidColorFormat,
    OpenGlLoad {
        name: String,
    },
    ShaderLoad {
        name: String,
        log: String,
    },
    FileNotFound {
        path: PathBuf,
    },
    FileRead {
        path: PathBuf,
    },
    InvalidGallery,
    InvalidScene,
    EmptyScenes,
    EmptyPhotos,
    FileIo {
        err: std::io::Error,
    },
    ParseInt {
        err: std::num::ParseIntError,
    },
    Win32 {
        code: i32,
    },
    WebP {
        err: miniwebp::Error,
    },
    Png {
        err: miniz::png_read::Error,
    },
    Serde {
        line: usize,
        column: usize,
        msg: String,
    },
}

// ----------------------------------------------------------------------------
impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let err = format!("{:?}", self);
        f.write_str(&err)
    }
}

impl std::error::Error for Error {}

// ----------------------------------------------------------------------------
impl From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Self {
        Error::FileIo { err }
    }
}

// ----------------------------------------------------------------------------
impl From<std::num::ParseIntError> for Error {
    fn from(err: std::num::ParseIntError) -> Self {
        Error::ParseInt { err }
    }
}

// ----------------------------------------------------------------------------
impl From<serde_json::Error> for Error {
    fn from(err: serde_json::Error) -> Self {
        Error::Serde {
            line: err.line(),
            column: err.column(),
            msg: err.to_string(),
        }
    }
}

// ----------------------------------------------------------------------------
#[cfg(target_os = "windows")]
impl From<windows::core::Error> for Error {
    fn from(err: windows::core::Error) -> Self {
        Error::Win32 { code: err.code().0 }
    }
}

// ----------------------------------------------------------------------------
impl From<miniwebp::Error> for Error {
    fn from(err: miniwebp::Error) -> Self {
        Error::WebP { err }
    }
}

// ----------------------------------------------------------------------------
impl From<miniz::png_read::Error> for Error {
    fn from(err: miniz::png_read::Error) -> Self {
        Error::Png { err }
    }
}

// ----------------------------------------------------------------------------
pub type Result<T> = std::result::Result<T, Error>;
