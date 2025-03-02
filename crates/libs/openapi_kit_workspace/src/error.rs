use std::fmt::Display;

#[derive(Debug)]
pub enum Error {
    IO(std::io::Error),
    NoWorkspaceFound,
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::IO(error) => f.write_str(error.to_string().as_str()),
            Error::NoWorkspaceFound => f.write_str("No workspace found"),
        }
    }
}

impl From<std::io::Error> for Error {
    fn from(value: std::io::Error) -> Self {
        Error::IO(value)
    }
}

impl std::error::Error for Error {}
