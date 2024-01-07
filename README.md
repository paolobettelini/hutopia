# Hutopia
Hutopia is comprised of a central server: `relay`. <br>
A user can host his own `space` server. <br>
When the space server starts up, it loads the plugins from the `plugins` folder.

# Compile
Prerequisites:
```bash
cargo install cargo-leptos
```
Central server:
```bash
cd hutopia-server-relay
cargo leptos build --release
```
Widgets:
```bash
cd widgets/example
./build.sh
```
Server space:
```bash
cd hutopia-server-space
cargo r -r
```

# Plugins
The plugins are loaded at the server start using FFI.

The client code is compiled with `wasm-pack`, generating the `pkg/` folder.
When the server-side code compiles, it embeds `pkg/` so it can serve the files.

# Websocket
Client-side widgets can open a socket on the space server, sending messages indicating the reference widget. The server forwards these messages to the server-side widget using FFI.

# Todo
Force the nightly version for the entire project.

The widget need to authenticate the user, access his usersname, profile picture and such.
The widget needs to know the server IP and port to access websocket and such.
The widget needs to access some global functions, like send notifications or play audio, and such.