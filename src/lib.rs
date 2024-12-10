use tauri::{
  plugin::{Builder, TauriPlugin},
  Manager, Runtime,
};

pub use models::*;

#[cfg(desktop)]
mod desktop;
#[cfg(mobile)]
mod mobile;

mod commands;
mod error;
mod models;

pub use error::{Error, Result};

#[cfg(desktop)]
use desktop::MachineUid;
#[cfg(mobile)]
use mobile::MachineUid;

/// Extensions to [`tauri::App`], [`tauri::AppHandle`] and [`tauri::Window`] to access the machine-uid APIs.
pub trait MachineUidExt<R: Runtime> {
  fn machine_uid(&self) -> &MachineUid<R>;
}

impl<R: Runtime, T: Manager<R>> crate::MachineUidExt<R> for T {
  fn machine_uid(&self) -> &MachineUid<R> {
    self.state::<MachineUid<R>>().inner()
  }
}

/// Initializes the plugin.
pub fn init<R: Runtime>() -> TauriPlugin<R> {
  Builder::new("machine-uid")
    .invoke_handler(tauri::generate_handler![commands::ping])
    .setup(|app, api| {
      #[cfg(mobile)]
      let machine_uid = mobile::init(app, api)?;
      #[cfg(desktop)]
      let machine_uid = desktop::init(app, api)?;
      app.manage(machine_uid);
      Ok(())
    })
    .build()
}
