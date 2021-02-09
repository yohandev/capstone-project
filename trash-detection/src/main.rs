use threshold::ThresholdAlgorithm;

mod convolution;
mod wonkyslidy;
mod threshold;
mod algo3;
mod hog;

fn main()
{
    framework::run::<wonkyslidy::WonkySlidy>();
}