use std::sync::{Arc, Mutex};
use std::thread;

struct AppState {
    counter: u32,
}

pub fn hello_arc_1() {
    let state = Arc::new(Mutex::new(AppState { counter: 0 }));
    let mut handles = vec![];

    for _ in 0..10 {
        let state_clone = Arc::clone(&state);

        let handle = thread::spawn(move || {
            let mut state = state_clone.lock().unwrap();
            state.counter += 1;
        });

        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Final counter value: {}", state.lock().unwrap().counter);
}
