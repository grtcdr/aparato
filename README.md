<div align="center">
<h1>pci_fetch</h1>

Get information about connected PCI devices; essentially "lspci" for Rust 🦀.


<a href="https://crates.io/crates/pci_fetch">
    <img src="https://img.shields.io/crates/v/pci_fetch" alt="Version" />
</a>

<a href="https://docs.rs/crate/pci_fetch/">
    <img src="https://docs.rs/pci_fetch/badge.svg" alt="Docs" />
</a>

</div>

### Usage

```toml
pci_fetch = "2.1.0"
```

### Examples

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
    println!("Class Name: {}", device.class_name()); // e.g. Display Controller

    // Alternatively, we can get information on PCI devices through fetching them in bulk!

    // Return a list of the available PCI devices of a specific class.
    //    Example: This should return all the available GPUs but with little amount of information.
    let list: Vec<PCIDevice> = fetch_by_class(DeviceClass::DisplayController);
    println!("{:?}", list);

    // Return a list of the available PCI devices of a specific class with detailed information.
    //    Example: This should return all the available GPUs.
    let gpus: Vec<PCIDevice> = fetch_by_class_detailed(DeviceClass::DisplayController);
    println!("{:?}", gpus);

    // Return a list of the available display controllers with detailed information.
    let detailed_list: Vec<PCIDevice> = fetch_detailed();
    println!("{:?}", detailed_list);
}

```

---

### Platform Support

| Platform  | Support |
| :-------: | :-----: |
| Linux     |    ✓    |
