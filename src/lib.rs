use extra::list_dir_entries;
use std::path::PathBuf;

mod extra;

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