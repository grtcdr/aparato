# pci_fetch

Get information on PCI devices

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
