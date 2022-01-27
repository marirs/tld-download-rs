#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Poisoned Error")]
    Poisoned,
    #[error("{0}")]
    Regex(#[from] regex::Error),
    #[error("{0}")]
    Reqwest(#[from] reqwest::Error),
    #[error("{0}")]
    Io(#[from] std::io::Error),
    #[error("{0}")]
    Generic(String),
    #[error("Unknown Error")]
    Unknown,
}

impl<T> From<std::sync::PoisonError<T>> for Error {
    fn from(_err: std::sync::PoisonError<T>) -> Self {
        Self::Poisoned
    }
}
