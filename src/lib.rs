pub mod rendering;
pub mod scene;
pub mod vector;
use image::{DynamicImage, GenericImageView};
use rendering::{Intersectable, Ray};
use scene::{Color, Scene, Sphere};
use vector::Vector3;

pub fn render(scene: &Scene) -> DynamicImage {
    let mut image = DynamicImage::new_rgb8(scene.width, scene.height);
    let black = image::Rgba::from_channels(0, 0, 0, 0);
    for x in 0..scene.width {
        for y in 0..scene.height {
            let ray = Ray::create_prime(x, y, scene);

            if scene.sphere.intersect(&ray) {
                image.put_pixel(x, y, to_rgba(&scene.sphere.color))
            } else {
                image.put_pixel(x, y, black);
            }
        }
    }
    image
}

#[test]
fn test_can_render_scene() {
    let scene = Scene {
        width: 800,
        height: 600,
        fov: 90.0,
        sphere: Sphere {
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
        },
    };

    let img: DynamicImage = render(&scene);
    assert_eq!(scene.width, img.width());
    assert_eq!(scene.height, img.height());
}
