# SeaORM usage example

## Prerequisites
1. Install Rust
2. Create and fill `.env`:
```bash
cp env-template .env
```

## Migration
Up:
```bash
cd migration
cargo run -- up
cd ..
```

Down:
```bash
cd migration
cargo run -- down
cd ..
```

## Run the App
Run from the project's root:
```bash
cargo run --release
```
