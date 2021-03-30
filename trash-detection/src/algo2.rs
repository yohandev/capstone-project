//! similar to the first iteration of this algorithm,
//! only it calculates the colour range in HSL space,
//! with insights from PhotoShop's own colour range
//! function.

use framework::prelude::*;

pub struct TrashDetection
{
    /// source image
    img: Image,
    /// current fuzziness
    tol: Vec3<f32>,
    /// mask background
    bg: [u8; 3],
    /// should dilate?
    dil: bool,
    /// should erode?
    ero: bool,
    /// should colour?
    col: bool,
}

impl Sketch for TrashDetection
{
    fn setup(app: &mut App) -> Self
    {
        // load source image
        let img = app.load_image(super::path(0)).unwrap();
        
        // create canvas
        app.create_canvas("trash detection", img.size());

        Self
        {
            img,
            tol: v![2.5, 10.0, 10.0],
            bg: [0, 50, 200],
            dil: true,
            ero: true,
            col: true,
        }
    }

    fn draw(&mut self, c: &mut Canvas)
    {
        // B/W mask of trash & obstacles
        let mask = util::cselect(&self.img, self.tol);

        // background blue
        let bg = c!
        [
            self.bg[0], // * 255.0) as u8,
            self.bg[1], // * 255.0) as u8,
            self.bg[2], // * 255.0) as u8,
            255
        ];

        let out = if self.dil { util::cdilate(&mask, v![3, 3]) } else { mask };
        let mut out = if self.ero { util::cerode(&out, v![3, 3]) } else { out };

        if self.col
        {
            // RGB mask of trash & obstacles
            util::capply(&self.img, &mut out, bg);
        }

        // draw coloured mask
        c.image(&out, v![0, 0]);
    }

    fn gui(&mut self, g: &mut Gui)
    {
        g.window("tolerance").build(|ui|
        {
            ui.label("configure the tolerance on each channel");
            ui
                .slider(&mut self.tol.x, 0.0..=7.5)
                .logarithmic(true)
                .text("hue")
                .build();
            ui
                .slider(&mut self.tol.y, 0.0..=20.0)
                .logarithmic(true)
                .text("saturation")
                .build();
            ui
                .slider(&mut self.tol.z, 0.0..=20.0)
                .logarithmic(true)
                .text("lightness")
                .build();
            ui
                .color_edit_button_srgb(&mut self.bg)
                .on_hover_text("background colour");
            ui.horizontal(|ui|
            {
                ui.checkbox(&mut self.dil, "dilate?");
                ui.checkbox(&mut self.ero, "erode?");
            });
            ui.checkbox(&mut self.col, "colour?");
            ui.horizontal(|ui|
            {
                if ui.button("prev").clicked()
                {
                    self.img = Image::open(super::path(-1)).unwrap();
                }
                ui.label(super::path(0));
                if ui.button("next").clicked()
                {
                    self.img = Image::open(super::path(1)).unwrap();
                }
            });
        });
    }
}

mod util
{
    use framework::{ c, v };
    use framework::math::*;
    use framework::draw::*;

    macro_rules! bitmap_t
    {
        (ref) => { &Bitmap<impl std::any::Any, impl FlatPixelBuf> };
        (mut) => { &mut Bitmap<impl std::any::Any, impl FlatPixelBufMut> };
    }

    /// RGB[0..=255] -> HSL[0..=1.0]
    pub fn hsl(rgb: Rgb<u8>) -> Vec3<f32>
    {
        // RGB[0..=255] -> RGB[0..=1.0]
        let Rgb { r, g, b } = rgb.as_::<f32>() / 255.0;

        // RGB[min, max]
        let min = r.min(g).min(b);
        let max = r.max(g).max(b);

        // chroma
        let chr = max - min;

        // lightness
        let l = (max + min) / 2.0;
        // hue[0..=1.0], saturation[0..=1.0]
        let (h, s) = if chr == 0.0
        {
            // achromatic
            (0.0, 0.0)
        }
        else
        {
            let hue = match max
            {
                m if m == r => (g - b) / chr + (if g < b { 6.0 } else { 0.0 }),
                m if m == b => (r - g) / chr + 4.0,
                m if m == g => (b - r) / chr + 2.0,
                _ => unreachable!()
            };
            let sat = if l > 0.5
            {
                chr / (2.0 - max - min)
            }
            else
            {
                chr / (max + min)
            };

            (hue / 6.0, sat)
        };
        v![h, s, l]
    }

    /// returns a completely opaque, black image of size `siz`
    pub fn black(siz: Extent2<usize>) -> Image
    {
        let mut buf = vec![c!("black"); siz.w * siz.h];

        // SAFETY: Rgba<u8> is 4 packed u8
        let buf = unsafe
        {
            let len = buf.len() * 4;
            let cap = buf.capacity() * 4;
            let ptr = buf.as_mut_ptr() as *mut u8;

            // skip destructor
            std::mem::forget(buf);

            // RGBA -> u8[4]
            Vec::from_raw_parts(ptr, len, cap)
        };
        Image::new((), buf, siz)
    }

    /// iterate over two bitmaps in parallel:
    /// - `a`: immutable bitmap
    /// - `b`: mutable bitmap
    /// - `f`: fn(pos, a[pos], b[pos]) -> ()
    pub fn iter2(a: bitmap_t!(ref), b: bitmap_t!(mut), f: impl Fn(Vec2<i32>, &Rgba<u8>, &mut Rgba<u8>) + Send + Sync)
    {
        a
            .par_iter_pixels()
            .zip(b.par_iter_pixels_mut())
            .for_each(|((pos, a), (_, b))| f(pos, a, b))
    }

    /// (mean, stdev) of image in HSL
    /// considers only fully opaque pixels
    pub fn cstats(img: bitmap_t!(ref)) -> (Vec3<f32>, Vec3<f32>)
    {
        /// mean of image in HSL
        fn mean(img: bitmap_t!(ref)) -> Vec3<f32>
        {
            // sum HSL[0..=n]
            let sum = img
                .par_iter_pixels()
                .filter_map(|(_, col)| (col.a == 255).then(|| hsl(col.rgb())))
                .sum::<Vec3<f32>>();

            // mean HSL[0..=1.0]
            sum / img.area() as f32
        }
        /// standard deviaiton of image is HSL
        fn stdev(img: bitmap_t!(ref), mean: Vec3<f32>) -> Vec3<f32>
        {
            // sum of differences squared HSL[0..=n]
            let sumd_sqr = img
                .par_iter_pixels()
                .filter_map(|(_, col)| (col.a == 255).then(|| (hsl(col.rgb()) - mean).map(|n| n.powi(2))))
                .sum::<Vec3<f32>>();
            // stdev HSL
            (sumd_sqr / (img.area() as f32 - 1.0)).sqrt()
        }
        let mean = mean(img);
        
        (mean, stdev(img, mean))
    }

    /// applies gaussian's normal distribution function to:
    /// - `x`: sample point
    /// - `mean`: sample mean
    /// - `stdev`: sample standard deviation
    pub fn gauss_fn(x: f32, mean: f32, stdev: f32) -> f32
    {
        /// √(2π)
        const SQRT_TAU: f32 = 2.50662827463;
        /// e
        const E: f32 = std::f32::consts::E;

        // formula: https://i.ytimg.com/vi/IIuXF5QRBTY/maxresdefault.jpg
        (1.0 / (stdev * SQRT_TAU)) * E.powf(-0.5 * ((x - mean) / stdev).powi(2))
    }

    /// colour range select
    /// - `sample`: colour to compare against
    /// - `tol`: deviations from mean in HSL
    ///
    /// returns the selection mask
    pub fn cselect(img: bitmap_t!(ref), tol: Vec3<f32>) -> Image
    {
        // HSL statistics
        let (mean, stdev) = cstats(img);

        // selection mask
        let mut mask = black(img.size());
        
        // write to mask in parallel
        iter2(img, &mut mask, |_, px, mask|
        {
            // deviations
            let val = hsl(px.rgb()).map3(mean, stdev, |x, mu, sig| gauss_fn(x, mu, sig));

            if val.x <= tol.x
            && val.y <= tol.y
            && val.z <= tol.z
            {
                *mask = c!("white")
            }
        });
        mask
    }

    /// dilate an image, setting pixels to white if any
    /// in its neighboring area is white, black otherwise
    pub fn cdilate(img: bitmap_t!(ref), siz: impl Into<Extent2<usize>>) -> Image
    {
        let siz = siz.into();
        let mid = v![siz.w as i32, siz.h as i32] / 2; 

        let mut out = black(img.size());

        for win in img.iter_pixel_windows(siz)
        {
            // if at least one pixel is white...
            out[win.id() + mid] = if win
                .iter_pixels()
                .find(|(_, px)| **px == c!("white"))
                .is_some()
            // ...set to white, otherwise black
            { c!("white") } else { c!("black") };
        }

        out
    }

    /// dilate an image, setting pixels to black if any
    /// in its neighboring area is black, white otherwise
    pub fn cerode(img: bitmap_t!(ref), siz: impl Into<Extent2<usize>>) -> Image
    {
        let siz = siz.into();
        let mid = v![siz.w as i32, siz.h as i32] / 2; 

        let mut out = black(img.size());

        for win in img.iter_pixel_windows(siz)
        {
            // if at least one pixel is white...
            out[win.id() + mid] = if win
                .iter_pixels()
                .find(|(_, px)| **px == c!("black"))
                .is_some()
            // ...set to white, otherwise black
            { c!("black") } else { c!("white") };
        }

        out
    }

    /// copies every pixel from `col` to `dst` where `dst`
    /// is white, otherwise applies the `or` colour
    pub fn capply(col: bitmap_t!(ref), dst: bitmap_t!(mut), or: Rgba<u8>)
    {
        assert_eq!(col.size(), dst.size(), "image sizes must match!");

        iter2(col, dst, |_, px, mask|
        {
            if *mask == c!("white")
            {
                *mask = *px;
            }
            else
            {
                *mask = or;
            }
        })
    }
}