# murmur-clipper

Renders captioned 9:16 waveform clips for Murmur. An axum service that cuts audio with ffmpeg, burns in
captions, uploads the MP4 to Cloudflare R2 and notifies the API.

`edge/` holds the Cloudflare Worker that serves finished clips from R2 on `clips.murmurclips.app`.

## Local dev

```sh
cp .env.example .env
cargo run
cd edge && npm install && npx wrangler dev
```

Requires `ffmpeg` on PATH.

## Deploy

Service: Fly.io (`fly deploy`, see `fly.toml`). Edge: `cd edge && npx wrangler deploy`.
Release images are pushed to GHCR by `.github/workflows/release.yml`.
