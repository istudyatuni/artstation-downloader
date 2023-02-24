pub type Result<T, E = CrateError> = std::result::Result<T, E>;

#[derive(thiserror::Error, Debug)]
pub enum CrateError {
    #[error("string template parse error: {0}")]
    FarError(#[from] far::Errors),
    // #[error("make template error: {0}")]
    // TemplateError(#[from] nanotemplate::TemplateError),
    #[error("reqwest error: {0}")]
    ReqwestError(#[from] reqwest::Error),

    #[error("invalid url: {0}")]
    InvalidURL(String),
    #[error("unsupported url: {0}")]
    UnsupportedURL(String),

    #[error("invalid pattern: {0}")]
    InvalidPattern(String),

    #[error("error: {0}")]
    /// Variant for all other errors
    Plain(String),
}
