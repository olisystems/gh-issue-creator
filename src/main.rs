use dotenv::dotenv;
use log::info;
use env_logger::Env;

mod config;
mod github;

#[tokio::main]
async fn main() {
    // Initialize the logger
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();

    dotenv().ok();

    let config = config::Config::new().unwrap();
    let github = github::GitHub::new(config);

    match github.get_repo_and_project_id().await {
        Ok(id) => info!("Repository ID: {:?}", id),
        Err(e) => eprintln!("Error: {}", e),
    }
}
