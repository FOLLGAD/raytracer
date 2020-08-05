use crate::scene::{Color, Light, Object, Scene, Sphere};
use crate::vector::Vector3;
use image::{Rgba};

impl Object {
    pub fn intersect(&self, ray: &Ray) -> Option<Intersection> {
        match self {
            Object::Sphere(ref s) => s.intersect(ray).and_then(|x| {
                Some(Intersection {
                    object: self,
                    dist: x,
                })
            }),
            // Object::Plane(ref s) => s.intersect(ray).and_then(|x| {
            //     Some(Intersection {
            //         object: self,
            //         dist: x,
            //     })
            // }),
        }
    }
    pub fn color(&self) -> &Color {
        match self {
            Object::Sphere(ref s) => &s.color,
            // Object::Plane(ref s) => &s.color,
        }
    }
    pub fn surface_normal(&self, hit_point: &Vector3) -> Vector3 {
        match self {
            Object::Sphere(ref s) => s.surface_normal(hit_point),
            // Object::Plane(ref s) => s.normal,
        }
    }
    pub fn albedo(&self) -> f32 {
        match self {
            Object::Sphere(ref s) => s.albedo,
            // Object::Plane(ref s) => s.albedo,
        }
    }
}

impl Scene {
    pub fn trace(&self, ray: &Ray) -> Option<Intersection> {
        self
            .objects
            .iter()
            .filter_map(|ob| ob.intersect(&ray))
            .min()
    }
    pub fn cast_ray(&self, x: u32, y: u32) -> Rgba<u8> {
        let ray = Ray::create_prime(x, y, self);

        let col = self.trace(&ray);

        if let Some(col) = col {
            let hit_point = ray.origin + (col.dist * ray.direction);
            let surface_normal = col.object.surface_normal(&hit_point);

            let total_color: Color =
                self.lights
                    .iter()
                    .fold(Color::black(), |p: Color, l: &Light| {
                        let direction = l.direction(hit_point);
                        let shadow_ray = Ray {
                            origin: hit_point + (surface_normal * 0.01),
                            direction,
                        };
                        let visible = self.trace(&shadow_ray).is_none();
                        if visible {
                            let light_power =
                                (surface_normal.dot(direction) as f32).max(0.0) * l.intensity();
                            let color: Color = *col.object.color() * l.color() * light_power;
                            color + p
                        } else {
                            p
                        }
                    });
            let light_reflected = col.object.albedo() / std::f32::consts::PI;

            (total_color * light_reflected).to_rgba()
        } else {
            self.skybox.to_rgba()
        }
    }
}

pub struct Intersection<'a> {
    pub object: &'a Object,
    pub dist: f64,
}

impl Ord for Intersection<'_> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.dist.partial_cmp(&other.dist).unwrap()
    }
}

impl PartialOrd for Intersection<'_> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Intersection<'_> {
    fn eq(&self, other: &Self) -> bool {
        self.dist == other.dist
    }
}
impl Eq for Intersection<'_> {}

pub trait Intersectable {
    fn intersect(&self, ray: &Ray) -> Option<f64>;
    fn surface_normal(&self, hit_point: &Vector3) -> Vector3;
}

impl Intersectable for Sphere {
    fn intersect(&self, ray: &Ray) -> Option<f64> {
        let l: Vector3 = self.center - ray.origin;
        let adj2 = l.dot(ray.direction);
        let d2 = l.dot(l) - (adj2 * adj2);
        if d2 > (self.radius * self.radius) {
            return None;
        }
        let diff = ray.origin - self.center;
        let rt = ray.direction.dot(diff);
        let right = (rt.powi(2) + self.radius.powi(2) - diff.norm()).sqrt();
        // let d1 = -rt + right;
        let d2 = -rt - right; // Closest intersection point

        if d2 >= 0.0 {
            Some(d2)
        } else {
            None
        }
    }
    fn surface_normal(&self, hit_point: &Vector3) -> Vector3 {
        (*hit_point - self.center).normalize()
    }
}

pub struct Ray {
    origin: Vector3,
    direction: Vector3,
}

impl Ray {
    pub fn create_prime(x: u32, y: u32, scene: &Scene) -> Ray {
        // assert!(scene.width > scene.height);

        let fov_adjustment = (scene.fov.to_radians() / 2.0).tan();
        let aspect_ratio = (scene.width as f64) / (scene.height as f64);
        let sensor_x =
            ((((x as f64 + 0.5) / scene.width as f64) * 2.0 - 1.0) * aspect_ratio) * fov_adjustment;
        let sensor_y = (1.0 - ((y as f64 + 0.5) / scene.height as f64) * 2.0) * fov_adjustment;

        Ray {
            origin: Vector3::zero(),
            direction: Vector3 {
                x: sensor_x,
                y: sensor_y,
                z: -1.0,
            }
            .normalize(),
        }
    }
}
