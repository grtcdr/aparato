use std::fmt;

pub enum DeviceClass {
    Unclassified(u8),                          // ID: 00
    MassStorageController(u8),             // ID: 01
    NetworkController(u8),                 // ID: 02
    DisplayController(u8),                 // ID: 03
    MultimediaController(u8),              // ID: 04
    MemoryController(u8),                  // ID: 05
    Bridge(u8),                            // ID: 06
    CommunicationController(u8),           // ID: 07
    GenericSystemPeripheral(u8),           // ID: 08
    InputDeviceController(u8),             // ID: 09
    DockingStation(u8),                    // ID: 0a
    Processor(u8),                         // ID: 0b
    SerialBusController(u8),               // ID: 0c
    WirelessController(u8),                // ID: 0d
    IntelligentController,                 // ID: 0e
    SatelliteCommunicationsController(u8), // ID: 0f
    EncryptionController(u8),              // ID: 10
    SignalProcessingController(u8),        // ID: 11
    ProcessingAccelerator(u8),             // ID: 12
    NonEssentialInstrumentation,           // ID: 13
    Coprocessor,                           // ID: 40
    Unassigned,                            // ID: ff
}

impl fmt::Display for DeviceClass {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self {
            DeviceClass::MassStorageController(i) => match i {
                0 => write!(f, "SCSI Storage Controller"),
                1 => write!(f, "IDE Interface"),
                2 => write!(f, "Floppy Disk Controller"),
                3 => write!(f, "IPI Bus Controller"),
                4 => write!(f, "RAID Bus Controller"),
                5 => write!(f, "ATA Controller"),
                6 => write!(f, "SATA Controller"),
                7 => write!(f, "Serial Attached SCSI Controller"),
                8 => write!(f, "Non-Volatile Memory Controller"),
                _ => write!(f, "Mass Storage Controller"),
            },
            DeviceClass::DisplayController(i) => match i {
                0 => write!(f, "VGA Compatible Controller"),
                1 => write!(f, "XGA Compatible Controller"),
                2 => write!(f, "3D Controller"),
                _ => write!(f, "Display Controller"),
            },
            DeviceClass::NetworkController(i) => match i {
                0 => write!(f, "Ethernet Controller"),
                1 => write!(f, "Token Ring Network Controller"),
                2 => write!(f, "FDDI Network Controller"),
                3 => write!(f, "ATM network Controller"),
                4 => write!(f, "ISDN Controller"),
                5 => write!(f, "WorldFip Controller"),
                6 => write!(f, "PICMG Controller"),
                7 => write!(f, "Infiniband Controller"),
                8 => write!(f, "Fabric Controller"),
                _ => write!(f, "Network Controller"),
            },
            DeviceClass::MultimediaController(i) => match i {
                0 => write!(f, "Multimedia Video Controller"),
                1 => write!(f, "Multimedia Audio Controller"),
                2 => write!(f, "Computer Telephony Device"),
                3 => write!(f, "Audio Device"),
                _ => write!(f, "Multimedia Controller"),
            },
            DeviceClass::MemoryController(i) => match i {
                0 => write!(f, "RAM Memory"),
                1 => write!(f, "FLASH Memory"),
                _ => write!(f, "Memory Controller"),
            },
            DeviceClass::Bridge(i) => match i {
                0 => write!(f, "Host Bridge"),
                1 => write!(f, "ISA Bridge"),
                2 => write!(f, "EISA Bridge"),
                3 => write!(f, "MicroChannel Bridge"),
                4 => write!(f, "PCI Bridge"),
                5 => write!(f, "PCMCIA Bridge"),
                6 => write!(f, "NuBus Bridge"),
                7 => write!(f, "CardBus Bridge"),
                8 => write!(f, "RACEway Bridge"),
                9 => write!(f, "Semi-Transparent PCI-to-PCI Bridge"),
                10 => write!(f, "InfiniBand to PCI Host Bridge"),
                _ => write!(f, "Bridge"),
            },
            DeviceClass::CommunicationController(i) => match i {
                0 => write!(f, "Serial Controller"),
                1 => write!(f, "Parallel Controller"),
                2 => write!(f, "Multiport Serial Controller"),
                3 => write!(f, "Modem"),
                4 => write!(f, "GPIB Controller"),
                5 => write!(f, "Smard Card Controller"),
                _ => write!(f, "Communication Controller"),
            },
            DeviceClass::GenericSystemPeripheral(i) => match i {
                0 => write!(f, "PIC"),
                1 => write!(f, "DMA Controller"),
                2 => write!(f, "Timer"),
                3 => write!(f, "RTC"),
                4 => write!(f, "PCI Hot-Plug Controller"),
                5 => write!(f, "SD Host Controller"),
                6 => write!(f, "IOMMU"),
                128 => write!(f, "System Peripheral"),
                153 => write!(f, "Timing Card"),
                _ => write!(f, "Generic System Peripheral"),
            },
            DeviceClass::InputDeviceController(i) => match i {
                0 => write!(f, "Keyboard Controller"),
                1 => write!(f, "Digitizer Pen"),
                2 => write!(f, "Mouse Controller"),
                3 => write!(f, "Scanner Controller"),
                4 => write!(f, "Gameport Controller"),
                _ => write!(f, "Input Device Controller"),
            },
            DeviceClass::Processor(i) => match i {
                0 => write!(f, "386"),
                1 => write!(f, "486"),
                2 => write!(f, "Pentium"),
                16 => write!(f, "Alpha"),
                26 => write!(f, "Power PC"),
                36 => write!(f, "MIPS"),
                46 => write!(f, "Coprocessor"),
                _ => write!(f, "Processor"),
            },
            DeviceClass::SerialBusController(i) => match i {
                0 => write!(f, "FireWire (IEEE 1394)"),
                1 => write!(f, "ACCESS Bus"),
                2 => write!(f, "SSA"),
                3 => write!(f, "USB Controller"),
                4 => write!(f, "Fibre Channel"),
                5 => write!(f, "SMBus"),
                6 => write!(f, "InfiniBand"),
                7 => write!(f, "IPMI Interface"),
                8 => write!(f, "SERCOS Interface"),
                9 => write!(f, "CANBUS"),
                _ => write!(f, "Serial Bus Controller"),
            },
            DeviceClass::WirelessController(i) => match i {
                0 => write!(f, "IRDA Controller"),
                1 => write!(f, "Consumer IR Controller"),
                16 => write!(f, "RF Controller"),
                17 => write!(f, "Bluetooth"),
                18 => write!(f, "Broadband"),
                26 => write!(f, "802.1a Controller"),
                27 => write!(f, "802.1b Controller"),
                _ => write!(f, "Wireless Controller"),
            },
            DeviceClass::IntelligentController => write!(f, "Intelligent Controller"),
            DeviceClass::SatelliteCommunicationsController(i) => match i {
                1 => write!(f, "Satellite TV Controller"),
                2 => write!(f, "Satellite Audio communication Controller"),
                3 => write!(f, "Satellite Voice communication Controller"),
                4 => write!(f, "Satellite Data communication Controller"),
                _ => write!(f, "Satellite Communications Controller"),
            },
            DeviceClass::EncryptionController(i) => match i {
                0 => write!(f, "Network and Computing Encryption Device"),
                16 => write!(f, "Entertainment Encryption Device"),
                _ => write!(f, "Encryption Controller"),
            },
            DeviceClass::SignalProcessingController(i) => match i {
                0 => write!(f, "DPIO Module"),
                1 => write!(f, "Performance Counters"),
                16 => write!(f, "Communication Synchronizer"),
                26 => write!(f, "Signal Processing Management"),
                _ => write!(f, "Signal Processing Controller"),
            },
            DeviceClass::ProcessingAccelerator(i) => match i {
                0 => write!(f, "Proccesing Accelerator"),
                _ => write!(f, "AI Inference Accelerator"),
            },
            DeviceClass::NonEssentialInstrumentation => write!(f, "Non-Essential Instrumentation"),
            DeviceClass::Coprocessor => write!(f, "Coprocessor"),
            DeviceClass::Unassigned => write!(f, "Unassigned Class"),
            DeviceClass::DockingStation(i) => match i {
                0 => write!(f, "Generic Docking Station"),
                _ => write!(f, "Docking Station"),
            },  
            DeviceClass::Unclassified(i) => match i {
                0 => write!(f, "Non-VGA Unclassified Device"),
                1 => write!(f, "VGA-Compatible Unclassified Device"),
                5 => write!(f, "Image Coprocessor"),
                _ => write!(f, "Unclassified"),
            },      
        }
    }
}
