<div align="center">
<h1>aparato</h1>

A pci.ids-compliant library for getting information about available PCI devices.

<a href="https://crates.io/crates/aparato">
    <img src="https://img.shields.io/crates/v/aparato" alt="version" />
</a>

<a href="https://docs.rs/crate/aparato/">
    <img src="https://docs.rs/aparato/badge.svg" alt="docs" />
</a>

</div>

### Usage

Add the following to your project's *Cargo.toml* file:

```toml
aparato = "3.0.0"
```

### Examples

```rust
use aparato::PCIDevice;
use aparato::classes::DeviceClass;
use aparato::traits::*;

fn main() {
    // Know the address domain of the PCI device?
    // Instantiate a new PCIDevice so we can get to know it a bit.
    let device: PCIDevice = PCIDevice::new("0000:00:02.0");

    println!("Class Name: {}", device.class_name());    // e.g. Display Controller
    println!("Vendor Name: {}", device.vendor_name());  // e.g. Intel Corporation
    println!("Device Name: {}", device.device_name());  // e.g. WhiskeyLake-U GT2 [UHD Graphics 620]

    // Alternatively, we can get information on PCI devices through fetching them in bulk!

    // Return a list of available PCI devices and their information.
    let detailed_list: Vec<PCIDevice> = PCIDevice::fetch();
    println!("{:?}", detailed_list);

    // Return a list of the available PCI devices of a specific class.
    // -> "thing" holds a list of all the detected network controllers and their information.
    let thing: Vec<PCIDevice> = PCIDevice::fetch_by_class(DeviceClass::NetworkController);
    println!("{:?}", thing);
}

```

---

| Platform  | Support |
| :-------: | :-----: |
| Linux     |    ✓    |
| Windows   |         |
| macOS     |         |
| NetBSD    |         |

_aparato_ is still a work in progress.
