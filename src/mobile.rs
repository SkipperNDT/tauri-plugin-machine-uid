use serde::de::DeserializeOwned;
use tauri::{
    AppHandle, Runtime,
    plugin::{PluginApi, PluginHandle},
};

use crate::models::*;

#[cfg(target_os = "ios")]
tauri::ios_plugin_binding!(init_plugin_machine_uid);

// initializes the Kotlin or Swift plugin classes
pub fn init<R: Runtime, C: DeserializeOwned>(
    _app: &AppHandle<R>,
    api: PluginApi<R, C>,
) -> crate::Result<MachineUid<R>> {
    #[cfg(target_os = "android")]
    let handle = api.register_android_plugin("com.plugin.machine_uid", "MachineUidPlugin")?;
    #[cfg(target_os = "ios")]
    let handle = api.register_ios_plugin(init_plugin_machine_uid)?;
    Ok(MachineUid(handle))
}

/// Access to the machine-uid APIs.
pub struct MachineUid<R: Runtime>(PluginHandle<R>);

impl<R: Runtime> MachineUid<R> {
    pub fn get_machine_uid(&self) -> crate::Result<MachineUidResponse> {
        self.0
            .run_mobile_plugin("get_machine_uid", ())
            .map_err(Into::into)
    }
}
