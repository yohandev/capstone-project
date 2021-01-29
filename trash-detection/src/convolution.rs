//! # Convolution
//! detects edges using convolution, similar to CNNs

use framework::prelude::*;

/// feature map to detect edges
const FEATURE_MAP: [f32; 9] =
[
    0.0, 1.0, 0.0,
    1.0, -4.0, 1.0,
    0.0, 1.0, 0.0,
];

pub struct Convolution
{
    img: Image,
}

impl Sketch for Convolution
{
    fn setup(app: &mut App) -> Self 
    {
        // the source image
        let src = app
            .load_image("res/Archive/03_out.png")
            .unwrap();

        // convolution process:
        let mut img = app.create_image(v![src.width() - 2, src.height() - 2]);

        for x in 0..img.width() as i32
        {
            for y in 0..img.height() as i32
            {
                /// get the grey level of a color
                fn grey_level(color: Rgba<u8>) -> f32
                {
                    color.as_::<f32>().average_rgb()
                }

                /// non-linearity function that squishes values
                /// from 0.0 to 1.0
                fn sigmoid(val: f32) -> f32
                {
                    1.0 / (1.0 + (-val).exp())
                }

                let c0 = FEATURE_MAP[0] * grey_level(src[v![x, y]]);
                let c1 = FEATURE_MAP[1] * grey_level(src[v![x + 1, y]]);
                let c2 = FEATURE_MAP[2] * grey_level(src[v![x + 2, y]]);
                let c3 = FEATURE_MAP[3] * grey_level(src[v![x, y + 1]]);
                let c4 = FEATURE_MAP[4] * grey_level(src[v![x + 1, y + 1]]);
                let c5 = FEATURE_MAP[5] * grey_level(src[v![x + 2, y + 1]]);
                let c6 = FEATURE_MAP[6] * grey_level(src[v![x, y + 2]]);
                let c7 = FEATURE_MAP[7] * grey_level(src[v![x + 1, y + 2]]);
                let c8 = FEATURE_MAP[8] * grey_level(src[v![x + 2, y + 2]]);

                let sum = c0 + c1 + c2 + c3 + c4 + c5 + c6 + c7 + c8;
                let col = ((1.0 - sigmoid(sum)) * 255.0) as u8;

                img[v![x, y]] = c![col, col, col];
            }
        }

        // create a canvas exactly the size of
        // that image
        app.create_canvas("edge detection", img.size());

        // keep track of the image to draw it later
        Self { img }
    }

    fn draw(&mut self, c: &mut Canvas)
    {
        c.image(&self.img, v![0, 0]);
    }
}