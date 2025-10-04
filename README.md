## Launch
Build and test everything (from repo root)
```
cargo build --release
cargo test --all
```
Or just one lab
```
cargo build -p lab1
cargo test  -p lab1
```

Run via root dispatcher (from repo root)
```
cargo run -- lab1
```
Or run a lab directly
```
cargo run -p lab1
```
