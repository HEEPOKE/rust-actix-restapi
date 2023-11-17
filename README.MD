# README

## Config Environment

```bash
cp .env.example .env
```

## How Create DB

```bash
docker network create HEEPOKE
```

```bash
docker compose up -d
```

## How To Migrate

```bash
diesel migration generate "table"
```

```bash
diesel migration run
```

## How To Run

```bash
cargo run
```

- watch run

```bash
cargo install cargo-watch
```

```bash
cargo watch -x run
```
