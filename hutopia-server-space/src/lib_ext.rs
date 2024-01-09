// Extension file
#[cfg(target_os = "windows")]
pub const LIB_EXTENSION: &str = ".dll";
#[cfg(target_os = "linux")]
pub const LIB_EXTENSION: &str = ".so";
#[cfg(target_os = "macos")]
pub const LIB_EXTENSION: &str = ".dylib";