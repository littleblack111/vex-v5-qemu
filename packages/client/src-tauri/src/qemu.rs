use std::{borrow::BorrowMut, path::PathBuf};

use serde::{Deserialize, Serialize};
use tauri::State;
use tokio::{process::Command, sync::Mutex};
use vex_v5_qemu_host::brain::{Binary, Brain};

use crate::AppState;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct QemuOptions {
    gdb: bool,
    kernel: PathBuf,
    qemu: PathBuf,
    binary: PathBuf,
    qemu_args: Vec<String>,
}

/// Kills the currently running QEMU subprocess.
#[tauri::command]
pub async fn kill_qemu(state: State<'_, Mutex<AppState>>) -> Result<(), String> {
    let mut guard = state.lock().await;
    if let Some(brain) = &mut guard.brain {
        brain
            .terminate()
            .await
            .map_err(|_| "Failed to kill QEMU process.".into())
    } else {
        Ok(())
    }
}

/// Spawns a new QEMU subprocess.
#[tauri::command]
pub async fn spawn_qemu(
    state: State<'_, Mutex<AppState>>,
    opts: QemuOptions,
) -> Result<(), String> {
    let mut guard = state.lock().await;

    let mut cmd = Command::new(opts.qemu.clone());
    cmd.borrow_mut().args(opts.qemu_args);

    let brain = Brain::new(
        cmd,
        opts.kernel,
        Binary {
            path: opts.binary,
            load_addr: 0x03800000,
        },
        None,
    )
    .map_err(|_| "Failed to start QEMU process.")?;

    guard.brain = Some(brain);

    Ok(())
}
