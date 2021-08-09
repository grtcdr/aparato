#![doc(hidden)]
use crate::device_class::*;
use crate::extra::*;
use crate::private::Properties;
use crate::Device;
use std::path::PathBuf;

#[derive(Debug)]
pub struct MacOSPCIDevice {
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

impl Device for MacOSPCIDevice {
    fn new(path: &str) -> Self {
        todo!()
    }

    fn path(&self) -> PathBuf {
        todo!()
    }

    fn address(&self) -> String {
        todo!()
    }

    fn class_id(&self) -> Vec<u8> {
        todo!()
    }

    fn vendor_id(&self) -> Vec<u8> {
        todo!()
    }

    fn device_id(&self) -> Vec<u8> {
        todo!()
    }

    fn numa_node(&self) -> isize {
        todo!()
    }

    fn class_name(&self) -> String {
        todo!()
    }

    fn subclass_name(&self) -> String {
        todo!()
    }

    fn vendor_name(&self) -> String {
        todo!()
    }

    fn device_name(&self) -> String {
        todo!()
    }

    fn enabled(&self) -> bool {
        todo!()
    }

    fn d3cold_allowed(&self) -> bool {
        todo!()
    }

    fn revision(&self) -> Vec<u8> {
        todo!()
    }

    fn subsystem_name(&self) -> String {
        todo!()
    }

    fn subsystem_vendor_id(&self) -> Vec<u8> {
        todo!()
    }

    fn subsystem_device_id(&self) -> Vec<u8> {
        todo!()
    }
}

impl Properties for MacOSPCIDevice {
    fn reserved_new(path: &str) -> Self {
        todo!()
    }
    
    fn set_path(&mut self, p: PathBuf) {
        todo!()
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

    fn set_revision(&mut self) {
        todo!()
    }

    fn set_numa_node(&mut self) {
        todo!()
    }

    fn set_subsystem_vendor_id(&mut self) {
        todo!()
    }

    fn set_subsystem_device_id(&mut self) {
        todo!()
    }

    fn set_class_name(&mut self) {
        todo!()
    }

    fn set_subclass_name(&mut self) {
        todo!()
    }

    fn set_vendor_name(&mut self) {
        todo!()
    }

    fn set_device_name(&mut self) {
        todo!()
    }

    fn set_subsystem_name(&mut self) {
        todo!()
    }

    fn set_enabled(&mut self) {
        todo!()
    }

    fn set_d3cold_allowed(&mut self) {
        todo!()
    }
}

impl Default for MacOSPCIDevice {
    fn default() -> Self {
        MacOSPCIDevice {
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
