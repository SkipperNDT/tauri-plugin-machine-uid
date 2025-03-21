use tauri::{AppHandle, Runtime};

use crate::MachineUidExt;
use crate::Result;
use crate::models::*;

#[tauri::command]
#[specta::specta]
pub(crate) async fn get_machine_uid<R: Runtime>(app: AppHandle<R>) -> Result<MachineUidResponse> {
    app.machine_uid().get_machine_uid()
}
