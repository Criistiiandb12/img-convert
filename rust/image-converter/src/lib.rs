use std::io::Cursor;

use image::ImageFormat;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn convert_to_png(input: &[u8]) -> Result<Vec<u8>, JsValue> {
    convert_to_format(input, "png")
}

#[wasm_bindgen]
pub fn convert_to_format(input: &[u8], format: &str) -> Result<Vec<u8>, JsValue> {
    let image = image::load_from_memory(input)
        .map_err(|e| JsValue::from_str(&e.to_string()))?;

    let output_format = match format.to_ascii_lowercase().as_str() {
        "jpg" | "jpeg" => ImageFormat::Jpeg,
        "png" => ImageFormat::Png,
        "tif" | "tiff" => ImageFormat::Tiff,
        "webp" => ImageFormat::WebP,
        "bmp" => ImageFormat::Bmp,
        "ico" => ImageFormat::Ico,
        "avif" => ImageFormat::Avif,
        _ => return Err(JsValue::from_str("Formato de salida no soportado")),
    };

    let mut output = Cursor::new(Vec::new());

    image
        .write_to(&mut output, output_format)
        .map_err(|e| JsValue::from_str(&e.to_string()))?;

    Ok(output.into_inner())
}
