use extra::list_dir_entries;
use std::path::PathBuf;

mod extra;

/// This function should return a list of connected PCI devices.
pub fn get_pci_addresses(absolute_path: bool) -> Vec<PathBuf> {
    let mut address_list: Vec<PathBuf> = Vec::new();
    let addresses = list_dir_entries(std::path::Path::new("/sys/bus/pci/devices"));

    for adr in addresses {
        if absolute_path {
            address_list.push(adr);
        } else {
            if let Some(str) = adr.to_str() {
                address_list.push(PathBuf::from(extra::basename(str)));
            }
        }
    }

    return address_list;
}

/// This function should return the `class` associated with the given `address`
/// 
/// Example:
/// - `skip_over_prefix = false` -> `0x030000`
/// - `skip_over_prefix = true` -> `030000`
pub fn get_class(address: &PathBuf, skip_over_prefix: bool) -> Result<String, ()> {
    if let Ok(mut str) = std::fs::read_to_string(address.join("class")) {
        if skip_over_prefix {
            str = str.trim_start_matches("0x").to_string();
        }
        str.pop();
        return Ok(str);
    }

    Err(())
}


/// This function should return the `device` associated with the given `address`
/// 
/// Example:
/// - `skip_over_prefix = false` -> `0x3ea0`
/// - `skip_over_prefix = true` -> `3ea0`
pub fn get_device(address: &PathBuf, skip_over_prefix: bool) -> Result<String, ()> {
    if let Ok(mut str) = std::fs::read_to_string(address.join("device")) {
        if skip_over_prefix {
            str = str.trim_start_matches("0x").to_string();
        }
        str.pop();
        return Ok(str);
    }

    Err(())
}

/// This function should return the `vendor` associated with the given `address`
/// 
/// Example:
/// - `skip_over_prefix = false` -> `0x8086`
/// - `skip_over_prefix = true` -> `8086`
pub fn get_vendor(address: &PathBuf, skip_over_prefix: bool) -> Result<String, ()> {
    if let Ok(mut str) = std::fs::read_to_string(address.join("vendor")) {
        if skip_over_prefix {
            str = str.trim_start_matches("0x").to_string();
        }
        str.pop();
        return Ok(str);
    }

    Err(())
}

/// This function should return the `revision` associated with the given `address`
/// 
/// Example:
/// - `skip_over_prefix = false` -> `0x02`
/// - `skip_over_prefix = true` -> `02`
pub fn get_revision(address: &PathBuf, skip_over_prefix: bool) -> Result<String, ()> {
    if let Ok(mut str) = std::fs::read_to_string(address.join("revision")) {
        if skip_over_prefix {
            str = str.trim_start_matches("0x").to_string();
        }
        str.pop();
        return Ok(str);
    }

    Err(())
}