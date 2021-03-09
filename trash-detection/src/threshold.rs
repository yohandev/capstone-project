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
    /// fill evolution(recursion limit on flood)
    fill: f32,

    /// cached mouse from last update
    mouse: Vec2<i32>,
}

impl Sketch for ThresholdAlgorithm
{
    fn setup(app: &mut App) -> Self
    {
        // load source image
        let img = app
            .load_image("res/dorito_00.png")
            .unwrap();
        // get the average colour in source image
        let avg = average(&img);
        // default threshold value
        let val = 0.5;
        // default flood fill recursion limit
        let fill = 100.0;

        // create a canvas to draw to
        app.create_canvas("hog", img.size());

        // return the app state
        Self
        {
            img, avg, val, fill,

            mouse: v![0, 0],
        }
    }

    fn draw(&mut self, c: &mut Canvas)
    {
        // draw the unprocessed image
        c.image(&self.img, v![0, 0]);

        // work directly on the canvas, where we just copied
        // the image
        threshold_algo(c, self.avg, self.val);

        // slow floodfill
        flood_fill(c, self.mouse, self.fill.max(0.0) as u64)
    }

    fn update(&mut self, app: &mut App)
    {
        // target slider to increment
        let (name, slider, speed) = if app.keys().down(btn!(" "))
        {
            ("FILL RECURSION", &mut self.fill, 50.0)
        }
        else
        {
            ("THRESHOLD", &mut self.val, 0.1)
        };

        if app.keys().down(btn!("right"))
        {
            *slider += speed * app.time().dt();

            println!("{}: {}", name, slider);
        }
        else if app.keys().down(btn!("left"))
        {
            *slider -= speed * app.time().dt();

            println!("{}: {}", name, slider);
        }
        self.mouse = app.mouse().position();
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

        *px = if grey > val { *px } else { c!("black") };
    })
}

fn draw_pos(c: &mut Canvas, i: usize)
{
    let siz = c.size().into();

    for (pos, px) in c.iter_pixels_mut().take(i)
    {
        let uv = pos.map2(siz, |p, s: usize| p as f32 / s as f32);
        let uv = (uv * 255.0).as_::<u8>();

        *px = c!(uv.x, uv.y, 0);
    }
}

fn flood_fill(c: &mut Canvas, pos: Vec2<i32>, n: u64)
{
    // recursion limit
    if n <= 0
    {
        return;
    }

    // out of bounds
    if pos.x < 0
    || pos.y < 0
    || pos.x >= c.width() as i32
    || pos.y >= c.height() as i32
    {
        return;
    }

    // not in prev colour
    if c[pos] == c!("black")
    // already filled
    || c[pos] == c!("royalblue")
    {
        return;
    }

    // fill
    c[pos] = c!("royalblue");

    // recurse
    flood_fill(c, pos + v![1, 0], n - 1);
    flood_fill(c, pos + v![0, 1], n - 1);
    flood_fill(c, pos - v![1, 0], n - 1);
    flood_fill(c, pos - v![0, 1], n - 1);
}