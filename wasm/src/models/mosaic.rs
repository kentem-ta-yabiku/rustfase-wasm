use wasm_bindgen::prelude::wasm_bindgen;

pub trait Average {
    type Output;
    fn average(self, rhs: Self::Output) -> Self::Output;
}

#[wasm_bindgen]
pub struct Row {
    cols: Vec<Rgb>,
}

#[wasm_bindgen]
impl Row {
    #[wasm_bindgen]
    pub fn cols(self) -> Vec<Rgb> {
        self.cols
    }
}

impl Row {
    pub fn new(cols: Vec<Rgb>) -> Self {
        Row { cols }
    }
}

impl Average for Row {
    type Output = Self;
    fn average(self, rhs: Self::Output) -> Self::Output {
        let cols = self
            .cols()
            .into_iter()
            .zip(rhs.cols())
            .map(|(acc, e)| acc.average(e))
            .collect();
        Row { cols }
    }
}

#[wasm_bindgen]
pub struct Rgb {
    r: u8,
    g: u8,
    b: u8,
}

impl Rgb {
    pub fn new(rgba: &[u8]) -> Self {
        Self {
            r: rgba[0],
            g: rgba[1],
            b: rgba[2],
        }
    }
}

#[wasm_bindgen]
impl Rgb {
    #[wasm_bindgen(getter)]
    pub fn r(&self) -> u8 {
        self.r
    }
    #[wasm_bindgen(getter)]
    pub fn g(&self) -> u8 {
        self.g
    }
    #[wasm_bindgen(getter)]
    pub fn b(&self) -> u8 {
        self.b
    }
}

impl Average for Rgb {
    type Output = Self;
    fn average(self, rhs: Self::Output) -> Self::Output {
        Rgb {
            r: ((self.r() as u16 + rhs.r() as u16) >> 1) as u8,
            g: ((self.g() as u16 + rhs.g() as u16) >> 1) as u8,
            b: ((self.b() as u16 + rhs.b() as u16) >> 1) as u8,
        }
    }
}
