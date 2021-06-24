use crate::classes::DeviceClass;
use crate::extra::*;
use crate::traits::*;
use std::path::PathBuf;
mod implement;

const BASE_PCI_DEVICES_PATH: &str = "/sys/bus/pci/devices";

#[derive(Debug)]
pub struct PCIDevice {
    path: PathBuf,
    address: String,
    class_id: String,
    class_name: String,
    vendor_id: String,
    vendor_name: String,
    device_id: String,
    device_name: String,
}

/// Returns a list of all available PCI devices and their information.
pub fn fetch() -> Vec<PCIDevice> {
    let mut devices: Vec<PCIDevice> = Vec::new();
    let dir_entries = list_dir_entries(BASE_PCI_DEVICES_PATH);
    for dir in dir_entries {
        let mut device = PCIDevice::new(dir);
        device.init();
        devices.push(device);
    }

    return devices;
}

/// Returns a list of all available PCI devices of a specific class and their information.
pub fn fetch_by_class(class: DeviceClass) -> Vec<PCIDevice> {
    let mut devices: Vec<PCIDevice> = Vec::new();
    let dir_entries = list_dir_entries(BASE_PCI_DEVICES_PATH);
    for dir in dir_entries {
        let mut device: PCIDevice = PCIDevice::new(dir);
        device.init();
        if device.class_name() == class.to_string() {
            devices.push(device);
        }
    }

    return devices;
}