extern crate ndarray;
use ndarray::*;

pub fn default() -> BinaryFilter {
    BinaryFilter{
        thresh: 200.
    }
}

pub struct BinaryFilter {
    pub thresh: f32
}

impl BinaryFilter {
    pub fn run (&self, img: Array2<f32>) -> Array2<i8> {
        img.mapv(|e| if e > self.thresh { 1 } else { 0 } )
    }
}