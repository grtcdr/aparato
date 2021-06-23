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
// Fetch all connected PCI devices.
let all_devices = get_pci_devices();

// Fetch PCI devices by class (classes are defined in https://pci-ids.ucw.cz/read/PD/)
let gpus = get_pci_devices_by_class(DeviceClass::DisplayController);

for g in gpus {
    println!("{}", g);
}    
```

---

### Platform Support

| Platform  | Support |
| :-------: | :-----: |
| Linux     |    ✓    |
