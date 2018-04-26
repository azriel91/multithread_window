# Multithread Window

(Sometimes) reproduces https://github.com/tomaka/winit/issues/458


1. Run with `cargo run`, or if you've compiled it: `while true; do target/debug/multithread_window; done`.
2. Hope it happens.

Sample when it happens:

```
$ cargo run
   Compiling multithread_window v0.1.0 (file:///home/azriel/work/github/azriel91/multithread_window)
    Finished dev [unoptimized + debuginfo] target(s) in 2.68 secs
     Running `target/debug/multithread_window`
Spawned threads.
[xcb] Unknown request in queue while appending request
[xcb] Most likely this is a multi-threaded client and XInitThreads has not been called
[xcb] Aborting, sorry about that.
multithread_window: ../../src/xcb_io.c:151: append_pending_request: Assertion `!xcb_xlib_unknown_req_pending' failed.
[1]    21599 abort      cargo run
```

