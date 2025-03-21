use tauri::{
    Manager, Runtime,
    plugin::{Builder, TauriPlugin},
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

const PLUGIN_NAME: &str = "machine-uid";

/// Extensions to [`tauri::App`], [`tauri::AppHandle`] and [`tauri::Window`] to access the machine-uid APIs.
pub trait MachineUidExt<R: Runtime> {
    fn machine_uid(&self) -> &MachineUid<R>;
    fn try_machine_uid(&self) -> Option<&MachineUid<R>>;
}

impl<R: Runtime, T: Manager<R>> crate::MachineUidExt<R> for T {
    fn machine_uid(&self) -> &MachineUid<R> {
        self.state::<MachineUid<R>>().inner()
    }
    fn try_machine_uid(&self) -> Option<&MachineUid<R>> {
        self.try_state::<MachineUid<R>>().map(|s| s.inner())
    }
}


fn builder<R: Runtime>() -> tauri_specta::Builder<R> {
    tauri_specta::Builder::new()
        .plugin_name(PLUGIN_NAME)
        .commands(tauri_specta::collect_commands![
            commands::get_machine_uid::<tauri::Wry>,
        ])
}

/// Initializes the plugin.
pub fn init<R: Runtime>() -> TauriPlugin<R> {
    let builder = builder();
    Builder::new(PLUGIN_NAME)
        .invoke_handler(builder.invoke_handler())
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

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn export_types() {
        builder::<tauri::Wry>()
            .export(
                specta_typescript::Typescript::default()
                    .bigint(specta_typescript::BigIntExportBehavior::BigInt)
                    .formatter(specta_typescript::formatter::prettier)
                    .header("// @ts-nocheck"),
                "./bindings.ts",
            )
            .expect("failed to export specta types");
    }
}
