use std::fmt;

pub enum DeviceSubclass {
    MassStorageController(u8),
    DisplayController(u8),
}

impl fmt::Display for DeviceSubclass {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self {
            DeviceSubclass::MassStorageController(v) => match v {
                0 => write!(f, "SCSI Storage Controller"),
                1 => write!(f, "IDE Interface"),
                2 => write!(f, "Floppy Disk Controller"),
                3 => write!(f, "IPI Bus Controller"),
                4 => write!(f, "RAID Bus Controller"),
                5 => write!(f, "ATA Controller"),
                6 => write!(f, "SATA Controller"),
                7 => write!(f, "Serial Attached SCSI Controller"),
                8 => write!(f, "Non-Volatile Memory Controller"),
                _ => write!(f, "Mass storage controller"),
            },
            DeviceSubclass::DisplayController(v) => match v {
                0 => write!(f, "VGA Compatible Controller"),
                1 => write!(f, "XGA Compatible Controller"),
                2 => write!(f, "3D Controller"),
                _ => write!(f, "Display Controller"),
            },
        }
    }
}