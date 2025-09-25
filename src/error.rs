use thiserror::Error;

#[derive(Error, Debug)]
pub enum ObolError {
    #[error("network error: {0}")]
    Network(#[from] reqwest::Error),

    #[error("no symbols provided")]
    NoSymbols,

    #[error("unexpected API response shape")]
    BadApiShape,

    #[error("unknown symbol(s): {0}")]
    UnknownSymbols(String),

    #[error("failed to render output: {0}")]
    Render(String),
}

pub type Result<T> = std::result::Result<T, ObolError>;
