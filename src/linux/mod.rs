use crate::classes::*;
use crate::subclasses::*;
use crate::extra::*;
use crate::traits::*;
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

impl Properties for LinuxPCIDevice {
    fn new(path: &str) -> Self {
        let mut path_vec = [path].to_vec();
        let mut device: LinuxPCIDevice = Default::default();

        // One of the following two conditions will try to autocomplete the path of the
        // PCI device if the one provided doesn't point to a real path in the filesystem.
        //   e.g.  0000:00:00.0 ->  /sys/bus/pci/devices/0000:00:00.0
        //         00:00.0      ->  /sys/bus/pci/devices/0000:00:00.0
        if !PathBuf::from(path_vec.concat()).is_dir() {
            path_vec.insert(0, PATH_TO_PCI_DEVICES);
            device.path = PathBuf::from(path_vec.concat());
            if !PathBuf::from(path_vec.concat()).is_dir() {
                let mut id = path.to_owned();
                id.insert_str(0, "0000:");
                std::mem::swap(&mut path_vec[1], &mut id.as_str());
                device.path = PathBuf::from(path_vec.concat());
            }
        }

        Self::init(&mut device);

        device
    }

    fn init(&mut self) {
        self.set_address();
        self.set_class_id();
        self.set_vendor_id();
        self.set_device_id();
        self.set_numa_node();
        self.set_enabled();
        self.set_d3cold_allowed();
        self.set_revision();
        self.set_subsystem_device_id();
        self.set_subsystem_vendor_id();
        self.set_class_name();
    }

    fn path(&self) -> PathBuf {
        self.path.to_owned()
    }

    fn address(&self) -> String {
        self.address.to_owned()
    }

    fn class_id(&self) -> Vec<u8> {
        self.class_id.to_owned()
    }

    fn vendor_id(&self) -> Vec<u8> {
        self.vendor_id.to_owned()
    }

    fn device_id(&self) -> Vec<u8> {
        self.device_id.to_owned()
    }

    fn numa_node(&self) -> isize {
        self.numa_node.to_owned()
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

    fn enabled(&self) -> bool {
        self.enabled
    }

    fn d3cold_allowed(&self) -> bool {
        self.d3cold_allowed
    }

    fn revision(&self) -> Vec<u8> {
        self.revision.to_owned()
    }

    fn subsystem_name(&self) -> String {
        self.subsystem_name.to_owned()
    }

    fn subsystem_vendor_id(&self) -> Vec<u8> {
        self.subsystem_vendor_id.to_owned()
    }

    fn subsystem_device_id(&self) -> Vec<u8> {
        self.subsystem_device_id.to_owned()
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
            let prefixless = str.trim_start_matches("0x").trim_end_matches("\n");
            self.vendor_id = prefixless.as_bytes().to_vec();
        }
    }

    fn set_device_id(&mut self) {
        if let Ok(str) = std::fs::read_to_string(&self.path.join("device")) {
            let prefixless = str.trim_start_matches("0x").trim_end_matches("\n");
            self.device_id = prefixless.as_bytes().to_vec();
        }
    }

    fn set_revision(&mut self) {
        if let Ok(str) = std::fs::read_to_string(&self.path.join("revision")) {
            let prefixless = str.trim_start_matches("0x").trim_end_matches("\n");
            self.revision = prefixless.as_bytes().to_vec();
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
            let prefixless = str.trim_start_matches("0x").trim_end_matches("\n");
            self.subsystem_vendor_id = prefixless.as_bytes().to_vec();
        }
    }

    fn set_subsystem_device_id(&mut self) {
        if let Ok(str) = std::fs::read_to_string(&self.path.join("subsystem_device")) {
            let prefixless = str.trim_start_matches("0x").trim_end_matches("\n");
            self.subsystem_device_id = prefixless.as_bytes().to_vec();
        }
    }

    fn set_class_name(&mut self) {
        // Associate class_id with class_name
        self.class_name = match (&self.class_id[0], &self.class_id[1]) {
            (1, subclass) => DeviceSubclass::MassStorageController(*subclass).to_string(),
            (2, _) => DeviceClass::NetworkController.to_string(),
            (3, subclass) => DeviceSubclass::DisplayController(*subclass).to_string(),
            (4, _) => DeviceClass::MultimediaController.to_string(),
            (5, _) => DeviceClass::MemoryController.to_string(),
            (6, _) => DeviceClass::Bridge.to_string(),
            (7, _) => DeviceClass::CommunicationController.to_string(),
            (8, _) => DeviceClass::GenericSystemPeripheral.to_string(),
            (9, _) => DeviceClass::InputDeviceController.to_string(),
            (10, _) => DeviceClass::DockingStation.to_string(),
            (11, _) => DeviceClass::Processor.to_string(),
            (12, _) => DeviceClass::SerialBusController.to_string(),
            (13, _) => DeviceClass::WirelessController.to_string(),
            (14, _) => DeviceClass::IntelligentController.to_string(),
            (15, _) => DeviceClass::SatelliteCommunicationsController.to_string(),
            (16, _) => DeviceClass::EncryptionController.to_string(),
            (17, _) => DeviceClass::SignalProcessingController.to_string(),
            (18, _) => DeviceClass::ProcessingAccelerators.to_string(),
            (19, _) => DeviceClass::NonEssentialInstrumentation.to_string(),
            (64, _) => DeviceClass::Unassigned.to_string(),
            _ => DeviceClass::Unclassified.to_string(),
        }
    }

    fn set_vendor_name(&mut self) {
        if let Ok(lines) = read_lines(PATH_TO_PCI_IDS) {
            for line in lines {
                if let Ok(l) = &line {
                    if l.len() == 0 && l.starts_with("#") && l.starts_with("C") {
                        continue;
                    } else if !l.starts_with("\t") {
                        // vendor parsing
                        let a = &self.vendor_id[..];
                        let ven_in_hex = format!("{:x?}", a);
                        if l.contains(&ven_in_hex) {
                            self.vendor_name = l[4..].trim_start().to_owned();
                        }
                    }
                }
            }
        }
    }

    fn set_device_name(&mut self) {
        if let Ok(lines) = read_lines(PATH_TO_PCI_IDS) {
            for line in lines {
                if let Ok(l) = &line {
                    if l.len() == 0 && l.starts_with("#") && l.starts_with("C") {
                        continue;
                    } else if l.starts_with("\t") {
                        // device parsing
                        let dev_in_hex = format!("{:x?}", &self.device_id);
                        if l.contains(&dev_in_hex) {
                            self.device_name = l[5..].trim_start().to_owned();
                        }
                    }
                }
            }
        }
    }

    fn set_subsystem_name(&mut self) {
        if let Ok(lines) = read_lines(PATH_TO_PCI_IDS) {
            for line in lines {
                if let Ok(l) = &line {
                    if l.len() == 0 && l.starts_with("#") && l.starts_with("C") {
                        continue;
                    } else if l.starts_with("\t\t") {
                        // subsystem parsing
                        let subvendor_in_hex = &format!("{:x?}", &self.subsystem_vendor_id);
                        let subdevice_in_hex = &format!("{:x?}", &self.subsystem_device_id);
                        if l.contains(subdevice_in_hex) && l.contains(subvendor_in_hex) {
                            self.subsystem_name = l
                                .replace(subvendor_in_hex, "")
                                .replace(subdevice_in_hex, "")
                                .trim()
                                .to_owned();
                        }
                    }
                }
            }
        }
    }

    fn set_enabled(&mut self) {
        if let Ok(str) = std::fs::read_to_string(&self.path.join("enable")) {
            match &str[..1] {
                "0" => self.enabled = false,
                _ => self.enabled = true,
            }
        }
    }

    fn set_d3cold_allowed(&mut self) {
        if let Ok(str) = std::fs::read_to_string(&self.path.join("d3cold_allowed")) {
            match &str[..1] {
                "0" => self.d3cold_allowed = false,
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
            class_id: vec![],
            class_name: String::new(),
            vendor_id: vec![],
            vendor_name: String::new(),
            device_id: vec![],
            device_name: String::new(),
            numa_node: -1,
            d3cold_allowed: false,
            enabled: false,
            revision: vec![],
            subsystem_vendor_id: vec![],
            subsystem_device_id: vec![],
            subsystem_name: String::new(),
        }
    }
}

impl std::fmt::Display for LinuxPCIDevice {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Path: {:?}\nAddress: {}\nClass Name: {}\nClass ID: {}", self.path, self.address, self.class_name, format!("{:x?}", self.class_id))
    }
}

impl Fetch for LinuxPCIDevice {
    fn fetch() -> Vec<LinuxPCIDevice> {
        let mut devices = Vec::new();
        let dir_entries = list_dir_entries(PATH_TO_PCI_DEVICES);
        for dir in dir_entries {
            if let Some(d) = dir.to_str() {
                let device = LinuxPCIDevice::new(d);
                devices.push(device);
            }
        }
        return devices;
    }

    fn fetch_by_class(class: DeviceClass) -> Vec<LinuxPCIDevice> {
        let mut devices = Vec::new();
        let dir_entries = list_dir_entries(PATH_TO_PCI_DEVICES);
        for dir in dir_entries {
            if let Some(d) = dir.to_str() {
                let device = LinuxPCIDevice::new(d);
                if device.class_name() == class.to_string() {
                    devices.push(device);
                }
            }
        }

        return devices;
    }

    fn fetch_gpus() -> Vec<LinuxPCIDevice> {
        let mut gpus = Self::fetch_by_class(DeviceClass::DisplayController);
        for gpu in &mut gpus {
            let whole_name = gpu.device_name();
            if let Some(start_bytes) = whole_name.find("[") {
                if let Some(end_bytes) = whole_name.rfind("]") {
                    gpu.device_name = whole_name[start_bytes + 1..end_bytes].to_owned();
                }
            }
            if gpu.vendor_name().contains("Corporation") {
                gpu.vendor_name = gpu.vendor_name().replace(" Corporation", "");
            }
        }
        gpus
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
        assert_ne!(device.device_id(), 0);
    }

    #[test]
    fn test_vendor_id() {
        let device = LinuxPCIDevice::new(PLACEHOLDER_PCI_DEVICE);
        assert_ne!(device.vendor_id(), 0);
    }

    #[test]
    fn test_device_id() {
        let device = LinuxPCIDevice::new(PLACEHOLDER_PCI_DEVICE);
        assert_ne!(device.device_id(), 0);
    }

    #[test]
    fn test_numa_node() {
        let device = LinuxPCIDevice::new(PLACEHOLDER_PCI_DEVICE);
        assert_ne!(device.numa_node().to_string(), "");
    }

    #[test]
    fn test_revision() {
        let device = LinuxPCIDevice::new(PLACEHOLDER_PCI_DEVICE);
        assert_ne!(device.revision(), 0);
    }

    #[test]
    fn test_subsystem_vendor_id() {
        let device = LinuxPCIDevice::new(PLACEHOLDER_PCI_DEVICE);
        assert_ne!(device.subsystem_vendor_id(), 0);
    }

    #[test]
    fn test_subsystem_device_id() {
        let device = LinuxPCIDevice::new(PLACEHOLDER_PCI_DEVICE);
        assert_ne!(device.subsystem_device_id(), 0);
    }

    #[test]
    fn test_enabled() {
        // We can't know for sure which devices are/aren't enabled, but we
        // can perform a test to make sure that at least one device is.
        let devices = LinuxPCIDevice::fetch();
        for device in devices {
            if device.enabled {
                assert_eq!((), ());
                break;
            }
        }
    }

    #[test]
    fn test_d3cold_allowed() {
        // We can't know for sure which devices have/don't have d3cold capabilities,
        // but we can perform a test to make sure that at least one device does.
        let devices = LinuxPCIDevice::fetch();
        for device in devices {
            if device.d3cold_allowed {
                assert_eq!((), ());
                break;
            }
        }
    }

    #[test]
    fn test_class_name() {
        let device = LinuxPCIDevice::new(PLACEHOLDER_PCI_DEVICE);
        assert_ne!(device.class_name(), "");
    }

    #[test]
    fn test_fetch() {
        let devices = LinuxPCIDevice::fetch();
        assert_ne!(devices.len(), 0);
    }

    #[test]
    fn test_fetch_gpus() {
        let gpus = LinuxPCIDevice::fetch_gpus();
        assert_ne!(gpus.len(), 0);
    }

    #[test]
    fn test_fetch_by_class() {
        let devices = LinuxPCIDevice::fetch_by_class(DeviceClass::MassStorageController);
        assert_ne!(devices.len(), 0);
    }
}
