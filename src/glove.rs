use btleplug::api::{Central, CharPropFlags, Manager as _, Peripheral, ScanFilter};
use btleplug::platform::Manager;
use futures::stream::StreamExt;
use std::error::Error;
use std::time::Duration;
use uuid::Uuid;
use std::str;
use std::sync::{Arc, Mutex};
use tokio::time::sleep;

use crate::hand::Hand;

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
        eprintln!("No Bluetooth adapters found!");
    }

    for adapter in adapter_list.iter()
    {
        println!("Starting scan...");
        adapter
            .start_scan(ScanFilter::default())
            .await
            .expect("Can't scan using the selected BLE adapter!");
        sleep(Duration::from_secs(2)).await;
        let peripherals = adapter.peripherals().await?;

        if peripherals.is_empty()
        {
            eprintln!("No peripheral found!");
        } 
        else
        {
            // All peripheral devices in range.
            for peripheral in peripherals.iter() {
                let properties = peripheral.properties().await?;
                let is_connected = peripheral.is_connected().await?;
                let found_local_name = properties
                    .unwrap()
                    .local_name
                    .unwrap_or(String::from("Unknown"));
                println!(
                    "Peripheral {:?} is connected: {:?}",
                    &found_local_name, is_connected
                );
                // Check if it's the peripheral we want.
                if found_local_name.eq(local_name) {
                    println!("Found matching peripheral {:?}...", &found_local_name);
                    if !is_connected {
                        // Connect if we aren't already connected.
                        if let Err(err) = peripheral.connect().await {
                            eprintln!("Error connecting to peripheral, skipping: {}", err);
                            continue;
                        }
                    }
                    let is_connected = peripheral.is_connected().await?;
                    println!(
                        "Now connected ({:?}) to peripheral {:?}.",
                        is_connected, &local_name
                    );
                    if is_connected {
                        println!("Discover peripheral {:?} services...", local_name);
                        peripheral.discover_services().await?;
                        for characteristic in peripheral.characteristics() {
                            println!("Checking characteristic {:?}", characteristic);
                            // Subscribe to notifications from the characteristic with the selected
                            // UUID.
                            if characteristic.uuid == data_uuid
                                && characteristic.properties.contains(CharPropFlags::NOTIFY)
                            {
                                println!("Subscribing to characteristic {:?}", characteristic.uuid);
                                peripheral.subscribe(&characteristic).await?;

                                let mut notification_stream =
                                    peripheral.notifications().await?;
                                // Process while the BLE connection is not broken or stopped.
                                while let Some(data) = notification_stream.next().await {
                                    // Convert serialized data from bytes to UTF8 string
                                    let serialized_data = match str::from_utf8(&data.value) {
                                        Ok(v) => v,
                                        Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
                                    };

                                    println!(
                                        "Received data from {:?} [{:?}]: {:?}",
                                        found_local_name, data.uuid, serialized_data
                                    );
                                    
                                    let mut hand = hand.lock().unwrap();
                                    (*hand).update_model(serde_json::from_str(serialized_data)?);
                                }
                            }
                        }
                        println!("Disconnecting from peripheral {:?}...", found_local_name);
                        peripheral.disconnect().await?;
                    }
                }
                else
                {
                    println!("Skipping unknown peripheral {:?}", peripheral);
                }
            }
        }
    }

    Ok(())
}