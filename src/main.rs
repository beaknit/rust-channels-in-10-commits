use std::fmt::Debug;
use tokio::sync::mpsc;

#[tokio::main]
async fn main() {
    let (tx, mut rx) = mpsc::channel(32);

    tokio::spawn(async move { run_client_pool(rx).await });

    route(tx).await;
}

// #["/create-pointer", "GET"]
async fn route(tx: mpsc::Sender<ServiceNum>) {
    biz_logix(tx).await;
}

async fn biz_logix(tx: mpsc::Sender<ServiceNum>) {
    for x in 1..=50 {
        println!("Call number {}", x);

        let msg1 = Message::new(x);
        let svc1msg = ServiceNum::SvcOne(msg1);
        send_msg(&tx, svc1msg).await;

        let msg2 = Message::new(format!("Call number {} done!!", x));
        let svc2msg = ServiceNum::SvcTwo(msg2);
        send_msg(&tx, svc2msg).await;
    }
}

async fn send_msg(tx: &mpsc::Sender<ServiceNum>, svc_num: ServiceNum) {
    let tx_clone = tx.clone();
    tx_clone.send(svc_num).await.unwrap();
}

async fn call_service1(msg: &str) {
    // create_service_conn();
    println!("Service 1: {}", msg)
}

async fn call_service2(msg: &str) {
    // create_service_conn();
    println!("Service 2: {}", msg)
}

fn create_service_conn() {
    let sleep_dur = std::time::Duration::from_secs(2);
    std::thread::sleep(sleep_dur);
}

// Client Pool
async fn run_client_pool(mut receiver: mpsc::Receiver<ServiceNum>) {
    create_service_conn();
    while let Some(message) = receiver.recv().await {
        match message {
            ServiceNum::SvcOne(m) => call_service1(&m.value).await,
            ServiceNum::SvcTwo(m) => call_service2(&m.value).await,
        }
    }
}

#[derive(Debug)]
struct Message {
    value: String,
}

impl Message {
    fn new<T: std::fmt::Display>(payload: T) -> Message {
        Message {
            value: payload.to_string(),
        }
    }
}

#[derive(Debug)]
enum ServiceNum {
    SvcOne(Message),
    SvcTwo(Message),
}
