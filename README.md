# Hutopia
Hutopia è suddiviso in un server centrale: `relay`. <br>
Un utente può hostare il proprio `space`. <br>
Quando il server dello space si avvia, carica i plugin. <br>
I Plugin possono fornire varie pagine (`widget`).

# Compile
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
I plugins sono caricati allo start del server mediante FFI.

Il codice client viene compilato con wasm-pack generando la cartella pkg/.
Quando il codice server side compila, fa l'embedding pkg/ così può servire i file

# Websocket
I widget client side possono aprire un socket sul server space,
manda i messaggi indicando il widget di riferimento. Il server li manda al widget server side con FFI.

# Todo
Forzare il nightly per tutto il progetto