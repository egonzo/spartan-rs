use std::env;
use log::{ error, info};
use mongodb::bson::doc;
use spartan::client::Server;
use spartan::{client, sys::mgo, spypoint};
use spartan::spypoint::Login;

pub mod built_info {
    include!(concat!(env!("OUT_DIR"), "/built.rs"));
}

#[tokio::main]
async fn main() {
    env_logger::init();

    info!(
        "version {:?} started... Config loaded...",
        built_info::PKG_VERSION
    );

    // Load and Initialize DB Client
    // Tuple returned (Client, DB)
    let mgo = mgo::load_mongo_client().await.expect("mongo client initialized");


    // Ping the server to see if we can connect to the cluster
    let db = mgo.0.database(&mgo.1);
    if let Err(e) = db.run_command(doc! {"ping": 1}).await {
        error!("error pinging db: {:?}", e);
        return;
    }

    info!("mongo connected to database, {:?}...", db.name());


    // Load Config
    let config = Config::from_env();

    // Spypoint
    let server = Server {
        user_name: config.spypoint_user,
        password: config.spypoint_pwd,
        host: config.spypoint_host,
    };

    let client = client::Client::new(server).expect("spypoint client");

    // Login
    let l = Login {
        username: client.user(),
        password: client.user_password()
    };

     match spypoint::login(&client, l).await {
        Ok(x)=> x,
        Err(e)=> {
            error!("sync::main error logging into spypoint, {:?}",e);
            return;
        }};

    info!("sync::main Logged into spypoint...");

    // Load Cameras
    let cameras = match spypoint::cameras(&client).await {
        Ok(x) => x,
        Err(e) => {
            error!("sync::main error loading cameras, {:?}",e);
            return;
        }
    };

    info!("sync:main {} cameras loaded...", cameras.len());

    // loop through cameras.
    for camera in cameras {

    }
            // Process Each Camera, Upsert
            // Process Pictures for Cameras

    // Sync Metrics.


    println!("Hello, world!");
}

pub struct Config {
    spypoint_user: String,
    spypoint_pwd:String,
    spypoint_host:String,
    gcp_bucket: String,
    gcp_job_key:String,
    sync_days: i32,
}

impl Config {
    pub fn from_env() -> Config {
        // "SPYPOINT_USER"
        // "SPYPOINT_PWD"
        // "SPYPOINT_HOST"

        let sp_user = env::var("SPYPOINT_USER").expect("SPYPOINT USER NAME");
        let sp_pwd = env::var("SPYPOINT_PWD").expect("SPYPOINT PASSWORD");
        let sp_host = env::var("SPYPOINT_HOST").expect("SPYPOINT HOST");

        Config {
            spypoint_user:sp_user,
            spypoint_pwd:sp_pwd,
            spypoint_host:sp_host,
            gcp_bucket:String::from(""),
            gcp_job_key:String::from(""),
            sync_days:2,
        }
    }
}
