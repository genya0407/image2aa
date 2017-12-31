extern crate ndarray;
use ndarray::*;
use ::utils;

pub fn default() -> ShrinkFilter {
    let filter = Array1::<f32>::from(
        vec![
            1.,1.,1.,
            1.,1.,1.,
            1.,1.,1.
        ]
    ).into_shape((3,3)).unwrap();

    ShrinkFilter{
        thresh: 5.,
        filter: filter,
    }
}

pub struct ShrinkFilter {
    pub thresh: f32,
    pub filter: Array2<f32>
}

impl ShrinkFilter {
    pub fn run (&self, img: Array2<i8>) -> Array2<i8> {
        utils::convolve2d(&img.mapv(|e| e as f32), &self.filter).mapv(|e| if e > self.thresh { 1 } else { 0 })
    }
}