use std::path::Path;

#[cfg(target_os = "windows")]
pub fn is_executable(path:&Path) -> bool {
    // TODO: correctly check for file executability
    path.is_file()
}


#[cfg(not(target_os = "windows"))]
pub fn is_executable(path:&Path) -> bool {
    use std::os::unix::fs::PermissionsExt;
    path.is_file() && path.metadata().unwrap().permissions().mode() & 0o111 != 0
}