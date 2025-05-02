//! Handles the rpe .json file.

use serde::{Deserialize, Serialize};
use serde_json::Value;

// TODO: add methods
#[derive(Serialize, Deserialize)]
pub struct Fraction(u32, u32, u32);

#[derive(Serialize, Deserialize)]
pub struct BPMListItem {
    bpm: f32,
    #[serde(rename = "startTime")]
    start_time: Fraction,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RPEChart<'a> {
    #[serde(rename = "BPMList")]
    bpm_list: Vec<BPMListItem>,
    #[serde(rename = "META", borrow)]
    meta: RPEMeta<'a>,
    #[serde(borrow)]
    judge_line_list: Vec<JudgeLine<'a>>,
    // RPE only keys
    _chart_time: f64,
    _judge_line_group: Vec<&'a str>,
    _multi_line_string: &'a str,
    _multi_scale: f32,
}

#[derive(Serialize, Deserialize)]
pub struct RPEMeta<'a> {
    #[serde(rename = "RPEVersion")]
    version: u8,
    background: &'a str,
    charter: &'a str,
    composer: &'a str,
    #[serde(default)]
    illustration: &'a str,
    level: &'a str,
    name: &'a str,
    offset: i32,
    song: &'a str,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct JudgeLine<'a> {
    #[serde(rename = "Group")]
    group: u32,
    #[serde(rename = "Name")]
    name: &'a str,
    #[serde(rename = "Texture")]
    texture: &'a str,
    anchor: (f32, f32),
    #[serde(default)]
    event_layers: Vec<Option<EventLayer>>,
    #[serde(borrow)]
    extended: ExtendedEventSet<'a>,
    #[serde(rename = "father")]
    parent: i32,
    is_cover: u8,
    #[serde(default)]
    notes: Vec<Note<'a>>,
    num_of_notes: u32,
    z_order: i32,
    #[serde(rename = "attachUI")]
    attach_ui: Option<&'a str>,
    is_gif: bool,
    #[serde(rename = "bpmfactor")]
    bpm_factor: f32,
    // RPE only keys
    _pos_control: Value,
    _size_control: Value,
    _skew_control: Value,
    _y_control: Value,
    _alpha_control: Value,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RPEEvent<T = f32> {
    bezier: u8,
    bezier_points: Vec<f32>,
    easing_left: f32,
    easing_right: f32,
    easing_type: u8,
    #[serde(rename = "linkgroup")]
    link_group: u32,
    start: T,
    end: T,
    start_time: Fraction,
    end_time: Fraction,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SpeedEvent {
    #[serde(rename = "linkgroup")]
    link_group: u32,
    start: f32,
    end: f32,
    start_time: Fraction,
    end_time: Fraction,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GifEvent {
    easing_type: u8,
    #[serde(rename = "linkgroup")]
    link_group: u32,
    start: f32,
    end: f32,
    start_time: Fraction,
    end_time: Fraction,
}

#[derive(Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase", default)]
pub struct EventLayer {
    move_x_events: Vec<RPEEvent>,
    move_y_events: Vec<RPEEvent>,
    rotate_events: Vec<RPEEvent>,
    alpha_events: Vec<RPEEvent>,
    speed_events: Vec<SpeedEvent>,
}

#[derive(Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase", default)]
pub struct ExtendedEventSet<'a> {
    color_events: Vec<RPEEvent<(u8, u8, u8)>>,
    scale_x_events: Vec<RPEEvent>,
    scale_y_events: Vec<RPEEvent>,
    #[serde(borrow)]
    text_events: Vec<RPEEvent<&'a str>>,
    gif_events: Vec<GifEvent>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Note<'a> {
    above: u8,
    alpha: u8,
    start_time: Fraction,
    end_time: Fraction,
    is_fake: u8,
    position_x: f32,
    size: f32,
    speed: f32,
    #[serde(rename = "type")]
    kind: u8,
    visible_time: f32,
    y_offset: f32,
    #[serde(rename = "hitsound")]
    hit_sfx: Option<&'a str>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;

    #[test]
    fn read() -> Result<(), Box<dyn std::error::Error>> {
        let chart: RPEChart = serde_json::from_str(include_str!("../../../../test_assets/test_rpe_json_1.json"))?;
        let mut file = File::create("../../test_assets/test_rpe_json_1_compact")?;
        postcard::to_io(&chart, &mut file)?;
        Ok(())
    }
}
