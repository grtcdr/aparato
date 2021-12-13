<div align="center">
<h1>aparato</h1>

A library for querying connected PCI devices and a <a href="https://pci-ids.ucw.cz/">pci.ids</a> parser. 

<a href="https://crates.io/crates/aparato">
    <img src="https://img.shields.io/crates/v/aparato" alt="version" />
</a>

<a href="https://docs.rs/crate/aparato/">
    <img src="https://docs.rs/aparato/badge.svg" alt="docs" />
</a>

</div>

### Disclaimer

- aparato is currently unmaintained.

### Usage

Add the following to your project's *Cargo.toml* file:

```toml
aparato = "6.0.2" # Be sure to use the latest version
```

### Examples

```rust
use aparato::{Device, PCIDevice};

fn main() {

    // Know the domain of the PCI device?
    // Instantiate a new PCIDevice so we can get to know it a bit.
    let device = PCIDevice::new("00:02.0");

    println!("Class Name: {}", device.class_name());       // e.g. Display Controller
    println!("Subclass Name: {}", device.subclass_name()); // e.g. VGA compatible controller
    println!("Vendor Name: {}", device.vendor_name());     // e.g. Intel Corporation
    println!("Device Name: {}", device.device_name());     // e.g. WhiskeyLake-U GT2 [UHD Graphics 620]
}

```


### Contributing

Any form of contribution is welcome, whether it be unit tests, refactoring, or bug-fixing. It's recommended you report issues before beginning to work on them.
