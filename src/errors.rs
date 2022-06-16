#[derive(Debug)]
pub enum KvsError {
    #[error("failed to parse command")]
    ParseCommandError(String),
}
