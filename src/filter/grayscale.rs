extern crate ndarray;
use ndarray::*;

pub fn default() -> GrayscaleFilter {
    GrayscaleFilter{}
}

pub struct GrayscaleFilter {}

impl GrayscaleFilter {
    pub fn run (&self, img: Array3<f32>) -> Array2<f32> {
        let target = if img.shape()[2] == 4 {
            img.select(Axis(2), &[0,1,2])
        } else {
            img
        };
        target.mean_axis(Axis(2))
    }
}