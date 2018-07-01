extern crate glutin;

use std::thread::{self, JoinHandle};
use std::time::Duration;

use glutin::{GlContext, GlProfile, GlRequest};

const THREAD_COUNT: usize = 10;

fn open_window() {
    let mut events_loop = glutin::EventsLoop::new();

    let context = glutin::ContextBuilder::new()
        .with_multisampling(0)
        .with_vsync(false)
        .with_gl_profile(GlProfile::Core)
        .with_gl(GlRequest::Latest);

    let window = glutin::WindowBuilder::new();
    let gl_window = glutin::GlWindow::new(window, context, &events_loop).unwrap();

    let _ = unsafe { gl_window.make_current() };

    // println!(
    //     "Pixel format of the window's GL context: {:?}",
    //     gl_window.get_pixel_format()
    // );

    let mut running = true;
    while running {
        events_loop.poll_events(|event| {
            // println!("{:?}", event);
            if let glutin::Event::WindowEvent { event, .. } = event {
                match event {
                    glutin::WindowEvent::CloseRequested => running = false,
                    glutin::WindowEvent::Resized(logical_size) => {
                        let dpi_factor = gl_window.get_hidpi_factor();
                        gl_window.resize(logical_size.to_physical(dpi_factor));
                    }
                    _ => (),
                }
            }
        });

        let _ = gl_window.swap_buffers();
    }
}

fn wait_for_threads<T>(thread_handles: Vec<JoinHandle<T>>) {
    for handle in thread_handles {
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
        let child = thread::spawn(open_window);
        thread_handles.push(child);
    }

    thread::spawn(move || wait_for_threads(thread_handles));
    println!("Spawned threads.");

    thread::sleep(Duration::from_millis(400));
    println!("Waited for 400 ms.");
}
