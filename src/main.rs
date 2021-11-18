use tokio::sync::mpsc;

#[tokio::main]
async fn main() {
    let (tx, mut rx) = mpsc::channel(32);

    tokio::spawn(async move { run_client_pool(rx).await });

    route(tx).await;
}

// #["/create-pointer", "GET"]
async fn route(tx: mpsc::Sender<String>) {
    biz_logix(tx).await;
}

async fn biz_logix(tx: mpsc::Sender<String>) {
    for x in 1..=50 {
        println!("Call number {}", x);
        let tx_clone = tx.clone();
        tx_clone
            .send(format!("Call number {} done!!", x))
            .await
            .unwrap()
    }
}

async fn call_service(msg: &str) {
    // create_service_conn();
    println!("{}", msg)
}

fn create_service_conn() {
    let sleep_dur = std::time::Duration::from_secs(2);
    std::thread::sleep(sleep_dur);
}

// Client Pool
async fn run_client_pool(mut receiver: mpsc::Receiver<String>) {
    create_service_conn();
    while let Some(message) = receiver.recv().await {
        call_service(&message).await
    }
}
