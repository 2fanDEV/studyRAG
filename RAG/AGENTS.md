# Agent Guidelines for RAG Project

## Build/Test Commands
- Build: `cargo build`
- Run: `cargo run`
- Test all: `cargo test`
- Test single: `cargo test test_name`
- Check: `cargo check`
- Format: `cargo fmt`
- Lint: `cargo clippy`

## Code Style
- Edition: 2024
- Use snake_case for functions/variables, PascalCase for types/structs
- Import organization: std imports first, then external crates, then local modules
- Error handling: Use `anyhow` for errors, `unwrap()` sparingly (prefer `?` operator)
- Async: Use `async/await` with tokio runtime
- HTTP responses: Return `HttpResponse` from actix-web handlers
- Database: Use Arc<MongoClient> for shared database connections
- Logging: Use `log::debug!()` for debug messages, `env_logger` for setup
- JSON: Use serde for serialization with `#[derive(Deserialize, Serialize)]`
- Structure: Organize code into modules (services/, endpoints/, database/, etc.)
- Dependencies: Check existing Cargo.toml before adding new dependencies

## Workflow
- Always ask for confirmation before applying any changes
- Present proposed changes clearly before implementation