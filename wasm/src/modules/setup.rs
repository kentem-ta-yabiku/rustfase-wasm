use crate::utils::detector::init_detector;
use crate::utils::model::MODEL;
use rustface;

pub fn setup(
    min_face_sizes: u32,
    score_thresh: f64,
    pyramid_scale_factor: f32,
    slide_window_step: u32,
) {
    let model = rustface::read_model(MODEL).expect("failed to read model.");

    let mut detector = rustface::create_detector_with_model(model);
    detector.set_min_face_size(min_face_sizes);
    detector.set_score_thresh(score_thresh);
    detector.set_pyramid_scale_factor(pyramid_scale_factor);
    detector.set_slide_window_step(slide_window_step, slide_window_step);
    // 検出器をグローバル変数にセット
    init_detector(detector);
}
