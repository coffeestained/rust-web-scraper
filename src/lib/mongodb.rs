use dotenvy::dotenv;
use mongodb::{Client, Collection, Database};
use mongodb::options::ClientOptions;
use serde::{Deserialize, Serialize};
use std::env;
use once_cell::sync::OnceCell;

pub struct MongoFactory {
    client: Client,
    db: Database,
    config_collection: String,
    data_collection: String,
}

impl MongoFactory {
    async fn new() -> Self {
        dotenvy::dotenv().ok(); 

        let uri = env::var("MONGO_URI").expect("MONGO_URI must be set in .env");
        let db_name = env::var("MONGO_DB_NAME").expect("MONGO_DB_NAME must be set");
        let config_collection = env::var("MONGO_CONFIG_COLLECTION").expect("MONGO_CONFIG_COLLECTION must be set");
        let data_collection = env::var("MONGO_DATA_COLLECTION").expect("MONGO_DATA_COLLECTION must be set");

        let client_options = ClientOptions::parse(&uri)
            .await
            .expect("Failed to parse Mongo URI");

        let client = Client::with_options(client_options)
            .expect("Failed to initialize MongoDB client");

        let db = client.database(&db_name);

        Self {
            client,
            db,
            config_collection,
            data_collection,
        }
    }

    pub fn db(&self) -> &Database {
        &self.db
    }

    pub fn config_collection<T>(&self) -> Collection<T> {
        self.db.collection(&self.config_collection)
    }

    pub fn data_collection<T>(&self) -> Collection<T> {
        self.db.collection(&self.data_collection)
    }
}

static MONGO_FACTORY: OnceCell<MongoFactory> = OnceCell::new();

pub async fn init_mongo() {
    let factory = MongoFactory::new().await;
    MONGO_FACTORY.set(factory).unwrap_or_else(|_| panic!("MongoFactory already initialized"));
}

pub fn get_mongo() -> &'static MongoFactory {
    MONGO_FACTORY.get().expect("MongoFactory not initialized")
}