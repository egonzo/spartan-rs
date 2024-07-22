use std::io::Cursor;

use bytes::Bytes;
use chrono::Datelike;
use image;
use image::RgbaImage;
use log::error;
use mongodb::{bson, Collection, Database};
use mongodb::bson::{DateTime, doc};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use thumbnailer::create_thumbnails;
use thumbnailer::ThumbnailSize::Custom;

use crate::spypoint::Photo;
use crate::sys::gdrive;

const COLLECTION: &str = "pictures";

#[derive(Serialize, Deserialize, Default, Debug, Clone)]
#[serde(default)]
pub struct WindDirection {
    cardinal_label_short: String,
    speed: f64,
    degrees: i64,
    cardinal_label: String,
}

#[derive(Serialize, Deserialize, Default, Debug, Clone)]
#[serde(default)]
pub struct WeatherData {
    pub barometric_pressure: f64,
    pub sun_phase: String,
    pub temperature: i64,
    pub weather_label: String,
    pub observation_time: String,
    pub moon_phase: String,
    pub wind_direction: WindDirection,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Picture {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<bson::oid::ObjectId>,
    #[serde(serialize_with = "bson::serde_helpers::serialize_bson_datetime_as_rfc3339_string")]
    pub date: DateTime,
    pub location: String,
    pub bucket: String,
    pub path: String,
    pub thumb_path: String,
    pub camera_id: String,
    pub picture_date: String,
    pub is_favorite: bool,
    pub photo_id: String,
    pub account_id: String,
    #[serde(serialize_with = "bson::serde_helpers::serialize_bson_datetime_as_rfc3339_string")]
    pub last_updated: DateTime,
    #[serde(serialize_with = "bson::serde_helpers::serialize_bson_datetime_as_rfc3339_string")]
    pub created: DateTime,
    pub photo_time_stamp: String,
    pub photo_url: String,
    pub weather_data: Option<WeatherData>,
}

impl From<Photo> for Picture {
    fn from(value: Photo) -> Self {
        let pic_date =
            DateTime::parse_rfc3339_str(value.origin_date.clone()).unwrap_or(DateTime::now());

        let url = format!("https://{}/{}", value.large.host, value.large.path);

        Picture {
            id: None,
            date: pic_date,
            location: String::from(""),
            bucket: String::from(""),
            path: String::from(""),
            thumb_path: String::from(""),
            camera_id: value.camera,
            picture_date: value.date.clone(),
            is_favorite: false,
            photo_id: value.id,
            account_id: String::from(""),
            last_updated: pic_date,
            created: pic_date,
            photo_time_stamp: value.origin_date.clone(),
            photo_url: url,
            weather_data: None,
        }
    }
}

impl Picture {
    pub async fn save(&self, db: Database) -> crate::Result<()> {
        let coll: Collection<Picture> = db.collection(COLLECTION);
        let filter = doc! {
            "photo_id": &self.photo_id,
        };

        let _res = coll
            .find_one_and_update(filter, bson::to_document(self).unwrap_or(doc! {}))
            .upsert(true)
            .await?;

        Ok(())
    }

    pub async fn exists(&self, db: &Database) -> crate::Result<bool> {
        let coll: Collection<Picture> = db.collection(COLLECTION);
        let filter = doc! {
            "photo_id": &self.photo_id,
        };

        match coll.find_one(filter).await? {
            Some(_) => Ok(true),
            None => Ok(false),
        }
    }

    pub async fn download_image(&self, client: &Client) -> reqwest::Result<Bytes> {
        client.get(&self.photo_url).send().await?.bytes().await
    }

    pub async fn upload(
        &mut self,
        db: Database,
        client: &Client,
        gcp_client: &cloud_storage::Client,
        camera_name: String,
        gcp_bucket: String,
    ) -> crate::Result<()> {
        // Download Pic
        let img_bytes = self.download_image(client).await?;

        // set id on Photo
        let id = bson::oid::ObjectId::new();
        self.id = Some(id);

        // create base path and image path
        let created = self.created.to_chrono();

        let base_path = format!(
            "locations/{}/{}-{}",
            camera_name,
            created.month(),
            created.year()
        );

        let img_path = format!("{}/{}.jpg", base_path, id.to_hex());

        // Save Image to cloud storage
        if let Err(e) = gdrive::save_to_bucket(
            gcp_client,
            gcp_bucket.as_str(),
            img_bytes.clone().to_vec(),
            img_path.as_str(),
            gdrive::MIME_JPEG,
        )
        .await
        {
            error!(
                "pictures::upload, error uploading to cloud storage, {:?}",
                e
            );
            return Err(Box::from(e));
        };

        // Make Thumbnail
        let thumb_bytes = match create_thumbnail(img_bytes) {
            Ok(t) => t,
            Err(e) => {
                error!("pictures::upload, error generating thumbbail, {:?}", e);
                return Err(Box::from(e));
            }
        };

        let thumb_path = format!("{}/{}-thumb.jpg", base_path, id.to_hex());

        // Save Image to cloud storage
        if let Err(e) = gdrive::save_to_bucket(
            gcp_client,
            gcp_bucket.as_str(),
            thumb_bytes.clone(),
            thumb_path.as_str(),
            gdrive::MIME_JPEG,
        )
        .await
        {
            error!(
                "pictures::upload, error uploading to cloud storage, {:?}",
                e
            );
            return Err(Box::from(e));
        };

        // Save Picture to DB.
        if let Err(e) = self.save(db).await {
            // TODO: Handle Error
        }

        Ok(())
    }
}

const THUMB_WIDTH: u32 = 400;
const THUMB_HEIGHT: u32 = 400;

fn create_thumbnail(bytes: Bytes) -> crate::Result<Vec<u8>> {
    let reader = Cursor::new(bytes.to_vec());
    let thumb = match create_thumbnails(
        reader,
        mime::IMAGE_JPEG,
        [Custom((THUMB_WIDTH, THUMB_HEIGHT))],
    ) {
        Ok(mut t) => t.pop(),
        Err(e) => {
            return Err(Box::from(e));
        }
    };

    if let Some(x) = thumb {
        let mut buf = Cursor::new(Vec::new());
        let _ = x.write_jpeg(&mut buf, 100).unwrap_or(());

        return Ok(buf.into_inner());
    }

    let image = RgbaImage::new(THUMB_WIDTH, THUMB_HEIGHT);
    Ok(image.to_vec())
}
