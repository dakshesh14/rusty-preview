use axum::Router;
use redis::Client as RedisClient;
use sqlx::migrate::MigrateError;
use sqlx::{postgres::PgPoolOptions, PgPool};
use std::{sync::Arc, time::Instant};
use thiserror::Error;

use super::state::AppState;
use super::{constants::Settings, url::get_routes};

#[derive(Error, Debug)]
pub enum MigrationError {
    #[error("Database migration failed: {0}")]
    DatabaseError(#[from] MigrateError),
    #[allow(dead_code)]
    #[error("Migration failed: {0}")]
    General(String),
}

/// Creates a Postgres connection pool.
///
/// # Returns
/// * `PgPool` - The Postgres connection pool
///
/// # Panics
/// This function will panic if the `DATABASE_URL` environment variable is not set
/// or if the connection pool cannot be created.
pub async fn create_pool() -> PgPool {
    let database_url = Settings::from_env().database_url;

    PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .expect("Failed to create pool")
}

/// Runs the server with the given Postgres connection pool.
///
/// # Arguments
/// * `pool` - A reference to the Postgres connection pool
///
/// # Panics
/// This function will panic if the server cannot be started.
pub async fn run_server(state: Arc<AppState>) {
    let routes: Router = get_routes().with_state(state);

    let tcp_listener = tokio::net::TcpListener::bind(Settings::from_env().app_host)
        .await
        .unwrap();
    println!("Listening on: {}", tcp_listener.local_addr().unwrap());
    axum::serve(tcp_listener, routes).await.unwrap();
}

/// Applies database migrations from the "migrations" directory.
///
/// # Arguments
/// * `pool` - A reference to the Postgres connection pool
///
/// # Returns
/// * `Ok(())` if migrations were successfully applied
/// * `Err(MigrationError)` if migrations failed
///
/// # Examples
/// ```
/// use your_crate::config::settings;
///
/// async fn migrate_db(pool: &PgPool) {
///     match settings::apply_migrations(&pool).await {
///         Ok(_) => println!("Migrations successful"),
///         Err(e) => eprintln!("Migration failed: {}", e),
///     }
/// }
/// ```
///
/// # Errors
/// This function will return an error if:
/// * The migrations directory cannot be found
/// * Any of the migration files are invalid
/// * The database connection fails
/// * Any of the migration queries fail to execute
pub async fn apply_migrations(pool: &PgPool) -> Result<(), MigrationError> {
    let start_time = Instant::now();
    println!("Starting database migrations...");

    match sqlx::migrate!("./migrations").run(pool).await {
        Ok(_) => {
            let duration = start_time.elapsed();
            println!("✅ Database migrations completed successfully!");
            println!("⏱️  Migration duration: {:.2?}", duration);
            Ok(())
        }
        Err(e) => {
            eprintln!("❌ Migration failed!");
            eprintln!("Error details: {}", e);

            eprintln!("Migration error: {}", e);

            Err(MigrationError::DatabaseError(e))
        }
    }
}

/// Creates a Redis client for caching if Redis URL is configured.
///
/// # Returns
/// * `Some(RedisClient)` - If Redis URL is configured and client creation is successful
/// * `None` - If Redis URL is not configured
///
/// # Panics
/// This function will panic if Redis client creation fails with the configured URL
///
/// # Example
/// ```
/// let cache_client = create_cache_client().await;
/// if let Some(client) = cache_client {
///     // Use Redis client for caching
/// } else {
///     // Fallback to non-cached operation
/// }
/// ```
pub async fn create_cache_client() -> Option<RedisClient> {
    let redis_url = Settings::from_env().cache_url;

    if redis_url.is_none() {
        print!("Skipping cache setup\n");
        return None;
    }

    redis_url.map(|url| redis::Client::open(url).expect("Failed to create Redis client"))
}
