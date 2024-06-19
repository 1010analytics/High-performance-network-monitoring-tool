
use pcap::{Device, Capture};

pub fn initialize(data_tx: tokio::sync::mpsc::UnboundedSender<String>) {
    tokio::spawn(async move {
        let device = Device::lookup().unwrap();
        let mut cap = Capture::from_device(device).unwrap()
            .open().unwrap();

        while let Ok(packet) = cap.next() {
            let data = format!("Captured packet: {:?}", packet);
            data_tx.send(data).unwrap();
        }
    });
}
