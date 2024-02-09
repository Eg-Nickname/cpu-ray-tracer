use crate::ray::Ray;
use glam::f32::Vec3;

pub mod material;
pub mod sphere;

pub trait Object {
    fn intersect(&self, ray: &Ray) -> Option<(f32, f32)>;
    fn get_material(&self) -> material::Material;
    fn get_uv(&self, point: Vec3) -> Vec3;
}
