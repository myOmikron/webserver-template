# Rust webserver template

To use this template, ensure `cargo-generate` is installed:

```bash
cargo install -f cargo-generate
```

To use this template, execute:

```bash
cargo generate --git https://github.com/myOmikron/webserver-template.git
```

## Frontend

To initialize the frontend run:

```bash
cd frontend/ && npm install
```

## Backend

To initialize the backend run:

```bash
cargo build -p webserver
```
