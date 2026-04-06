use std::error::Error;
use std::fmt;
use std::path::PathBuf;

#[derive(Debug)]
pub enum CompilerError {
    CabalNotFound,
    PathNotSet,
    CabalBuildFailed {
        target: String,
        reason: String,
    },
    UnsupportedOS,
    LibraryNotFound {
        library: String,
        path: PathBuf,
    },
    TargetDirResolutionFailed {
        reason: String,
    },
    InvalidCabalPath {
        path: PathBuf,
    },
    DirectoryCreationFailed {
        path: PathBuf,
        reason: String,
    },
    FileCopyFailed {
        src: PathBuf,
        dst: PathBuf,
        reason: String,
    },
    CargoMetadataReadFailed {
        reason: String,
    },
    NoRootPackage,
    ManifestDirResolutionFailed {
        path: PathBuf,
    },
    InvalidConfigPartial,
    CabalFileNotFound {
        path: PathBuf,
    },
    ExportsDirNotFound {
        path: PathBuf,
    },
    FallbackCabalNotFound {
        path: PathBuf,
    },
    FallbackExportsDirNotFound {
        path: PathBuf,
    },
    FileReadFailed {
        path: PathBuf,
        reason: String,
    },
    FileWriteFailed {
        path: PathBuf,
        reason: String,
    },
    DirectoryReadFailed {
        path: PathBuf,
        reason: String,
    },
    InvalidUtf8Path {
        path: PathBuf,
    },
    LibrarySearchFailed {
        path: PathBuf,
    },
    HaskellCompilationStopped,
    Unknown {
        message: String,
    },
}

impl fmt::Display for CompilerError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CompilerError::CabalNotFound => {
                write!(f, "cabal executable not found in PATH")
            }
            CompilerError::PathNotSet => {
                write!(f, "PATH environment variable not set")
            }
            CompilerError::CabalBuildFailed { target, reason } => {
                write!(
                    f,
                    "failed to run cabal build for target '{}': {}",
                    target, reason
                )
            }
            CompilerError::UnsupportedOS => {
                write!(f, "unsupported target OS for dynamic libraries")
            }
            CompilerError::LibraryNotFound { library, path } => {
                write!(f, "library '{}' not found in {}", library, path.display())
            }
            CompilerError::TargetDirResolutionFailed { reason } => {
                write!(f, "failed to resolve target directory: {}", reason)
            }
            CompilerError::InvalidCabalPath { path } => {
                write!(f, "invalid cabal file path: {}", path.display())
            }
            CompilerError::DirectoryCreationFailed { path, reason } => {
                write!(
                    f,
                    "failed to create directory '{}': {}",
                    path.display(),
                    reason
                )
            }
            CompilerError::FileCopyFailed { src, dst, reason } => {
                write!(
                    f,
                    "failed to copy file from '{}' to '{}': {}",
                    src.display(),
                    dst.display(),
                    reason
                )
            }
            CompilerError::CargoMetadataReadFailed { reason } => {
                write!(f, "failed to read cargo metadata: {}", reason)
            }
            CompilerError::NoRootPackage => {
                write!(f, "no root package found in cargo metadata")
            }
            CompilerError::ManifestDirResolutionFailed { path } => {
                write!(
                    f,
                    "failed to get manifest directory from: {}",
                    path.display()
                )
            }
            CompilerError::InvalidConfigPartial => {
                write!(
                    f,
                    "invalid config: define ALL of `cabal_file`, `exports_dir`, `foreign_library`, or NONE to use fallbacks"
                )
            }
            CompilerError::CabalFileNotFound { path } => {
                write!(f, "cabal file does not exist: {}", path.display())
            }
            CompilerError::ExportsDirNotFound { path } => {
                write!(f, "exports directory does not exist: {}", path.display())
            }
            CompilerError::FallbackCabalNotFound { path } => {
                write!(f, "fallback cabal file does not exist: {}", path.display())
            }
            CompilerError::FallbackExportsDirNotFound { path } => {
                write!(
                    f,
                    "fallback exports directory does not exist: {}",
                    path.display()
                )
            }
            CompilerError::FileReadFailed { path, reason } => {
                write!(f, "failed to read file '{}': {}", path.display(), reason)
            }
            CompilerError::FileWriteFailed { path, reason } => {
                write!(f, "failed to write file '{}': {}", path.display(), reason)
            }
            CompilerError::DirectoryReadFailed { path, reason } => {
                write!(
                    f,
                    "failed to read directory '{}': {}",
                    path.display(),
                    reason
                )
            }
            CompilerError::InvalidUtf8Path { path } => {
                write!(f, "invalid UTF-8 in path: {}", path.display())
            }
            CompilerError::LibrarySearchFailed { path } => {
                write!(f, "failed to find library recursive in {}", path.display())
            }
            CompilerError::HaskellCompilationStopped => {
                write!(f, "haskell compilation stopped due to previous errors")
            }
            CompilerError::Unknown { message } => {
                write!(f, "unknown compiler error: {}", message)
            }
        }
    }
}

impl Error for CompilerError {}
