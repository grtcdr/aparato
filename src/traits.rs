use crate::{classes::DeviceClass};
use std::path::PathBuf;
use crate::PCIDevice;

pub trait Properties {
    /// This function returns a new instance of PCIDevice
    fn new(path: PathBuf) -> Self;
    
    /// After creating a new instance of PCIDevice, `init()` should be
    /// run to fetch the data of the newly created instance.
    fn init(&mut self);

    // -v- The getters -v-
    /// Return the `PCIDevice` path
    fn path(&self) -> PathBuf;
    
    /// Return the `PCIDevice` address
    fn address(&self) -> String;
    
    /// Return the `PCIDevice` class ID
    fn class_id(&self) -> String;
    
    /// Return the `PCIDevice` vendor ID
    fn vendor_id(&self) -> String;
    
    /// Return the `PCIDevice` device ID
    fn device_id(&self) -> String;
    
    /// Return the `PCIDevice` class name
    fn class_name(&self) -> String;
    
    /// Return the `PCIDevice` vendor name
    fn vendor_name(&self) -> String;
    
    /// Return the `PCIDevice` device name
    fn device_name(&self) -> String;

    // -v- The setters -v-
    /// Set the `path` field for the struct that implements it
    fn set_path(&mut self, p: PathBuf);
    
    /// Set the `address` field for the struct that implements it
    fn set_address(&mut self);
    
    /// Set the `device_id` field for the struct that implements it
    fn set_class_id(&mut self);
    
    /// Set the `device_id` field for the struct that implements it
    fn set_vendor_id(&mut self);
    
    /// Set the `device_id` field for the struct that implements it
    fn set_device_id(&mut self);
    
    /// Set the `class_name` field for the struct that implements it
    fn set_class_name(&mut self);
    
    /// Set the `vendor_name` field for the struct that implements it
    fn set_vendor_name(&mut self, name: String);
    
    /// Set the `device_name` field for the struct that implements it
    fn set_device_name(&mut self, name: String);
}

pub trait Fetch {
    /// Returns a list of available PCI devices and their information.
    ///
    /// *This function returns a minute amount of information.*
    fn fetch() -> Vec<PCIDevice>;
    
    /// Returns a list of available PCI devices with *detailed information*.
    fn fetch_detailed() -> Vec<PCIDevice>;
    
    /// Returns a list of available PCI devices of a specific class and their information.  \
    /// *This function returns a minute amount of information.*
    fn fetch_by_class(class: DeviceClass) -> Vec<PCIDevice>;
    
    /// Returns a list of available PCI devices of a specific class with detailed information.
    fn fetch_by_class_detailed(class: DeviceClass) -> Vec<PCIDevice>;
    
    /// Returns a list of available GPUs with only the necessary information. \
    /// This is essentially a wrapper for `fetch_by_class(DeviceClass::DisplayController)`
    /// but tries to extract only the necessary data from device names, for example: \
    /// - `TU117M [GeForce GTX 1650 Mobile / Max-Q]` can become `GeForce GTX 1650 Mobile / Max-Q`
    fn fetch_gpus() -> Vec<PCIDevice>;
}
