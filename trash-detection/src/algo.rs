use framework::prelude::*;

pub struct TrashDetection
{
    /// source image
    img: Image,
    /// current mouse position
    cur: Vec2<i32>,
    /// current tolerances
    tol: (f32, f32),
    /// gaussian falloff arguments
    gauss: (f32, f32, f32),
}

impl Sketch for TrashDetection
{
    fn setup(app: &mut App) -> Self
    {
        // load the source image
        let img = app
            .load_image(super::PATH)
            .unwrap();

        // image canvas
        app.create_canvas(format!("\"{}\"", super::PATH), v![img.width(), img.height()]);
        
        // ...defaults
        let cur = v![0, 0];
        let tol = (0.25, 0.25);
        let gauss = (1.0, 0.0, 1.0);

        Self { img, cur, tol, gauss }
    }

    fn draw(&mut self, c: &mut Canvas)
    {
        // STEP 01: remove water -> remainder: lane markers, shadows, trash
        let rm_water =
        {
            // average colour is water
            let avg = draw::avg_col(&self.img);

            // remove water with threshold -> b/w mask
            let mut mask = draw::select(&self.img, avg, self.tol.0, self.gauss);

            // b/w mask -> coloured image
            draw::mask(&mut mask, &self.img, true);

            mask
        };
        c.image(&rm_water, v![0, 0]);

        // STEP 02: remove most dominant obstacle
        // let rm_obstacle = 
        // {
        //     // average colour is water
        //     let avg = draw::avg_col(&rm_water);

        //     // remove water with threshold -> b/w mask
        //     let mut mask = draw::select(&rm_water, avg, self.tol.1, self.gauss);

        //     // b/w mask -> coloured image
        //     draw::mask(&mut mask, &rm_water, true);

        //     mask
        // };
        // c.image(&rm_obstacle, v![self.img.width() as i32, 0]);

        // // draw to: source image
        // c.image(&self.img, v![0, 0]);
    }

    fn update(&mut self, app: &mut App)
    {
        // current mouse position
        self.cur = app.mouse().position();
        // clamp to display image
        self.cur.x = self.cur.x.min(self.img.width() as i32 - 1);
        self.cur.y = self.cur.y.min(self.img.height() as i32 - 1);
    }

    fn gui(&mut self, gui: &mut Gui)
    {
        gui
            .window("inspector")
            .resizable(false)
            .build(|ui|
            {
                ui.horizontal(|ui|
                {
                    if ui.button("open image").clicked()
                    {
                        if let nfd::Response::Okay(path) = nfd::dialog()
                            .filter("jpg")
                            .open()
                            .unwrap()
                        {
                            self.img = Image::open(path).unwrap();
                        }
                    }
                });
                ui
                    .slider(&mut self.tol.0, 0.0..=3f32.sqrt())
                    .text("water")
                    .suffix("tolerance")
                    .build();
                ui
                    .slider(&mut self.tol.1, 0.0..=3f32.sqrt())
                    .text("lane markers")
                    .suffix("tolerance")
                    .build();

                ui.separator();
                ui.label("gaussian falloff");
                ui.slider(&mut self.gauss.0, 0.0..=1.0).build();
                ui.slider(&mut self.gauss.1, -(3f32.sqrt())..=5.0).build();
                ui.slider(&mut self.gauss.2, 0.0..=1.0).build();
            })
    }
}

mod draw
{
    use framework::{ c, v };
    use framework::math::*;
    use framework::draw::*;

    /// shorthand for a generic type that implements `Bitmap`
    macro_rules! bitmap_t
    {
        (ref) => { &Bitmap<impl std::any::Any, impl FlatPixelBuf> };
        (mut) => { &mut Bitmap<impl std::any::Any, impl FlatPixelBufMut> };
    }

    /// creates a completely black image from the dimensions of the input image
    fn create_mask(img: bitmap_t!(ref)) -> Image
    {
        Image::new((), vec![0; img.area() * 4], img.size())
    }

    /// distance between two colours, ignoring alpha
    fn dist(a: Rgba<u8>, b: Rgba<u8>) -> f32
    {
        v![
            a.r as f32 - b.r as f32,    // r
            a.g as f32 - b.g as f32,    // g
            a.b as f32 - b.b as f32     // b
        ].magnitude() / 255.0
    }

    /// applies the gaussian falloff function to `x`
    ///
    /// - `a`: height of the curve's peak
    /// - `b`: position of the center of peak
    /// - `c`: standard deviation
    fn gauss_falloff(x: f32, a: f32, b: f32, c: f32) -> f32
    {
        a * std::f32::consts::E.powf(-0.5 * ((x - b) / c).powi(2))
    }

    /// selects pixels based on the colour at a given point and a given tolerance
    /// value, using the flood fill algorithm. returns the mask of the selection
    pub fn qfill(img: bitmap_t!(ref), pt: Vec2<i32>, tol: f32, depth: usize) -> Image
    {
        // create the selection mask, defaults to black
        let mut mask = create_mask(img);
        
        // colour to base floodfill on
        let col = img[pt];

        /// tests that a colour should be flood filled
        fn colour_test(a: Rgba<u8>, b: Rgba<u8>, tol: f32) -> bool
        {
            dist(a, b) <= tol
        }

        /// recrusively flood fill
        fn recursive_fill(img: bitmap_t!(ref), mask: bitmap_t!(mut), col: Rgba<u8>, tol: f32, pt: Vec2<i32>, i: usize)
        {
            // recursion limit
            if i == 0
            {
                return;
            }

            // out of bounds -> bail
            if pt.x < 0
            || pt.y < 0
            || pt.x >= img.width() as i32
            || pt.y >= img.height() as i32
            {
                return;
            }
            // pixel shouldn't be filled
            if !colour_test(img[pt], col, tol) { return; }
            // already filled this pixel
            if mask[pt] == c!("white") { return; }

            // fill on mask
            mask[pt] = c!("white");

            // recurse
            recursive_fill(img, mask, col, tol, pt + v![1, 0], i - 1);
            recursive_fill(img, mask, col, tol, pt - v![1, 0], i - 1);
            recursive_fill(img, mask, col, tol, pt + v![0, 1], i - 1);
            recursive_fill(img, mask, col, tol, pt - v![0, 1], i - 1);
        }

        // start recurse
        recursive_fill(img, &mut mask, col, tol, pt, depth);

        // return mask
        mask
    }

    /// selects all pixels that match the given colour within a certain tolerance
    /// threshold
    ///
    /// returns a mask where white = selected, black = unselected
    pub fn select(img: bitmap_t!(ref), col: Rgba<u8>, tol: f32, (a, b, c): (f32, f32, f32)) -> Image
    {
        // create the selection mask, defaults to black
        let mut mask = create_mask(img);
        
        // write to mask in parallel
        img
            .par_iter_pixels()
            .zip(mask.par_iter_pixels_mut())
            .for_each(|((_, px), (_, mask))|
        {
            if gauss_falloff(dist(*px, col), a, b, c) <= tol
            {
                *mask = c!("white")
            }
        });

        mask
    }

    /// determines the average colour in a bitmap, ignoring alpha
    pub fn avg_col(img: bitmap_t!(ref)) -> Rgba<u8>
    {
        // sum as a [r, g, b] vector
        let sum: Vec3<f32> = img
            .par_iter_pixels()
            .filter_map(|(_, n)| (n.a == 255).then(|| v![n.r as f32, n.g as f32, n.b as f32]))
            .sum();
        // average [r, g, b] vector in n<[0.0..=255.0] range
        let avg = sum / img.area() as f32;

        // average colour as rgba<u8>
        c![avg.x as u8, avg.y as u8, avg.z as u8, 255]
    }

    /// places the pixels of `col` onto `mask` for every pixel in `mask`
    /// that's white(or black, if `invert`ed)
    ///
    /// the two images must be of the same size
    pub fn mask(mask: bitmap_t!(mut), col: bitmap_t!(ref), invert: bool)
    {
        // two images must match in size!
        assert_eq!(mask.size(), col.size());

        // compare function
        let cmp: fn(Rgba<u8>) -> bool = if !invert
        {
            |c| c == c!("white")
        }
        else
        {
            |c| c == c!(0, 0, 0, 0)
        };

        // write to mask in parallel
        col
            .par_iter_pixels()
            .zip(mask.par_iter_pixels_mut())
            .for_each(|((_, col), (_, mask))|
        {
            if cmp(*mask)
            {
                *mask = *col;
            }
            else
            {
                *mask = c!(0, 0, 0, 0);
            }
        });
    }
}