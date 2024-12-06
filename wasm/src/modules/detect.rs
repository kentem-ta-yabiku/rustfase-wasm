use crate::utils::detector::with_detector;

pub fn detect(rgba: &[u8], width: u32, height: u32) -> Vec<Info> {
    // RGBA 画像をグレースケールに変換。
    let grayscale = rgba
        .chunks(4)
        // 整数演算で高速化
        .map(|v| ((19 * v[0] as u16) >> 8) + ((183 * v[1] as u16) >> 8) + ((53 * v[2] as u16) >> 8))
        .map(|v| v as u8)
        .collect::<Vec<_>>();

    // ImageData 形式に変換
    let img = rustface::ImageData::new(&grayscale, width, height);

    // グローバル変数に保持している検出器を取得
    with_detector(|detector| {
        detector
            .detect(&img)
            .iter()
            // x, y 座標、幅、高さだけ抜き出す
            .map(|info| Info {
                x: info.bbox().x(),
                y: info.bbox().y(),
                width: info.bbox().width(),
                height: info.bbox().height(),
            })
            .collect()
    })
}
