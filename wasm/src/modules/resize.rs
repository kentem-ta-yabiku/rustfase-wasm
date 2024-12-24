use crate::models::mosaic::Rgb;
use crate::models::mosaic::Row;
use image::{imageops, DynamicImage, ImageBuffer, Rgba};

pub fn resize(
    overlay_image: &[u8], // オーバーレイ画像を追加
    face_width: u32,
    face_height: u32,
) -> Vec<Row> {
    // ImageDataからImageBufferを作成
    let overlay: ImageBuffer<Rgba<u8>, _> =
        ImageBuffer::from_raw(face_height, face_width, overlay_image.to_vec())
            .expect("Failed to create ImageBuffer from overlay data");

    // ImageBufferをDynamicImageに変換
    let overlay_dynamic = DynamicImage::ImageRgba8(overlay);

    // リサイズ処理
    let overlay_resized =
        overlay_dynamic.resize_exact(face_width, face_height, imageops::FilterType::Nearest); // リサイズ方法を指定

    // リサイズ後の画像をRGBAに変換
    let overlay_resized = overlay_resized.to_rgba8();
    let mut rows = Vec::new();

    for y in 0..overlay_resized.height() {
        let mut cols = Vec::new();
        for x in 0..overlay_resized.width() {
            let pixel = overlay_resized.get_pixel(x, y);
            let rgba = pixel.0; // [R, G, B, A]
            cols.push(Rgb::new(&rgba[0..3])); // A チャネルは無視
        }
        rows.push(Row::new(cols));
    }
    rows
}
