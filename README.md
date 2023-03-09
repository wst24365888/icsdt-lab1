# icsdt-lab1

## About

This is lab 1 for the course Innovation of Cloud Service and Development Tools at NYCU, Spring 2023.

## Getting Started

### Prerequisites

- [Docker Compose](https://docs.docker.com/compose/install/)
- [Diesel CLI](https://diesel.rs/guides/getting-started/) (with `pg`)
  - Install [Rust](https://www.rust-lang.org/) first if you don't have it.
  - Then run `cargo install diesel_cli --no-default-features --features postgres`

### Setup

1. Clone this repository.
```bash
git clone https://github.com/wst24365888/icsdt-lab1
cd icsdt-lab1
```
2. Create `prod.env` file in the root directory of this project, and fill in the environment variables according to `env.example`.
3. Create `db.env` file in the root directory of this project, and fill in the environment variables according to `db.env.example`.
4. Start the database and the app. The app will be available at <http://localhost:8100>
```bash
docker-compose up -d
```
5. Run the migrations.
```bash
diesel migration run
```
