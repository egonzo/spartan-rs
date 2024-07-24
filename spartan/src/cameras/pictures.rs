use std::io::Cursor;

use bytes::Bytes;
use chrono::Datelike;
use image;
use image::{ImageFormat, RgbImage};
use image::codecs::jpeg::JpegEncoder;
use image::imageops::FilterType;
use log::error;
use mongodb::{bson, Collection, Database};
use mongodb::bson::{DateTime, doc};
use reqwest::Client;
use serde::{Deserialize, Serialize};

use crate::spypoint::Photo;
use crate::sys::gdrive;
use crate::sys::gdrive::GCPClient;

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
    /// Saves a picture to the database
    ///
    /// Arguments:
    ///
    /// db: MongoDB database
    pub async fn save(&self, db: &Database) -> crate::Result<()> {
        let coll: Collection<Picture> = db.collection(COLLECTION);
        let filter = doc! {
            "photo_id": &self.photo_id,
        };

        let _res = coll
            .find_one_and_update(filter, bson::to_document(self).unwrap_or_default())
            .upsert(true)
            .await?;

        Ok(())
    }

    /// Determines whether a picture exists or not in the database.
    ///
    /// Arguments:
    ///
    /// db: MongoDB Database
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

    /// Downloads an image from the client provider.
    ///
    /// Arguments:
    ///
    /// client: The client provider from where to get the picture from.
    pub async fn download_image(&self, client: &Client) -> reqwest::Result<Bytes> {
        client.get(&self.photo_url).send().await?.bytes().await
    }

    /// Uploads pictures to cloud storage. Generates a thumbnail of the picture and uploads that to
    /// cloud storage as well. A new picture record is created in the database for the new picture.
    ///
    /// Arguments:
    ///
    /// db: MongoDB Database
    /// client: Spypoint client used to download picture.
    /// camera_name: The name of the camera that the picture belongs to.
    /// gcp_client: Google cloud storage client.
    /// gcp_bucket: The name of the bucket in cloud storage where the picture will be saved.
    pub async fn upload(
        &mut self,
        db: &Database,
        client: &Client,
        camera_name: String,
        gcp_client: &GCPClient,
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
        if let Err(e) = gcp_client
            .save_to_bucket(
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
        let thumb_bytes = match create_thumbnail(img_bytes.as_ref(), THUMB_WIDTH, THUMB_HEIGHT) {
            Ok(t) => t,
            Err(e) => {
                error!("pictures::upload, error generating thumbnail, {:?}", e);
                return Err(e);
            }
        };

        let thumb_path = format!("{}/{}-thumb.jpg", base_path, id.to_hex());

        // Save Image to cloud storage
        if let Err(e) = gcp_client
            .save_to_bucket(
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
            error!(
                "pictures::upload, unable to save picture to Database, {:?}",
                e
            );

            return Err(e);
        }

        Ok(())
    }
}

const THUMB_WIDTH: u32 = 400;
const THUMB_HEIGHT: u32 = 400;

/// Resizes the image represented by the bytes parameters to the size (width and height) parameters.
pub fn create_thumbnail(bytes: &[u8], width: u32, height: u32) -> crate::Result<Vec<u8>> {
    let img = match image::load_from_memory_with_format(bytes, ImageFormat::Jpeg) {
        Ok(t) => t,
        Err(e) => {
            error!(
                "pictures::create_thumbnail, error generating thumbnail, {:?}",
                e
            );
            return basic_thumbnail(width, height);
        }
    };

    let thumb = img.resize(width, height, FilterType::Triangle);

    let mut cursor = Cursor::new(Vec::new());
    let encoder = JpegEncoder::new_with_quality(&mut cursor, 95);

    thumb.write_with_encoder(encoder)?;
    Ok(cursor.into_inner())
}

/// Returns a black jpeg image based on the width and height parameters passed it.
pub fn basic_thumbnail(width: u32, height: u32) -> crate::Result<Vec<u8>> {
    let mut image = RgbImage::new(width, height);
    image.fill(0);

    let mut cursor = Cursor::new(Vec::new());
    let encoder = JpegEncoder::new_with_quality(&mut cursor, 95);
    image.write_with_encoder(encoder)?;
    Ok(cursor.into_inner())
}

#[cfg(test)]
mod tests {
    use std::fs::File;
    use std::io::{BufReader, Read, Write};

    use crate::cameras::pictures::{basic_thumbnail, create_thumbnail, THUMB_HEIGHT, THUMB_WIDTH};

    #[test]
    fn basic_create_thumbnail() {
        let mut buf = BufReader::new(File::open("buck77.jpg").unwrap());
        let mut buffer = Vec::new();
        buf.read_to_end(&mut buffer).expect("File to be read");

        assert!(buffer.len() > 0);
        let output = create_thumbnail(buffer.as_slice(), THUMB_WIDTH, THUMB_HEIGHT).expect("Image");

        assert!(output.len() > 0);
        let mut file = File::create("thumb.jpg").expect("File to be created");
        file.write_all(&output)
            .expect("Thumbnail Image to be saved");
    }

    #[test]
    fn black_thumbnail() {
        let bytes = basic_thumbnail(THUMB_WIDTH, THUMB_HEIGHT).expect("Black Thumbnail");
        assert!(bytes.len() > 0);
        let mut file = File::create("thumb_black.jpg").expect("File to be created");
        file.write_all(&bytes).expect("Thumbnail Image to be saved");
    }
}
