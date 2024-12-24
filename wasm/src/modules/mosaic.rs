use image::{DynamicImage, Rgba};
use std::io::Cursor;

pub fn mosaic(
    base_image: &[u8],
    x: i32,
    y: i32,
    face_width: u32,
    face_height: u32,
    block_size: usize,
) -> Result<Vec<u8>, String> {
    // 1. 元画像をロード
    let mut image_buffer = image::load_from_memory(&base_image)
        .map_err(|e| format!("Failed to decode base image: {}", e))?
        .to_rgba8();
    let (width, height) = image_buffer.dimensions();

    // モザイクをかける部分（顔領域）の各ブロックに処理を適用
    for y in (y..(y + face_height as i32)).step_by(block_size) {
        for x in (x..(x + face_width as i32)).step_by(block_size) {
            // 顔領域内の各ブロックの平均値を計算
            let mut block_pixels = Vec::new();

            // 1ブロック内のピクセルを集める
            for by in y..(y + block_size as i32) {
                for bx in x..(x + block_size as i32) {
                    if bx >= 0 && by >= 0 && bx < width as i32 && by < height as i32 {
                        let pixel = image_buffer.get_pixel(bx as u32, by as u32).0;
                        block_pixels.push(pixel);
                    }
                }
            }

            // 平均化処理を行ってモザイク効果を適用
            let avg_pixel = average_pixels(&block_pixels);
            for by in y..(y + block_size as i32) {
                for bx in x..(x + block_size as i32) {
                    if bx >= 0 && by >= 0 && bx < width as i32 && by < height as i32 {
                        image_buffer.put_pixel(bx as u32, by as u32, Rgba(avg_pixel));
                    }
                }
            }
        }
    }

    // 3. 結果を Vec<u8> に変換
    let mut output_bytes = Cursor::new(Vec::new());
    DynamicImage::ImageRgba8(image_buffer)
        .write_to(&mut output_bytes, image::ImageOutputFormat::Jpeg(90))
        .expect("Failed to encode image");

    Ok(output_bytes.into_inner())
}

// ピクセルを平均化する関数
fn average_pixels(pixels: &Vec<[u8; 4]>) -> [u8; 4] {
    let mut r_sum = 0u32;
    let mut g_sum = 0u32;
    let mut b_sum = 0u32;
    let mut a_sum = 0u32;

    for pixel in pixels {
        r_sum += pixel[0] as u32;
        g_sum += pixel[1] as u32;
        b_sum += pixel[2] as u32;
        a_sum += pixel[3] as u32;
    }

    let len = pixels.len() as u32;
    [
        (r_sum / len) as u8,
        (g_sum / len) as u8,
        (b_sum / len) as u8,
        (a_sum / len) as u8,
    ]
}
