use std::fmt::Debug;
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
        let msg1 = Message { value: x };
        send_msg(&tx, msg1);

        let msg2 = Message {
            value: format!("Call number {} done!!", x),
        };
        send_msg(&tx, msg2);
    }
}

async fn send_msg(tx: &mpsc::Sender<Message>, msg: Message) {
    let tx_clone = tx.clone();
    tx_clone.send(msg).await.unwrap();
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
struct Message<T> {
    value: T,
}
