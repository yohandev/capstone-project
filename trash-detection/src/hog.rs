#[allow(dead_code)]

use framework::prelude::*;

pub struct Hog(Image, f32, Rgba<u8>);

impl Sketch for Hog
{
    fn setup(app: &mut App) -> Self
    {
        // load image
        let mut img = app
            .load_image("res/08.jpg")
            .unwrap();
        
        // process image
        //normalize(&mut img);
        // colour
        let dom_col = dominant_colour(&img);
        //println!("{}", dom_col);

        //remove_colour(&mut img, dom_col);
        //extremize(&mut img, 5);
        //threshold(&mut img, 0.75);

        app.create_canvas("hog", img.size());

        Self(img, 0.5, dom_col)
    }

    fn draw(&mut self, c: &mut Canvas)
    {
        c.image(&self.0, v![0, 0]);

        threshold_col(c, self.2, self.1);
    }

    fn update(&mut self, app: &mut App)
    {
        if app.keys().down(btn!("right"))
        {
            self.1 += 0.1 * app.time().dt();

            println!("THRESHOLD: {}", self.1);
        }
        else if app.keys().down(btn!("left"))
        {
            self.1 -= 0.1 * app.time().dt();

            println!("THRESHOLD: {}", self.1);
        }
    }
}

fn colour_magnitude(px: Rgba<f32>) -> f32
{
    let mag_sqr = px.r * px.r
                + px.g * px.g
                + px.b * px.b;

    mag_sqr.sqrt()
}

/// normalize an image's pixels
fn normalize(img: &mut Image)
{
    img.par_iter_pixels_mut().for_each(|(_, px)|
    {
        *px =
        {
            let mut px = px.as_();
            let mul = 255.0 / colour_magnitude(px);

            px.r *= mul;
            px.g *= mul;
            px.b *= mul;

            px.as_()
        };        
    })
}

fn dominant_colour(img: &Image) -> Rgba<u8>
{
    let sum: Rgba<f32> = img
        .iter_pixels()
        .map(|(_, px)| px.as_())
        .sum();
    let avg: Rgba<f32> = sum / (img.area() as f32);
    let avg: Rgba<u8> = avg.map(|n| (if n > 255.0 { 255.0 } else { n }) as u8);

    avg
}

fn remove_colour(img: &mut Image, col: Rgba<u8>)
{
    let col = col.as_();

    img.par_iter_pixels_mut().for_each(|(_, px)|
    {
        *px =
        {
            let px = px.as_();
            let dt = px - col;

            /// non-linearity function that squishes values
            /// from 0.0 to 1.0
            fn sigmoid(val: f32) -> f32
            {
                1.0 / (1.0 + (-val).exp())
            }

            let mag = colour_magnitude(dt) / (255.0);
            let val = sigmoid(mag) * 255.0;

            Rgba::grey(val as u8)
        }
    })
}

fn extremize(img: &mut Image, n: i32)
{
    img.par_iter_pixels_mut().for_each(|(_, px)|
    {
        *px =
        {
            let px = px.as_();

            let mag = colour_magnitude(px);
            let deg = 255.0 * (mag / 255.0).powi(n);

            Rgba::grey(deg as u8)
        };     
    })
}

fn invert(img: &mut Image)
{
    img.par_iter_pixels_mut().for_each(|(_, px)|
    {
        px.r = 255 - px.r;
        px.g = 255 - px.g;
        px.b = 255 - px.b;
    })
}

fn threshold(img: &mut Canvas, val: f32)
{
    img.par_iter_pixels_mut().for_each(|(_, px)|
    {
        let grey = px.b as f32 / 255.0;

        *px = if grey > val { c!("black") } else { c!("white") };
    })
}

fn threshold_col(img: &mut Canvas, discriminant: Rgba<u8>, val: f32)
{
    let col = discriminant.as_();

    img.par_iter_pixels_mut().for_each(|(_, px)|
    {
        let pxf32 = px.as_();
        let dt = pxf32 - col;

        let grey = colour_magnitude(dt) / 255.0;

        *px = if grey > val { c!("black") } else { c!("white") };
    })
}

// fn gradient(app: &mut App, src: &mut Image)
// {
//     const BLOCK_SIZE: usize = 16;

//     let mut out = app.create_image((src.width() / BLOCK_SIZE, src.height() / BLOCK_SIZE));

//     for block in src.iter_pixel_chunks(v![BLOCK_SIZE, BLOCK_SIZE].into())
//     {
//         let pos = block.id() / BLOCK_SIZE;
//     }
    
// }