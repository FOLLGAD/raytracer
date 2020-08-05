use crate::vector::Vector3;
use image::{Pixel, Rgba};
use std::ops::{Add, Mul};

const GAMMA: f32 = 1.5;

#[derive(Copy, Clone, Debug)]
pub struct Color {
    pub red: f32,
    pub green: f32,
    pub blue: f32,
}

impl Color {
    pub fn black() -> Color {
        Color {
            red: 0.0,
            green: 0.0,
            blue: 0.0,
        }
    }
    pub fn white() -> Color {
        Color {
            red: 1.0,
            green: 1.0,
            blue: 1.0,
        }
    }

    pub fn clamp(&self) -> Color {
        Color {
            red: self.red.min(1.0).max(0.0),
            blue: self.blue.min(1.0).max(0.0),
            green: self.green.min(1.0).max(0.0),
        }
    }

    pub fn to_rgba(&self) -> Rgba<u8> {
        let clamped = self.clamp();
        Rgba::from_channels(
            (gamma_encode(clamped.red) * 255.0) as u8,
            (gamma_encode(clamped.green) * 255.0) as u8,
            (gamma_encode(clamped.blue) * 255.0) as u8,
            255,
        )
    }

    pub fn from_rgba(rgba: Rgba<u8>) -> Color {
        Color {
            red: gamma_decode(rgba.0[0] as f32 / 255.0),
            green: gamma_decode(rgba.0[1] as f32 / 255.0),
            blue: gamma_decode(rgba.0[2] as f32 / 255.0),
        }
    }
}

impl Default for Color {
    fn default() -> Self {
        Color {
            red: 0.0,
            blue: 0.0,
            green: 0.0,
        }
    }
}

impl Mul for Color {
    type Output = Color;

    fn mul(self, other: Color) -> Color {
        Color {
            red: self.red * other.red,
            blue: self.blue * other.blue,
            green: self.green * other.green,
        }
    }
}
impl Mul<f32> for Color {
    type Output = Color;

    fn mul(self, other: f32) -> Color {
        Color {
            red: self.red * other,
            blue: self.blue * other,
            green: self.green * other,
        }
    }
}
impl Mul<Color> for f32 {
    type Output = Color;
    fn mul(self, other: Color) -> Color {
        other * self
    }
}
impl Add for Color {
    type Output = Color;
    fn add(self, other: Color) -> Color {
        Color {
            red: self.red + other.red,
            blue: self.blue + other.blue,
            green: self.green + other.green,
        }
    }
}

fn gamma_encode(linear: f32) -> f32 {
    linear.powf(GAMMA.recip())
}

fn gamma_decode(encoded: f32) -> f32 {
    encoded.powf(GAMMA)
}

pub enum Object {
    Sphere(Sphere),
    // Plane(Plane),
}

pub enum Light {
    Directional(DirectionalLight),
    Spherical(SphericalLight),
    Global(GlobalLight),
}

impl Light {
    pub fn direction(&self, from: Vector3) -> Vector3 {
        match self {
            Light::Directional(ref l) => -l.direction,
            Light::Spherical(ref l) => (l.pos - from).normalize(),
            Light::Global(ref _l) => -from.normalize(),
        }
    }
    pub fn intensity(&self) -> f32 {
        match self {
            Light::Directional(ref l) => l.intensity,
            Light::Spherical(ref l) => l.intensity,
            Light::Global(ref l) => l.intensity,
        }
    }
    pub fn color(&self) -> Color {
        match self {
            Light::Directional(ref l) => l.color,
            Light::Spherical(ref l) => l.color,
            Light::Global(ref l) => l.color,
        }
    }
}

pub struct SphericalLight {
    pub pos: Vector3,
    pub color: Color,
    pub intensity: f32,
}

pub struct DirectionalLight {
    pub direction: Vector3,
    pub color: Color,
    pub intensity: f32,
}

pub struct GlobalLight {
    pub color: Color,
    pub intensity: f32,
}

pub struct Sphere {
    pub center: Vector3,
    pub radius: f64,
    pub color: Color,
    pub albedo: f32,
}

pub struct Plane {
    pub center: Vector3,
    pub normal: Vector3,
    pub color: Color,
    pub albedo: f32,
}

pub struct Scene {
    pub width: u32,
    pub height: u32,
    pub fov: f64,
    pub objects: Vec<Object>,
    pub lights: Vec<Light>,
    pub skybox: Color,
}
