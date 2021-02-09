use framework::prelude::*;

/// as of now, this is effectively down-scaling the source
/// image
pub struct WonkySlidy
{
    /// unprocessed input image
    img: Image,
}

/// amount to slide the window by
const SLIDE_AMT: Extent2<i32> = Extent2 { w: 20, h: 20 };

impl Sketch for WonkySlidy
{
    fn setup(app: &mut App) -> Self
    {
        // load source image
        let img = app
            .load_image("res/DJI_0019.jpg")
            .unwrap();
        
        // processed image is smaller than source
        // image by a factor SLIDE_AMT in their
        // respective dimensions
        let siz: Extent2<usize> = img
            .size()
            .map2(SLIDE_AMT, |a, b| a / b as usize);

        // create a canvas to draw to
        app.create_canvas("hog", siz);

        // lower frame-rate to not explode
        // potatoes with a CPU
        app.time().frame_rate(15.0);

        // return the app state
        Self { img }
    }

    fn draw(&mut self, c: &mut Canvas)
    {
        for chunk in self.img
            .iter_pixel_overlapping_chunks(v![20, 20].into(), SLIDE_AMT.as_().into())
        {
            // chunk.id() is the position of the top-left corner
            let mut pos = *chunk.id();

            // output image is smaller than source by a factor of
            // SLIDE_AMT
            pos.x /= SLIDE_AMT.w;
            pos.y /= SLIDE_AMT.h;
            
            // set to top left in original
            c[pos] = chunk[v![0, 0]];
        }
    }
}