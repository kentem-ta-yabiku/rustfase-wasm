use wasm_bindgen::prelude::wasm_bindgen;

use super::mosaic::Row;

#[wasm_bindgen]
pub struct BboxInfo {
    x: i32,
    y: i32,
    mosaic: Vec<Row>,
}

#[wasm_bindgen]
impl BboxInfo {
    #[wasm_bindgen]
    pub fn x(&self) -> i32 {
        self.x
    }
    #[wasm_bindgen]
    pub fn y(&self) -> i32 {
        self.y
    }
    #[wasm_bindgen(getter)]
    pub fn mosaic(self) -> Vec<Row> {
        self.mosaic
    }
    pub fn new(x: i32, y: i32, mosaic: Vec<Row>) -> Self {
        BboxInfo { x, y, mosaic }
    }
}
