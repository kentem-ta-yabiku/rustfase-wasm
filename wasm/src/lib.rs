use models::info::BboxInfo;
use wasm_bindgen::prelude::wasm_bindgen;

pub mod models;
mod modules;
mod utils;

#[wasm_bindgen]
pub fn setup_detector(
    min_face_sizes: u32,
    score_thresh: f64,
    pyramid_scale_factor: f32,
    slide_window_step: u32,
) {
    modules::setup::setup(
        min_face_sizes,
        score_thresh,
        pyramid_scale_factor,
        slide_window_step,
    )
}

#[wasm_bindgen]
pub fn detect_bounding_box(
    rgba: &[u8],
    width: u32,
    height: u32,
    block_size: usize,
) -> Vec<BboxInfo> {
    modules::detect::detect(rgba, width, height, block_size)
}
