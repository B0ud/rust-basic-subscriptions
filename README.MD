It must be invoked as a cargo subcommand
All options after `--` are passed to cargo itself
We need to point it at our library since it contains
all our SQL queries.

```
cargo sqlx prepare -- --lib
```

CONFIG WHIOT Environement variable 

APP_ENVIRONMENT=local APP_APPLICATION_PORT=1010 cargo run
APP_APPLICATION_PORT=9999 APP_APPLICATION_HOST=0.0.0.0 cargo run