use ndarray::*;

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