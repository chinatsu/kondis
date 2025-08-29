use std::sync::mpsc::Receiver;

use async_trait::async_trait;

mod bluetooth;
pub mod devices;
mod ftms;

use devices::{DebugBike, Iconsole0028Bike, NonBluetoothDevice};

/// Equipment types supported
///
/// This enum represents the different types of equipment that can be used.
///
/// If you are contributing a new type of equipment, please add it here as well.
#[allow(dead_code)]
pub enum EquipmentType {
    /// iConsole+0028 bike
    Iconsole0028Bike,
    /// debug bike, any bluetooth bike containing "Console" in its name
    DebugBike,
    /// a bogus device, implemented without any connection, printing states when functions are called
    NonBluetoothDevice,
}

/// Equipment trait for all equipment types
#[async_trait]
pub trait Equipment {
    /// Create a new instance of the equipment.
    /// `max_level` is used to prevent the equipment from being set to a level higher than its capabilities
    /// `shutdown_rx`` is used in the discovery/scanning phase of connecting to the equipment, allowing it to be shut down gracefully
    ///
    /// # Examples
    /// ```
    /// use kondis::{devices::NonBluetoothDevice, Equipment};
    ///
    /// #[tokio::main]
    /// async fn main() -> anyhow::Result<()> {
    ///     let (_, mut shutdown_rx) = std::sync::mpsc::channel();
    ///     let device = NonBluetoothDevice::new(32, &mut shutdown_rx).await?;
    ///
    ///     Ok(())
    /// }
    /// ```
    async fn new(max_level: i16, shutdown_rx: &mut Receiver<()>) -> anyhow::Result<Self>
    where
        Self: Sized;
    /// Connect to the equipment, discover its capabilities for reading and writing
    ///
    /// # Examples
    ///
    /// ```
    /// use kondis::{devices::NonBluetoothDevice, Equipment};
    ///
    /// #[tokio::main]
    /// async fn main() -> anyhow::Result<()> {
    ///     let (_, mut shutdown_rx) = std::sync::mpsc::channel();
    ///     let mut device = NonBluetoothDevice::new(32, &mut shutdown_rx).await?;
    ///     device.connect().await?;
    ///     Ok(())
    /// }
    /// ```
    async fn connect(&mut self) -> anyhow::Result<bool>;
    /// Disconnect from the equipment, disconnecting from any subscriptions, and sending any stop signals if required
    ///
    /// # Examples
    ///
    /// ```
    /// use kondis::{devices::NonBluetoothDevice, Equipment};
    ///
    /// #[tokio::main]
    /// async fn main() -> anyhow::Result<()> {
    ///     let (_, mut shutdown_rx) = std::sync::mpsc::channel();
    ///     let mut device = NonBluetoothDevice::new(32, &mut shutdown_rx).await?;
    ///     device.connect().await?;
    ///     device.disconnect().await?;
    ///     Ok(())
    /// }
    async fn disconnect(&self) -> anyhow::Result<()>;
    /// Set the equipment target cadence
    ///
    /// # Examples
    ///
    /// ```
    /// use kondis::{devices::NonBluetoothDevice, Equipment};
    ///
    /// #[tokio::main]
    /// async fn main() -> anyhow::Result<()> {
    ///     let (_, mut shutdown_rx) = std::sync::mpsc::channel();
    ///     let mut device = NonBluetoothDevice::new(32, &mut shutdown_rx).await?;
    ///     device.connect().await?;
    ///     device.set_target_cadence(32).await?;
    ///     Ok(())
    /// }
    async fn set_target_cadence(&self, rpm: i16) -> anyhow::Result<()>;
    /// Set the equipment target power
    ///
    /// # Examples
    ///
    /// ```
    /// use kondis::{devices::NonBluetoothDevice, Equipment};
    ///
    /// #[tokio::main]
    /// async fn main() -> anyhow::Result<()> {
    ///     let (_, mut shutdown_rx) = std::sync::mpsc::channel();
    ///     let mut device = NonBluetoothDevice::new(32, &mut shutdown_rx).await?;
    ///     device.connect().await?;
    ///     device.set_target_power(32).await?;
    ///     Ok(())
    /// }
    async fn set_target_power(&self, watts: i16) -> anyhow::Result<()>;
    /// Read the latest notification received and process it to an easy to use FTMS format
    ///
    /// # Examples
    ///
    /// ```
    /// use kondis::{devices::NonBluetoothDevice, Equipment};
    ///
    /// #[tokio::main]
    /// async fn main() -> anyhow::Result<()> {
    ///     let (_, mut shutdown_rx) = std::sync::mpsc::channel();
    ///     let mut device = NonBluetoothDevice::new(32, &mut shutdown_rx).await?;
    ///     device.connect().await?;
    ///     println!("{:?}", device.read().await?);
    ///     Ok(())
    /// }
    /// ```
    async fn read(&self) -> anyhow::Result<Option<ftms::FTMSData>>;
}

/// Convert an equipment type to an instance of an equipment
///
/// This function takes an `EquipmentType`, a maximum resistance level, and a shutdown receiver,
/// and returns an instance of the corresponding equipment type.
///
/// If you are contributing a new type of equipment, remember to add it here as well.
///
/// # Examples
///
/// ```
/// use kondis::{EquipmentType, equipment_type_to_equipment, Equipment};
///
/// #[tokio::main]
/// async fn main() -> anyhow::Result<()> {
///     let (_, mut shutdown_rx) = std::sync::mpsc::channel();
///     let equipment = equipment_type_to_equipment(EquipmentType::NonBluetoothDevice, 10, &mut shutdown_rx).await;
///     if let Some(mut equip) = equipment {
///         equip.connect().await?;
///         equip.read().await?;
///     }
///     Ok(())
/// }
/// ```
pub async fn equipment_type_to_equipment(
    equipment_type: EquipmentType,
    max_level: i16,
    shutdown_rx: &mut Receiver<()>,
) -> Option<Box<dyn Equipment>> {
    match equipment_type {
        EquipmentType::Iconsole0028Bike => {
            let equip = Iconsole0028Bike::new(max_level, shutdown_rx).await;
            if equip.is_err() {
                return None;
            }
            Some(Box::new(equip.unwrap()))
        }
        EquipmentType::DebugBike => {
            let equip = DebugBike::new(max_level, shutdown_rx).await;
            if equip.is_err() {
                return None;
            }
            Some(Box::new(equip.unwrap()))
        }
        EquipmentType::NonBluetoothDevice => {
            let equip = NonBluetoothDevice::new(max_level, shutdown_rx).await;
            if equip.is_err() {
                return None;
            }
            Some(Box::new(equip.unwrap()))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_equipment_type_to_equipment() -> anyhow::Result<()> {
        let (_, mut shutdown_rx) = std::sync::mpsc::channel();
        let equipment =
            equipment_type_to_equipment(EquipmentType::NonBluetoothDevice, 10, &mut shutdown_rx)
                .await;
        assert!(equipment.is_some());
        let mut equipment = equipment.unwrap();
        assert!(equipment.connect().await?);
        assert!(equipment.read().await?.is_some());
        Ok(())
    }

    #[tokio::test]
    async fn test_shutdown_while_scanning() -> anyhow::Result<()> {
        let (shutdown_tx, mut shutdown_rx) = std::sync::mpsc::channel();

        let _ = shutdown_tx.send(());

        let equipment = Iconsole0028Bike::new(10, &mut shutdown_rx).await;

        assert!(equipment.is_err());
        Ok(())
    }

    #[tokio::test]
    async fn test_shutdown_while_scanning_from_equipment_type() -> anyhow::Result<()> {
        let (shutdown_tx, mut shutdown_rx) = std::sync::mpsc::channel();

        let _ = shutdown_tx.send(());

        let equipment =
            equipment_type_to_equipment(EquipmentType::Iconsole0028Bike, 10, &mut shutdown_rx)
                .await;

        assert!(equipment.is_none());
        Ok(())
    }
}
