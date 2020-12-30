# Rust Server Frameworks

The aim of this repo was be experiment using the Rust language the monorepo style.

---

## Prerequisites

### Install rustup

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

```

### Add rustfmt

```bash
rustup component add rustfmt
# Whenever you need to format the workspace coe
cargo fmt
```

---

## Usage

### Checking workspace

```bash
cargo check
```

### Build all apis

```bash
cargo build # or cargo build --release
cargo build -p hyper-api
```

### Build all modules

```bash
cargo build # or cargo build --release
```

### Build specific modules

```bash
cargo build -p <module name> # example: cargo build -p hyper-api
```

### Testing workspace

```bash
cargo test
```

---

## Actix

### Build

```bash
cargo build -p actix-api
```

### Run

```bash
./target/debug/actix-api
```

---

## Hyper

### Build

```bash
cargo build -p hyper-api
```

### Run

```bash
./target/debug/hyper-api
```

---

## Tide

### Build

```bash
cargo build -p tide-api
```

### Run

```bash
./target/debug/tide-api
```
