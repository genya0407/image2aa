extern crate ndarray;
use ndarray::*;
use std::f32;

pub struct Character {
    pub rho: f32,
    pub theta: f32,
    pub string: String
}

fn new_char(theta: f32, rho: f32, string: &'static str) -> Character {
    Character { rho: rho, theta: theta, string: string.to_string() }
}

impl Character {
    pub fn score(&self, theta: f32, rho: f32) -> f32 {
        (self.theta - theta).abs() * 50. + (self.rho - rho).abs()
    }
}

pub struct AsciiArtFilter {
    pub character_list: Vec<Character>
}

pub fn default() -> AsciiArtFilter {
    let character_list = vec![
        new_char(0., 0.25, "| "),
        new_char(0., 0.5,  "｜"),
        new_char(0., 0.75, " |"),
        new_char(f32::asin(1./f32::sqrt(5.)), 1./f32::sqrt(5.), "/ "),
        new_char(f32::asin(1./f32::sqrt(5.)), 2./f32::sqrt(5.), " /"),
        new_char(f32::asin(1./f32::sqrt(5.)), 1.5/f32::sqrt(5.), "／"),
        new_char(1./4.*f32::consts::PI, 1./f32::sqrt(32.), "\" "),
        new_char(1./4.*f32::consts::PI, 1./f32::sqrt(2.),  "／"),
        new_char(1./4.*f32::consts::PI, 7./f32::sqrt(32.), " ."),
        new_char(1./2.*f32::consts::PI, 0.25, "￣"),
        new_char(1./2.*f32::consts::PI, 0.5,  "ー"),
        new_char(1./2.*f32::consts::PI, 0.75, "＿"),
        new_char(3./4.*f32::consts::PI, 3./f32::sqrt(32.), ". "),
        new_char(3./4.*f32::consts::PI, 1./f32::sqrt(2.),  "＼"),
        //new_char(3./4.*f32::consts::PI, 7./f32::sqrt(32.),  " \""),
        new_char(
            f32::consts::PI - f32::asin(1./f32::sqrt(5.)),
            0.,
            "\\ "),
        new_char(
            f32::consts::PI - f32::asin(1./f32::sqrt(5.)),
            0.5/f32::sqrt(5.),
            "＼"),
        new_char(
            f32::consts::PI - f32::asin(1./f32::sqrt(5.)),
            1./f32::sqrt(5.),
            " \\"),
    ];

    let aaf: AsciiArtFilter = AsciiArtFilter {
        character_list: character_list
    };

    return aaf;
}

impl AsciiArtFilter {
    pub fn run(&self, img: Array3<f32>) -> String {
        let mut result = String::from("");
        for yi in 0..img.shape()[0] {
            for xi in 0..img.shape()[1] {
                let slope = img[[yi, xi, 0]];
                let rho = img[[yi, xi, 1]];
                let character = self.nearest_character(slope, rho);
                result += &character;
            }
            result += "\n";
        }
        return result;
    }

    fn nearest_character(&self, theta: f32, rho: f32) -> String {
        if theta.is_nan() || rho.is_nan() {
            return String::from("　");
        }
        let minimum_character_by_theta_opt = self.character_list.iter()
                                            .min_by(|left, right|
                                                left.score(theta, rho)
                                                    .partial_cmp(&right.score(theta, rho))
                                                    .unwrap());
        let minimum_character_by_theta = minimum_character_by_theta_opt.unwrap();
        return minimum_character_by_theta.string.clone();
    }
}