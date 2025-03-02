pub enum Error {
    Io(std::io::Error),
    Serde(serde_yaml::Error),
}
