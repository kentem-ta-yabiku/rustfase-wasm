use rustface::Detector;
use std::cell::RefCell;

thread_local! {
  pub static DETECTOR: RefCell<Option<Box<dyn Detector>>> = RefCell::new(None);
}

pub fn init_detector(detector: Box<dyn Detector>) {
    DETECTOR.with(|cell| {
        *cell.borrow_mut() = Some(detector);
    });
}
