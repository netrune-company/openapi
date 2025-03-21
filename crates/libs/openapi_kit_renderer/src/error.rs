#[derive(Debug)]
pub enum Error {
    Render(tera::Error),
    FilterError(String),
}

impl From<tera::Error> for Error {
    fn from(value: tera::Error) -> Self {
        Error::Render(value)
    }
}
