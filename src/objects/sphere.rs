use glam::f32::Vec3;
use crate::ray::Ray;

use super::Object;
use super::material::Material;
pub struct Sphere{
    pub position: Vec3,
    pub radius: f32,
    pub material: Material,
}

impl Sphere {
    pub fn new(position: Vec3, radius: f32, material: Material) -> Self{
        Self { position: position, radius: radius, material: material }
    }
}

impl Default for Sphere{
    fn default() -> Self {
        Sphere::new(Vec3::new(0.0, 0.0, 0.0), 1.0, Material::default())
    }
}

impl Object for Sphere{
    // calculate ray sphere intersection using quadratioc formula
    fn intersect(&self, ray: &Ray) -> Option<(f32, f32)>{
        let ray_origin = ray.get_origin() - self.position.clone(); 
        let ray_direction = ray.get_direction().normalize();
        

        let a: f32 = ray_direction.dot(ray_direction);
        let b: f32 = 2.0 * ray_origin.dot(ray_direction);
        let c: f32 = ray_origin.dot(ray_origin) - self.radius * self.radius;

        // Also called discriminant
        let delta = (b*b) - (4.0*a*c);

        if delta < 0.0{
            return None;
        }
        let t1 = (-b - delta.sqrt())/(2.0*a);
        let t2 = (-b + delta.sqrt())/(2.0*a);
        Some((t1, t2))
    }

    fn get_material(&self) -> Material {
        self.material.clone()
    }

    fn get_uv(&self, point: Vec3) -> Vec3 {
        point.normalize()
    }
}


mod tests{
    #[allow(unused_imports)]
    use super::*;
    #[test]
    fn ray_2point_sphere_intersection(){
        let sphere = Sphere::default();
        let ray = Ray::new(Vec3::new(0.0, 0.0, -5.0), Vec3::new(0.0, 0.0, 1.0), Vec3::default());
        let intersection = sphere.intersect(&ray);

        assert_eq!(intersection, Some((4.0, 6.0)));
    }

    #[test]
    fn ray_in_front_of_sphere(){
        let sphere = Sphere::default();
        let ray = Ray::new(Vec3::new(0.0, 0.0, 5.0), Vec3::new(0.0, 0.0, 1.0), Vec3::default());
        let intersection = sphere.intersect(&ray);

        assert_eq!(intersection, Some((-6.0, -4.0)));
    }

    #[test]
    fn ray_miss_sphere(){
        let sphere = Sphere::default();
        let ray = Ray::new(Vec3::new(0.0, 4.0, 0.0), Vec3::new(0.0, 0.0, 1.0), Vec3::default());
        let intersection = sphere.intersect(&ray);

        assert_eq!(intersection, None);
    }
}