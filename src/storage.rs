//! Cloudflare R2 uploads through the S3-compatible API.
use std::path::Path;

use anyhow::Result;
use aws_sdk_s3::{primitives::ByteStream, Client};
use tokio::io::AsyncRead;

pub struct Storage {
    client: Client,
    bucket: String,
}

impl Storage {
    pub async fn from_env() -> Self {
        let endpoint = std::env::var("R2_ENDPOINT").expect("R2_ENDPOINT");
        let conf = aws_config::from_env().endpoint_url(endpoint).region("auto").load().await;
        Self {
            client: Client::new(&conf),
            bucket: std::env::var("R2_BUCKET").unwrap_or_else(|_| "murmur-clips".into()),
        }
    }

    pub async fn upload(&self, key: &str, path: &Path) -> Result<()> {
        self.client
            .put_object()
            .bucket(&self.bucket)
            .key(key)
            .content_type("video/mp4")
            .body(ByteStream::from_path(path).await?)
            .send()
            .await?;
        Ok(())
    }

    /// Opens an object for streaming. Returns `None` when the key does not exist.
    pub async fn open(&self, key: &str) -> Result<Option<(Box<dyn AsyncRead + Send + Unpin>, Option<i64>)>> {
        match self.client.get_object().bucket(&self.bucket).key(key).send().await {
            Ok(out) => {
                let len = out.content_length();
                Ok(Some((Box::new(out.body.into_async_read()), len)))
            }
            Err(e) if e.as_service_error().map_or(false, |se| se.is_no_such_key()) => Ok(None),
            Err(e) => Err(e.into()),
        }
    }
}
