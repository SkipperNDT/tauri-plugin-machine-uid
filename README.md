# Tauri Plugin machine-uid

![License](https://img.shields.io/badge/license-MIT-blue)

A Tauri plugin that provides access to unique device identifiers across different platforms.

## Installation

Add the following to your `Cargo.toml`:

```toml
[dependencies]
tauri-plugin-machine-uid = "0.1.2"
```

Add the following typescript bindings if you want:

```bash
yarn add @skipperndt/plugin-machine-uid
```

## Usage

First, register the plugin in your Tauri application:

```rust
fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_machine_uid::init())
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
```

Then you can use it in your application (if you installed the typescript bindings):

```typescript
import { commands } from '@skipperndt/plugin-machine-uid';

// Get the unique device identifier
const result = await commands.getMachineUid();
if (result.status === "ok") {
    console.log(result.data.id); // e.g. "550e8400-e29b-41d4-a716-446655440000"
}
```

The `getMachineUid()` function returns a `Result` type with the following structure:

```typescript
type Result<T, E> = 
    | { status: "ok"; data: { id: string | null } }
    | { status: "error"; error: E };
```

If you use Rust, import the MachineUidExt trait:

```rust
use tauri_plugin_machine_uid::MachineUidExt;

let id = app_handle.machine_uid().get_machine_uid().unwrap().id.unwrap();
```

## Platform Support

- Windows: Uses WMI to get the system UUID
- macOS: Uses IOKit to get the system UUID
- Linux: Uses D-Bus to get the system machine ID
- iOS: Uses UIDevice's identifierForVendor
- Android: Uses Settings.Secure.ANDROID_ID

## Contributing

PRs are welcome! Please feel free to submit issues and pull requests.

## License

MIT © Florian Stasse
