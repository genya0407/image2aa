use crate::utils;
use ndarray::*;

pub fn default() -> GradientFilter {
    let horizontal_edge_filter = Array1::<f32>::from(vec![-1., 0., 1., -2., 0., 2., -1., 0., 1.])
        .into_shape((3, 3))
        .unwrap();
    let vertical_edge_filter = Array1::<f32>::from(vec![-1., -2., -1., 0., 0., 0., 1., 2., 1.])
        .into_shape((3, 3))
        .unwrap();

    GradientFilter {
        horizontal_edge_filter: horizontal_edge_filter,
        vertical_edge_filter: vertical_edge_filter,
    }
}

pub struct GradientFilter {
    pub horizontal_edge_filter: Array2<f32>,
    pub vertical_edge_filter: Array2<f32>,
}

impl GradientFilter {
    pub fn run(&self, img: Array2<f32>) -> Array2<f32> {
        return self.gradient_abs(img);
    }

    fn gradient_abs(&self, img: Array2<f32>) -> Array2<f32> {
        let vertical_gradient = utils::convolve2d(&img, &(self.vertical_edge_filter));
        let horizontal_gradient = utils::convolve2d(&img, &(self.horizontal_edge_filter));
        let mut result = Array2::<f32>::zeros((
            horizontal_gradient.shape()[0],
            horizontal_gradient.shape()[1],
        ));
        Zip::from(&mut result)
            .and(&horizontal_gradient)
            .and(&vertical_gradient)
            .apply(|r, h, v| *r = f32::sqrt(h * h + v * v));
        return result;
    }
}
