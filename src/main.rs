extern crate winit;

use std::thread::{self, JoinHandle};
use std::time::Duration;

use winit::{ControlFlow, EventsLoop, Window};

const THREAD_COUNT: usize = 10;

fn open_window() {
    let mut events_loop = EventsLoop::new();
    let _window = Window::new(&events_loop).unwrap();
    events_loop.run_forever(|_| ControlFlow::Break);
}

fn wait_for_threads<T>(thread_handles: Vec<JoinHandle<T>>) {
    for handle in thread_handles.into_iter() {
        let thread_id = handle.thread().id();
        match handle.join() {
            Ok(_) => println!("{:?}: success", thread_id),
            Err(ref _e) => println!("{:?}: fail", thread_id),
        };
    }
}

fn main() {
    let mut thread_handles = vec![];

    for _ in 0..THREAD_COUNT {
        let child = thread::spawn(|| open_window());
        thread_handles.push(child);
    }

    thread::spawn(move || wait_for_threads(thread_handles));
    println!("Spawned threads.");

    thread::sleep(Duration::from_millis(100));
    println!("Waited for 100 ms.");
}
