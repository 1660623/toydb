use serde_derive::{Deserialize, Serialize};

/// Errors, all except Internal are considered user-facing
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum Error {
    Config(String),
    Internal(String),
    Parse(String),
    ReadOnly,
    Serialization,
    Value(String),
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Error::Config(s) | Error::Internal(s) | Error::Parse(s) | Error::Value(s) => {
                write!(f, "{}", s)
            }
            Error::Serialization => write!(f, "Serialization failure, retry transaction"),
            Error::ReadOnly => write!(f, "Read-only transaction"),
        }
    }
}

impl From<config::ConfigError> for Error {
    fn from(err: config::ConfigError) -> Self {
        Error::Config(err.to_string())
    }
}

impl From<log::ParseLevelError> for Error {
    fn from(err: log::ParseLevelError) -> Self {
        Error::Config(err.to_string())
    }
}

impl From<log::SetLoggerError> for Error {
    fn from(err: log::SetLoggerError) -> Self {
        Error::Config(err.to_string())
    }
}

impl From<regex::Error> for Error {
    fn from(err: regex::Error) -> Self {
        Error::Value(err.to_string())
    }
}

impl From<rmp_serde::decode::Error> for Error {
    fn from(err: rmp_serde::decode::Error) -> Self {
        Error::Internal(err.to_string())
    }
}

impl From<rmp_serde::encode::Error> for Error {
    fn from(err: rmp_serde::encode::Error) -> Self {
        Error::Internal(err.to_string())
    }
}

impl From<rustyline::error::ReadlineError> for Error {
    fn from(err: rustyline::error::ReadlineError) -> Self {
        Error::Internal(err.to_string())
    }
}

impl From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Self {
        Error::Internal(err.to_string())
    }
}

impl From<std::net::AddrParseError> for Error {
    fn from(err: std::net::AddrParseError) -> Self {
        Error::Internal(err.to_string())
    }
}

impl From<std::num::ParseFloatError> for Error {
    fn from(err: std::num::ParseFloatError) -> Self {
        Error::Parse(err.to_string())
    }
}

impl From<std::num::ParseIntError> for Error {
    fn from(err: std::num::ParseIntError) -> Self {
        Error::Parse(err.to_string())
    }
}

impl<T> From<std::sync::PoisonError<T>> for Error {
    fn from(err: std::sync::PoisonError<T>) -> Self {
        Error::Internal(err.to_string())
    }
}

impl From<tokio::task::JoinError> for Error {
    fn from(err: tokio::task::JoinError) -> Self {
        Error::Internal(err.to_string())
    }
}

impl From<tokio::sync::mpsc::error::TryRecvError> for Error {
    fn from(err: tokio::sync::mpsc::error::TryRecvError) -> Self {
        Error::Internal(err.to_string())
    }
}

impl From<tokio::sync::oneshot::error::RecvError> for Error {
    fn from(err: tokio::sync::oneshot::error::RecvError) -> Self {
        Error::Internal(err.to_string())
    }
}

impl<T> From<tokio::sync::mpsc::error::SendError<T>> for Error {
    fn from(err: tokio::sync::mpsc::error::SendError<T>) -> Self {
        Error::Internal(err.to_string())
    }
}
