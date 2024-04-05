## Example WASM App

Before running the client, you must paste the server's certificate hash into the client's Rust code. See `echo_server_wt/README.md`.

To run:

- Install `wasm-pack`
- Install `npm`

```bash
wasm-pack build \
&& cd wasm-app \
&& npm install \
&& npm run start
```

The app will only work on browsers [that support](https://caniuse.com/mdn-api_webtransport_webtransport_options_servercertificatehashes_parameter) self-signed certificates. Major browsers with support (as of April 8, 2024): Chrome, Edge, Opera.
