//! Shells out to ffmpeg to produce a 9:16 waveform video with burned-in captions.
use std::path::{Path, PathBuf};

use anyhow::{bail, Context, Result};
use tokio::process::Command;

pub struct RenderInput<'a> {
    pub audio_url: &'a str,
    pub start: f64,
    pub end: f64,
    pub ass_path: &'a Path,
    pub out_path: PathBuf,
}

pub async fn render(input: RenderInput<'_>) -> Result<PathBuf> {
    let duration = input.end - input.start;
    let filter = format!(
        "[0:a]showwaves=s=1080x700:mode=cline:colors=0xffd166,format=yuv420p[w];\
         color=c=0x111111:s=1080x1920:d={duration}[bg];\
         [bg][w]overlay=0:610,ass={}[v]",
        input.ass_path.display()
    );
    let status = Command::new("ffmpeg")
        .args(["-y", "-ss", &input.start.to_string(), "-t", &duration.to_string()])
        .args(["-i", input.audio_url])
        .args(["-filter_complex", &filter])
        .args(["-map", "[v]", "-map", "0:a", "-c:v", "libx264", "-c:a", "aac", "-shortest"])
        .arg(&input.out_path)
        .status()
        .await
        .context("spawning ffmpeg")?;
    if !status.success() {
        bail!("ffmpeg exited with {status}");
    }
    Ok(input.out_path)
}
