cfg_if::cfg_if! {
    if #[cfg(target_os = "linux")] {
        pub mod linux;
        pub type PCIDevice = linux::LinuxPCIDevice;

    } else {
        compile_error!("aparato does not support this platform, at least not yet.");
    }
}

pub mod classes;
mod extra;
pub mod traits;
