use dotenv::dotenv;
use env_logger::Env;
use log::info;

mod config;
mod description;
mod file_handler;
mod github;

#[tokio::main]
async fn main() {
    // Initialize the logger
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();

    dotenv().ok();

    let config = config::Config::new().unwrap();
    let json_file_path = config.json_file_path.clone();
    let github = github::GitHub::new(config);

    match github.get_repo_and_project_id().await {
        Ok((repo_id, project_id)) => {
            info!("Repository ID: {}", repo_id);
            info!("Project ID: {}", project_id);

            // Read tasks and generate description text
            match file_handler::read_and_generate_description(&json_file_path) {
                Ok(description_text) => println!("Description Text:\n{:?}", description_text),
                Err(e) => eprintln!("Error reading tasks from file: {}", e),
            }
        }
        Err(e) => eprintln!("Error: {}", e),
    }
}
