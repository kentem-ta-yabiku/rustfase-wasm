use crate::models::mosaic::Average;
use crate::models::mosaic::Rgb;
use crate::models::mosaic::Row;

pub fn mosaic(
    rgba: &[u8],
    img_width: u32,
    x: i32,
    y: i32,
    face_width: u32,
    face_height: u32,
    block_size: usize,
) -> Vec<Row> {
    rgba.chunks(4 * img_width as usize)
        .skip(y as usize)
        .take(face_height as usize)
        .collect::<Vec<_>>()
        .chunks_exact(block_size)
        .filter_map(|rows| {
            rows.iter()
                .filter_map(|row| {
                    let left = 4 * x as usize;
                    let cols = row
                        .get(left..left + 4 * face_width as usize)?
                        .chunks_exact(4 * block_size)
                        .filter_map(|pixels| {
                            pixels
                                .chunks(4)
                                .map(Rgb::new)
                                .reduce(|acc, e| acc.average(e))
                        })
                        .collect();
                    Some(Row::new(cols))
                })
                .reduce(|acc, e| acc.average(e))
        })
        .collect()
}
