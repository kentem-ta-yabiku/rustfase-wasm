use image::{DynamicImage, ImageOutputFormat};
use std::io::Cursor;

pub fn replace(
    base_image_bytes: &[u8],
    overlay_image_bytes: &[u8],
    x: i32,
    y: i32,
    width: u32,
    height: u32,
) -> Result<Vec<u8>, String> {
    // 1. バイト列を画像としてデコード
    let mut base_image = image::load_from_memory(base_image_bytes)
        .map_err(|e| format!("Failed to decode base image: {}", e))?
        .to_rgba8();

    let overlay_image = image::load_from_memory(overlay_image_bytes)
        .map_err(|e| format!("Failed to decode overlay image: {}", e))?
        .to_rgba8();

    // 2. Overlay 画像をリサイズ
    let resized_overlay = image::imageops::resize(
        &overlay_image,
        width,
        height,
        image::imageops::FilterType::Lanczos3,
    );

    // 3. Base 画像の指定領域に Overlay を貼り付け
    for (overlay_x, overlay_y, pixel) in resized_overlay.enumerate_pixels() {
        let target_x = x as u32 + overlay_x;
        let target_y = y as u32 + overlay_y;

        if target_x < base_image.width() && target_y < base_image.height() {
            base_image.put_pixel(target_x, target_y, *pixel);
        }
    }

    // 4. 処理後の画像をバイト列としてエンコード
    let mut output_bytes = Cursor::new(Vec::new());

    DynamicImage::ImageRgba8(base_image)
        .write_to(&mut output_bytes, ImageOutputFormat::Jpeg(90))
        .map_err(|e| format!("Failed to encode output image: {}", e))?;

    Ok(output_bytes.into_inner())
}
