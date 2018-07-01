extern crate amethyst;
extern crate fern;

use std::thread::{self, JoinHandle};
use std::time::Duration;

use amethyst::{
    prelude::*,
    renderer::{ColorMask, DisplayConfig, DrawFlat, Pipeline, PosTex, RenderBundle, Stage, ALPHA},
};

const THREAD_COUNT: usize = 10;

fn open_window() {
    let display_config = DisplayConfig {
        title: "test".to_string(),
        fullscreen: false,
        dimensions: Some((800, 600)),
        min_dimensions: Some((400, 300)),
        max_dimensions: None,
        vsync: false,
        multisampling: 0, // Must be multiple of 2, use 0 to disable
        visibility: false,
    };
    let pipe = Pipeline::build().with_stage(
        Stage::with_backbuffer()
            .clear_target([0., 0., 0., 0.], 1.)
            .with_pass(DrawFlat::<PosTex>::new().with_transparency(ColorMask::all(), ALPHA, None)),
    );
    let game_data = GameDataBuilder::default()
        .with_bundle(RenderBundle::new(pipe, Some(display_config)))
        .unwrap();
    Application::new("assets", EmptyState, game_data)
        .unwrap()
        .run();
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
    fern::Dispatch::new().apply().unwrap(); // Disables noisy logging

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

// Boilerplate

#[derive(Debug)]
pub struct EmptyState;
impl<T> State<T> for EmptyState {
    fn update(&mut self, _data: StateData<T>) -> Trans<T> {
        Trans::Pop
    }
}
