use std::fmt;

/// Each `PCIDevice` has a class, and `DeviceClass`
/// is the collection of all of them. \
/// Keep in mind that:
///
/// - The naming scheme of classes (enum variants) complies with that used by the PCI ID repository.
/// - `Unknown` is an additional class (not part of the official PCI ID classes) that is reserved for
/// when a PCI device's class is none of the other variants.
/// When the library parses a device's class, and determines it to be `Unknown` (highly unlikely),
/// the device's class will be set to an empty string.
pub enum DeviceClass {
    Unclassified,                      // ID: 00
    MassStorageController,             // ID: 01
    NetworkController,                 // ID: 02
    DisplayController,                 // ID: 03
    MultimediaController,              // ID: 04
    MemoryController,                  // ID: 05
    PCIBridge,                         // ID: 06
    CommunicationController,           // ID: 07
    GenericSystemPeripheral,           // ID: 08
    InputDeviceController,             // ID: 09
    DockingStation,                    // ID: 0a
    Processor,                         // ID: 0b
    SerialBusController,               // ID: 0c
    WirelessController,                // ID: 0d
    IntelligentController,             // ID: 0e
    SatelliteCommunicationsController, // ID: 0f
    EncryptionController,              // ID: 10
    SignalProcessingController,        // ID: 11
    ProcessingAccelerators,            // ID: 12
    NonEssentialInstrumentation,       // ID: 13
    Unknown,
}

impl fmt::Display for DeviceClass {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeviceClass::Unclassified => write!(f, "Unclassified"),
            DeviceClass::MassStorageController => write!(f, "Mass Storage Controller"),
            DeviceClass::NetworkController => write!(f, "Network Controller"),
            DeviceClass::DisplayController => write!(f, "Display Controller"),
            DeviceClass::MultimediaController => write!(f, "Multimedia Controller"),
            DeviceClass::MemoryController => write!(f, "Memory Controller"),
            DeviceClass::PCIBridge => write!(f, "PCI Bridge"),
            DeviceClass::CommunicationController => write!(f, "Communication Controller"),
            DeviceClass::GenericSystemPeripheral => write!(f, "Generic System Peripheral"),
            DeviceClass::InputDeviceController => write!(f, "Input Device Controller"),
            DeviceClass::DockingStation => write!(f, "Docking Station"),
            DeviceClass::Processor => write!(f, "Processor"),
            DeviceClass::SerialBusController => write!(f, "Serial Bus Controller"),
            DeviceClass::WirelessController => write!(f, "Wireless Controller"),
            DeviceClass::IntelligentController => write!(f, "Intelligent Controller"),
            DeviceClass::SatelliteCommunicationsController => {
                write!(f, "Satellite Communications Controller")
            }
            DeviceClass::EncryptionController => write!(f, "Encryption Controller"),
            DeviceClass::SignalProcessingController => write!(f, "Signal Processing Controller"),
            DeviceClass::ProcessingAccelerators => write!(f, "Processing Accelerators"),
            DeviceClass::NonEssentialInstrumentation => write!(f, "Non Essential Instrumentation"),
            DeviceClass::Unknown => write!(f, ""),
        }
    }
}

impl From<DeviceClass> for &'static str {
    fn from(class: DeviceClass) -> &'static str {
        match class {
            DeviceClass::Unclassified => "Unclassified",
            DeviceClass::MassStorageController => "Mass Storage Controller",
            DeviceClass::NetworkController => "Network Controller",
            DeviceClass::DisplayController => "Display Controller",
            DeviceClass::MultimediaController => "Multimedia Controller",
            DeviceClass::MemoryController => "Memory Controller",
            DeviceClass::PCIBridge => "Bridge",
            DeviceClass::CommunicationController => "Communication Controller",
            DeviceClass::GenericSystemPeripheral => "Generic System Peripheral",
            DeviceClass::InputDeviceController => "Input Device Controller",
            DeviceClass::DockingStation => "Docking Station",
            DeviceClass::Processor => "Processor",
            DeviceClass::SerialBusController => "Serial Bus Controller",
            DeviceClass::WirelessController => "Wireless Controller",
            DeviceClass::IntelligentController => "Intelligent Controller",
            DeviceClass::SatelliteCommunicationsController => "Satellite Communications Controller",
            DeviceClass::EncryptionController => "Encryption Controller",
            DeviceClass::SignalProcessingController => "Signal Processing Controller",
            DeviceClass::ProcessingAccelerators => "Processing Accelerators",
            DeviceClass::NonEssentialInstrumentation => "Non Essential Instrumentation",
            DeviceClass::Unknown => "",
        }
    }
}
