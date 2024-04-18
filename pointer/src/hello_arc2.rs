use std::sync::{Arc, Mutex};
use std::thread;

struct App {
    state: Mutex<u32>,
}

impl App {
    fn new() -> Arc<App> {
        Arc::new(App {
            state: Mutex::new(0),
        })
    }

    fn increment(&self) {
        let mut num = self.state.lock().unwrap();
        *num += 1;
    }

    fn run(self: Arc<Self>) {
        let mut handles = vec![];

        for _ in 0..10 {
            let arc_clone = self.clone();
            let handle = thread::spawn(move || {
                arc_clone.increment();
            });
            handles.push(handle);
        }

        for handle in handles {
            handle.join().unwrap();
        }

        println!("Final state value: {}", *self.state.lock().unwrap());
    }
}

pub fn hello_arc_2() {
    let app = App::new();
    app.run();
}
