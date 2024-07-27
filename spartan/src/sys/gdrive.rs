use cloud_storage::{Client, Object};

pub const MIME_JPEG: &str = "image/jpeg";

pub struct GCPClient {
    inner: Client,
}

impl Default for GCPClient {
    /// Creates a new client by calling Default on the inner client.
    ///
    /// Checks for the environment variable SERVICE_ACCOUNT, and if it exists, reads the file at the path specified there as a credentials json file.
    /// It attempts to do the same with the GOOGLE_APPLICATION_CREDENTIALS var.
    /// It reads the SERVICE_ACCOUNT_JSON environment variable directly as json and uses that
    /// It attempts to do the same with the GOOGLE_APPLICATION_CREDENTIALS_JSON var.
    ///
    /// GOOGLE_APPLICATION_CREDENTIALS_JSON will contain the JSON.
    fn default() -> Self {
        GCPClient {
            inner: Client::default(),
        }
    }
}

impl GCPClient {
    pub async fn save_to_bucket(
        &self,
        bucket: &str,
        img: Vec<u8>,
        path: &str,
        mime: &str,
    ) -> cloud_storage::Result<Object> {
        self.inner.object().create(bucket, img, path, mime).await
    }
}
