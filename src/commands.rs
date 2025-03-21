use tauri::{AppHandle, Runtime, command};

use crate::MachineUidExt;
use crate::Result;
use crate::models::*;

#[command]
pub(crate) async fn get_machine_uid<R: Runtime>(app: AppHandle<R>) -> Result<MachineUidResponse> {
    app.machine_uid().get_machine_uid()
}
