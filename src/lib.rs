use std::sync::mpsc::Receiver;

use async_trait::async_trait;

mod bluetooth;
mod devices;
mod ftms;

use devices::{DebugBike, Iconsole0028Bike, NonBluetoothDevice};

#[allow(dead_code)]
pub enum EquipmentType {
    Iconsole0028Bike,
    DebugBike,
    NonBluetoothDevice,
}

#[async_trait]
pub trait Equipment {
    async fn new(max_level: i16, shutdown_rx: &mut Receiver<()>) -> anyhow::Result<Self>
    where
        Self: Sized;
    async fn connect(&mut self) -> anyhow::Result<bool>;
    async fn disconnect(&self) -> anyhow::Result<()>;
    async fn set_level(&self, level: i16) -> anyhow::Result<()>;
    async fn read(&self) -> anyhow::Result<Option<ftms::FTMSData>>;
}

pub async fn equipment_type_to_equipment(
    name: String,
    max_level: i16,
    shutdown_rx: &mut Receiver<()>,
) -> Option<Box<dyn Equipment>> {
    match name.as_str() {
        "28" => Some(Box::new(
            Iconsole0028Bike::new(max_level, shutdown_rx).await.unwrap(),
        )),
        "debug" => Some(Box::new(
            DebugBike::new(max_level, shutdown_rx).await.unwrap(),
        )),
        "device" => Some(Box::new(
            NonBluetoothDevice::new(max_level, shutdown_rx)
                .await
                .unwrap(),
        )),
        _ => {
            eprintln!("Unknown bike type: {name}");
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_equipment_type_to_equipment() {
        let (_, mut shutdown_rx) = std::sync::mpsc::channel();
        let mut equipment = equipment_type_to_equipment("device".into(), 10, &mut shutdown_rx)
            .await
            .unwrap();
        assert!(equipment.connect().await.unwrap());
        assert!(equipment.read().await.unwrap().is_some());
    }
}
