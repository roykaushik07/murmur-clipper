mod captions;
mod notify;
mod render;
mod storage;

use std::{path::PathBuf, sync::Arc};

use axum::{extract::State, http::StatusCode, routing::{get, post}, Json, Router};
use serde::Deserialize;
use serde_json::{json, Value};

struct AppState {
    storage: storage::Storage,
    http: reqwest::Client,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct RenderRequest {
    episode_id: String,
    audio_url: String,
    start: f64,
    end: f64,
    #[serde(default)]
    segments: Vec<captions::Segment>,
}

async fn healthz() -> &'static str {
    "ok"
}

async fn render_clip(State(st): State<Arc<AppState>>, Json(req): Json<RenderRequest>) -> (StatusCode, Json<Value>) {
    let render_id = uuid::Uuid::new_v4().to_string();
    let id = render_id.clone();
    tokio::spawn(async move {
        if let Err(e) = run(&st, &id, req).await {
            tracing::error!(render_id = %id, error = %e, "render failed");
        }
    });
    (StatusCode::ACCEPTED, Json(json!({ "renderId": render_id })))
}

async fn run(st: &AppState, render_id: &str, req: RenderRequest) -> anyhow::Result<()> {
    let dir = std::env::temp_dir();
    let ass_path: PathBuf = dir.join(format!("{render_id}.ass"));
    tokio::fs::write(&ass_path, captions::to_ass(&req.segments, req.start, req.end)).await?;
    let out = render::render(render::RenderInput {
        audio_url: &req.audio_url,
        start: req.start,
        end: req.end,
        ass_path: &ass_path,
        out_path: dir.join(format!("{render_id}.mp4")),
    })
    .await?;
    let key = format!("clips/{}/{render_id}.mp4", req.episode_id);
    st.storage.upload(&key, &out).await?;
    notify::clip_ready(&st.http, render_id, &req.episode_id, &key).await
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt().with_env_filter(tracing_subscriber::EnvFilter::from_default_env()).init();
    let state = Arc::new(AppState { storage: storage::Storage::from_env().await, http: reqwest::Client::new() });
    let app = Router::new().route("/healthz", get(healthz)).route("/v1/render", post(render_clip)).with_state(state);
    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await?;
    axum::serve(listener, app).await?;
    Ok(())
}
