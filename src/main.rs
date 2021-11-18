fn main() {
    route();
}

// #["/create-pointer", "GET"]
fn route() {
    biz_logix();
}

fn biz_logix() {
    call_service();
}

fn call_service() {
    create_service_conn();
    println!("Done!")
}

fn create_service_conn() {
    let sleep_dur = std::time::Duration::from_secs(2);
    std::thread::sleep(sleep_dur);
}