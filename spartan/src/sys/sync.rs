use bson::doc;
use mongodb::{Collection, Database};
use mongodb::bson::DateTime;
use serde::{Deserialize, Serialize};

const SYNC_COLLECTION: &str = "sync";

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
    #[serde(serialize_with = "bson::serde_helpers::serialize_bson_datetime_as_rfc3339_string")]
    pub date: DateTime,
    pub camera_id: String,
    pub camera_name: String,
    pub location: String,
    pub uploaded: i64,
    pub skipped: i64,
    pub errors: i64,
}

impl SyncResult {
    pub async fn save(&self, db: &Database) -> crate::Result<()> {
        let coll: Collection<SyncResult> = db.collection(SYNC_COLLECTION);
        coll.insert_one(self).await?;

        Ok(())
    }
}
