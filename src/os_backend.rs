pub const LINUX_HOSTS_PATH: &str = "/etc/hosts";
pub const MACOS_HOSTS_PATH: &str = "/etc/hosts";
pub const WINDOWS_HOSTS_PATH: &str = "C:\\Windows\\System32\\drivers\\etc\\hosts";

#[inline]
pub const fn get_hosts_path() -> &'static str {
    if cfg!(target_os = "linux") {
        LINUX_HOSTS_PATH
    } else if cfg!(target_os = "windows") {
        WINDOWS_HOSTS_PATH
    } else if cfg!(target_os = "macos") {
        MACOS_HOSTS_PATH
    } else {
        panic!("Unsupported operating system");
    }
}
