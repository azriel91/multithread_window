# Multithread Window

(Sometimes) reproduces https://github.com/tomaka/glutin/issues/1038


1. Run with `cargo run`, or if you've compiled it: `for i in {1..20}; do target/debug/multithread_window; done`.
2. Hope it happens.

Sample output:

```
Spawned threads.
Waited for 100 ms.
Spawned threads.
[xcb] Unknown sequence number while processing queue
[xcb] Most likely this is a multi-threaded client and XInitThreads has not been called
[xcb] Aborting, sorry about that.
multithread_window: ../../src/xcb_io.c:259: poll_for_event: Assertion `!xcb_xlib_threads_sequence_lost' failed.
Spawned threads.
Waited for 100 ms.
thread '<unnamed>' panicked at 'called `Result::unwrap()` on an `Err` value: OsError("glxGetVisualFromFBConfig failed")', libcore/result.rs:945:5
Spawned threads.
[xcb] Unknown sequence number while processing queue
[xcb] Most likely this is a multi-threaded client and XInitThreads has not been called
[xcb] Aborting, sorry about that.
multithread_window: ../../src/xcb_io.c:259: poll_for_event: Assertion `!xcb_xlib_threads_sequence_lost' failed.
Spawned threads.
Waited for 100 ms.
Spawned threads.
Waited for 100 ms.
Spawned threads.
[winit X11 error] XError {
    description: "GLXBadFBConfig",
    error_code: 179,
    request_code: 155,
    minor_code: 34
}
thread '<unnamed>' panicked at 'Failed to get root window: XError { description: "GLXBadFBConfig", error_code: 179, request_code: 155, minor_code: 34 }', libcore/result.rs:945:5
note: Run with `RUST_BACKTRACE=1` for a backtrace.

```
