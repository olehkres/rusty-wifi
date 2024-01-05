use std::error::Error;

use thiserror::Error;

#[derive(Debug, Error)]
pub enum AppError {
    #[error("Thread error!")]
    Tokio(#[from] tokio::task::JoinError),
    #[error("Serialize/Deserialize error!")]
    Serde(#[from] serde_json::Error),
    #[error("Input/output error!")]
    IO(#[from] std::io::Error),
    #[error("Native dialog error")]
    NativeDialog(#[from] native_dialog::Error),
}

impl AppError {
    pub fn show(&self) {
        match native_dialog::MessageDialog::new()
            .set_title("Error!")
            .set_text(&format!(
                "Eror: \"{}\". Caused by: \"{}\"",
                self,
                self.source().unwrap()
            ))
            .show_alert()
        {
            Ok(()) => (),
            Err(e) => println!("{}", e),
        }
    }
}
