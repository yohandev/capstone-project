use framework::prelude::*;

/// threshold algorithm
pub struct ThresholdAlgorithm
{
    /// unprocessed input image
    img: Image,
    /// average pixel colour in `img`
    avg: Rgba<u8>,
    /// threshold value, [0..1] ± 0.1
    val: f32,
}

impl Sketch for ThresholdAlgorithm
{
    fn setup(app: &mut App) -> Self
    {
        // load source image
        let img = app
            .load_image("res/08.jpg")
            .unwrap();
        // get the average colour in source image
        let avg = average(&img);
        // default threshold value
        let val = 0.5;

        // create a canvas to draw to
        app.create_canvas("hog", img.size());

        // return the app state
        Self { img, avg, val }
    }

    fn draw(&mut self, c: &mut Canvas)
    {
        // draw the unprocessed image
        c.image(&self.img, v![0, 0]);

        // work directly on the canvas, where we just copied
        // the image
        threshold_algo(c, self.avg, self.val);
    }

    fn update(&mut self, app: &mut App)
    {
        if app.keys().down(btn!("right"))
        {
            self.val += 0.1 * app.time().dt();

            println!("THRESHOLD: {}", self.val);
        }
        else if app.keys().down(btn!("left"))
        {
            self.val -= 0.1 * app.time().dt();

            println!("THRESHOLD: {}", self.val);
        }
    }
}

/// returns the average of each channel in the colours of an image
fn average(img: &Image) -> Rgba<u8>
{
    let sum: Rgba<f32> = img
        .iter_pixels()
        .map(|(_, px)| px.as_())
        .sum();
    let avg: Rgba<f32> = sum / (img.area() as f32);
    let avg: Rgba<u8> = avg.map(|n| (if n > 255.0 { 255.0 } else { n }) as u8);

    avg
}

/// returns the magnitude of a colour's R, G, and B components
fn colour_magnitude(px: Rgba<f32>) -> f32
{
    let mag_sqr = px.r * px.r
                + px.g * px.g
                + px.b * px.b;

    mag_sqr.sqrt()
}

/// the™ threshold algorithm
/// img: the image(hard coded to be a canvas)
/// base: colour to base threshold on
/// val: threshold value
fn threshold_algo(img: &mut Canvas, base: Rgba<u8>, val: f32)
{
    // convert to Rgba<f32> ; need the precision
    let col = base.as_();

    img.par_iter_pixels_mut().for_each(|(_, px)|
    {
        let pxf32 = px.as_();
        let dt = pxf32 - col;

        let grey = colour_magnitude(dt) / 255.0;

        *px = if grey > val { c!("black") } else { c!("white") };
    })
}