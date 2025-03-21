use tauri::{AppHandle, command, Runtime};

use crate::models::*;
use crate::Result;
use crate::MachineUidExt;

#[command]
pub(crate) async fn get_machine_uid<R: Runtime>(
    app: AppHandle<R>,
) -> Result<MachineUidResponse> {
    app.machine_uid().get_machine_uid()
}
