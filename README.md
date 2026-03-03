# orion_infra

`orion_infra` is a lightweight Rust infrastructure utility crate for:

- TOML config loading/saving lifecycle
- Logger initialization based on `flexi_logger`
- Common filesystem path helpers
- Small helpers for auto-exit logging scenarios

## Installation

Add dependency in `Cargo.toml`:

```toml
[dependencies]
orion_infra = "0.2"
```

> If you are using local workspace path dependency, replace with `path = "..."`.

## Modules

- `orion_infra::config`
  - `ConfigLifecycle` trait for config lifecycle (`load/init/save/try_load`)
  - utility functions: `save_data`, `read_file`, `backup_clean`, `clear_file`
- `orion_infra::logging`
  - `LogConf`, `configure_logging`, `BoolFlag`
  - exported macro: `auto_exit_log!`
- `orion_infra::path`
  - `make_clean_path`, `ensure_path`, `make_new_path`, `get_sub_dirs`
- `orion_infra::traits`
  - `TomlStore` trait (`from_toml` / `save_toml`)

## Quick Start

### 1) Config lifecycle

```rust
use orion_infra::config::ConfigLifecycle;
use serde_derive::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
struct AppConf {
    name: String,
    workers: u16,
}

fn demo_conf() {
    let path = "output/app.toml";
    let conf = AppConf {
        name: "demo".to_string(),
        workers: 4,
    };

    conf.init(path).expect("init config");
    let loaded = AppConf::load(path).expect("load config");
    println!("loaded: {:?}", loaded);
}
```

### 2) Logging setup

```rust
use orion_infra::logging::{configure_logging, LogConf};

fn demo_log() {
    let conf = LogConf::new_console("info").rec_positon();
    configure_logging(&conf).expect("init logger");

    log::info!("service started");
}
```

### 3) Path helpers

```rust
use std::path::Path;
use orion_infra::path::{ensure_path, get_sub_dirs, make_clean_path, make_new_path};

fn demo_path() {
    let base = Path::new("output/demo");
    make_clean_path(base).expect("clean path");
    ensure_path("output/demo/sub").expect("ensure path");
    let _ = make_new_path(Path::new("output/unique"));
    let _dirs = get_sub_dirs(Path::new("output")).expect("list sub dirs");
}
```

### 4) Auto-exit log macro

```rust
use orion_infra::auto_exit_log;

fn run_task() {
    let mut flag = auto_exit_log!(
        log::info!("task success"),
        log::error!("task failed")
    );

    // ... do work

    flag.mark_suc();
}
```

## Notes

- `configure_logging` should usually be called once during process startup.
- `ConfigLifecycle::try_load` returns `Option<Self>` and logs warning on load failure.
- `make_new_path` returns error when target path already exists.

## Development

```bash
cargo check
cargo clippy -- -D warnings
cargo test
```

## License

No license file is provided in this repository yet.
