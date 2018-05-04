use std::fmt;
use std::fmt::Display;
use std::result;
use failure::{
    Fail,
    Context,
    Backtrace,
    Error,
};

#[derive(Debug)]
pub struct CompismError {
    inner: Context<CompismErrorKind>,
}

#[derive(Debug, Fail, Clone, Eq, PartialEq, Copy)]
pub enum CompismErrorKind {
    #[fail(display = "No such route point")]
    NoSuchRoutePoint,
    #[fail(display = "Invalid mount path")]
    MountPathInvalid,
    #[fail(display = "Invalid route point path")]
    InvalidRoutePath,
    #[fail(display = "Option got a None result")]
    OptionNoneError,
    #[fail(display = "Unable to load plugin library")]
    UnableLoadLibError,
}

impl Fail for CompismError {
    fn cause(&self) -> Option<&Fail> {
        self.inner.cause()
    }

    fn backtrace(&self) -> Option<&Backtrace> {
        self.inner.backtrace()
    }
}

impl Display for CompismError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        Display::fmt(&self.inner, f)
    }
}

impl CompismError {
    pub fn kind(&self) -> CompismErrorKind {
        *self.inner.get_context()
    }
}

impl From<CompismErrorKind> for CompismError {
    fn from(kind: CompismErrorKind) -> CompismError {
        CompismError {
            inner: Context::new(kind)
        }
    }
}

impl From<Context<CompismErrorKind>> for CompismError {
    fn from(inner: Context<CompismErrorKind>) -> CompismError {
        CompismError {
            inner: inner
        }
    }
}

pub type Result<T> = result::Result<T, Error>;
