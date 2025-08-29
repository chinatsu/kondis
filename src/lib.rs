use std::sync::mpsc::Receiver;

use async_trait::async_trait;

mod bluetooth;
mod devices;
mod ftms;

use devices::{DebugBike, Iconsole0028Bike, NonBluetoothDevice};

/// Equipment types supported
///
/// This enum represents the different types of equipment that can be used.
///
/// If you are contributing a new type of equipment, please add it here as well.
#[allow(dead_code)]
pub enum EquipmentType {
    Iconsole0028Bike,
    DebugBike,
    NonBluetoothDevice,
}

/// Equipment trait for all equipment types
#[async_trait]
pub trait Equipment {
    /// Create a new instance of the equipment.
    /// `max_level` is used to prevent the equipment from being set to a level higher than its capabilities
    /// `shutdown_rx`` is used in the discovery/scanning phase of connecting to the equipment, allowing it to be shut down gracefully
    async fn new(max_level: i16, shutdown_rx: &mut Receiver<()>) -> anyhow::Result<Self>
    where
        Self: Sized;
    /// Connect to the equipment, discover its capabilities for reading and writing
    async fn connect(&mut self) -> anyhow::Result<bool>;
    /// Disconnect from the equipment, disconnecting from any subscriptions, and sending any stop signals if required
    async fn disconnect(&self) -> anyhow::Result<()>;
    /// Set the equipment resistance
    /// `level` is the desired resistance level to set, and is used as target watts and target RPM
    async fn set_level(&self, level: i16) -> anyhow::Result<()>;
    /// Read the latest notification received and process it to an easy to use FTMS format
    async fn read(&self) -> anyhow::Result<Option<ftms::FTMSData>>;
}

/// Convert an equipment type to an instance of an equipment
///
/// This function takes an `EquipmentType`, a maximum resistance level, and a shutdown receiver,
/// and returns an instance of the corresponding equipment type.
///
/// If you are contributing a new type of equipment, remember to add it here as well.
pub async fn equipment_type_to_equipment(
    equipment_type: EquipmentType,
    max_level: i16,
    shutdown_rx: &mut Receiver<()>,
) -> Option<Box<dyn Equipment>> {
    match equipment_type {
        EquipmentType::Iconsole0028Bike => Some(Box::new(
            Iconsole0028Bike::new(max_level, shutdown_rx).await.unwrap(),
        )),
        EquipmentType::DebugBike => Some(Box::new(
            DebugBike::new(max_level, shutdown_rx).await.unwrap(),
        )),
        EquipmentType::NonBluetoothDevice => Some(Box::new(
            NonBluetoothDevice::new(max_level, shutdown_rx)
                .await
                .unwrap(),
        )),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_equipment_type_to_equipment() {
        let (_, mut shutdown_rx) = std::sync::mpsc::channel();
        let mut equipment =
            equipment_type_to_equipment(EquipmentType::NonBluetoothDevice, 10, &mut shutdown_rx)
                .await
                .unwrap();
        assert!(equipment.connect().await.unwrap());
        assert!(equipment.read().await.unwrap().is_some());
    }
}
