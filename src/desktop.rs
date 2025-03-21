use serde::de::DeserializeOwned;
use tauri::{AppHandle, Runtime, plugin::PluginApi};

use crate::models::*;

pub fn init<R: Runtime, C: DeserializeOwned>(
    app: &AppHandle<R>,
    _api: PluginApi<R, C>,
) -> crate::Result<MachineUid<R>> {
    Ok(MachineUid(app.clone()))
}

/// Access to the machine-uid APIs.
pub struct MachineUid<R: Runtime>(AppHandle<R>);

impl<R: Runtime> MachineUid<R> {
    pub fn get_machine_uid(&self) -> crate::Result<MachineUidResponse> {
        Ok(MachineUidResponse {
            id: machine_uid::get().ok(),
        })
    }
}
