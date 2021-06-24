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
///
/// *This function returns a minute amount of information.*
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

/// Returns a list of all available PCI devices with *detailed information*.
pub fn fetch_detailed() -> Vec<PCIDevice> {
    if let Ok(lines) = read_lines("/usr/share/hwdata/pci.ids") {
        let mut devices = fetch();
        for line in lines {
            if let Ok(l) = line {
                // Ignore empty lines, comments, and class definitions
                if l.len() == 0 || l.starts_with("#") || l.starts_with("C") {
                    continue;
                }

                if !l.starts_with("\t") {
                    // This is the condition for vendors
                    let vendor_id = l[..4].trim_start();
                    let vendor_name = l[4..].trim_start();

                    if devices.iter().any(|i| &i.vendor_id == vendor_id) {
                        let index = devices.iter().position(|p| p.vendor_id == vendor_id).unwrap();
                        devices[index].set_vendor_name(vendor_name.to_string());
                    }
                } else if l.starts_with("\t") {
                    // This is the condition for devices
                    let device_id = l[..5].trim_start();
                    let device_name = l[5..].trim_start();

                    if devices.iter().any(|i| &i.device_id == device_id) {
                        let index = devices.iter().position(|p| p.device_id == device_id).unwrap();
                        devices[index].set_device_name(device_name.to_string());
                    }
                }
            }
        }

        return devices;
    }

    vec![]
}


/// Returns a list of all available PCI devices of a specific class and their information.
///
/// *This function returns a minute amount of information.*
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

/// Returns a list of all available PCI devices of a specific class with detailed information.
pub fn fetch_by_class_detailed(class: DeviceClass) -> Vec<PCIDevice> {
    if let Ok(lines) = read_lines("/usr/share/hwdata/pci.ids") {
        let mut devices = fetch_by_class(class);
        for line in lines {
            if let Ok(l) = line {
                // Ignore empty lines, comments, and class definitions
                if l.len() == 0 || l.starts_with("#") || l.starts_with("C") {
                    continue;
                }

                if !l.starts_with("\t") {
                    // This is the condition for vendors
                    let vendor_id = l[..4].trim_start();
                    let vendor_name = l[4..].trim_start();

                    if devices.iter().any(|i| &i.vendor_id == vendor_id) {
                        let index = devices.iter().position(|p| p.vendor_id == vendor_id).unwrap();
                        devices[index].set_vendor_name(vendor_name.to_string());
                    }
                } else if l.starts_with("\t") {
                    // This is the condition for devices
                    let device_id = l[..5].trim_start();
                    let device_name = l[5..].trim_start();

                    if devices.iter().any(|i| &i.device_id == device_id) {
                        let index = devices.iter().position(|p| p.device_id == device_id).unwrap();
                        devices[index].set_device_name(device_name.to_string());
                    }
                }
            }
        }

        return devices;
    }

    vec![]
}
