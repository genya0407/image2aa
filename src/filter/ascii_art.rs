extern crate ndarray;
use ndarray::*;
use std::f32;

pub fn default() -> AsciiArtFilter {
    let standard_slopes: Vec<f32> = vec![
        0.,
        f32::asin(1./f32::sqrt(5.)),
        1./4.*f32::consts::PI,
        1./2.*f32::consts::PI,
        3./4.*f32::consts::PI,
    ];
    let vh_standard_rho: Vec<f32> = vec![0.25, 0.5, 0.75];
    let diag_standard_rho: Vec<f32> = vec![
        1./2.*f32::cos(f32::asin(1./f32::sqrt(5.))),
        f32::cos(f32::asin(1./f32::sqrt(5.)))
    ];

    let aaf: AsciiArtFilter = AsciiArtFilter {
        standard_slopes: standard_slopes,
        vh_standard_rho: vh_standard_rho,
        diag_standard_rho: diag_standard_rho,
    };

    return aaf;
}

pub struct AsciiArtFilter {
    pub standard_slopes:   Vec<f32>,
    pub vh_standard_rho:   Vec<f32>,
    pub diag_standard_rho: Vec<f32>,
}

pub enum SlopeType {
    Vertical { pos: u8 },
    Horizontal { pos: u8 },
    SlashZenkaku,
    SlashHankaku { pos: u8 },
    BackSlashZenkaku,
    BackSlashHankaku { pos: u8 },
    Empty
}

impl AsciiArtFilter {
    pub fn run(&self, img: Array3<f32>) -> String {
        let mut result = String::from("");
        for yi in 0..img.shape()[0] {
            for xi in 0..img.shape()[1] {
                let slope = img[[yi, xi, 0]];
                let rho = img[[yi, xi, 1]];
                let t = self.categorize(slope, rho);
                result += &self.render(t);
            }
            result += "\n";
        }
        return result;
    }

    fn categorize(&self, slope: f32, rho: f32) -> SlopeType {
        if f32::is_nan(slope) {
            return SlopeType::Empty {};
        } else {
            let slope_index: u8 = self.standardize(&self.standard_slopes, slope);
            match slope_index {
                0 => SlopeType::Vertical { pos: self.standardize(&self.vh_standard_rho, rho) },
                1 => SlopeType::SlashHankaku { pos: self.standardize(&self.diag_standard_rho, rho) },
                2 => SlopeType::SlashZenkaku {},
                3 => SlopeType::Horizontal { pos: self.standardize(&self.vh_standard_rho, rho) },
                4 => SlopeType::BackSlashZenkaku {},
                5 => SlopeType::BackSlashHankaku { pos: self.standardize(&self.diag_standard_rho, rho) },
                _ => panic!("unsupported slope index"),
            }
        }
    }

    // valueとしてNaNは入らないものとする
    fn standardize(&self, standards: &Vec<f32>, value: f32) -> u8 {
        standards.iter().map(|e| (*e-value).abs())
                        .enumerate()
                        .min_by(|l, r| l.1.partial_cmp(&r.1).unwrap())
                        .unwrap().0 as u8
    }

    fn render(&self, t: SlopeType) -> String {
        match t {
            SlopeType::Vertical { pos } => String::from(["| ", "｜", " |"][pos as usize]),
            SlopeType::Horizontal { pos } => String::from(["\"\"", "ー", "＿"][pos as usize]),
            SlopeType::SlashZenkaku => String::from("／"),
            SlopeType::SlashHankaku { pos } => String::from(["/ ", " /"][pos as usize]),
            SlopeType::BackSlashZenkaku => String::from("＼"),
            SlopeType::BackSlashHankaku { pos } => String::from(["\\ ", " \\"][pos as usize]),
            SlopeType::Empty => String::from("　"),
        }
    }
}