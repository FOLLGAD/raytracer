pub mod rendering;
pub mod scene;
pub mod vector;
use image::{ImageBuffer, Rgba};
use scene::{Color, DirectionalLight, GlobalLight, Light, Object, Scene, Sphere, SphericalLight};
use vector::Vector3;

pub fn render(scene: &Scene) -> ImageBuffer<Rgba<u8>, Vec<u8>> {
    ImageBuffer::from_fn(scene.width, scene.height, |x, y| scene.cast_ray(x, y))
}

fn main() {
    let scene = Scene {
        width: 500,
        height: 500,
        fov: 90.0,
        skybox: Color {
            red: 0.2,
            green: 0.2,
            blue: 0.95,
        },
        lights: vec![
            Light::Spherical(SphericalLight {
                color: Color {
                    red: 1.0,
                    green: 0.1,
                    blue: 0.1,
                },
                pos: Vector3 {
                    x: 0.0,
                    y: -3.0,
                    z: -5.0,
                },
                // intensity: 2.0,
                intensity: 0.0,
            }),
            Light::Directional(DirectionalLight {
                color: Color {
                    red: 1.0,
                    green: 1.0,
                    blue: 1.0,
                },
                direction: Vector3 {
                    x: 0.3,
                    y: -3.0,
                    z: -1.0,
                }
                .normalize(),
                // intensity: 3.6,
                intensity: 0.0,
            }),
            Light::Global(GlobalLight {
                color: Color::white(),
                intensity: 5.000,
            }),
        ],
        objects: vec![
            Object::Sphere(Sphere {
                center: Vector3 {
                    x: -1.0,
                    y: 0.0,
                    z: -5.0,
                },
                radius: 1.0,
                color: Color {
                    red: 0.4,
                    green: 1.0,
                    blue: 0.4,
                },
                albedo: 1.0,
            }),
            Object::Sphere(Sphere {
                center: Vector3 {
                    x: -3.0,
                    y: -1.0,
                    z: -5.0,
                },
                radius: 1.0,
                color: Color {
                    red: 1.0,
                    green: 0.4,
                    blue: 0.5,
                },
                albedo: 0.5,
            }),
            Object::Sphere(Sphere {
                center: Vector3 {
                    x: 1.0,
                    y: -1.0,
                    z: -7.0,
                },
                radius: 2.0,
                color: Color {
                    red: 1.0,
                    green: 1.0,
                    blue: 1.0,
                },
                albedo: 1.0,
            }),
        ],
    };

    let image = render(&scene);
    image.save("./test.png").expect("bruh");
}

#[test]
fn test_can_render_scene() {
    let scene = Scene {
        width: 800,
        height: 600,
        fov: 90.0,
        skybox: Color {
            red: 0.0,
            green: 0.0,
            blue: 0.0,
        },
        objects: vec![Object::Sphere(Sphere {
            center: Vector3 {
                x: 0.0,
                y: 0.0,
                z: -5.0,
            },
            radius: 1.0,
            color: Color {
                red: 0.4,
                green: 1.0,
                blue: 0.4,
            },
            albedo: 1.0,
        })],
        lights: vec![Light::Global(GlobalLight {
            color: Color::black(),
            intensity: 0.0,
        })],
    };

    let img = render(&scene);
    assert_eq!(scene.width, img.width());
    assert_eq!(scene.height, img.height());
}
