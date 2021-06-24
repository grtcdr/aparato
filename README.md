<div align="center">
<h1>pci_fetch</h1>

Get information about connected PCI devices

<a href="https://crates.io/crates/pci_fetch">
    <img src="https://img.shields.io/crates/v/pci_fetch" alt="Version" />
</a>

<a href="https://docs.rs/crate/pci_fetch/">
    <img src="https://docs.rs/pci_fetch/badge.svg" alt="Docs" />
</a>

</div>

### Usage

```rust
use pci_fetch::classes::DeviceClass;
use pci_fetch::linux::*;
use pci_fetch::traits::*;
use std::path::PathBuf;

fn main() {
    // Instantiate a new PCIDevice so we can get to know it a bit.
    let mut device: PCIDevice = PCIDevice::new(PathBuf::from("/sys/bus/pci/devices/0000:00:02.0"));
    // This little guy is important :)
    device.init();

    println!("Path: {:?}", device.path());         // e.g. /sys/bus/pci/devices/0000:00:02.0
    println!("Address: {}", device.address());     // e.g. 00:02.0
    println!("Class ID: {}", device.class_id());   // e.g. 03
    println!("Class ID: {}", device.class_name()); // e.g. Display Controller

    // Alternatively, we can get information on PCI devices through fetching them in bulk!

    // Returns a list of the available display controllers.
    let list: Vec<PCIDevice> = fetch_by_class(DeviceClass::DisplayController);
    println!("{:?}", list);
}

```

---

### Platform Support

| Platform  | Support |
| :-------: | :-----: |
| Linux     |    ✓    |
