use framework::prelude::*;

/// returns the average of each channel in the colours of an image
///
/// TODO: downscale first for speed
pub fn average(img: &Image) -> Rgba<u8>
{
    let sum: Rgba<f32> = img
        .iter_pixels()
        .map(|(_, px)| px.as_())
        .sum();
    let avg: Rgba<f32> = sum / (img.area() as f32);
    let avg: Rgba<u8> = avg.map(|n| (if n > 255.0 { 255.0 } else { n }) as u8);

    avg
}

pub fn gamma_correction(img: &mut Image, gamma: f32)
{
    
}

/// returns the magnitude of a colour's R, G, and B components
pub fn colour_magnitude(px: Rgba<f32>) -> f32
{
    let mag_sqr = px.r * px.r
                + px.g * px.g
                + px.b * px.b;

    mag_sqr.sqrt()
}

