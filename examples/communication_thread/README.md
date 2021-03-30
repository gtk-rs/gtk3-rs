# Communication Thread

Example on how to use a communication thread alongside with the GUI thread.

Tricks used here:
- Use a channel to show data on the GUI.
- Run an `async` function on the GUI event loop.
- Use a separate thread to handle incoming data and put it into a channel.

Run it by executing:

```bash
cargo run --bin communication_thread
```


![screenshot](screenshot.png)