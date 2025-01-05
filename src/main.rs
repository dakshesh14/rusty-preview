use std::{env, sync::Arc};

mod config;
mod preview;

#[tokio::main]
async fn main() {
    let pool = config::settings::create_pool().await;
    let pool = Arc::new(pool);

    let args: Vec<String> = env::args().collect();

    if args.len() > 1 {
        match args[1].as_str() {
            "migrate" => match config::settings::apply_migrations(&pool).await {
                Ok(_) => println!("Migrations successful"),
                Err(e) => eprintln!("Migration failed: {}", e),
            },
            "server" => {
                config::settings::run_server(pool).await;
            }
            _ => {
                eprintln!("Invalid command provided");
                print_usage();
            }
        }
    } else {
        config::settings::run_server(pool).await;
    }
}

fn print_usage() {
    println!("Usage: cargo run [migrate|server]");
}
