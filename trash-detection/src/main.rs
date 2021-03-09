use histogram::HistogramAlgo;

//use threshold::ThresholdAlgorithm;

mod convolution;
// mod wonkyslidy;
mod threshold;
// mod algo3;
// mod hog;
mod histogram;
mod util;

fn main()
{
    framework::run::<threshold::ThresholdAlgorithm>();
}