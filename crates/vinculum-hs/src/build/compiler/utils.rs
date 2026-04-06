use std::fs;
use std::path::{Path, PathBuf};

use crate::build::compiler::errors::CompilerError;

pub(crate) fn prepare_vinculum(exports_dir: &Path) -> Result<(), CompilerError> {
    let ffi_lib_dir = Path::new(env!("CARGO_MANIFEST_DIR")).join("ffi");
    let ffi_generated_dir = Path::new(env!("CARGO_MANIFEST_DIR")).join("ffi/generated");

    let vinculum_dir = exports_dir.join("vinculum");
    let generated_dir = vinculum_dir.join("generated");

    fs::create_dir_all(&generated_dir).map_err(|e| CompilerError::DirectoryCreationFailed {
        path: generated_dir.clone(),
        reason: e.to_string(),
    })?;

    copy_file(
        &ffi_lib_dir.join("Codec.hs"),
        &vinculum_dir.join("Codec.hs"),
    )?;
    copy_file(
        &ffi_lib_dir.join("Runtime.hs"),
        &vinculum_dir.join("Runtime.hs"),
    )?;
    copy_file(
        &ffi_lib_dir.join("StubbsRTS.c"),
        &vinculum_dir.join("StubbsRTS.c"),
    )?;

    let dispatch_src = ffi_generated_dir.join("Dispatch.hs");
    if dispatch_src.exists() {
        copy_file(&dispatch_src, &generated_dir.join("Dispatch.hs"))?;
    }

    let user_functions_src = ffi_generated_dir.join("UserFunctions.hs");
    if user_functions_src.exists() {
        copy_file(&user_functions_src, &generated_dir.join("UserFunctions.hs"))?;
    }

    Ok(())
}

fn copy_file(from: &PathBuf, to: &PathBuf) -> Result<(), CompilerError> {
    if !from.exists() {
        return Err(CompilerError::FileReadFailed {
            path: from.clone(),
            reason: "file not found".to_string(),
        });
    }

    fs::copy(from, to).map_err(|e| CompilerError::FileCopyFailed {
        src: from.clone(),
        dst: to.clone(),
        reason: e.to_string(),
    })?;

    Ok(())
}
