# Rust Server Frameworks

The aim of this repo was to experiment Rust web frameworks using cargo workspaces.

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

### Build and Run Docker image

```bash
docker build ./actix-api -f ./actix-api/Dockerfile -t <Container Registry User>/actix-api:1.0.0
docker run -p 8089:8089 <Container Registry User>/actix-api:1.0.0
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

### Build and Run Docker image

```bash
docker build ./hyper-api -f ./hyper-api/Dockerfile -t <Container Registry User>/hyper-api:1.0.0
docker run -p 3000:3000 <Container Registry User>/hyper-api:1.0.0
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

### Build and Run Docker image

```bash
docker build ./tide-api -f ./tide-api/Dockerfile -t <Container Registry User>/tide-api:1.0.0
docker run -p 8090:8090 <Container Registry User>/tide-api:1.0.0
```
