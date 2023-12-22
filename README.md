# Connectoman
Connectoman - a contact manager web app

# Basic help
## Dev (REPL)

> NOTE: Install cargo watch with `cargo install cargo-watch`.

```sh
# Terminal 1 - To run the server.
cargo watch -q -c -w src/ -w .cargo/ -x "run"

# Terminal 2 - To run the quick_dev.
cargo watch -q -c -w examples/ -x "run --example quick_dev"
```

## Unit Test (REPL)

```sh
cargo watch -q -c -x "test -- --nocapture"

# Specific test with filter.
cargo watch -q -c -x "test model::task::tests::test_create -- --nocapture"
```

## Dev

```sh
# Terminal 1 - To run the server.
cargo run

# Terminal 2 - To run the tests.
cargo run --example quick_dev
```

## Unit Test

```sh
cargo test -- --nocapture

cargo watch -q -c -x test model::task::tests::test_create -- --nocapture
```
