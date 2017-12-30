extern crate ndarray;
use ndarray::*;

const LINE_COLOR_VALUE: f32 = 200.;

pub fn default() -> BinaryFilter {
    BinaryFilter{
        thresh: 200.
    }
}

pub struct BinaryFilter {
    pub thresh: f32
}

impl BinaryFilter {
    pub fn run (&self, img: Array2<f32>) -> Array2<f32> {
        img.mapv(|e| if e > self.thresh { LINE_COLOR_VALUE } else { 0. } )
    }
}