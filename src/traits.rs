use std::path::PathBuf;

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
