use std::env;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    /// For starter, to remove as code matures.
    #[error("Generic error: {0}")]
    Generic(String),

    /// For starter, to remove as code matures.
    #[error("Static error: {0}")]
    Static(&'static str),

    #[error(transparent)]
    ReqwestError(#[from] reqwest::Error),
    
    #[error(transparent)]
    EnvError(#[from]  env::VarError),
    
    #[error(transparent)]
    IO(#[from] std::io::Error),
}
