# Install cargo-watch
```bash
cargo install cargo-watch
```
# Watch for fast dev on the server
```bash
cargo watch -q -c -w src/ -x "run"
```

# Watch for fast dev on the tests
```bash
cargo watch --why -x check -x "test -- --nocapture --color always"
```