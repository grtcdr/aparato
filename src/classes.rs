use std::fmt;

pub enum DeviceClass {
    Unclassified,                      // ID: 00
    MassStorageController,             // ID: 01
    NetworkController,                 // ID: 02
    DisplayController,                 // ID: 03
    MultimediaController,              // ID: 04
    MemoryController,                  // ID: 05
    Bridge,                            // ID: 06
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
    ProcessingAccelerator,             // ID: 12
    NonEssentialInstrumentation,       // ID: 13
    Coprocessor,                       // ID: 40
    Unassigned,                        // ID: ff
}

impl fmt::Display for DeviceClass {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeviceClass::Unassigned => write!(f, "Unassigned"),
            DeviceClass::Unclassified => write!(f, "Unclassified"),
            DeviceClass::MassStorageController => write!(f, "Mass Storage Controller"),
            DeviceClass::NetworkController => write!(f, "Network Controller"),
            DeviceClass::DisplayController => write!(f, "Display Controller"),
            DeviceClass::MultimediaController => write!(f, "Multimedia Controller"),
            DeviceClass::MemoryController => write!(f, "Memory Controller"),
            DeviceClass::Bridge => write!(f, "Bridge"),
            DeviceClass::CommunicationController => write!(f, "Communication Controller"),
            DeviceClass::GenericSystemPeripheral => write!(f, "Generic System Peripheral"),
            DeviceClass::InputDeviceController => write!(f, "Input Device Controller"),
            DeviceClass::DockingStation => write!(f, "Docking Station"),
            DeviceClass::Processor => write!(f, "Processor"),
            DeviceClass::Coprocessor => write!(f, "Coprocessor"),
            DeviceClass::SerialBusController => write!(f, "Serial Bus Controller"),
            DeviceClass::WirelessController => write!(f, "Wireless Controller"),
            DeviceClass::IntelligentController => write!(f, "Intelligent Controller"),
            DeviceClass::SatelliteCommunicationsController => {
                write!(f, "Satellite Communications Controller")
            }
            DeviceClass::EncryptionController => write!(f, "Encryption Controller"),
            DeviceClass::SignalProcessingController => write!(f, "Signal Processing Controller"),
            DeviceClass::ProcessingAccelerator => write!(f, "Processing Accelerators"),
            DeviceClass::NonEssentialInstrumentation => write!(f, "Non Essential Instrumentation"),
        }
    }
}
