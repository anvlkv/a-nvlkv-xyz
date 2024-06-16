# a.nvlkv.xyz (yet another Human-Centered Design enthusiast)

## About

The app ([a.nvlkv.xyz/process](https://a.nvlkv.xyz/process)) draws inspiration from Human-Centered design methods, the GV design sprint, and other approaches. While it does not strictly adhere to these methodologies, it presents an original perspective, informed by the author's experience and a sincere desire for a better world.

The author (@a.nvlkv) makes zero claims regarding effectiveness of the method. Leaving it to the user to decide for themselves and make judgements. Feedback is welcome.

## Privacy

This app is build with privacy in mind. It won't store or send any entered data unless the user choses to do so. Stored data can be deleted upon request via [contact form](https://a.nvlkv.xyz/contact).

### Tracking

Following events are tracked but not linked to any personal data:

- First request to any page (date of the event is stored)
- Printing or export of the worksheets (date of the event and id of the first request is stored)
- LLM inferrence (response, date of the event and id of the first request is stored)
- **If user has previously given permission to use local storage**, id of the very first known request is restored (date of the event, old and new ids)

One event is tracked and linked to submitted data:

- Contact form submission (date of the event and id of the first reqest is stored)

### Storage

Worksheets data is stored in **user's browser** and only if they didn't opt out. Depending on user's choice `LocalStorage` or `SessionStorage` is used. The setting can be changed at [the about page of the worksheets](https://a.nvlkv.xyz/process/0).

Worksheets data is only stored in the remote data base **only** if the user chooses to receive **personalized feedback**.

#### Remote storage and processing

Data is stored in [Xata](https://xata.io/) data base, region `eu-west-1` aka Ireland

Data is processed in [Fermyon cloud](https://developer.fermyon.com/cloud/fermyon-cloud)

### Transfer

Worksheets data is transfered once the user chooses to:

1. Receive feedback using LLM inferrence
2. Receive personalzied feedback

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

## Contributions and Derivatives

This project is licensed under CC-BY-SA 4.0 (Attribution-ShareAlike 4.0 International).

You're welcome to create derivatives of this work or any part of it as long as you share it with necessary attributions and under the same license.

Being a project in itself this app welcomes contributions via pull requests. And I would be really happy to meet those who find it beneficial.
