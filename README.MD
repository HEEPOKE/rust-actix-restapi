# README

## How Create DB

```bash
docker create network HEEPOKE
```

```bash
docker compose up -d
```

## Config Environment

```bash
cp .env.example .env
```

## How To Migrate

```bash
diesel migratio generate "table"
```

```bash
diesel migration run
```

## How To Run

```bash
cargo run
```
