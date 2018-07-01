# Multithread Window

(Sometimes) reproduces https://github.com/tomaka/glutin/issues/1034

1. Run with `cargo run`, or if you've compiled it: `for i in {1..5}; do target/debug/multithread_window; done`. I've limited it to 5 because it can be hard to stop when the windows keep popping up.
2. Hope it happens.

Sample output:

```
$ for i in {1..5}; do target/debug/multithread_window; done
Spawned threads.
Xlib: sequence lost (0x1015e > 0x162) in reply type 0x0!
[xcb] Unknown sequence number while processing queue
[xcb] Most likely this is a multi-threaded client and XInitThreads has not been called
[xcb] Aborting, sorry about that.
multithread_window: ../../src/xcb_io.c:259: poll_for_event: Assertion `!xcb_xlib_threads_sequence_lost' failed.
Spawned threads.
Waited for 400 ms.
Spawned threads.
Waited for 400 ms.
Spawned threads.
Spawned threads.
Xlib: sequence lost (0x1015f > 0x163) in reply type 0x0!
Xlib: sequence lost (0x10173 > 0x177) in reply type 0x0!
Xlib: sequence lost (0x10171 > 0x177) in reply type 0x0!
Xlib: sequence lost (0x10183 > 0x187) in reply type 0x0!
```
