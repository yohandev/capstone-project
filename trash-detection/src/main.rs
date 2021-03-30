use framework::math::Wrap;

//mod algo;
mod algo2;

fn main()
{
    framework::run::<algo2::TrashDetection>();
}

/// get the next image path, incrementing by `inc`
pub fn path(inc: isize) -> String
{
    // current image index
    static mut CUR: isize = 25;

    // SAFETY: I'm kidding, this is totally unsafe. Assuming
    // this is called from the main thread only
    unsafe
    {
        // increment
        CUR = (CUR + inc).wrapped_between(24, 34);

        // return path
        format!("res/downscaled/DJI_00{}.jpg", CUR)
    }
}