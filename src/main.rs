extern crate gfx;
extern crate gfx_window_glutin;
extern crate glutin;

use std::thread::{self, JoinHandle};
use std::time::Duration;

use glutin::{GlContext, GlProfile, GlRequest};

const THREAD_COUNT: usize = 10;

// Amethyst uses these
pub type SurfaceFormat = gfx::format::R8_G8_B8_A8;
pub type ChannelFormat = gfx::format::Unorm;
pub type ColorFormat = (SurfaceFormat, ChannelFormat);
pub type DepthFormat = gfx::format::DepthStencil;

fn open_window() {
    let window_builder = glutin::WindowBuilder::new()
        .with_visibility(false)
        .with_dimensions((800, 600).into());
    let mut events_loop = glutin::EventsLoop::new();
    let context = glutin::ContextBuilder::new()
        .with_multisampling(0)
        .with_vsync(false)
        .with_gl_profile(GlProfile::Core)
        .with_gl(GlRequest::Latest);

    let (gl_window, _dev, _fac, mut color, mut depth) =
        gfx_window_glutin::init::<ColorFormat, DepthFormat>(window_builder, context, &events_loop);

    // ---

    let _ = unsafe { gl_window.make_current() };

    gfx_window_glutin::update_views(&gl_window, &mut color, &mut depth);

    let mut running = true;
    while running {
        events_loop.poll_events(|event| {
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
