use crate::classes::*;
use crate::traits::*;
use std::path::PathBuf;

#[derive(Debug)]
pub struct MacOSPCIDevice {
    path: PathBuf,
    address: String,
    class_id: String,
    class_name: String,
    vendor_id: String,
    vendor_name: String,
    device_id: String,
    device_name: String,
    numa_node: isize,
}

impl Properties for MacOSPCIDevice {
    fn new(path: &str) -> Self {
        todo!()
    }

    fn init(&mut self) {
        todo!()
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

    fn set_path(&mut self, p: PathBuf) {
        self.path = p;
    }

    fn set_address(&mut self) {
        todo!()
    }

    fn set_class_id(&mut self) {
        todo!()
    }

    fn set_vendor_id(&mut self) {
        todo!()
    }

    fn set_device_id(&mut self) {
        todo!()
    }

    fn set_numa_node(&mut self) {
        todo!()
    }

    fn set_class_name(&mut self) {
        todo!()
    }

    fn set_vendor_name(&mut self, name: String) {
        self.vendor_name = name;
    }

    fn set_device_name(&mut self, name: String) {
        self.device_name = name;
    }
}

impl std::fmt::Display for MacOSPCIDevice {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.address)
    }
}

impl Fetch for MacOSPCIDevice {
    fn fetch() -> Vec<MacOSPCIDevice> {
        todo!()
    }

    fn fetch_by_class(class: DeviceClass) -> Vec<MacOSPCIDevice> {
        todo!()
    }

    fn fetch_gpus() -> Vec<MacOSPCIDevice> {
        todo!()
    }
}
