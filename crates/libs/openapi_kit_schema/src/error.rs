#[derive(Debug)]
pub enum Error {
    Io(std::io::Error),
    Serde(serde_yml::Error),
}
