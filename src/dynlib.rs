use std::{io::Write, sync::OnceLock};

use tempfile::NamedTempFile;

use crate::{GameContext, gamemaker::Code, primitives::CsString};

// FFI definitions ------>
#[repr(C)]
pub struct ReturnValue {
    pub string: CsString,
    pub error: u8,
}

type DecompileFn = extern "C" fn(*const GameContext, *const Code) -> ReturnValue;
type FreeCsStringFn = extern "C" fn(*const u8);
// <------- FFI definitions

const DYN_LIB_DATA: &[u8] = include_bytes!(concat!(env!("OUT_DIR"), "/dynlib"));

struct ExternFns {
    decompile: DecompileFn,
    free_cs_string: FreeCsStringFn,
    _lib: libloading::Library,
}

fn load_externs() -> Result<ExternFns, String> {
    let mut file = NamedTempFile::new()
        .map_err(|e| format!("Could not create temporary file for dynamic library: {e}"))?;
    file.write_all(DYN_LIB_DATA)
        .map_err(|e| format!("Could not write data to dynamic library tempfile: {e}"))?;

    let lib = unsafe {
        libloading::Library::new(file.path())
            .map_err(|e| format!("Failed to load Underanalyzer dynamic library: {e}"))?
    };

    let decompile: libloading::Symbol<DecompileFn> = unsafe {
        lib.get(b"decompile_to_string")
            .map_err(|e| format!("Failed to load decompile_to_string: {e}"))?
    };

    let free_cs_string: libloading::Symbol<FreeCsStringFn> = unsafe {
        lib.get(b"free_cs_string")
            .map_err(|e| format!("Failed to load free_cs_string: {e}"))?
    };

    Ok(ExternFns {
        decompile: *decompile,
        free_cs_string: *free_cs_string,
        _lib: lib,
    })
}

fn force_load_externs() -> ExternFns {
    load_externs().expect("Could not load dynamic library")
}

pub fn init_externs() -> libgm::Result<()> {
    use libgm::error::Context;
    if EXTERNS.get().is_some() {
        return Ok(());
    }
    let ext = load_externs().context("loading dynamic library")?;
    let _ = EXTERNS.set(ext);
    Ok(())
}

static EXTERNS: OnceLock<ExternFns> = OnceLock::new();

pub fn decompile_to_string(game_context: *const GameContext, code: *const Code) -> ReturnValue {
    let ext = EXTERNS.get_or_init(force_load_externs);
    (ext.decompile)(game_context, code)
}

pub fn free_cs_string(ptr: *const u8) {
    let ext = EXTERNS.get_or_init(force_load_externs);
    (ext.free_cs_string)(ptr);
}
