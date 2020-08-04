#![allow(dead_code)]

extern crate rgb;
extern crate hex;

use std::{i32, f32};
use rgb::RGBA8;

pub type RGBA = rgb::RGBA<f32>;

const SPECTRAL_COLORS_STRS: [&'static str; 9] = [
    "fc8d59ffffbf99d594",
    "d7191cfdae61abdda42b83ba",
    "d7191cfdae61ffffbfabdda42b83ba",
    "d53e4ffc8d59fee08be6f59899d5943288bd",
    "d53e4ffc8d59fee08bffffbfe6f59899d5943288bd",
    "d53e4ff46d43fdae61fee08be6f598abdda466c2a53288bd",
    "d53e4ff46d43fdae61fee08bffffbfe6f598abdda466c2a53288bd",
    "9e0142d53e4ff46d43fdae61fee08be6f598abdda466c2a53288bd5e4fa2",
    "9e0142d53e4ff46d43fdae61fee08bffffbfe6f598abdda466c2a53288bd5e4fa2",
];

pub trait Colors {
    fn scale_sequential(&self, x: f32, clamp: bool, x0: f32, x1: f32, interpolator: &dyn Fn(&Self, f32) -> RGBA) -> RGBA8 {
        let t = (x - x0) / (x1 - x0);
        let color = interpolator(self, if clamp {
            if 0. > t { 0. } else { if 1. > t { 1. } else { t } }
        } else {
            t
        });

        let r = (color.r * 255.) as u8;
        let g = (color.g * 255.) as u8;
        let b = (color.b * 255.) as u8;
        let a = (color.a * 255.) as u8;

        RGBA8{r: r, g: g, b: b, a: a}
    }
}

pub struct SpectralColors {
    spectral_colors: Vec<RGBA>,
}

impl Colors for SpectralColors {}

impl SpectralColors {
    pub fn new() -> Self{
        let colors = SpectralColors::get_colors(SPECTRAL_COLORS_STRS[SPECTRAL_COLORS_STRS.len() - 1]);
        SpectralColors {
            spectral_colors: colors,
        }       
    }

    fn get_colors(specifier: &str) -> Vec<RGBA> {
        let num_colors = (specifier.len() / 6) | 0;
        let mut colors: Vec<RGBA> = vec![RGBA::new(0., 0., 0., 0.); num_colors];
        let mut i = 0;
    
        while i < num_colors {
            let color_string = &specifier[i*6..(i*6+6)];
            let r_str = &color_string[0..2];
            let g_str = &color_string[2..4];
            let b_str = &color_string[4..6];
    
            let r = i32::from_str_radix(r_str, 16).unwrap();
            let g = i32::from_str_radix(g_str, 16).unwrap();
            let b = i32::from_str_radix(b_str, 16).unwrap();
    
            let color = RGBA::new((r as f32) / 255., (g as f32) / 255., (b as f32) / 255., 1.);
            colors[i] = color;
            i += 1;
        }
    
        colors
    }
    
    fn basis(&self, t1: f32, v0: f32, v1: f32, v2: f32, v3: f32) -> f32 {
        let t2 = t1 * t1;
        let t3 = t2 * t1;
    
        ((1.0 - 3.0 * t1 + 3.0 * t2 - t3) * v0
        + (4.0 - 6.0 * t2 + 3.0 * t3) * v1
        + (1.0 + 3.0 * t1 + 3.0 * t2 - 3.0 * t3) * v2
        + t3 * v3) / 6.0
    }
    
    fn spline(&self, t: f32, values: &[f32]) -> f32 {
        let n = values.len();
        let i: usize;
        let t_prime = t;
        if t <= 0. {
            let t_prime = 0.;
            i = t_prime as usize;
        }
        else {
            i = (t_prime * n as f32).floor() as usize; 
        }
    
        let v1 = values[i];
        let v2 = values[i+1];
        let v0 = if i > 0 {
            values[i - 1]
        } else {
            2. * v1 - v2
        };
        let v3 = if i < n {
            values[1 + 2]
        } else {
            2. * v2 - v1
        };
    
        self.basis(t_prime * (n - i) as f32, v0, v1, v2, v3)
    }
    
    fn rgb_spline(&self, t: f32, colors: &[RGBA], spline_func: &dyn Fn(&Self, f32, &[f32]) -> f32) -> RGBA {
        let n = colors.len();
        let mut r_values: Vec<f32> = vec![0.; n];
        let mut b_values: Vec<f32> = vec![0.; n];
        let mut g_values: Vec<f32> = vec![0.; n];
    
        for i in 0..n {
            r_values[i] = colors[i].r;
            g_values[i] = colors[i].g;
            b_values[i] = colors[i].b;
        }
    
        let r = spline_func(self, t, &r_values);
        let g = spline_func(self, t, &g_values);
        let b = spline_func(self, t, &b_values);
    
        RGBA::new(r, g, b, 1.)
    }
    
    pub fn interpolate_spectral(&self, t: f32) -> RGBA {
        self.rgb_spline(t, &self.spectral_colors, &SpectralColors::spline)
    }
    
    pub fn scale_spectral(&self, x: f32) -> RGBA8 {
        self.scale_sequential(x, false, 0., 1., &SpectralColors::interpolate_spectral)
    }
    
    pub fn scale_spectral_clamp(&self, x: f32) -> RGBA8 {
        self.scale_sequential(x, true, 0., 1., &SpectralColors::interpolate_spectral)
    }
}
