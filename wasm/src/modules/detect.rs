use crate::modules::mosaic;
use crate::modules::replace;
use crate::utils::detector::DETECTOR;

pub fn detect(
    rgba: &[u8],
    width: u32,
    height: u32,
    block_size: Option<usize>,
    overlay: &[u8],
    is_mosaic: bool,
) -> Vec<u8> {
    // RGBA 画像をグレースケールに変換。
    let grayscale = rgba
        .chunks(4)
        // 整数演算で高速化
        .map(|v| ((19 * v[0] as u16) >> 8) + ((183 * v[1] as u16) >> 8) + ((53 * v[2] as u16) >> 8))
        .map(|v| v as u8)
        .collect::<Vec<_>>();

    // ImageData 形式に変換
    let img = rustface::ImageData::new(&grayscale, width, height);

    // 検出結果を格納するための Vec<u8>
    let mut result: Vec<u8> = Vec::new();

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
                if is_mosaic {
                    let mosaic_result = mosaic::mosaic(
                        rgba,
                        x,
                        y,
                        info.bbox().width(),
                        info.bbox().height(),
                        match block_size {
                            Some(size) => size,
                            None => 0,
                        },
                    );
                    if let Ok(mosaic_img) = mosaic_result {
                        result.extend(mosaic_img);
                    }
                } else {
                    let replace_result = replace::replace(
                        rgba,
                        overlay,
                        x,
                        y,
                        info.bbox().width(),
                        info.bbox().height(),
                    );
                    if let Ok(replace_img) = replace_result {
                        result.extend(replace_img);
                    }
                }
            })
            .collect()
    });
    result
}
