use rustface::Detector;
use std::cell::RefCell;

thread_local! {
  static DETECTOR: RefCell<Option<Box<dyn Detector>>> = RefCell::new(None);
}

pub fn init_detector(detector: Box<dyn Detector>) {
    DETECTOR.with(|cell| {
        *cell.borrow_mut() = Some(detector);
    });
}

// DETECTORを利用するための関数
pub fn with_detector<F, R>(f: F) -> Option<R>
where
    F: FnOnce(&mut dyn Detector) -> R,
{
    DETECTOR.with(|cell| {
        let mut borrowed = cell.borrow_mut();
        if let Some(ref mut detector) = *borrowed {
            Some(f(detector.as_mut()))
        } else {
            None
        }
    })
}
