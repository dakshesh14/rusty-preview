use std::{env, sync::Arc};

mod config;
mod preview;

#[tokio::main]
async fn main() {
    let pool = config::settings::create_pool().await;
    let pool = Arc::new(pool);

    let cache_pool = config::settings::create_cache_client().await;
    let cache_pool = Arc::new(cache_pool);

    let state = Arc::new(config::state::AppState { pool, cache_pool });

    let args: Vec<String> = env::args().collect();

    if args.len() > 1 {
        match args[1].as_str() {
            "migrate" => match config::settings::apply_migrations(&state.pool).await {
                Ok(_) => println!("Migrations successful"),
                Err(e) => eprintln!("Migration failed: {}", e),
            },
            "server" => {
                config::settings::run_server(state).await;
            }
            _ => {
                eprintln!("Invalid command provided");
                print_usage();
            }
        }
    } else {
        config::settings::run_server(state).await;
    }
}

fn print_usage() {
    println!("Usage: cargo run [migrate|server]");
}
