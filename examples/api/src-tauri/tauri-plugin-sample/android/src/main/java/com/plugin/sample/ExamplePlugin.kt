package com.plugin.sample

import android.app.Activity
import app.tauri.annotation.PluginMethod
import app.tauri.annotation.TauriPlugin
import app.tauri.plugin.JSObject
import app.tauri.plugin.Plugin
import app.tauri.plugin.Invoke

@TauriPlugin
class ExamplePlugin(private val activity: Activity): Plugin(activity) {
    private val implementation = Example()

    @PluginMethod
    fun ping(invoke: Invoke) {
        val value = invoke.getString("value") ?: ""
        val ret = JSObject()
        ret.put("value", implementation.pong(value))
        invoke.resolve(ret)
    }
}