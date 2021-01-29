use framework::prelude::*;

pub struct Algo3
{
    /* you can have fields here */
    image: Image
}

fn process_image(img: &mut Image)
{
    for (position, pixel) in img.iter_pixels_mut()
    {
        *pixel = Rgba::new(0, 255, 0, 255); // set to green(R, G, B, A)
    }
}

impl Sketch for Algo3
{
    fn setup(app: &mut App) -> Self
    {
        let image = app
            .load_image("res/minecraft_water.png")
            .unwrap();

        app.create_canvas("algo3", image.size());

        Self { image, /* initialize fields here */ }
    }

    fn draw(&mut self, c: &mut Canvas)
    {
        // draw the image at (0, 0)
        c.image(&self.image, v![0, 0]);
    }
}