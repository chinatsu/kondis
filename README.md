# kondis

***kondis*** [n, slang] *physical condition*

a simple library to communicate with exercise equipment

## supports

- [ ] iConsole+0028
    - [x] set target cadence (RPM)
    - [x] set target power (W)
        - **note** target cadence and target power is currently linked to the same number with `bike.set_level(n)`.
    - [ ] read FTMS data (kind of, incomplete)

## usage

```
cargo add kondis
```

also, needs tokio and wants anyhow.

```rust
use kondis::equipment_type_to_equipment;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let (shutdown_tx, mut shutdown_rx) = channel();
    tokio::spawn(async move {
        tokio::signal::ctrl_c().await.unwrap();
        shutdown_tx.send(()).unwrap();
    });

    let mut equipment = equipment_type_to_equipment(
        "device".into(),
        32,
        &mut shutdown_rx,
    )
    .await
    .unwrap();
    if !equipment.connect().await? {
        return Ok(());
    }
    loop {
        if let Some(data) = equipment.read().await? {
            let state = format!(
                "{:03} rpm :: {:03} W :: {:.2} km/h",
                data.cadence, data.power, data.speed
            );
            println!("{state}");
        }
    }
    equipment.disconnect().await?;
}
```