use std::{sync::{mpsc::{self, Receiver}, Arc, Mutex}, thread, time::Duration};

use steamworks::{Client, SingleClient, UserStatsReceived};
use tauri::async_runtime::{block_on, channel};

pub async fn request_current_stats(client: &Client, simple: SingleClient) {
    let done_mutex = Arc::new(Mutex::new(false));
    let clone_mutex = done_mutex.clone();
    let (sender, mut receiver) = channel::<bool>(1);
    let thread = thread::spawn(move || loop {
        simple.run_callbacks();
        if let Ok(done) = clone_mutex.lock() {
            if (*done) {
                break;
            }
        } else {
            break;
        }
        thread::sleep(Duration::from_millis(200));
    });
    client.register_callback(move |val: UserStatsReceived| {
        block_on(sender.send(val.result.is_ok())).expect("Doesn't work");
    });
    client.user_stats().request_current_stats();
    receiver.recv().await;
    if let Ok(mut done) = done_mutex.lock() {
        *done = true;
    }
    return;
}