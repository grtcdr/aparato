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
//! There's always a chance that the address you provided to [`PCIDevice::new()`] could be non-existant, which will result
//! in an empty object being returned.
//!
//! We can complete avoid this behavior by letting [`Fetch`] handle everything for you. It'll traverse the filesystem to
//! to get only the correct and available PCI devices, and will return their information as well.

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

    /// This function returns the `PCIDevice` path.
    fn path(&self) -> std::path::PathBuf;

    /// This function returns the `PCIDevice` address.
    fn address(&self) -> String;

    /// This function returns the `PCIDevice` class ID.
    fn class_id(&self) -> String;

    /// This function returns the `PCIDevice` vendor ID.
    fn vendor_id(&self) -> String;

    /// This function returns the `PCIDevice` device ID.
    fn device_id(&self) -> String;

    /// This function returns the `PCIDevice` NUMA node.
    fn numa_node(&self) -> isize;

    /// This function returns the `PCIDevice` class name.
    fn class_name(&self) -> String;

    /// This function returns the `PCIDevice` subclass name.
    fn subclass_name(&self) -> String;

    /// This function returns the `PCIDevice` vendor name.
    fn vendor_name(&self) -> String;

    /// This function returns the `PCIDevice` device name.
    fn device_name(&self) -> String;

    /// This function returns whether the `PCIDevice` is enabled.
    fn enabled(&self) -> bool;

    /// This function returns whether the `PCIDevice` is enabled.
    fn d3cold_allowed(&self) -> bool;

    /// This function returns whether the `PCIDevice` is enabled.
    fn revision(&self) -> String;

    /// This function returns the `PCIDevice` subsystem vendor.
    fn subsystem_name(&self) -> String;

    /// This function returns the `PCIDevice` subsystem vendor.
    fn subsystem_vendor_id(&self) -> String;

    /// This function returns the `PCIDevice` subsystem vendor.
    fn subsystem_device_id(&self) -> String;
}

pub(crate) mod private {
    pub(crate) trait Properties {
        // This trait contains exclusively the setters.

        /// This function is reserved for use by some of the mods provided by `Fetcher`
        fn reserved_new(path: &str) -> Self;

        /// Set the `path` field of the `PCIDevice`.
        fn set_path(&mut self, p: std::path::PathBuf);

        /// This function sets the `address` field of the `PCIDevice`
        fn set_address(&mut self);

        /// This function sets the `device_id` field of the `PCIDevice`
        fn set_class_id(&mut self);

        /// This function sets the `device_id` field of the `PCIDevice`
        fn set_vendor_id(&mut self);

        /// This function sets the `device_id` field of the `PCIDevice`
        fn set_device_id(&mut self);

        /// This function sets the `numa_node` field of the `PCIDevice`
        fn set_numa_node(&mut self);

        /// This function sets the `class_name` field of the `PCIDevice`
        fn set_class_name(&mut self);

        /// This function sets the `subclass_name` field of the `PCIDevice`
        fn set_subclass_name(&mut self);

        /// This function sets the `revision` field of the `PCIDevice`
        fn set_revision(&mut self);

        /// This function sets the `enabled` field of the `PCIDevice`
        fn set_enabled(&mut self);

        /// This function sets the `d3cold_allowed` field of the `PCIDevice`
        fn set_d3cold_allowed(&mut self);

        /// This function sets the `vendor_name` field of the `PCIDevice`
        fn set_vendor_name(&mut self);

        /// This function sets the `device_name` field of the `PCIDevice`
        fn set_device_name(&mut self);

        /// This function sets the `subsystem_vendor_id` field of the `PCIDevice`
        fn set_subsystem_device_id(&mut self);

        /// This function sets the `subsystem_device_id` field of the `PCIDevice`
        fn set_subsystem_vendor_id(&mut self);

        /// This function sets the `subsystem_name` field of the `PCIDevice`
        fn set_subsystem_name(&mut self);
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
