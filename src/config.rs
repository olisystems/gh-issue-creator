use dotenv::dotenv;
use log::{error, info};
use std::env;

#[derive(Debug, Clone)]
pub struct Config {
    pub gh_access_token: String,
    pub gh_graphql_api_url: String,
    pub repository_name: String,
    pub repository_owner: String,
    pub json_file_path: String,
    pub project_number: i32,
}

impl Config {
    pub fn new() -> Result<Config, &'static str> {
        dotenv().ok();
        info!("Reading environment variables...");

        let gh_access_token = match env::var("GH_ACCESS_TOKEN") {
            Ok(val) => val,
            Err(_) => {
                error!("GH_ACCESS_TOKEN not found.");
                return Err("GH_ACCESS_TOKEN not found.");
            }
        };

        let gh_graphql_api_url = match env::var("GH_GRAPHQL_API_URL") {
            Ok(val) => val,
            Err(_) => {
                error!("GH_GRAPHQL_API_URL not found.");
                return Err("GH_GRAPHQL_API_URL not found.");
            }
        };

        let repository_name = match env::var("REPOSITORY_NAME") {
            Ok(val) => val,
            Err(_) => {
                error!("REPOSITORY_NAME not found.");
                return Err("REPOSITORY_NAME not found.");
            }
        };

        let repository_owner = match env::var("REPOSITORY_OWNER") {
            Ok(val) => val,
            Err(_) => {
                error!("REPOSITORY_OWNER not found.");
                return Err("REPOSITORY_OWNER not found.");
            }
        };

        let json_file_path = match env::var("JSON_FILE_PATH") {
            Ok(val) => val,
            Err(_) => {
                error!("JSON_FILE_PATH not found.");
                return Err("JSON_FILE_PATH not found.");
            }
        };

        let project_number = match env::var("PROJECT_NUMBER") {
            Ok(val) => val.parse::<i32>().unwrap(),
            Err(_) => {
                error!("PROJECT_NUMBER not found.");
                return Err("PROJECT_NUMBER not found.");
            }
        };

        info!("Environment variables read successfully.");

        Ok(Config {
            gh_access_token,
            gh_graphql_api_url,
            repository_name,
            repository_owner,
            json_file_path,
            project_number,
        })
    }
}
