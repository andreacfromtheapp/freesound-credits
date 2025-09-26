use thiserror::Error;

/// Main application error wrapper
#[derive(Debug)]
pub struct AppError(anyhow::Error);

#[derive(Error, Debug)]
pub enum AppErrorKind {
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),

    #[error("File operation failed: {0}")]
    FileOperation(String),

    #[error("Sample parsing error: {0}")]
    SampleParsing(String),

    #[error("Directory access error: {0}")]
    DirectoryAccess(String),

    #[error("Invalid sample format: {0}")]
    InvalidSample(String),
}

impl AppError {
    pub fn file_op(msg: impl Into<String>) -> Self {
        Self(AppErrorKind::FileOperation(msg.into()).into())
    }

    pub fn sample_parsing(msg: impl Into<String>) -> Self {
        Self(AppErrorKind::SampleParsing(msg.into()).into())
    }

    pub fn directory_access(msg: impl Into<String>) -> Self {
        Self(AppErrorKind::DirectoryAccess(msg.into()).into())
    }

    pub fn invalid_sample(msg: impl Into<String>) -> Self {
        Self(AppErrorKind::InvalidSample(msg.into()).into())
    }
}

impl From<AppErrorKind> for AppError {
    fn from(kind: AppErrorKind) -> Self {
        Self(kind.into())
    }
}

impl From<std::io::Error> for AppError {
    fn from(err: std::io::Error) -> Self {
        Self(AppErrorKind::Io(err).into())
    }
}

impl From<anyhow::Error> for AppError {
    fn from(err: anyhow::Error) -> Self {
        Self(err)
    }
}

impl std::fmt::Display for AppError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl std::error::Error for AppError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        self.0.source()
    }
}
