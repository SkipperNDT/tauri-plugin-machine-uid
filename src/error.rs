use serde::Serialize;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, thiserror::Error, specta::Type, Serialize)]
#[serde(rename_all = "camelCase", tag = "type", content = "content")]
pub enum Error {
    #[error("IO error: {0}")]
    Io(String),
    #[error("Mobile error: {0}")]
    PluginInvoke(String),
}

impl From<std::io::Error> for Error {
    fn from(e: std::io::Error) -> Self {
        Error::Io(e.to_string())
    }
}

#[cfg(mobile)]
impl From<tauri::plugin::mobile::PluginInvokeError> for Error {
    #[cfg(mobile)]
    fn from(e: tauri::plugin::mobile::PluginInvokeError) -> Self {
        Error::PluginInvoke(e.to_string())
    }
}