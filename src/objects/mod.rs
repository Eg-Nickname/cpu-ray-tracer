use crate::ray::Ray;

pub mod material;
pub mod sphere;

pub trait Object {
    fn intersect(&self, ray: &Ray) -> Option<(f32, f32)>;
    fn get_material(&self) -> material::Material;
}
