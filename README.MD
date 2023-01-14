It must be invoked as a cargo subcommand
All options after `--` are passed to cargo itself
We need to point it at our library since it contains
all our SQL queries.

```
cargo sqlx prepare -- --lib
```
sqlx migrate run

CONFIG WHIOT Environement variable 

APP_ENVIRONMENT=local APP_APPLICATION_PORT=1010 cargo run
APP_APPLICATION_PORT=9999 APP_APPLICATION_HOST=0.0.0.0 cargo run


# sqlx logs are a bit spammy, cutting them out to reduce noise
export RUST_LOG="sqlx=error,info"
export TEST_LOG=enabled
cargo t subscribe_fails_if_there_is_a_fatal_database_error | bunyan

cargo t newsletter_creation_is_idempotent | bunyan

cargo sqlx prepare --merged --database-url=postgres://postgres:postgres@localhost:5432/newsletter

cargo sqlx prepare --check -- --bin zero2prod