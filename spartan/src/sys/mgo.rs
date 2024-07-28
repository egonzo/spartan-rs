use std::env;
use log::{debug, error};
use mongodb::options::ClientOptions;
use mongodb::Client as MongoClient;

// Config is used to hold app Config.
#[derive(Debug)]
pub struct Config {
    pub host: String,
    pub user: String,
    pub password: String,
    pub database: String,
    pub is_cluster: bool,
    pub port: String,
    pub replica_set: String,
}

pub async fn load_mongo_client() -> crate::Result<(mongodb::Client, String)> {
    // Load and Initialize DB Client
    let mongo_config = match Config::from_env() {
        Ok(c) => c,
        Err(e) => {
            error!("error loading mongo Config: {:?}", e);
            return Err(e);
        }
    };

    let uri: String = match mongo_config.is_cluster {
        true => mongo_config.uri_cluster(),
        false => mongo_config.uri(),
    };

    let client_options = ClientOptions::parse(uri)
        .await
        .expect("mongodb client options");

    // Get a handle to the cluster
    Ok((
        MongoClient::with_options(client_options).expect("mongodb client"),
        mongo_config.database,
    ))
}

impl Config {
    // from_env loads the mongo Config from the environment.
    pub fn from_env() -> crate::Result<Self> {
        let _ = dotenvy::dotenv(); // Ignoring error - it's ok to not have .env files

        let cluster = match env::var("MONGO_CLUSTER") {
            Ok(x) => x.to_lowercase() == "true",
            Err(_) => false,
        };

        let host = env::var("MONGO_HOSTS").expect("MONGO_HOSTS env var");
        let user = env::var("MONGO_USERNAME").expect("MONGO_USERNAME env var");
        let password = env::var("MONGO_PASSWORD").expect("MONGO_PASSWORD env var");
        let db = env::var("MONGO_DATABASE").expect("MONGO_DATABASE env var");
        let port = env::var("MONGO_PORT").expect("MONGO_PORT env var");
        let replica = env::var("MONGO_REPLICASET").expect("MONGO_REPLICASET env var");

        debug!("all env vars loaded...");

        Ok(Self {
            host,
            user,
            password,
            database: db,
            is_cluster: cluster,
            port,
            replica_set: replica,
        })
    }

    // uri returns the uri from the Config.
    pub fn uri(&self) -> String {
        format!("mongodb://{}:{}@{}:{}/?retryWrites=true&loadBalanced=false&serverSelectionTimeoutMS=5000&connectTimeoutMS=10000&authSource=admin&authMechanism=SCRAM-SHA-256",
                self.user, self.password, self.host, self.port)
    }

    // uri returns the uri from the Config.
    pub fn uri_cluster(&self) -> String {
        format!(
            "mongodb+srv://{}:{}@{}/admin?retryWrites=true&replicaSet={}&readPreference=primary&w=majority&authSource=admin&authMechanism=SCRAM-SHA-1",
            self.user, self.password, self.host,self.replica_set,
        )
    }
}
