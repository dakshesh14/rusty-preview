use dotenv::dotenv;
use std::env;

#[derive(Debug, Clone)]
pub struct Settings {
    pub database_url: String,
    pub app_host: String,
    pub use_headless_browser_only: bool,
    pub cache_url: Option<String>,
}

impl Settings {
    pub fn from_env() -> Self {
        dotenv().ok();

        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        let app_host = env::var("APP_HOST").unwrap_or_else(|_| "127.0.0.1:8080".to_string());
        let use_headless_browser_only = env::var("ONLY_USE_HEADLESS_BROWSER")
            .unwrap_or_else(|_| "false".to_string())
            .parse::<bool>()
            .unwrap_or(false);
        let cache_url = env::var("CACHE_DATABASE_URL").ok();

        Self {
            database_url,
            app_host,
            use_headless_browser_only,
            cache_url,
        }
    }
}
