use crate::models::info::BboxInfo;
use crate::modules::mosaic;
use crate::modules::resize;
use crate::utils::detector::DETECTOR;

pub fn detect(
    rgba: &[u8],
    width: u32,
    height: u32,
    block_size: usize,
    is_mosaic: bool,
    overlay_image: &[u8],
) -> Vec<BboxInfo> {
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
    DETECTOR.with(|detector| {
        let Some(ref mut detector) = *detector.borrow_mut() else {
            return vec![];
        };
        // 検出
        detector
            .detect(&img)
            .iter()
            // x, y 座標、幅、高さだけ抜き出す
            .map(|info| {
                let x = info.bbox().x();
                let y = info.bbox().y();
                if !is_mosaic {
                    let resize = resize::resize(overlay_image, width, height);
                    return BboxInfo::new(x, y, resize);
                }
                let mosaic = mosaic::mosaic(
                    rgba,
                    width,
                    x,
                    y,
                    info.bbox().width(),
                    info.bbox().height(),
                    block_size,
                );
                BboxInfo::new(x, y, mosaic)
            })
            .collect()
    })
}
