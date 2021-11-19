use std::fmt::Debug;
use tokio::sync::{mpsc, oneshot};

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

        let (ostx1, osrx1) = oneshot::channel();
        let msg1 = Message::new(x, ostx1);
        let svc1msg = ServiceNum::SvcOne(msg1);
        send_msg(&tx, svc1msg).await;
        println!("{}", osrx1.await.unwrap());

        let (ostx2, osrx2) = oneshot::channel();
        let msg2 = Message::new(format!("Call number {} done!!", x), ostx2);
        let svc2msg = ServiceNum::SvcTwo(msg2);
        send_msg(&tx, svc2msg).await;
        println!("{}", osrx2.await.unwrap());
    }
}

async fn send_msg(tx: &mpsc::Sender<ServiceNum>, svc_num: ServiceNum) {
    let tx_clone = tx.clone();
    tx_clone.send(svc_num).await.unwrap();
}

async fn call_service1(msg: Message) {
    // create_service_conn();
    msg.response_chan.send(format!("Service 1: {}", msg.value));
}

async fn call_service2(msg: Message) {
    // create_service_conn();
    msg.response_chan.send(format!("Service 2: {}", msg.value));
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
            ServiceNum::SvcOne(m) => call_service1(m).await,
            ServiceNum::SvcTwo(m) => call_service2(m).await,
        }
    }
}

#[derive(Debug)]
struct Message {
    value: String,
    response_chan: oneshot::Sender<String>,
}

impl Message {
    fn new<T: std::fmt::Display>(payload: T, response_chan: oneshot::Sender<String>) -> Message {
        Message {
            value: payload.to_string(),
            response_chan,
        }
    }
}

#[derive(Debug)]
enum ServiceNum {
    SvcOne(Message),
    SvcTwo(Message),
}
