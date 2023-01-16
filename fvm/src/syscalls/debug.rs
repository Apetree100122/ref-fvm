// Copyright 2021-2023 Protocol Labs
// SPDX-License-Identifier: Apache-2.0, MIT
use crate::kernel::{ClassifyResult, Result};
use crate::syscalls::context::Context;
use crate::Kernel;

use fuzzing_tracker::instrument;
#[cfg(feature="tracing")]
// Injected during build
#[no_mangle]
extern "Rust" {
    fn set_custom_probe(line: u64) -> ();
}
// Injected during build
#[no_mangle]
extern "Rust" {
    fn set_syscall_probe(probe: &'static str) -> ();
}

#[instrument()]
pub fn log(context: Context<'_, impl Kernel>, msg_off: u32, msg_len: u32) -> Result<()> {
    #[cfg(feature = "instrument-syscalls")]
    unsafe { set_syscall_probe("syscall.debug.log") };
    // No-op if disabled.
    if !context.kernel.debug_enabled() {
        return Ok(());
    }

    let msg = context.memory.try_slice(msg_off, msg_len)?;
    let msg = String::from_utf8(msg.to_owned()).or_illegal_argument()?;
    context.kernel.log(msg);
    Ok(())
}

#[instrument()]
pub fn enabled(context: Context<'_, impl Kernel>) -> Result<i32> {
    #[cfg(feature = "instrument-syscalls")]
    unsafe { set_syscall_probe("syscall.debug.enabled") };
    Ok(if context.kernel.debug_enabled() {
        0
    } else {
        -1
    })
}

#[instrument()]
pub fn store_artifact(
    context: Context<'_, impl Kernel>,
    name_off: u32,
    name_len: u32,
    data_off: u32,
    data_len: u32,
) -> Result<()> {
    #[cfg(feature = "instrument-syscalls")]
    unsafe { set_syscall_probe("syscall.debug.store_artifact") };
    // No-op if disabled.
    if !context.kernel.debug_enabled() {
        return Ok(());
    }

    let data = context.memory.try_slice(data_off, data_len)?;
    let name = context.memory.try_slice(name_off, name_len)?;
    let name =
        std::str::from_utf8(name).or_error(fvm_shared::error::ErrorNumber::IllegalArgument)?;

    context.kernel.store_artifact(name, data)?;

    Ok(())
}
