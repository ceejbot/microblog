//! This is a Rust module-level documentation string. It documents the module.
//! This is a flammenwerfer. It werfs flammen.

// I turn on some extra linting I find useful.
#![deny(future_incompatible, clippy::unwrap_used)]
#![warn(rust_2018_idioms, trivial_casts, missing_docs)]

// A thread-safe atomically reference-counted pointer, which we'll use to manage
// our shared app state.
use std::sync::Arc;

use axum::routing::{delete, get, post, put};
use axum::Router;
use once_cell::sync::Lazy;
use sqlx::postgres::PgPoolOptions;
use sqlx::PgPool;

// This is how we tell the compiler that these submodules exist.
// Marking them as `pub` allows other modules in the crate to use them.
// Rust looks for either a subdir named `routes` or `routes.rs`.
pub mod routes;
pub mod status;

/// Our randomly-selected ping response in a lazily-evaluated static cell.
/// This lives for the lifetime of the process.
static PING_RESPONSE: Lazy<String> = Lazy::new(gsv_culture_ships::random);

/// The simplest of all possible axum handlers: a get that returns string data
async fn ping() -> &'static str {
    log::trace!("ping");
    PING_RESPONSE.as_str()
}

/// Application runtime state. This is how axum manages shared state.
#[derive(Debug, Clone)]
pub struct AppState {
    /// our postgres connection pool
    db: PgPool,
}

/// main() at the bottom of the file, as tradition demands. The next line is a macro
/// that does code generation to set up the tokio async runtime. Rust has several
/// kinds of hygienic macros that we'll see examples of throughout this demo.
#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // dotenv() is fallible and returns a Result enum.
    // The compiler wants us to do something with it. This is one approach.
    let _ignored = dotenvy::dotenv();

    pretty_env_logger::init();
    // The ! tells us that these are declarative function-like macros. The `log` crate is a
    // general logging facade used by most of the Rust ecosystem. We can use its macros anywhere,
    // whether logging is configured or not. Variable-argument functions are implemented via
    // macros in rust.
    log::info!("Microblogging service starting up!");
    // The first place ownership crops up: we do not own the data in the ping response
    // cell, so we have to borrow its underlying data briefly to fill in this format string.
    // `str` is a reference to utf8 bytes; String is a heap-allocated owned data type.
    log::info!("Ping response is {}", PING_RESPONSE.as_str());

    // std::env::var() returns an Option<String> that we must check somehow.
    // Here, we choose to fail fast because we're missing required config.
    // The process will panic with the error message in the expect().
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL env var must be set");

    // Another approach is the ? sugar. This returns from the current function with
    // any error, but sets db to the ok value if the fallible function succeeded.
    let db = PgPoolOptions::new()
        .max_connections(20)
        .connect(&database_url)
        .await?;

    // Handle a fallible function call that shouldn't crash the process
    // using an if-let assignment. Diverging let-else exists to support the
    // early-return pattern as well.
    if let Err(e) = sqlx::migrate!().run(&db).await {
        log::error!("Encountered problem running migrations: {e:?}");
    }

    // We can create this structure by using its fields directly because it's in the
    // same module with us.
    let state = AppState { db };

    // This is how we create an axum server and mount routes on it.
    // Each route invokes a function that must be a valid axum handler type.
    // If you are using an editor with a rust language server integration,
    // you can see the error markings that indicate these are not. Let's fix this!
    let app = Router::new()
        .route("/ping", get(ping))
        .route("/statuses", post(routes::create))
        .route("/statuses/:id", get(routes::read))
        .route("/statuses/:id", put(routes::update))
        .route("/statuses/:id", delete(routes::delete))
        .route("/statuses", get(routes::list))
        .with_state(Arc::new(state));

    let host = std::env::var("HOST").unwrap_or("localhost".to_string());
    let port = std::env::var("PORT").unwrap_or("6000".to_string());
    let bindstr = format!("{}:{}", host, port);
    let listener = tokio::net::TcpListener::bind(bindstr).await?;
    log::info!("listening on port {}", 6000);
    axum::serve(listener, app)
        .await
        .expect("failed to start serving content");

    Ok(())
}
