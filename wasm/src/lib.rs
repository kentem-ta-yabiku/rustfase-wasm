use wasm_bindgen::prelude::wasm_bindgen;

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
