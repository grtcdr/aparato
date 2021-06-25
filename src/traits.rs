use crate::{classes::DeviceClass};
use std::path::PathBuf;
use crate::PCIDevice;

pub trait Properties {
    /// This function returns a new instance of PCIDevice
    fn new(path: &str) -> Self;
    
    /// After creating a new instance of PCIDevice, `init()` should be
    /// run to fetch the data of the newly created instance.
    fn init(&mut self);

    // -v- The getters -v-
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

    // -v- The setters -v-
    /// Set the `path` field of the struct that implements it
    fn set_path(&mut self, p: PathBuf);
    
    /// This function sets the `address` field of the struct that implements it
    fn set_address(&mut self);
    
    /// This function sets the `device_id` field of the struct that implements it
    fn set_class_id(&mut self);
    
    /// This function sets the `device_id` field of the struct that implements it
    fn set_vendor_id(&mut self);
    
    /// This function sets the `device_id` field of the struct that implements it
    fn set_device_id(&mut self);

    /// This function sets the `numa_node` field of the struct that implements it
    fn set_numa_node(&mut self);
    
    /// This function sets the `class_name` field of the struct that implements it
    fn set_class_name(&mut self);
    
    /// This function sets the `vendor_name` field of the struct that implements it
    fn set_vendor_name(&mut self, name: String);
    
    /// This function sets the `device_name` field of the struct that implements it
    fn set_device_name(&mut self, name: String);
}

pub trait Fetch {
    /// This function returns a list of available PCI devices and their information.
    ///
    /// *This function returns a minute amount of information.*
    fn fetch() -> Vec<PCIDevice>;
    
    /// This function returns a list of available PCI devices with *detailed information*.
    fn fetch_detailed() -> Vec<PCIDevice>;
    
    /// This function returns a list of available PCI devices of a specific class and their information.  \
    /// *This function returns a minute amount of information.*
    fn fetch_by_class(class: DeviceClass) -> Vec<PCIDevice>;
    
    /// This function returns a list of available PCI devices of a specific class with detailed information.
    fn fetch_by_class_detailed(class: DeviceClass) -> Vec<PCIDevice>;
    
    /// This function returns a list of available GPUs with only the necessary information. \
    /// This is essentially a wrapper for `fetch_by_class(DeviceClass::DisplayController)`
    /// but tries to extract only the necessary data from device names, for example: \
    /// - `TU117M [GeForce GTX 1650 Mobile / Max-Q]` can become `GeForce GTX 1650 Mobile / Max-Q`
    fn fetch_gpus() -> Vec<PCIDevice>;
}
