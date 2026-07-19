// Modules must be `pub` here so integration tests in `tests/` can import
// types like `reth_custom_db::db::SqliteDb` and `reth_custom_db::rpc::*`.
pub mod cmd;

// The test server's SqliteEntityApiImpl needs a SqliteDb to function, it's not an in-memory API,
// it actually saves/gets/deletes from a database. A temp db avoids polluting real data
// and gives each test isolated state.
pub mod db;

// Exposed for integration tests that exercise the RPC server via HTTP client.
pub mod rpc;
