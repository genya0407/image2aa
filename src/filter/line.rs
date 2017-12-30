use ndarray::*;

pub fn default() -> GradientFilter {
    let horizontal_edge_filter = Array1::<f32>::from(
        vec![
            -1., 0., 1.,
            -2., 0., 2.,
            -1., 0., 1.,
        ]
    ).into_shape((3,3)).unwrap();
    let vertical_edge_filter = Array1::<f32>::from(
        vec![
            -1.,-2.,-1.,
            0. , 0., 0.,
            1. , 2., 1.,
        ]
    ).into_shape((3,3)).unwrap();

    GradientFilter{
        horizontal_edge_filter: horizontal_edge_filter,
        vertical_edge_filter: vertical_edge_filter
    }
}

pub struct GradientFilter {
    pub horizontal_edge_filter: Array2<f32>,
    pub vertical_edge_filter: Array2<f32>
}

impl GradientFilter {
    pub fn run (&self, img: Array2<f32>) -> Array2<f32> {
        return self.gradient_abs(img)
    }

    fn gradient_abs(&self, img: Array2<f32>) -> Array2<f32> {
        let vertical_gradient = convolve2d(&img, &(self.vertical_edge_filter));
        let horizontal_gradient = convolve2d(&img, &(self.horizontal_edge_filter));
        let mut result = Array2::<f32>::zeros((horizontal_gradient.shape()[0], horizontal_gradient.shape()[1]));
        Zip::from(&mut result).and(&horizontal_gradient)
                              .and(&vertical_gradient)
                              .apply(|r, h, v| *r = f32::sqrt(h*h + v*v));
        return result;
    }
}

pub fn convolve2d(base_arr: &Array2<f32>, filter: &Array2<f32>) -> Array2<f32> {
    let ys: isize = base_arr.shape()[0] as isize;
    let xs: isize = base_arr.shape()[1] as isize;
    let mut result: Array2<f32> = Array2::<f32>::zeros((ys as usize, xs as usize));
    for yt in 1..((result.shape()[0]-1) as isize) {
        for xt in 1..((result.shape()[1]-1) as isize) {
            result[[yt as usize, xt as usize]] = (&base_arr.slice(s![yt-1..yt+2, xt-1..xt+2]) * filter).scalar_sum();
        }
    }
    return result;
}