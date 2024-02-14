use glam::f32::Vec3;
use crate::ray::Ray;

use super::Object;
pub struct Sphere{
    pub position: Vec3,
    pub radius: f32,
    pub material_id: usize,
}

impl Sphere {
    pub fn new(position: Vec3, radius: f32, material_id: usize) -> Self{
        Self { position: position, radius: radius, material_id: material_id }
    }
}

impl Default for Sphere{
    fn default() -> Self {
        Sphere::new(Vec3::new(0.0, 0.0, 0.0), 1.0, 0)
    }
}

impl Object for Sphere{
    // calculate ray sphere intersection using quadratioc formula
    fn intersect(&self, ray: &Ray) -> f32{
        let ray_origin = ray.get_origin() - self.position.clone(); 
        let ray_direction = ray.get_direction().normalize();

        let a: f32 = ray_direction.dot(ray_direction);
        let b: f32 = 2.0 * ray_origin.dot(ray_direction);
        let c: f32 = ray_origin.dot(ray_origin) - self.radius * self.radius;

        // Also called discriminant
        let delta = (b*b) - (4.0*a*c);

        if delta < 0.0{
            return -1.0;
        }
        let t1 = (-b - delta.sqrt())/(2.0*a);
        t1
    }

    fn get_material_id(&self) -> usize {
        self.material_id
    }

    fn get_uv(&self, point: Vec3) -> Vec3 {
        (point - self.position).normalize()
    }
}