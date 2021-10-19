### Building

First, enter the nix-shell in running this command in the root folder of this repository:

```bash
nix-shell
```

The first time this will take several minutes or longer, but will provide you with the latest RSM binaries. Then you can build with:

```bash
CARGO_TARGET_DIR=target cargo build --release --target wasm32-unknown-unknown
hc dna pack workdir
```

### Testing

```bash
cd test
npm install
npm tests
```
