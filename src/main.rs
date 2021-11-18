use tokio::sync::mpsc;

fn main() {
    route();
}

// #["/create-pointer", "GET"]
fn route() {
    biz_logix();
}

fn biz_logix() {
    for x in (1..5) {
        println!("Call number {}", x);
        call_service();
    }
}

async fn call_service() {
    // create_service_conn();
    println!("Done!")
}

fn create_service_conn() {
    let sleep_dur = std::time::Duration::from_secs(2);
    std::thread::sleep(sleep_dur);
}

// Client Pool
async fn run_client_pool(mut receiver: mpsc::Receiver<&str>) {
    create_service_conn();
    while let Some(message) = receiver.recv().await {
        call_service().await
    }
}
