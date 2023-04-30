# axum-api-sqlx-sqlite

This is a simple example of an API using Axum and SQLx with SQLite.

## Run the migrations


First install the sqlx-cli:

```bash
cargo install sqlx-cli
```

Then run the migrations:

```bash
sqlx database create
sqlx migrate run
```

Now you can run the server:

```bash
cargo run
```
