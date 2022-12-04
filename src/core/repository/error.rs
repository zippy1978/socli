use std::fmt::Display;

#[derive(Debug)]
pub enum RepoError {
    Io,
    Read(String),
    Write(String),
    Other,
    Cache,
}

impl Display for RepoError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RepoError::Io => write!(f, "io"),
            RepoError::Write(msg) => write!(f, "write failed: {}", msg),
            RepoError::Other => write!(f, "data access failed"),
            RepoError::Read(msg) => write!(f, "read failed: {}", msg),
            RepoError::Cache => write!(f, "data access failed"),
        }
    }
}

impl From<reqwest::Error> for RepoError {
    fn from(_: reqwest::Error) -> Self {
        Self::Io
    }
}