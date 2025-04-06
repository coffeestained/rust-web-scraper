mod lib;
mod utils;
mod schema;

use crate::utils::{logging, environment};
use crate::lib::mongodb::{get_mongo, init_mongo};
use crate::schema::config::{ConfigDocument, SourceConfig};

use tokio::time::{sleep, Duration};
use dotenvy::dotenv;
use reqwest::Client;
use scraper::{Html, Selector};
use mongodb::bson::doc;
use mongodb::Collection;

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();
    logging::init(); 

    // Initial logging statements
    logging::info!("Starting scraper.");
    logging::debug!("Debug logs are enabled.");
    logging::error!("Error logs are enabled.");

    // Initialize MongoDB
    init_mongo().await;
    let db = get_mongo().db();
    match db.run_command(doc! { "ping": 1 }, None).await {
        Ok(_) => logging::info!("Mongo connection successful."),
        Err(e) => logging::error!("Mongo connection failed: {}", e),
    }

    // Get all configs
    logging::info!("Bootstrapping configuration.");
    let collection = get_mongo().config_collection::<ConfigDocument>();
    let result = collection.find_one(doc! {}, None).await;

    // Initial Checks
    if let Some(config) = result.unwrap() {
        if !config.sources.is_empty() {
            for source in config.sources {
                logging::debug!("Found source: {:?}", source);
            }
            scraper_active(&collection).await;
        } else {
            logging::error!("No sources found in config.");
            panic!("Config document exists, but contains no sources.");
        }
    } else {
        logging::error!("No config found.");
        panic!("No config found.");
    }
}

async fn scraper_active(collection: &Collection<ConfigDocument>) {
    let interval_minutes = environment::get_scrape_interval_minutes();
    
    logging::info!("Starting main loop. Checking for sources every {} minutes.", interval_minutes);
    
    loop {
        logging::info!("Refreshing source configurations...");
        let result = collection.find_one(doc! {}, None).await;
    
        if let Some(config) = result.unwrap() {
            if !config.sources.is_empty() {
                for source in config.sources {
                    match &source {
                        SourceConfig::Api { _id, .. } | SourceConfig::Web { _id, .. } => {
                            let source_id = _id.clone();
                            logging::info!("Spawning scrape task for source: {}", source_id);
                
                            tokio::spawn(async move {
                                logging::info!("Scraper task started for: {}", source_id);
                                // TODO: Actual scrape workflow
                            });
                        }
                    }
                }
            } else {
                logging::warn!("No sources found in config.");
            }
        } else {
            logging::warn!("No config found.");
        }
    
        logging::info!("Sleeping for {} minutes...", interval_minutes);
        sleep(Duration::from_secs(interval_minutes * 60)).await;
    }
}