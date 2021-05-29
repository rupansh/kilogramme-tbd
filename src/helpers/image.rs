// Copyright 2021 - 2021, Rupansh Sekar and the Kilogramme (TBD) contributors
// SPDX-License-Identifier: MPL-2.0
use std::io::Cursor;
use image::{DynamicImage, ImageResult, GenericImageView, imageops::FilterType, codecs::png::PngEncoder};

/// Load image from u8 buffer
pub fn image_from_buf(buf: &[u8]) -> ImageResult<DynamicImage> {
    image::io::Reader::new(Cursor::new(buf))
        .with_guessed_format()
        .unwrap() // Cursor doesn't cause an io error for this method
        .decode()
}

/// Resize image to largest possible size that fits the target w, h
/// uses Linear filter
pub fn im_resize_clamped(im: &DynamicImage, w: u32, h: u32) -> DynamicImage {
    im.resize(w, h, FilterType::Triangle)
}

pub fn png_encode(im: DynamicImage) -> ImageResult<Vec<u8>> {
    let mut out = Vec::new();
    let encoder = PngEncoder::new(&mut out);
    let w = im.width();
    let h = im.height();
    let color = im.color();
    encoder.encode(&im.into_bytes(), w, h, color)?;
    Ok(out)
}
