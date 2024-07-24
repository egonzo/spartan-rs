use std::{env, process};
use std::time::Duration;

use log::{error, info};
use mongodb::bson::{DateTime, doc};

use spartan::{client, spypoint, sys, sys::mgo};
use spartan::cameras::Camera;
use spartan::cameras::pictures::Picture;
use spartan::client::Server;
use spartan::spypoint::Login;
use spartan::sys::gdrive::GCPClient;
use spartan::sys::sync::SyncResult;

pub mod built_info {
    include!(concat!(env!("OUT_DIR"), "/built.rs"));
}

const DAYS_OF_PICS: u64 = 2;

#[tokio::main]
async fn main() {
    env_logger::init();

    info!(
        "version {:?} started... Config loaded...",
        built_info::PKG_VERSION
    );

    // Load and Initialize DB Client
    // Tuple returned (Client, DB)
    let mgo = mgo::load_mongo_client()
        .await
        .expect("mongo client initialized");

    // Ping the server to see if we can connect to the cluster
    let db = mgo.0.database(&mgo.1);
    if let Err(e) = db.run_command(doc! {"ping": 1}).await {
        error!("error pinging db: {:?}", e);
        process::exit(1);
    }

    info!("mongo connected to database, {:?}...", db.name());

    // Load Config
    let config = Config::from_env();

    // Load GCP Client
    // It loads the GCP JSON Key from the env. See GCPClient for more details.
    let gcp_client = GCPClient::default();

    // Spypoint Server
    let server = Server {
        user_name: config.spypoint_user,
        password: config.spypoint_pwd,
        host: config.spypoint_host,
    };

    // New Spypoint client
    let client = client::Client::new(server).expect("spypoint client");

    // Login
    let l = Login {
        username: client.user(),
        password: client.user_password(),
    };

    match spypoint::login(&client, l).await {
        Ok(x) => x,
        Err(e) => {
            error!("sync::main error logging into spypoint, {:?}", e);
            process::exit(1);
        }
    };

    info!("sync::main Logged into spypoint...");

    // Load Cameras
    let cameras = match spypoint::cameras(&client).await {
        Ok(x) => x,
        Err(e) => {
            error!("sync::main error loading cameras, {:?}", e);
            process::exit(1);
        }
    };

    info!("sync:main {} cameras loaded...", cameras.len());

    // loop through cameras.
    for camera in cameras {
        info!(
            "sync::main processing camera, {}...",
            camera.config.name.clone()
        );

        let mut sync_result = SyncResult {
            date: DateTime::now(),
            camera_id: camera.id.clone(),
            camera_name: camera.config.name.clone(),
            location: camera.config.name.clone(),
            uploaded: 0,
            skipped: 0,
            errors: 0,
        };

        // Loads camera details
        let camera_detail = match spypoint::camera(&client, camera.clone().id).await {
            Ok(c) => c,
            Err(e) => {
                error!(
                    "sync::main getting camera detail, {}...{:?}",
                    camera.config.name.clone(),
                    e
                );

                // TODO: Slack
                continue;
            }
        };

        //  Convert and Upsert Camera
        let spartan_camera = Camera::from(camera_detail);
        match spartan_camera.save(&db).await {
            Ok(()) => {}
            Err(e) => {
                error!(
                    "sync::main saving camera, {}...{:?}",
                    camera.config.name.clone(),
                    e
                );
                // TODO: Slack
                continue;
            }
        }

        // Sleep Thread.
        tokio::time::sleep(Duration::new(2, 0)).await;

        // Load Last X Camera Pictures
        let photo_response = match spypoint::camera_photos(&client, camera.id, Some(125)).await {
            Ok(p) => p,
            Err(e) => {
                error!(
                    "sync::main retrieving photos for camera, {}...{:?}",
                    camera.config.name.clone(),
                    e
                );
                sync_result.errors += 1;
                // TODO: Slack

                continue;
            }
        };

        for photo in photo_response.photos {
            let mut picture = Picture::from(photo);

            // check if pic exists and date
            let pic_date = picture.date.to_system_time();
            let cutoff = sys::sub_date(pic_date, DAYS_OF_PICS);
            if cutoff > pic_date {
                sync_result.skipped += 1;
                continue;
            }

            // check DB to see if pic exists.
            if let Ok(x) = picture.exists(&db).await {
                if x {
                    sync_result.skipped += 1;
                    continue;
                }
            }

            // Set fields
            picture.account_id = spartan_camera.clone().account_id;

            // Download Pic, Save to Cloud Storage, Gen Thumbnail, Save thumb to Cloud storage
            // and save Pic to db.
            if let Err(e) = picture
                .upload(
                    &db,
                    &client.http_client(),
                    picture.location.clone(),
                    &gcp_client,
                    config.gcp_bucket.clone(),
                )
                .await
            {
                error!(
                    "sync::main upload photo with date {} for camera, {}...{:?}",
                    picture.picture_date,
                    camera.config.name.clone(),
                    e
                );
                sync_result.errors += 1;
                // TODO: Slack

                continue;
            }

            // Sleep Thread?
            // tokio::time::sleep(Duration::new(2, 0)).await;
            sync_result.uploaded += 1;
        }

        info!(
            "sync::main processing camera, {}, complete",
            camera.config.name.clone()
        );

        // TODO: Save Sync Metrics for Camera.

        // Sleep Thread
        tokio::time::sleep(Duration::new(2, 0)).await;
    }
}

pub struct Config {
    spypoint_user: String,
    spypoint_pwd: String,
    spypoint_host: String,
    gcp_bucket: String,
    gcp_job_key: String,
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
            spypoint_user: sp_user,
            spypoint_pwd: sp_pwd,
            spypoint_host: sp_host,
            gcp_bucket: String::from(""),
            gcp_job_key: String::from(""),
            sync_days: 2,
        }
    }
}
