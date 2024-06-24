mod analysis;
mod capture;
mod ui;



#[tokio::main]
async fn main() {
    println!("Starting High-Performance Network Monitoring Tool...");

    let (data_tx, mut data_rx) = tokio::sync::mpsc::unbounded_channel();
    
    capture::initialize(data_tx.clone());

    tokio::spawn(async move {
        while let Some(data) = data_rx.recv().await {
            let analyzed_data = analysis::start_analysis(&data);
            data_tx.send(analyzed_data).unwrap();
        }
    });
    

    ui::start_server().await;
}
