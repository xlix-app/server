#[macro_use]
extern crate tracing;

#[macro_use]
extern crate serde;

#[macro_use]
extern crate lazy_static;

mod core;
mod utils;
mod config;
mod database;
mod handler;
mod error;

#[cfg(feature = "tls")]
mod tls;

#[cfg(not(debug_assertions))]
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    core::main().await
}

#[cfg(debug_assertions)]
fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    // SurrealDB often overflows its stack on debug mode.
    // Increasing the stack size fixes this issue.
    let runtime = tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .thread_stack_size(32 * 1024 * 1024)
        .build()
        .unwrap();

    runtime.block_on(core::main())
}
