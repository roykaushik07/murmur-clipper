//! Builds an ASS subtitle file with word-grouped captions.
use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct Segment {
    pub start: f64,
    pub end: f64,
    pub text: String,
}

fn ts(secs: f64) -> String {
    let cs = (secs * 100.0).round() as u64;
    format!("{}:{:02}:{:02}.{:02}", cs / 360000, (cs / 6000) % 60, (cs / 100) % 60, cs % 100)
}

/// Segments are shifted so the clip starts at t=0.
pub fn to_ass(segments: &[Segment], clip_start: f64, clip_end: f64) -> String {
    let mut out = String::from(
        "[Script Info]\nPlayResX: 1080\nPlayResY: 1920\n\n[V4+ Styles]\n\
         Format: Name,Fontname,Fontsize,PrimaryColour,Alignment,MarginV\n\
         Style: Default,Inter,64,&H00FFFFFF,2,420\n\n[Events]\n\
         Format: Layer,Start,End,Style,Text\n",
    );
    for s in segments.iter().filter(|s| s.end > clip_start && s.start < clip_end) {
        let a = (s.start - clip_start).max(0.0);
        let b = (s.end.min(clip_end)) - clip_start;
        out.push_str(&format!("Dialogue: 0,{},{},Default,{}\n", ts(a), ts(b), s.text.replace('\n', "\\N")));
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn timestamps_format() {
        assert_eq!(ts(3661.5), "1:01:01.50");
    }
}
