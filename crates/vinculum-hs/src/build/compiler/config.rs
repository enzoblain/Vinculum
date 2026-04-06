use std::path::PathBuf;

use cargo_metadata::MetadataCommand;

use super::errors::CompilerError;

pub struct HaskellConfig {
    pub cabal_file: PathBuf,
    pub exports_dir: PathBuf,
    pub foreign_library: String,
}

const FALLBACK_CABAL_FILE: &str = "haskell_fallback/haskell.cabal";
const FALLBACK_EXPORTS_DIR: &str = "haskell_fallback/app/exports";
const FALLBACK_FOREIGN_LIBRARY: &str = "lib";

pub(crate) fn load_haskell_config() -> Result<HaskellConfig, CompilerError> {
    let metadata =
        MetadataCommand::new()
            .exec()
            .map_err(|e| CompilerError::CargoMetadataReadFailed {
                reason: e.to_string(),
            })?;

    let package = metadata
        .root_package()
        .ok_or(CompilerError::NoRootPackage)?;

    let manifest_dir = PathBuf::from(&package.manifest_path)
        .parent()
        .ok_or_else(|| CompilerError::ManifestDirResolutionFailed {
            path: PathBuf::from(&package.manifest_path),
        })?
        .to_path_buf();

    let cabal_meta = package.metadata.get("cabal_file").and_then(|v| v.as_str());
    let exports_meta = package.metadata.get("exports_dir").and_then(|v| v.as_str());
    let foreign_meta = package
        .metadata
        .get("foreign_library")
        .and_then(|v| v.as_str());

    let all_defined = cabal_meta.is_some() && exports_meta.is_some() && foreign_meta.is_some();
    let none_defined = cabal_meta.is_none() && exports_meta.is_none() && foreign_meta.is_none();

    if !all_defined && !none_defined {
        return Err(CompilerError::InvalidConfigPartial);
    }

    if all_defined {
        let cabal_file = manifest_dir.join(cabal_meta.unwrap());
        let exports_dir = manifest_dir.join(exports_meta.unwrap());
        let foreign_library = foreign_meta.unwrap().to_string();

        if !cabal_file.exists() {
            return Err(CompilerError::CabalFileNotFound { path: cabal_file });
        }

        if !exports_dir.exists() {
            return Err(CompilerError::ExportsDirNotFound { path: exports_dir });
        }

        return Ok(HaskellConfig {
            cabal_file,
            exports_dir,
            foreign_library,
        });
    }

    let cabal_file = manifest_dir.join(FALLBACK_CABAL_FILE);
    let exports_dir = manifest_dir.join(FALLBACK_EXPORTS_DIR);
    let foreign_library = FALLBACK_FOREIGN_LIBRARY.to_string();

    if !cabal_file.exists() {
        return Err(CompilerError::FallbackCabalNotFound { path: cabal_file });
    }

    if !exports_dir.exists() {
        return Err(CompilerError::FallbackExportsDirNotFound { path: exports_dir });
    }

    println!(
        "cargo:warning=[vinculum] Using fallback Haskell config:\n  cabal_file: {}\n  exports_dir: {}\n  foreign_library: {}",
        cabal_file.display(),
        exports_dir.display(),
        foreign_library
    );

    Ok(HaskellConfig {
        cabal_file,
        exports_dir,
        foreign_library,
    })
}
