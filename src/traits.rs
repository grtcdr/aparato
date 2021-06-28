use crate::classes::DeviceClass;
use crate::PCIDevice;
use std::path::PathBuf;

// A lot of methods are hidden due to the fact that the library
// does most of the work behind the scenes, so there isn't a
// reason for such functions to appear in the documentation.

pub trait Properties {
    /// This function returns a new instance of `PCIDevice` struct using the given `path`.
    ///
    /// ## Examples:
    /// ```
    /// use aparato::PCIDevice;
    /// use aparato::traits::*;
    ///
    /// // PCIDevice::new() can autocomplete the path to the PCIDevice
    /// // if it isn't provided.
    ///
    /// // The following statements all point to the same device.
    /// let device_1 = PCIDevice::new("00:02.0");
    /// let device_2 = PCIDevice::new("0000:00:02.0");
    /// let device_3 = PCIDevice::new("/sys/bus/pci/devices/0000:00:02.0");
    /// ```
    fn new(path: &str) -> Self;

    /// `PCIDevice::new()` calls this function to initialize the device's fields
    /// by calling several *setters*.
    ///
    /// The following are fields that `init()` sets for the caller:
    /// - `address`
    /// - `class_id`
    /// - `vendor_id`
    /// - `device_id`
    /// - `class_name`
    /// - `numa_node`
    #[doc(hidden)]
    fn init(&mut self);

    // Getters...

    /// This function returns the `PCIDevice` path.
    fn path(&self) -> PathBuf;

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

    /// This function returns the `PCIDevice` vendor name.
    fn vendor_name(&self) -> String;

    /// This function returns the `PCIDevice` device name.
    fn device_name(&self) -> String;

    /// This function returns whether the `PCIDevice` is enabled.
    fn enabled(&self) -> bool;

    /// This function returns whether the `PCIDevice` is enabled.
    fn revision(&self) -> String;

    // Setters...

    /// Set the `path` field of the `PCIDevice`.
    #[doc(hidden)]
    fn set_path(&mut self, p: PathBuf);

    /// This function sets the `address` field of the `PCIDevice`
    #[doc(hidden)]
    fn set_address(&mut self);

    /// This function sets the `device_id` field of the `PCIDevice`.
    #[doc(hidden)]
    fn set_class_id(&mut self);

    /// This function sets the `device_id` field of the `PCIDevice`.
    #[doc(hidden)]
    fn set_vendor_id(&mut self);

    /// This function sets the `device_id` field of the `PCIDevice`.
    #[doc(hidden)]
    fn set_device_id(&mut self);

    /// This function sets the `numa_node` field of the `PCIDevice`.
    #[doc(hidden)]
    fn set_numa_node(&mut self);

    /// This function sets the `class_name` field of the `PCIDevice`.
    #[doc(hidden)]
    fn set_class_name(&mut self);

    /// This function sets the `revision` field of the `PCIDevice`.
    #[doc(hidden)]
    fn set_revision(&mut self);

    /// This function sets the `enabled` field of the `PCIDevice`.
    #[doc(hidden)]
    fn set_enabled(&mut self);

    /// This function sets the `vendor_name` field of the `PCIDevice`.
    #[doc(hidden)]
    fn set_vendor_name(&mut self, name: String);

    /// This function sets the `device_name` field of the `PCIDevice`.
    #[doc(hidden)]
    fn set_device_name(&mut self, name: String);
}

pub trait Fetch {
    /// This function returns a list of available PCI devices and their information.
    fn fetch() -> Vec<PCIDevice>;

    /// This function returns a list of available PCI devices of a specific class and their information.
    fn fetch_by_class(class: DeviceClass) -> Vec<PCIDevice>;

    /// This function returns a list of available GPUs and their information.
    ///
    /// This essentially wraps `fetch_by_class(DeviceClass::DisplayController)`
    /// but masks unnecessary data from device and vendor names, for example: \
    /// - `TU117M [GeForce GTX 1650 Mobile / Max-Q]` becomes `GeForce GTX 1650 Mobile / Max-Q`
    /// - `NVIDIA Corporation` becomes `NVIDIA`
    fn fetch_gpus() -> Vec<PCIDevice>;
}
