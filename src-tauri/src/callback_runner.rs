use std::{
    sync::{Arc, Mutex},
    thread,
    time::Duration,
};

use steamworks::SingleClient;

pub struct Callback_Runner {
    done_mutex: Arc<Mutex<bool>>,
}

impl Callback_Runner {
    pub fn new(simple: SingleClient) -> Self {
        let done_mutex = Arc::new(Mutex::new(false));
        let clone_mutex = done_mutex.clone();
        thread::spawn(move || loop {
            simple.run_callbacks();
            if let Ok(done) = clone_mutex.lock() {
                if *done {
                    break;
                }
            } else {
                break;
            }
            thread::sleep(Duration::from_millis(200));
        });

        return Self {
            done_mutex
        };
    }
}

impl Drop for Callback_Runner {
    fn drop(&mut self) {
        if let Ok(mut done) = self.done_mutex.lock() {
            *done = true;
        }
    }
}
