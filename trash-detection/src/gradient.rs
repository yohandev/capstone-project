use framework::math::*;
use framework::*;

#[derive(Debug, Default)]
pub struct GradientResolver { }

impl App for GradientResolver
{
   fn render(&mut self, frame: &mut Frame)
   {
        frame.par_iter_pixels_mut().for_each(|(_, px)|
        {
            *px = Rgba::blue();
        });
   }
}