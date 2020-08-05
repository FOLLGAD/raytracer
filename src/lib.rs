pub mod rendering;
pub mod scene;
pub mod vector;
use image::{DynamicImage, GenericImageView, ImageBuffer, Pixel, Pixels, Rgba};
use rendering::{Intersectable, Ray};
use scene::{Color, Object, Scene, Sphere};
use vector::Vector3;

// pub fn render(scene: &Scene) -> ImageBuffer<Rgba<u8>, Vec<u8>> {
//     let black = Rgba::from_channels(0, 0, 0, 0);

//     let mut img = ImageBuffer::from_fn(scene.width, scene.height, |x, y| {
//         let ray = Ray::create_prime(x, y, scene);
//         if scene.sphere.intersect(&ray) {
//             Color::to_rgba(&scene.sphere.color)
//         } else {
//             black
//         }
//     });

//     img
// }
