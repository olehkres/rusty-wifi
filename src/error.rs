use thiserror::Error;

#[derive(Debug, Error)]
pub enum AppError {
    #[error("Thread error!")]
    Tokio(#[from] tokio::task::JoinError),
    #[error("Serde error!")]
    Serde(#[from] serde_json::Error),
    #[error("Input/output error!")]
    IO(#[from] std::io::Error),
    #[error("Native dialog error")]
    NativeDialog(#[from] native_dialog::Error),
}
