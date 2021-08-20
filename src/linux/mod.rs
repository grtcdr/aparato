#![doc(hidden)]
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
    class: String,
    class_name: String,
    subclass: String,
    subclass_name: String,
    prog_if: String,
    prog_if_name: String,
    vendor: String,
    vendor_name: String,
    device: String,
    device_name: String,
    subsystem_vendor: String,
    subsystem_device: String,
    subsystem_name: String,
    path: PathBuf,
    enabled: bool,
}

impl Device for LinuxPCIDevice {
    fn new(path: &str) -> Self {
        let device_path = PathBuf::from("/sys/bus/pci/devices").join(path);

        if !device_path.is_dir() {
            panic!()
        }

        let mut device: Self = Default::default();

        device.set_path(device_path);
        device.set_class();
        device.set_class_properties();
        device.set_vendor();
        device.set_device();
        device.set_enabled();
        device.set_subsystem_device();
        device.set_subsystem_vendor();

        device
    }

    fn path(&self) -> PathBuf {
        self.path.to_owned()
    }

    fn class(&self) -> String {
        self.class.to_owned()
    }

    fn subclass(&self) -> String {
        self.subclass.to_owned()
    }

    fn prog_if(&self) -> String {
        self.prog_if.to_owned()
    }

    fn vendor(&self) -> String {
        self.vendor.to_owned()
    }

    fn device(&self) -> String {
        self.device.to_owned()
    }

    fn class_name(&self) -> String {
        self.class_name.to_owned()
    }

    fn subclass_name(&self) -> String {
        self.subclass_name.to_owned()
    }

    fn prog_if_name(&self) -> String {
        self.prog_if_name.to_owned()
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

    fn subsystem_name(&self) -> String {
        self.subsystem_name.to_owned()
    }

    fn subsystem_vendor(&self) -> String {
        self.subsystem_vendor.to_owned()
    }

    fn subsystem_device(&self) -> String {
        self.subsystem_device.to_owned()
    }
}

impl Properties for LinuxPCIDevice {
    fn set_path(&mut self, p: PathBuf) {
        self.path = PathBuf::from("/sys/bus/pci/devices").join(p);
    }

    fn set_class(&mut self) {
        if let Ok(str) = std::fs::read_to_string(&self.path.join("class")) {
            self.class = str[2..4].to_owned();
            self.subclass = str[4..6].to_owned();
            self.prog_if = str[6..8].to_owned();
        }
    }

    fn set_vendor(&mut self) {
        if let Ok(str) = std::fs::read_to_string(&self.path.join("vendor")) {
            self.vendor = str[2..6].to_owned();
        }
    }

    fn set_device(&mut self) {
        if let Ok(str) = std::fs::read_to_string(&self.path.join("device")) {
            self.device = str[2..6].to_owned();
        }
    }

    fn set_subsystem_vendor(&mut self) {
        if let Ok(str) = std::fs::read_to_string(&self.path.join("subsystem_vendor")) {
            self.subsystem_vendor = str[2..6].to_owned();
        }
    }

    fn set_subsystem_device(&mut self) {
        if let Ok(str) = std::fs::read_to_string(&self.path.join("subsystem_device")) {
            self.subsystem_device = str[2..6].to_owned();
        }
    }

    fn set_enabled(&mut self) {
        if let Ok(str) = std::fs::read_to_string(&self.path.join("enable")) {
            self.enabled = !str.contains("0");
        }
    }

    fn set_class_properties(&mut self) {
        if let Ok(lines) = read_lines(PATH_TO_PCI_IDS) {
            let mut found_it = false;
            for line in lines {
                if let Ok(l) = line {
                    if l.starts_with("C") && !l.contains(&self.class) {
                        found_it = false;
                    } else if l.starts_with("C") && l.contains(&self.class) {
                        self.class_name = l
                            .replace("C", "")
                            .replace(&self.class, "")
                            .trim()
                            .to_owned();
                        found_it = true;
                    } else if l.starts_with("\t\t") && l.contains(&self.prog_if) && found_it {
                        self.prog_if_name = l.replace(&self.prog_if, "").trim().to_owned();
                    } else if l.starts_with("\t") && l.contains(&self.subclass) && found_it {
                        self.subclass_name = l.replace(&self.subclass, "").trim().to_owned();
                    }
                }
            }
        }
    }
}

impl Default for LinuxPCIDevice {
    fn default() -> Self {
        LinuxPCIDevice {
            class: String::new(),
            class_name: String::new(),
            subclass: String::new(),
            subclass_name: String::new(),
            prog_if: String::new(),
            prog_if_name: String::new(),
            vendor: String::new(),
            vendor_name: String::new(),
            device: String::new(),
            device_name: String::new(),
            subsystem_vendor: String::new(),
            subsystem_device: String::new(),
            subsystem_name: String::new(),
            path: PathBuf::new(),
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
                        continue;
                    }
                }

                let device = LinuxPCIDevice::new(d);
                devices.push(device);
            }
        }

        return devices;
    }

    fn fetch_by_class(maximum_devices: Option<u8>) -> Vec<LinuxPCIDevice> {
        todo!()
    }

    fn fetch_gpus(maximum_devices: Option<u8>) -> Vec<String> {
        todo!()
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
    fn test_class() {
        let device = LinuxPCIDevice::new(PLACEHOLDER_PCI_DEVICE);
        assert_ne!(device.device(), "");
    }

    #[test]
    fn test_vendor() {
        let device = LinuxPCIDevice::new(PLACEHOLDER_PCI_DEVICE);
        assert_ne!(device.vendor(), "");
    }

    #[test]
    fn test_device() {
        let device = LinuxPCIDevice::new(PLACEHOLDER_PCI_DEVICE);
        assert_ne!(device.device(), "");
    }

    #[test]
    fn test_subsystem_vendor() {
        let device = LinuxPCIDevice::new(PLACEHOLDER_PCI_DEVICE);
        assert_ne!(device.subsystem_vendor(), "");
    }

    #[test]
    fn test_subsystem_device() {
        let device = LinuxPCIDevice::new(PLACEHOLDER_PCI_DEVICE);
        assert_ne!(device.subsystem_device(), "");
    }

    #[ignore]
    #[test]
    fn test_class_name() {
        let device = LinuxPCIDevice::new(PLACEHOLDER_PCI_DEVICE);
        assert_ne!(device.class_name(), "");
    }
}
