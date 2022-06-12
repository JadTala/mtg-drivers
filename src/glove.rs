use btleplug::api::{Central, CharPropFlags, Manager as _, Peripheral, ScanFilter};
use btleplug::platform::Manager;
use futures::stream::StreamExt;
use log::{error, info, debug};
use std::error::Error;
use std::time::Duration;
use uuid::Uuid;
use std::str;
use std::sync::{Arc, Mutex};
use tokio::time::sleep;
use ansi_term::Colour::Green;
use ansi_term::Colour::Black;

use crate::hand::{Hand, HandModel};

pub fn connect(local_name: &'static str, data_uuid: Uuid) -> Arc<Mutex<Hand>>
{
    pretty_env_logger::init();

    // Hand model data concurrency
    let hand_original = Arc::new(Mutex::new(Hand::default()));

    let hand = hand_original.clone();
    // Connect to the glove and subscribe for updates
    tokio::spawn(async move {
        subscribe(hand, local_name, data_uuid).await.unwrap();
        
        Ok::<_, btleplug::Error>(())
    });

    hand_original.clone()
}

async fn subscribe(hand: Arc<Mutex<Hand>>, local_name: &str, data_uuid: Uuid) -> core::result::Result<(), Box<dyn Error>>
{
    let manager = Manager::new().await?;
    let adapter_list = manager.adapters().await?;
    if adapter_list.is_empty()
    {
        error!("No Bluetooth adapters found!");
    }

    for adapter in adapter_list.iter()
    {
        info!("{}", Black.underline().paint("Starting scan..."));
        adapter
            .start_scan(ScanFilter::default())
            .await
            .expect("Can't scan using the selected BLE adapter!");
        sleep(Duration::from_secs(2)).await;
        let peripherals = adapter.peripherals().await?;

        if peripherals.is_empty()
        {
            error!("No peripheral found!");
        } 
        else
        {
            // All peripheral devices in range.
            for peripheral in peripherals.iter() {
                let properties = peripheral.properties().await?;
                let is_connected = peripheral.is_connected().await?;
                let target_local_name = properties
                    .unwrap()
                    .local_name
                    .unwrap_or(String::from("Unknown"));
                info!(
                    "Peripheral {} found.",
                    Green.bold().paint(&target_local_name)
                );
                // Check if it's the peripheral we want.
                if target_local_name.eq(local_name) {
                    info!("Found matching peripheral {}.", Green.bold().paint(&target_local_name));
                    if !is_connected {
                        // Connect if we aren't already connected.
                        if let Err(err) = peripheral.connect().await {
                            error!("Error connecting to peripheral, skipping: {}", err);
                            continue;
                        }
                    }
                    let is_connected = peripheral.is_connected().await?;
                    debug!(
                        "Now connected ({:?}) to peripheral {:?}.",
                        is_connected, &local_name
                    );
                    if is_connected {
                        info!("{}", Black.underline().paint("Discovering peripheral services..."));
                        peripheral.discover_services().await?;
                        for characteristic in peripheral.characteristics() {
                            info!("Checking characteristic {:?}.", characteristic);
                            // Subscribe to notifications from the characteristic with the selected
                            // UUID.
                            if characteristic.uuid == data_uuid
                                && characteristic.properties.contains(CharPropFlags::NOTIFY)
                            {
                                info!("Subscribing to characteristic {:?}.", characteristic.uuid);
                                peripheral.subscribe(&characteristic).await?;
                                
                                let mut notification_stream =
                                    peripheral.notifications().await?;
                                // Process while the BLE connection is not broken or stopped.
                                while let Some(data) = notification_stream.next().await {
                                    let mut hand = hand.lock().unwrap();
                                    (*hand).update_model(HandModel::from_raw_data(data.value));
                                }
                            }
                        }
                        info!("Disconnecting from peripheral {:?}...", Green.bold().paint(target_local_name));
                        peripheral.disconnect().await?;
                    }
                }
                else
                {
                    info!("{}", Black.italic().paint("Not the desired peripheral, skipping..."));
                }
            }
        }
    }

    Ok(())
}