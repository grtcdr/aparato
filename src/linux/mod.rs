use crate::classes::*;
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
    class_id: String,
    class_name: String,
    vendor_id: String,
    vendor_name: String,
    device_id: String,
    device_name: String,
    revision: String,
    numa_node: isize,
    enabled: bool,
    d3cold_allowed: bool,
    subsystem_vendor_id: String,
    subsystem_device_id: String,
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
        // This function sets the values of a PCIDevice
        // which are read from files within its path.

        // This is extracted from the path file.
        self.set_address();
        // This is extracted from the class file.
        self.set_class_id();
        // This is extracted from the vendor file.
        self.set_vendor_id();
        // This is extracted from the device file.
        self.set_device_id();
        // This is extracted from the numa_node file.
        self.set_numa_node();
        // After class_id is extracted, we associate the class_id
        // with its appropriate class name (as defined by pci.ids)
        // and lastly, we assign it to the PCIDevice's class_name field.
        self.set_class_name();
        // This is extracted from the enable file.
        self.set_enabled();
        // This is extracted from the d3cold_allowed file.
        self.set_d3cold_allowed();
        // This is extracted from the revision file.
        self.set_revision();
        // This is extracted from the subsystem_device file.
        self.set_subsystem_device_id();
        // This is extracted from the subsystem_vendor file.
        self.set_subsystem_vendor_id();
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

    fn revision(&self) -> String {
        self.revision.to_owned()
    }

    fn subsystem_name(&self) -> String {
        self.subsystem_name.to_owned()
    }

    fn subsystem_vendor_id(&self) -> String {
        self.subsystem_vendor_id.to_owned()
    }

    fn subsystem_device_id(&self) -> String {
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
            // This file is guaranteed to end with an EOL character, so let's remove that.
            self.class_id = str
                .trim_start_matches("0x")
                .trim_end_matches("\n")
                .chars()
                .take(2)
                .collect();
        }
    }

    fn set_vendor_id(&mut self) {
        if let Ok(str) = std::fs::read_to_string(&self.path.join("vendor")) {
            // This file is guaranteed to end with an EOL character, so let's remove that.
            self.vendor_id = str
                .trim_start_matches("0x")
                .trim_end_matches("\n")
                .to_owned();
        }
    }

    fn set_device_id(&mut self) {
        if let Ok(str) = std::fs::read_to_string(&self.path.join("device")) {
            // This file is guaranteed to end with an EOL character, so let's remove that.
            self.device_id = str
                .trim_start_matches("0x")
                .trim_end_matches("\n")
                .to_owned();
        }
    }

    fn set_revision(&mut self) {
        if let Ok(str) = std::fs::read_to_string(&self.path.join("revision")) {
            // This file is guaranteed to end with an EOL character, so let's remove that.
            self.revision = str
                .trim_start_matches("0x")
                .trim_end_matches("\n")
                .to_owned();
        }
    }

    fn set_numa_node(&mut self) {
        if let Ok(str) = std::fs::read_to_string(&self.path.join("numa_node")) {
            if let Ok(val) = str.parse::<isize>() {
                self.numa_node = val;
            }
        }
    }

    fn set_subsystem_vendor_id(&mut self) {
        if let Ok(str) = std::fs::read_to_string(&self.path.join("subsystem_vendor")) {
            // This file is guaranteed to end with an EOL character, so let's remove that.
            self.subsystem_vendor_id = str
                .trim_start_matches("0x")
                .trim_end_matches("\n")
                .to_owned();
        }
    }

    fn set_subsystem_device_id(&mut self) {
        if let Ok(str) = std::fs::read_to_string(&self.path.join("subsystem_device")) {
            // This file is guaranteed to end with an EOL character, so let's remove that.
            self.subsystem_device_id = str
                .trim_start_matches("0x")
                .trim_end_matches("\n")
                .to_string();
        }
    }

    fn set_class_name(&mut self) {
        // This function sets the PCI device's `class_name` associated
        // with its `class_id` *as defined by **pci.ids***.
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

    fn set_vendor_name(&mut self) {
        if let Ok(lines) = read_lines(PATH_TO_PCI_IDS) {
            for line in lines {
                if let Ok(l) = &line {
                    if l.len() == 0 && l.starts_with("#") && l.starts_with("C") {
                        continue;
                    } else if !l.starts_with("\t") {
                        if l.contains(&self.vendor_id()) {
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
                        if l.contains(&self.device_id()) {
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
                        if l.contains(&self.subsystem_vendor_id())
                            && l.contains(&self.subsystem_device_id())
                        {
                            self.subsystem_name = l
                                .replace(&self.subsystem_device_id, "")
                                .replace(&self.subsystem_vendor_id, "")
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
            class_id: String::new(),
            class_name: String::new(),
            vendor_id: String::new(),
            vendor_name: String::new(),
            device_id: String::new(),
            device_name: String::new(),
            numa_node: -1,
            d3cold_allowed: false,
            enabled: false,
            revision: String::new(),
            subsystem_vendor_id: String::new(),
            subsystem_device_id: String::new(),
            subsystem_name: String::new(),
        }
    }
}

impl std::fmt::Display for LinuxPCIDevice {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.address)
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

    const PLACEHOLDER_PCI_DEVICE: &str = "0000:00:00.0";

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
