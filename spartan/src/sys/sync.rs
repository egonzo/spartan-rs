use mongodb::bson::DateTime;
use serde::{Deserialize, Serialize};

const SYNC_COLLECTION: &str = "sync";
const SYNC_ERROR_COLLECTION: &str = "sync_errors";

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SyncError {
    date: DateTime,
    stage: String,
    camera_type: String,
    camera_id: String,
    camera_account_id: String,
    photo_id: String,
    photo_url: String,
    photo_file_name: String,
    photo_timestamp: String,
    photo_date_utc: DateTime,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SyncResult {
    date: DateTime,
    camera_id: String,
    camera_name: String,
    location: String,
    uploaded: i64,
    skipped: i64,
    errors: i64,
}
