//! # Histogram Algorithm
//! a subset of the threshold method
//!
//! ## Overview
//! in: arbitrary water threshold, wt(float)
//! in: arbitrary chunk threshold, ct(int)
//! in: recursion limit, n(int)
//! in: source image(raster)
//! 1. Determine the average water colour for entire image, µ
//! 2. Subdivide image in k chunks of equal size
//! 3. For-each chunk, count the number of pixels that fail
//!    the threshold test, wt, where value evaluated is pixel
//!    to µ(average water colour) and store in a histogram
//! 4. For-each entry in histogram(corresponding to a chunk),
//!    we can conclude that an object is contained if its value
//!    exceeds the chunk threshold, ct
//! 5. Increment k towards the recursion limit n for finer details

use framework::prelude::*;

use crate::util;

const SOURCE_PATH: &str = "res/YellowBall.jpg";

pub struct HistogramAlgo
{
    /// unprocessed source image, img
    src: Image,
    /// average colour of the water, µ
    avg: Rgba<u8>,

    /// arbitrary water threshold
    wt: f32,
    /// arbitrary chunk threshold
    ct: i32,
    /// recursion limit
    n: i32,
}

impl Sketch for HistogramAlgo
{
    fn setup(app: &mut App) -> Self
    {
        // load source image
        let img = app.load_image(SOURCE_PATH).unwrap();

        // get the average colour in source image
        let mu = util::average(&img);

        // create a canvas to draw to
        app.create_canvas("bbc operation", img.size());

        // return the app state
        Self
        {
            src: img,
            avg: mu,

            wt: 0.5,
            ct: 3,
            n: 20
        }
    }

    fn draw(&mut self, c: &mut Canvas)
    {
        // copy image to the canvas, then work on canvas bitmap directly
        c.image(&self.src, v![0, 0]);

        
    }
}