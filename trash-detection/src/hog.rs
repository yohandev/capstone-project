use framework::prelude::*;

pub struct Hog(Image);

impl Sketch for Hog
{
    fn setup(app: &mut App) -> Self
    {
        // load image
        let mut img = app
            .load_image("res/minecraft_water.png")
            .unwrap();
        
        // process image
        normalize(&mut img);
        // colour
        let dom_col = dominant_colour(&img);
        println!("{}", dom_col);

        remove_colour(&mut img, dom_col);
        //remove_blue(&mut img);

        app.create_canvas("hog", img.size());

        Self(img)
    }

    fn draw(&mut self, c: &mut Canvas)
    {
        c.image(&self.0, v![0, 0]);
    }
}

/// normalize an image's pixels
fn normalize(img: &mut Image)
{
    img.par_iter_pixels_mut().for_each(|(_, px)|
    {
        let mut px_f32 = px.as_::<f32>();
        let mag_sqr = px_f32.r * px_f32.r
                    + px_f32.g * px_f32.g
                    + px_f32.b * px_f32.b;
        let mul = 255.0 / mag_sqr.sqrt();

        px_f32.r *= mul;
        px_f32.g *= mul;
        px_f32.b *= mul;

        *px = px_f32.as_();
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
    img.par_iter_pixels_mut().for_each(|(_, px)|
    {
        *px /= col; 
        // undo alpha
        px.a *= col.a;
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