extern crate ndarray;
use ndarray::*;
use std::f32;

pub fn default() -> BlockHoughFilter {
    BlockHoughFilter{
        block_size: 32,
        theta_resolution: 20,
        slope_count_thresh: 0
    }
}

pub struct BlockHoughFilter {
    pub block_size: usize,
    pub theta_resolution: usize,
    pub slope_count_thresh: u32
}

pub struct HoughContainer {
    pub data: Array2<u32>,
    pub rho_offset: usize,
}

impl HoughContainer {
    pub fn new(max_rho_i: usize, max_slope_i: usize) -> Self {
        let data = Array2::<u32>::zeros((max_rho_i * 2, max_slope_i));
        Self { data: data, rho_offset: max_rho_i }
    }

    pub fn count_up(&mut self, rho_i: i32, slope_i: usize) {
        self.data[[((self.rho_offset as i32) + rho_i) as usize, slope_i]] += 1;
    }

    pub fn max_rho_i(&self) -> i32 {
        ((self.data.indexed_iter().max_by_key(|e| e.1).unwrap().0).0 as i32) - (self.rho_offset as i32)
    }

    pub fn max_slope_i(&self) -> i32 {
        (self.data.indexed_iter().max_by_key(|e| e.1).unwrap().0).1 as i32
    }

    pub fn max_count(&self) -> u32 {
        self.data.iter().max().unwrap().clone()
    }
}

impl BlockHoughFilter {
    pub fn run (&self, img: Array2<f32>) -> Array3<f32> {
        let ys = img.shape()[0];
        let xs = img.shape()[1];
        let y_block_count: usize = ys / self.block_size;
        let x_block_count: usize = xs / self.block_size;
        let mut maximum_rho_slopes = Array3::<f32>::zeros((y_block_count, x_block_count, 2));
        for y_block_i in 0..y_block_count {
            for x_block_i in 0..x_block_count {
                let yr = (y_block_i * self.block_size) as isize..((y_block_i+1) * self.block_size) as isize;
                let xr = (x_block_i * self.block_size) as isize..((x_block_i+1) * self.block_size) as isize;
                let block = img.slice(s![yr, xr]);

                let container = self.hough_transform(block);
                if container.max_count() > self.slope_count_thresh {
                    let max_rho_i = container.max_rho_i();
                    let max_slope_i = container.max_slope_i();
                    maximum_rho_slopes[[y_block_i, x_block_i, 0]] = self.index2slope(max_slope_i as usize);
                    maximum_rho_slopes[[y_block_i, x_block_i, 1]] = (max_rho_i as f32)/(self.block_size as f32);
                } else {
                    maximum_rho_slopes[[y_block_i, x_block_i, 0]] = f32::NAN;
                    maximum_rho_slopes[[y_block_i, x_block_i, 1]] = f32::NAN;
                }
            }
        }
        return maximum_rho_slopes;
    }

    fn hough_transform(&self, block: ArrayView2<f32>) -> HoughContainer {
        let ys = block.shape()[0];
        let xs = block.shape()[1];
        let ys_f = ys as f32;
        let xs_f = xs as f32;
        let max_rho_i = (f32::sqrt(xs_f.powi(2)+ys_f.powi(2)+xs_f*ys_f) + 1.) as usize;
        let max_slope_i = self.theta_resolution as usize;
        // y方向: rho, x方向: theta_i
        let mut container = HoughContainer::new(max_rho_i, max_slope_i);
        for y in 0..ys {
            for x in 0..xs {
                if block[[y, x]] > 0. {
                    for slope_i in 0..max_slope_i {
                        let slope = self.index2slope(slope_i);
                        let rho_i = self.calc_rho(x as f32, y as f32, slope) as i32;
                        container.count_up(rho_i, slope_i);
                    }
                }
            }
        }
        return container;
    }

    fn index2slope(&self, slope_i: usize) -> f32 {
        return (slope_i as f32 / self.theta_resolution as f32) * f32::consts::PI;
    }

    fn calc_rho(&self, x: f32, y: f32, slope: f32) -> f32 {
        let rho = x * f32::cos(slope) + y * f32::sin(slope);
        return rho
    }
}
