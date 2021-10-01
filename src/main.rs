use std::{process::Command, thread, time};
fn main() {
    Command::new("shutdown")
        .args(["-s", "-t", "10800"])
        .output()
        .expect("Failed");

    thread::sleep(time::Duration::from_millis(1000));
}
