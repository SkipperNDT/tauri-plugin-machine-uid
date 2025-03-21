package com.plugin.machine_uid

import android.app.Activity
import android.provider.Settings.Secure
import app.tauri.annotation.Command
import app.tauri.annotation.InvokeArg
import app.tauri.annotation.TauriPlugin
import app.tauri.plugin.JSObject
import app.tauri.plugin.Plugin
import app.tauri.plugin.Invoke

@TauriPlugin
class MachineUidPlugin(private val activity: Activity): Plugin(activity) {
    @Command
    fun get_machine_uid(invoke: Invoke) {
        val ret = JSObject()
        ret.put("id", Secure.getString(activity.getApplicationContext().getContentResolver(), Secure.ANDROID_ID))
        invoke.resolve(ret)
    }
}
