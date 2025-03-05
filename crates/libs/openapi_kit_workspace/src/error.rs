use std::fmt::Display;

#[derive(Debug)]
pub enum Error {
    IO(std::io::Error),
    InvalidConfig(serde_yml::Error),
    NoWorkspaceFound,
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::IO(error) => f.write_str(error.to_string().as_str()),
            Error::NoWorkspaceFound => f.write_str("No workspace found"),
            Error::InvalidConfig(error) => f.write_str(error.to_string().as_str()),
        }
    }
}

impl From<std::io::Error> for Error {
    fn from(value: std::io::Error) -> Self {
        Error::IO(value)
    }
}

impl From<serde_yml::Error> for Error {
    fn from(value: serde_yml::Error) -> Self {
        Error::InvalidConfig(value)
    }
}

impl std::error::Error for Error {}
