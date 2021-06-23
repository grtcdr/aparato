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
    use pci_fetch::*;

    // Specify a device
    let pci_device = std::path::PathBuf::from("/sys/bus/pci/devices/0000:00:00.0");

    // Fetch PCI device class
    let class = get_class(&pci_device, true).unwrap();
    // Fetch PCI device device
    let device = get_device(&pci_device, true).unwrap();
    // Fetch PCI device vendor
    let vendor = get_vendor(&pci_device, true).unwrap();
    // Fetch PCI device revision
    let revision = get_revision(&pci_device, true).unwrap();

    println!("Class: {:?}", class);
    println!("Device: {:?}", device);
    println!("Vendor: {:?}", vendor);
    println!("Revision: {:?}", revision);

```

---

### Platform Support

| Platform  | Support |
| :-------: | :-----: |
| Linux     |    ✓    |
