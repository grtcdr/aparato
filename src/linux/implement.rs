use std::path::PathBuf;

use crate::classes::*;
use crate::linux::PCIDevice;
use crate::traits::*;
use crate::extra::*;

impl Properties for PCIDevice {
    fn new(path: PathBuf) -> Self {
        return PCIDevice {
            path: path,
            address: String::new(),
            class_id: String::new(),
            class_name: String::new(),
            vendor_id: String::new(),
            vendor_name: String::new(),
            device_id: String::new(),
            device_name: String::new(),
        };
    }

    fn init(&mut self) {
        self.set_address();
        self.set_class_id();
        self.set_vendor_id();
        self.set_device_id();
        self.set_class_name();
    }

    fn path(&self) -> PathBuf {
        self.path.to_owned()
    }

    fn address(&self) -> String {
        self.address.to_owned()
    }

    fn class_id(&self) -> String {
        self.class_id.to_owned()
    }

    fn vendor_id(&self) -> String {
        self.vendor_id.to_owned()
    }

    fn device_id(&self) -> String {
        self.device_id.to_owned()
    }

    fn class_name(&self) -> String {
        self.class_name.to_owned()
    }

    fn vendor_name(&self) -> String {
        self.vendor_name.to_owned()
    }

    fn device_name(&self) -> String {
        self.device_name.to_owned()
    }

    fn set_path(&mut self, p: PathBuf) {
        self.path = p;
    }

    fn set_address(&mut self) {
        self.address = basename(
            self
                .path()
                .as_path()
                .display()
                .to_string()
                .replace("0000:", ""),
        );
    }

    fn set_class_id(&mut self) {
        if let Ok(mut str) = std::fs::read_to_string(&self.path().join("class")) {
            str.pop();
            str = str.trim_start_matches("0x").chars().take(2).collect();
            self.class_id = str;
        }
    }

    fn set_vendor_id(&mut self) {
        if let Ok(mut str) = std::fs::read_to_string(&self.path().join("vendor")) {
            str = str.trim_start_matches("0x").to_string();
            str.pop();
            self.vendor_id = str;
        }
    }

    fn set_device_id(&mut self) {
        if let Ok(mut str) = std::fs::read_to_string(&self.path().join("device")) {
            str = str.trim_start_matches("0x").to_string();
            str.pop();
            self.device_id = str;
        }
    }

    fn set_vendor_name(&mut self, name: String) {
        self.vendor_name = name;
    }

    fn set_device_name(&mut self, name: String) {
        self.device_name = name;
    }

    /// This function sets the PCIDevice's `class_name` associated
    /// with its `class_id` *as defined by **pci.ids***.
    fn set_class_name(&mut self) {
        if !&self.class_id.is_empty() {
            self.class_name = match &self.class_id[..] {
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
    }
}

impl std::fmt::Display for PCIDevice {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.address)
    }
}
