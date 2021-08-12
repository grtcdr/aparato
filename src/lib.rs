//! Welcome!
//!
//! aparato is a library that can provide you with information about one or all your connected PCI devices.
//!
//! # Examples
//!
//! A quick example to get started is to run the following chunk of code:
//!
//! ```
//! use aparato::{Device, PCIDevice};
//!
//! fn main() {
//!     let device = PCIDevice::new("00:02.0");
//!     println!("{:?}", device);
//! }
//! ```
//!
//! Information about `02:00.0` should be printed to the screen.
//! There's always a chance that the address you provided to [`Device::new()`] could be non-existant, which will result
//! in an empty object being returned.
//!
//! If you're unsure what PCI device you want to query, you can let [`Fetch`] do that for you.
//! It can return a list of PCI devices with all their information.

cfg_if::cfg_if! {
    if #[cfg(target_os = "linux")] {
        pub mod linux;
        pub type PCIDevice = linux::LinuxPCIDevice;
        pub type DeviceClass = device_class::DeviceClass;
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

/// A trait that provides the necessary methods which can initialize a single PCIDevice and fetch its information.
pub trait Device {
    /// This function returns a new instance of `PCIDevice` struct using the given `path`.
    ///
    /// # Examples
    ///
    /// ```
    /// use aparato::{Device, PCIDevice};
    ///
    /// // foo, bar and baz all point to the same device.
    /// let foo = PCIDevice::new("00:04.0");
    /// let bar = PCIDevice::new("0000:00:04.0");
    /// let baz = PCIDevice::new("/sys/bus/pci/devices/0000:00:04.0");
    /// ```
    fn new(path: &str) -> Self;

    // Getters...

    /// This function returns the path.
    fn path(&self) -> std::path::PathBuf;

    /// This function returns the class identifier.
    fn class(&self) -> String;

    /// This function returns the subclass identifier.
    fn subclass(&self) -> String;

    /// This function returns the program interface identifier.
    fn prog_if(&self) -> String;

    /// This function returns the vendor ID.
    fn vendor(&self) -> String;

    /// This function returns the device ID.
    fn device(&self) -> String;

    /// This function returns the class name.
    fn class_name(&self) -> String;

    /// This function returns the subclass name.
    fn subclass_name(&self) -> String;

    /// This function returns the program interface name.
    fn prog_if_name(&self) -> String;

    /// This function returns the vendor name.
    fn vendor_name(&self) -> String;

    /// This function returns the device name.
    fn device_name(&self) -> String;

    /// This function returns whether the is enabled.
    fn enabled(&self) -> bool;

    /// This function returns the subsystem vendor.
    fn subsystem_name(&self) -> String;

    /// This function returns the subsystem vendor.
    fn subsystem_vendor(&self) -> String;

    /// This function returns the subsystem vendor.
    fn subsystem_device(&self) -> String;
}

pub(crate) mod private {
    pub(crate) trait Properties {
        /// This function sets the `path` field.
        fn set_path(&mut self, p: std::path::PathBuf);

        /// This function sets the `class` field.
        fn set_class(&mut self);

        /// This function sets the `vendor` field.
        fn set_vendor(&mut self);

        /// This function sets the `device` field.
        fn set_device(&mut self);

        /// This function sets the `enabled` field.
        fn set_enabled(&mut self);

        /// This function sets the `subsystem_vendor` field.
        fn set_subsystem_device(&mut self);

        /// This function sets the `subsystem_device` field.
        fn set_subsystem_vendor(&mut self);
    }
}

/// A trait that provides a set of methods which can fetch the information of multiple PCI devices all at once.
///
/// `Fetch` can take care of initializing PCI devices and fetching their information for you.
/// It does this by traversing the filesystem, and getting the appropriate data of each device it finds.
pub trait Fetch {
    /// This function returns a **list** of available PCI devices and their information.
    ///
    /// If anything other than `None` or `Some(0)` is provided to the function,
    /// it will limit itself to fetch only the given amount.
    ///
    /// # Examples
    /// ```
    /// use aparato::{PCIDevice, Fetch, Device};
    ///
    /// let devices = PCIDevice::fetch(Some(2));
    ///
    /// // Print to the screen the class name of the fetched PCI devices.
    /// for device in devices {
    ///     println!("{}", device.class_name());    
    /// }
    /// ```
    fn fetch(maximum_devices: Option<u8>) -> Vec<PCIDevice>;

    /// This function returns a **list** of available PCI devices of a specific class and their information.
    ///
    /// # Examples
    /// ```
    /// use aparato::{Fetch, PCIDevice, DeviceClass};
    ///
    /// fn main() {
    ///     // foo is a list of a maximum of 3 serial bus controllers.
    ///     let foo = PCIDevice::fetch_by_class(DeviceClass::SerialBusController, Some(3));
    ///
    ///     // bar is a list of bridges with no maximum size.
    ///     // we'll get as many bridges as aparato can find.
    ///     let bar = PCIDevice::fetch_by_class(DeviceClass::Bridge, Some(0));
    ///
    ///     // baz is a list of wireless controllers with no maximum size.
    ///     let baz = PCIDevice::fetch_by_class(DeviceClass::WirelessController, None);
    ///
    /// }
    /// ```
    fn fetch_by_class(
        class: crate::device_class::DeviceClass,
        maximum_devices: Option<u8>,
    ) -> Vec<PCIDevice>;

    /// This function returns a **list** of available and enabled GPUs,
    /// masking unnecessary data from device and vendor names. for example:
    /// - `TU117M [GeForce GTX 1650 Mobile / Max-Q]` becomes `GeForce GTX 1650 Mobile / Max-Q`
    /// - `NVIDIA Corporation` becomes `NVIDIA`
    ///
    /// # Examples
    /// ```
    /// use aparato::{Fetch, PCIDevice};
    ///
    /// fn main() {
    ///     // Returns a list of "device_vendor + device_name"
    ///     let devices = PCIDevice::fetch_gpus(None);
    ///
    ///     // Example output: ["NVIDIA GeForce GTX 1650 Mobile / Max-Q", "Intel UHD Graphics 620"]
    ///     println!("{:?}", devices);
    /// }
    /// ```
    fn fetch_gpus(maximum_devices: Option<u8>) -> Vec<String>;
}

pub mod device_class;
mod extra;

pub enum Error {
    Invalid,
    Other,
}