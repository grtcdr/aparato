use crate::classes::DeviceClass;
use crate::extra::*;
use std::path::PathBuf;

#[derive(Debug)]
pub struct PCIDevice {
    path: PathBuf,
    address: String,
    class_id: String,
    class_value: String,
}

impl PCIDevice {
    /// This function returns a __pci.ids__ compliant instance of PCIDevice with the appropriate values.
    pub fn new(path: PathBuf) -> Self {
        PCIDevice {
            path: path,
            address: String::new(),
            class_id: String::new(),
            class_value: String::new(),
        }
    }

    /// This function sets the class value associated with its class ID *as defined by pci.ids*.
    pub fn convert_id_to_value(&mut self) -> String {
        if let Ok(id) = get_class_id(&self.path, true, true) {
            self.class_id = id;
            self.class_value = match &self.class_id[..] {
                "00" => DeviceClass::Unclassified.to_string(),
                "01" => DeviceClass::MassStorageController.to_string(),
                "02" => DeviceClass::NetworkController.to_string(),
                "03" => DeviceClass::DisplayController.to_string(),
                "04" => DeviceClass::MultimediaController.to_string(),
                "05" => DeviceClass::MemoryController.to_string(),
                "06" => DeviceClass::PCIBridge.to_string(),
                "07" => DeviceClass::CommunicationController.to_string(),
                "08" => DeviceClass::GenericSystemPeripheral.to_string(),
                "09" => DeviceClass::InputDeviceController.to_string(),
                "0a" => DeviceClass::DockingStation.to_string(),
                "0b" => DeviceClass::Processor.to_string(),
                "0c" => DeviceClass::SerialBusController.to_string(),
                "0d" => DeviceClass::WirelessController.to_string(),
                "0e" => DeviceClass::IntelligentController.to_string(),
                "0f" => DeviceClass::SatelliteCommunicationsController.to_string(),
                "10" => DeviceClass::EncryptionController.to_string(),
                "11" => DeviceClass::SignalProcessingController.to_string(),
                "12" => DeviceClass::ProcessingAccelerators.to_string(),
                "13" => DeviceClass::NonEssentialInstrumentation.to_string(),
                _ => DeviceClass::Unknown.to_string(),
            }
        }

        String::new()
    }
}

impl std::fmt::Display for PCIDevice {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}: {}", self.address, self.class_value)
    }
}

pub fn get_pci_devices() -> Vec<PCIDevice> {
    let mut devices: Vec<PCIDevice> = Vec::new();
    let dir_entries = list_dir_entries(std::path::Path::new("/sys/bus/pci/devices"));
    for dir in dir_entries {
        // Create a new instance of PCIDevice
        let mut device = PCIDevice::new(dir);
        // Get the class ID associated with the PCIDevice path and
        // set the PCIDevice's class_value field accordingly.
        device.convert_id_to_value();
        // Set the address field by running basename on the directory we're in
        device.address = basename(
            device
                .path
                .as_path()
                .display()
                .to_string()
                .replace("0000:", ""),
        );

        devices.push(device);
    }

    return devices;
}

pub fn get_pci_devices_by_class(class: DeviceClass) -> Vec<PCIDevice> {
    let mut devices: Vec<PCIDevice> = Vec::new();
    let dir_entries = list_dir_entries(std::path::Path::new("/sys/bus/pci/devices"));
    for dir in dir_entries {
        // Create a new instance of PCIDevice
        let mut device = PCIDevice::new(dir);
        // Get the class ID associated with the PCIDevice path and
        // set the PCIDevice's class_value field accordingly.
        device.convert_id_to_value();
        if device.class_value == class.to_string() {
            // Set the address field by running basename on the directory we're in
            device.address = basename(
                device
                    .path
                    .as_path()
                    .display()
                    .to_string()
                    .replace("0000:", ""),
            );

            devices.push(device);
        }
    }

    return devices;
}

/// This function should return a list of connected PCI devices.
pub fn get_pci_addresses() -> Vec<String> {
    let mut addresses: Vec<String> = Vec::new();
    let dir_entries = list_dir_entries(std::path::Path::new("/sys/bus/pci/devices"));

    for dir in dir_entries {
        if let Some(str) = dir.to_str() {
            addresses.push(basename(str.to_string()));
        }
    }

    return addresses;
}

/// This function should return the `class` associated with the given `address`
///
/// - `skip_over_prefix` tells the function to ignore the first two characters, i.e. `0x`
/// - `pciids_compliant` tells the function to extract the class and also comply with [pci.ids](https://pci-ids.ucw.cz/)
pub fn get_class_id(
    address: &PathBuf,
    skip_over_prefix: bool,
    pciids_compliant: bool,
) -> Result<String, ()> {
    if let Ok(mut str) = std::fs::read_to_string(PathBuf::from(address).join("class")) {
        str.pop();
        if skip_over_prefix {
            str = str.trim_start_matches("0x").to_string();
        }
        if pciids_compliant {
            str = str.chars().take(2).collect();
        }

        return Ok(str);
    }

    Err(())
}

/// This function should return the `device` associated with the given `address`
///
/// - `skip_over_prefix` tells the function to ignore the first two characters, i.e. `0x`
pub fn get_device(address: &PathBuf, skip_over_prefix: bool) -> Result<String, ()> {
    if let Ok(mut str) = std::fs::read_to_string(address.join("device")) {
        if skip_over_prefix {
            str = str.trim_start_matches("0x").to_string();
        }
        str.pop();
        return Ok(str);
    }

    Err(())
}

/// This function should return the `vendor` associated with the given `address`
///
/// - `skip_over_prefix` tells the function to ignore the first two characters, i.e. `0x`
pub fn get_vendor(address: &PathBuf, skip_over_prefix: bool) -> Result<String, ()> {
    if let Ok(mut str) = std::fs::read_to_string(address.join("vendor")) {
        if skip_over_prefix {
            str = str.trim_start_matches("0x").to_string();
        }
        str.pop();
        return Ok(str);
    }

    Err(())
}

/// This function should return the `revision` associated with the given `address`
///
/// - `skip_over_prefix` tells the function to ignore the first two characters, i.e. `0x`
pub fn get_revision(address: &PathBuf, skip_over_prefix: bool) -> Result<String, ()> {
    if let Ok(mut str) = std::fs::read_to_string(address.join("revision")) {
        if skip_over_prefix {
            str = str.trim_start_matches("0x").to_string();
        }
        str.pop();
        return Ok(str);
    }

    Err(())
}
