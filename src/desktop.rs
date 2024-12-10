use serde::de::DeserializeOwned;
use tauri::{plugin::PluginApi, AppHandle, Runtime};

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
  pub fn ping(&self, payload: PingRequest) -> crate::Result<PingResponse> {
    Ok(PingResponse {
      value: payload.value,
    })
  }
}
