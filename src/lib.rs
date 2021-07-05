cfg_if::cfg_if! {
    if #[cfg(target_os = "linux")] {
        pub mod linux;
        pub type PCIDevice = linux::LinuxPCIDevice;
    } else if #[cfg(target_os = "macos")] {
        pub mod macos;
        pub type PCIDevice = macos::MacOSPCIDevice;
    } else if #[cfg(target_os = "netbsd")] {
        pub mod netbsd;
        pub type PCIDevice = netbsd::NetBSDPCIDevice;
    } else if #[cfg(target_os = "windows")] {
        pub mod windows;
        pub type PCIDevice = windows::WindowsPCIDevice;
    } else {
        compile_error!("aparato does not support this platform, at least not yet.");
    }
}

mod classes;
mod extra;
pub mod traits;
