use cloud_storage::{Client, Object};

pub const MIME_JPEG: &str = "image/jpeg";

pub async fn save_to_bucket(
    client: &Client,
    bucket: &str,
    img: Vec<u8>,
    path: &str,
    mime: &str,
) -> cloud_storage::Result<Object> {
    client.object().create(bucket, img, path, mime).await
}
