use pcap::{Device, Capture, Error};
use tokio::sync::mpsc::UnboundedSender;
use std::{thread, time};


pub fn initialize(data_tx: UnboundedSender<String>) {
    tokio::spawn(async move {
        let mut retries = 3;  
        let retry_delay = time::Duration::from_secs(5);

        loop {
            match setup_capture() {
                Ok(mut cap) => {
                    while let Ok(packet) = cap.next() {
                        let data = format!("Captured packet: {:?}", packet);
                        if data_tx.send(data).is_err() {
                            eprintln!("Error sending data to channel");
                            break;
                        }
                    }
                },
                Err(e) => {
                    eprintln!("Error setting up capture: {:?}", e);
                    if retries > 0 {
                        retries -= 1;
                        eprintln!("Retrying in {:?} seconds...", retry_delay.as_secs());
                        thread::sleep(retry_delay);
                    } else {
                        eprintln!("Failed to start capture after retries, shutting down.");
                        break;
                    }
                }
            }
        }
    });
}


fn setup_capture() -> Result<Capture<Device>, Error> {
    let device = Device::lookup()?;
    let cap = Capture::from_device(device)?.open()?;
    Ok(cap)
}
