#![doc(hidden)]
#![allow(unused_variables)]
use crate::classes::*;
use crate::extra::*;
use crate::private::Properties;
use crate::Device;
use crate::Fetch;
use std::path::PathBuf;

/// This is where PCI devices are located.
const PATH_TO_PCI_DEVICES: &str = "/sys/bus/pci/devices/";
/// This is where the pci.ids file is located.
const PATH_TO_PCI_IDS: &str = "/usr/share/hwdata/pci.ids";

#[derive(Debug)]
pub struct LinuxPCIDevice {
    path: PathBuf,
    address: String,
    class_id: Vec<u8>,
    class_name: String,
    subclass_name: String,
    vendor_id: Vec<u8>,
    vendor_name: String,
    device_id: Vec<u8>,
    device_name: String,
    revision: Vec<u8>,
    numa_node: isize,
    enabled: bool,
    d3cold_allowed: bool,
    subsystem_vendor_id: Vec<u8>,
    subsystem_device_id: Vec<u8>,
    subsystem_name: String,
}

impl Device for LinuxPCIDevice {
    fn new(path: &str) -> Self {
        let mut device: LinuxPCIDevice = Default::default();
        let mut path_vec = [path].to_vec();

        // One of the following two conditions will try to autocomplete the path of the
        // PCI device if the one provided doesn't point to a real path in the filesystem.
        if !PathBuf::from(path_vec.concat()).is_dir() {
            // e.g. 0000:00:00.0 ->  /sys/bus/pci/devices/0000:00:00.0
            path_vec.insert(0, PATH_TO_PCI_DEVICES);
            device.set_path(PathBuf::from(path_vec.concat()));
            if !PathBuf::from(path_vec.concat()).is_dir() {
                // e.g. 00:00.0      ->  /sys/bus/pci/devices/0000:00:00.0
                let mut id = path.to_owned();
                id.insert_str(0, "0000:");
                std::mem::swap(&mut path_vec[1], &mut id.as_str());
                device.set_path(PathBuf::from(path_vec.concat()));
            }
        } else {
            device.set_path(PathBuf::from(path_vec.concat()));
        }

        device.set_address();
        device.set_class_id();
        device.set_vendor_id();
        device.set_device_id();
        device.set_numa_node();
        device.set_enabled();
        device.set_d3cold_allowed();
        device.set_revision();
        device.set_subsystem_device_id();
        device.set_subsystem_vendor_id();
        device.set_class_name();
        device.set_device_name();
        device.set_vendor_name();
        device.set_subsystem_name();
        device.set_subclass_name();

        device
    }

    fn path(&self) -> PathBuf {
        self.path.to_owned()
    }

    fn address(&self) -> String {
        self.address.to_owned()
    }

    fn class_id(&self) -> String {
        hex::encode(self.class_id.to_owned())
    }

    fn vendor_id(&self) -> String {
        hex::encode(self.vendor_id.to_owned())
    }

    fn device_id(&self) -> String {
        hex::encode(self.device_id.to_owned())
    }

    fn numa_node(&self) -> isize {
        self.numa_node.to_owned()
    }

    fn class_name(&self) -> String {
        self.class_name.to_owned()
    }

    fn subclass_name(&self) -> String {
        self.subclass_name.to_owned()
    }

    fn vendor_name(&self) -> String {
        self.vendor_name.to_owned()
    }

    fn device_name(&self) -> String {
        self.device_name.to_owned()
    }

    fn enabled(&self) -> bool {
        self.enabled
    }

    fn d3cold_allowed(&self) -> bool {
        self.d3cold_allowed
    }

    fn revision(&self) -> String {
        hex::encode(self.revision.to_owned())
    }

    fn subsystem_name(&self) -> String {
        self.subsystem_name.to_owned()
    }

    fn subsystem_vendor_id(&self) -> String {
        hex::encode(self.subsystem_vendor_id.to_owned())
    }

    fn subsystem_device_id(&self) -> String {
        hex::encode(self.subsystem_device_id.to_owned())
    }
}

impl Properties for LinuxPCIDevice {
    fn reserved_new(path: &str) -> Self {
        let mut device: LinuxPCIDevice = Default::default();
        let mut path_vec = [path].to_vec();

        // One of the following two conditions will try to autocomplete the path of the
        // PCI device if the one provided doesn't point to a real path in the filesystem.
        if !PathBuf::from(path_vec.concat()).is_dir() {
            // e.g. 0000:00:00.0 ->  /sys/bus/pci/devices/0000:00:00.0
            path_vec.insert(0, PATH_TO_PCI_DEVICES);
            device.set_path(PathBuf::from(path_vec.concat()));
            if !PathBuf::from(path_vec.concat()).is_dir() {
                // e.g. 00:00.0  ->  /sys/bus/pci/devices/0000:00:00.0
                let mut id = path.to_owned();
                id.insert_str(0, "0000:");
                std::mem::swap(&mut path_vec[1], &mut id.as_str());
                device.set_path(PathBuf::from(path_vec.concat()));
            }
        } else {
            device.set_path(PathBuf::from(path_vec.concat()));
        }

        // reserved_new tries to fetch the least amount of data at first.
        // All the other fields can be populated later on.
        device.set_address();
        device.set_class_id();
        device.set_class_name();

        device
    }
    
    fn set_path(&mut self, p: PathBuf) {
        self.path = p;
    }

    fn set_address(&mut self) {
        self.address = basename(
            self.path()
                .as_path()
                .display()
                .to_string()
                .replace("0000:", ""),
        );
    }

    fn set_class_id(&mut self) {
        if let Ok(str) = std::fs::read_to_string(&self.path.join("class")) {
            let new_str = str.trim_start_matches("0x").trim_end_matches("\n");
            if let Ok(decoded) = hex::decode(&new_str[..4]) {
                self.class_id = decoded;
            }
        }
    }

    fn set_vendor_id(&mut self) {
        if let Ok(str) = std::fs::read_to_string(&self.path.join("vendor")) {
            let new_str = str.trim_start_matches("0x").trim_end_matches("\n");
            if let Ok(decoded) = hex::decode(&new_str) {
                self.vendor_id = decoded;
            }
        }
    }

    fn set_device_id(&mut self) {
        if let Ok(str) = std::fs::read_to_string(&self.path.join("device")) {
            let new_str = str.trim_start_matches("0x").trim_end_matches("\n");
            if let Ok(decoded) = hex::decode(&new_str) {
                self.device_id = decoded;
            }
        }
    }

    fn set_revision(&mut self) {
        if let Ok(str) = std::fs::read_to_string(&self.path.join("revision")) {
            let new_str = str.trim_start_matches("0x").trim_end_matches("\n");
            if let Ok(decoded) = hex::decode(&new_str) {
                self.revision = decoded;
            }
        }
    }

    fn set_numa_node(&mut self) {
        if let Ok(str) = std::fs::read_to_string(&self.path.join("numa_node")) {
            let prefixless = str.trim_start_matches("0x").trim_end_matches("\n");
            if let Ok(v) = prefixless.parse::<isize>() {
                self.numa_node = v;
            }
        }
    }

    fn set_subsystem_vendor_id(&mut self) {
        if let Ok(str) = std::fs::read_to_string(&self.path.join("subsystem_vendor")) {
            let new_str = str.trim_start_matches("0x").trim_end_matches("\n");
            if let Ok(decoded) = hex::decode(&new_str) {
                self.subsystem_vendor_id = decoded;
            }
        }
    }

    fn set_subsystem_device_id(&mut self) {
        if let Ok(str) = std::fs::read_to_string(&self.path.join("subsystem_device")) {
            let new_str = str.trim_start_matches("0x").trim_end_matches("\n");
            if let Ok(decoded) = hex::decode(&new_str) {
                self.subsystem_device_id = decoded;
            }
        }
    }

    fn set_class_name(&mut self) {
        if self.class_id.is_empty() {
            return;
        }

        // Associate class_id with class_name
        self.class_name = match &self.class_id[0] {
            1 => DeviceClass::MassStorageController.to_string(),
            2 => DeviceClass::NetworkController.to_string(),
            3 => DeviceClass::DisplayController.to_string(),
            4 => DeviceClass::MultimediaController.to_string(),
            5 => DeviceClass::MemoryController.to_string(),
            6 => DeviceClass::Bridge.to_string(),
            7 => DeviceClass::CommunicationController.to_string(),
            8 => DeviceClass::GenericSystemPeripheral.to_string(),
            9 => DeviceClass::InputDeviceController.to_string(),
            10 => DeviceClass::DockingStation.to_string(),
            11 => DeviceClass::Processor.to_string(),
            12 => DeviceClass::SerialBusController.to_string(),
            13 => DeviceClass::WirelessController.to_string(),
            14 => DeviceClass::IntelligentController.to_string(),
            15 => DeviceClass::SatelliteCommunicationsController.to_string(),
            16 => DeviceClass::EncryptionController.to_string(),
            17 => DeviceClass::SignalProcessingController.to_string(),
            18 => DeviceClass::ProcessingAccelerator.to_string(),
            19 => DeviceClass::NonEssentialInstrumentation.to_string(),
            46 => DeviceClass::Coprocessor.to_string(),
            255 => DeviceClass::Unassigned.to_string(),
            _ => DeviceClass::Unclassified.to_string(),
        }
    }

    fn set_subclass_name(&mut self) {
        if self.class_id.is_empty() {
            return;
        }

        if let Ok(lines) = read_lines(PATH_TO_PCI_IDS) {
            let class: [u8; 1] = [self.class_id[0]];
            let encoded_class = hex::encode(&class);
            let subclass: [u8; 1] = [self.class_id[1]];
            let encoded_subclass = hex::encode(&subclass);
            let mut found_my_class = false;

            for line in lines {
                if let Ok(l) = &line {
                    if l.is_empty() || l.starts_with("#") {
                        continue;
                    } else if l.starts_with("C") && l.contains(&encoded_class) {
                        found_my_class = true;
                    } else if l.starts_with("\t") && l.contains(&encoded_subclass) && found_my_class
                    {
                        self.subclass_name = l.replace(&encoded_subclass, "").trim().to_owned();
                        return;
                    }
                }
            }
        }
    }

    fn set_vendor_name(&mut self) {
        if self.vendor_id.is_empty() {
            return;
        }

        if let Ok(lines) = read_lines(PATH_TO_PCI_IDS) {
            let ven = hex::encode(self.vendor_id.to_owned());

            for line in lines {
                if let Ok(l) = &line {
                    if l.len() == 0 || l.starts_with("#") || l.starts_with("C") {
                        continue;
                    } else if !l.starts_with("\t") && l.contains(&ven) {
                        self.vendor_name = l.replace(&ven, "").trim().to_owned();
                        return;
                    }
                }
            }
        }
    }

    fn set_device_name(&mut self) {
        if self.device_id.is_empty() {
            return;
        }

        if let Ok(lines) = read_lines(PATH_TO_PCI_IDS) {
            let dev = hex::encode(self.device_id.to_owned());
            for line in lines {
                if let Ok(l) = &line {
                    if l.len() == 0 || l.starts_with("#") || l.starts_with("C") {
                        continue;
                    } else if l.starts_with("\t") && l.contains(&dev) {
                        self.device_name = l.replace(&dev, "").trim().to_owned();
                        return;
                    }
                }
            }
        }
    }

    fn set_subsystem_name(&mut self) {
        if self.subsystem_device_id.is_empty() {
            return;
        }

        if let Ok(lines) = read_lines(PATH_TO_PCI_IDS) {
            let sub_dev = hex::encode(self.subsystem_device_id.to_owned());
            let sub_ven = hex::encode(self.subsystem_vendor_id.to_owned());

            for line in lines {
                if let Ok(l) = &line {
                    if l.len() == 0 && l.starts_with("#") && l.starts_with("C") {
                        continue;
                    } else if l.starts_with("\t\t") && l.contains(&sub_dev) && l.contains(&sub_ven)
                    {
                        self.subsystem_name = l
                            .replace(&sub_dev, "")
                            .replace(&sub_ven, "")
                            .trim()
                            .to_owned();
                        return;
                    }
                }
            }
        }
    }

    fn set_enabled(&mut self) {
        if let Ok(str) = std::fs::read_to_string(&self.path.join("enable")) {
            match &str[..] {
                "0\n" => self.enabled = false,
                _ => self.enabled = true,
            }
        }
    }

    fn set_d3cold_allowed(&mut self) {
        if let Ok(str) = std::fs::read_to_string(&self.path.join("d3cold_allowed")) {
            match &str[..] {
                "0\n" => self.d3cold_allowed = false,
                _ => self.d3cold_allowed = true,
            }
        }
    }
}

impl Default for LinuxPCIDevice {
    fn default() -> Self {
        LinuxPCIDevice {
            path: PathBuf::new(),
            address: String::new(),
            class_name: String::new(),
            subclass_name: String::new(),
            vendor_name: String::new(),
            device_name: String::new(),
            subsystem_name: String::new(),
            class_id: vec![],
            subsystem_vendor_id: vec![],
            subsystem_device_id: vec![],
            device_id: vec![],
            revision: vec![],
            vendor_id: vec![],
            numa_node: -1,
            d3cold_allowed: false,
            enabled: false,
        }
    }
}

impl Fetch for LinuxPCIDevice {
    fn fetch(maximum_devices: Option<u8>) -> Vec<LinuxPCIDevice> {
        let mut devices = Vec::new();
        let entries = list_dir_entries(PATH_TO_PCI_DEVICES);
        let mut i = 0u8;
        for dir in entries {
            if let Some(d) = dir.to_str() {
                if let Some(m) = maximum_devices {
                    i = i + 1;
                    if i > m {
                        break;
                    }
                }

                let device = LinuxPCIDevice::new(d);
                devices.push(device);
            }
        }
        return devices;
    }

    fn fetch_by_class(class: crate::classes::DeviceClass) -> Vec<LinuxPCIDevice> {
        todo!()
    }

    fn fetch_gpus() -> Vec<LinuxPCIDevice> {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const PLACEHOLDER_PCI_DEVICE: &str = "00:00.0";

    #[test]
    fn test_path() {
        let device = LinuxPCIDevice::new(PLACEHOLDER_PCI_DEVICE);
        assert_ne!(device.path(), PathBuf::new());
    }

    #[test]
    fn test_address() {
        let device = LinuxPCIDevice::new(PLACEHOLDER_PCI_DEVICE);
        assert_ne!(device.address(), "");
    }

    #[test]
    fn test_class_id() {
        let device = LinuxPCIDevice::new(PLACEHOLDER_PCI_DEVICE);
        assert_ne!(device.device_id(), "");
    }

    #[test]
    fn test_vendor_id() {
        let device = LinuxPCIDevice::new(PLACEHOLDER_PCI_DEVICE);
        assert_ne!(device.vendor_id(), "");
    }

    #[test]
    fn test_device_id() {
        let device = LinuxPCIDevice::new(PLACEHOLDER_PCI_DEVICE);
        assert_ne!(device.device_id(), "");
    }

    #[test]
    fn test_numa_node() {
        let device = LinuxPCIDevice::new(PLACEHOLDER_PCI_DEVICE);
        assert_ne!(device.numa_node().to_string(), "");
    }

    #[test]
    fn test_revision() {
        let device = LinuxPCIDevice::new(PLACEHOLDER_PCI_DEVICE);
        assert_ne!(device.revision(), "");
    }

    #[test]
    fn test_subsystem_vendor_id() {
        let device = LinuxPCIDevice::new(PLACEHOLDER_PCI_DEVICE);
        assert_ne!(device.subsystem_vendor_id(), "");
    }

    #[test]
    fn test_subsystem_device_id() {
        let device = LinuxPCIDevice::new(PLACEHOLDER_PCI_DEVICE);
        assert_ne!(device.subsystem_device_id(), "");
    }

    #[test]
    fn test_class_name() {
        let device = LinuxPCIDevice::new(PLACEHOLDER_PCI_DEVICE);
        assert_ne!(device.class_name(), "");
    }
}
