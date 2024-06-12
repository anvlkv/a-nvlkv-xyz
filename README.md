# a.nvlkv.xyz (yet another Human-Centered design method)

## About



## Privacy



## Development

### Prequisites:

- Rust [with the `wasm32-wasi` target](https://developer.fermyon.com/spin/v2/install) - `rustup target add wasm32-wasi`
- [Spin](https://developer.fermyon.com/spin/v2/install)
- [`cargo-leptos`](https://github.com/leptos-rs/cargo-leptos#getting-started) - `cargo install --locked cargo-leptos`

### Build and run:

- `spin up --build` to build and run the server. It will print the application URL.
- `spin watch` to watch and rebuild.

### `env`

```bash
export SPIN_VARIABLE_XATA_KEY=...
export SPIN_VARIABLE_XATA_WORKSPACE=...
export SPIN_VARIABLE_XATA_REGION=...
export SPIN_VARIABLE_DB_NAME=...
export SPIN_VARIABLE_DB_BRANCH=...
```

### Assets

```bash
npx svgtofont --sources ./assets/icons/svg --output ./assets/icons/font
```
