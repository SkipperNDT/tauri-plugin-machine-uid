import SwiftRs
import Tauri
import UIKit
import WebKit

class MachineUidPlugin: Plugin {
  @objc public func get_machine_uid(_ invoke: Invoke) throws {
    if let uuid = UIDevice.current.identifierForVendor?.uuidString {
      invoke.resolve(["id": uuid])
    } else {
      invoke.reject("Could not retrieve device identifier")
    }
  }
}

@_cdecl("init_plugin_machine_uid")
func initPlugin() -> Plugin {
  return MachineUidPlugin()
}