use crate::models::mosaic::Rgb;
use crate::models::mosaic::Row;
use image::DynamicImage;
use image::GenericImageView;
use web_sys::console;

pub fn resize(image_data: &[u8], width: u32, height: u32) -> Vec<Row> {
    if image_data.to_vec().len() == 0 {
        return vec![];
    };

    // 画像を読み込み
    let img = match image::load_from_memory(&image_data.to_vec()) {
        Ok(img) => {
            // 成功時にログを表示
            console::log_1(&"Image loaded successfully".into());
            img
        }
        Err(e) => {
            // エラー時にログを表示
            console::error_1(&format!("Failed to load image: {}", e).into());
            DynamicImage::new_rgba8(0, 0)
        }
    };

    // 指定された幅と高さでリサイズ
    let resized_img = img.resize(width, height, image::imageops::FilterType::Lanczos3);

    // 画像の幅と高さを取得
    let (img_width, img_height) = resized_img.dimensions();

    // 各行をVec<Row>として格納
    let mut rows: Vec<Row> = Vec::new();

    for y in 0..img_height {
        let mut cols: Vec<Rgb> = Vec::new();
        for x in 0..img_width {
            // ピクセルを取得
            let rgba = resized_img.get_pixel(x, y).0;
            // ピクセルのRGBA値からRgbを作成
            cols.push(Rgb::new(&rgba));
        }
        // 行としてRowを作成し、Vec<Row>に追加
        rows.push(Row::new(cols));
    }
    rows
}
