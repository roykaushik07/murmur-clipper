//! Tells the Murmur API a clip is ready.
use anyhow::Result;
use serde_json::json;

pub async fn clip_ready(http: &reqwest::Client, render_id: &str, episode_id: &str, key: &str) -> Result<()> {
    let base = std::env::var("MURMUR_API_URL")?;
    http.post(format!("{base}/webhooks/clipper"))
        .json(&json!({ "event": "clip.ready", "renderId": render_id, "episodeId": episode_id, "key": key }))
        .send()
        .await?
        .error_for_status()?;
    Ok(())
}
