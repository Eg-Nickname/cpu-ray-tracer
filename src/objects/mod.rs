use crate::ray::Ray;
use glam::f32::Vec3;

pub mod material;
pub mod sphere;

pub trait Object {
    fn intersect(&self, ray: &Ray) -> f32;
    fn get_material_id(&self) -> usize;
    fn get_uv(&self, point: Vec3) -> Vec3;
}
