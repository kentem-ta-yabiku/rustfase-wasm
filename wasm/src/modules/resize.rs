use crate::models::mosaic::Rgb;
use crate::models::mosaic::Row;
use image::{imageops, DynamicImage, ImageBuffer, Rgba};

pub fn embed_overlay(
    rgba: &[u8],
    img_width: u32,
    x: i32,
    y: i32,
    face_width: u32,
    face_height: u32,
    overlay_image: &[u8], // オーバーレイ画像を追加
) -> Vec<Row> {
    // ImageDataからImageBufferを作成
    let overlay: ImageBuffer<Rgba<u8>, _> =
        ImageBuffer::from_raw(face_width, face_height, overlay_image.to_vec())
            .expect("Failed to create ImageBuffer from overlay data");

    // ImageBufferをDynamicImageに変換
    let overlay_dynamic = DynamicImage::ImageRgba8(overlay);

    // リサイズ処理
    let overlay_resized =
        overlay_dynamic.resize_exact(face_width, face_height, imageops::FilterType::Nearest); // リサイズ方法を指定

    // リサイズ後の画像をRGBAに変換
    let overlay_resized = overlay_resized.to_rgba8();

    rgba.chunks(4 * img_width as usize)
        .skip(y as usize)
        .take(face_height as usize)
        .enumerate()
        .map(|(i, row)| {
            let left = 4 * x as usize;
            let new_row = row.get(left..left + 4 * face_width as usize).unwrap_or(&[]);

            // オーバーレイ画像のピクセルを埋め込む
            let start_idx = i * face_width as usize * 4;
            let end_idx = start_idx + (face_width * face_height * 4) as usize;

            for (idx, pixel) in overlay_resized.pixels().enumerate() {
                let rgba_pixel = pixel.0; // (r, g, b, a)
                let new_pixel_idx = start_idx + idx * 4;
                if new_pixel_idx < end_idx {
                    new_row[new_pixel_idx..new_pixel_idx + 4].copy_from_slice(&rgba_pixel);
                }
            }

            Row::new(new_row.chunks(4).map(Rgb::new).collect())
        })
        .collect()
}
