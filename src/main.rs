use tokio::sync::mpsc;

#[tokio::main]
async fn main() {
    let (tx, mut rx) = mpsc::channel(32);

    tokio::spawn(async move { run_client_pool(rx).await });

    route(tx).await;
}

// #["/create-pointer", "GET"]
async fn route(tx: mpsc::Sender<Message>) {
    biz_logix(tx).await;
}

async fn biz_logix(tx: mpsc::Sender<Message>) {
    for x in 1..=50 {
        println!("Call number {}", x);
        let tx_clone = tx.clone();
        let msg = Message {
            value: format!("Call number {} done!!", x),
        };
        tx_clone.send(msg).await.unwrap();

        let msg2 = Message { value: 2 };
        tx_clone.send(msg2).await.unwrap()
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
async fn run_client_pool(mut receiver: mpsc::Receiver<Message>) {
    create_service_conn();
    while let Some(message) = receiver.recv().await {
        call_service(&message.value).await
    }
}

#[derive(Debug)]
struct Message {
    value: String,
}
