# Hutopia
Hutopia is comprised of a central server: `relay`. <br>
A user can host his own `space` server. <br>
When the space server starts up, it loads the plugins from the `plugins` folder.

# Compile
Prerequisites:
```bash
cargo install cargo-leptos
rustup target add wasm32-unknown-unknown
sudo pacman -S wasm-pack diesel-cli jq
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
A plugin is a custom component that a server may use.
The plugins are loaded at the server start using FFI.
Plugins usually export custom HTML tags to be used within the space HTML space.

The client code is compiled with `wasm-pack`, generating the `pkg/` folder.
Then, `npm` is used to compile it to a bundle, alongisde the script `index.js`,
which is the entry point for the client.
To load the plugin it is necessary to load this script.

When the server-side code compiles, it embeds `dist/` so it can serve the files.

## Config
Plugins may require a config file (possibly a `.toml`) to be placed aside its file.
If the plugin needs to connect to a database, it will read an enviromental variable.

# Websocket
Client-side widgets can open a socket on the space server, sending messages indicating the reference widget. The server forwards these messages to the server-side widget using FFI.

# Todo and notes
Force the nightly version for the entire project.

- The widget need to authenticate the user, access his usersname, profile picture and such.
- The widget needs to access some global functions, like send notifications or play audio, open the profile of somebody and such.
- Maybe: A plugin may export a set of required dependency plugins (then we need priority loading).

Idea for plugin authentication:
Relay has pub/priv keys. When user connects to space,
it asks relay to sign a token contaning "spaceId + userId + expirationDateUnix",
so tht the space can check the signature.
Plugin are authenticated through the space server begin authenticated.
The expiration date is 1 minute.

https://github.com/leptos-rs/leptos/tree/main/examples/sso_auth_axum