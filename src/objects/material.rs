use glam::f32::Vec3;

use crate::ray::Ray;

#[derive(Clone)]
pub struct Material{
    pub albedo: Vec3,
    // fuzzines
    pub roughness: f32,
    pub metalic: f32,
    pub emmision_power: f32,
    pub emmision_color: Vec3,
}

impl Material{
    pub fn new(albedo: Vec3, roughness: f32, metalic: f32, emmision_power: f32, emmision_color: Vec3) -> Self{
        Material { albedo: albedo,
            roughness: roughness,
            metalic: metalic,
            emmision_power: emmision_power,
            emmision_color: emmision_color }
    }
    
    pub fn get_emmision(&self) -> Vec3{
        self.emmision_color * self.emmision_power
    }

    pub fn get_scattered_ray_dir(&self, ray_dir: Vec3,  uv: Vec3) -> Vec3{
        if self.metalic == 1.0{
            (ray_dir - 2.0 * ray_dir.dot(uv) * uv) + self.roughness * Ray::random_unit_vector()
        }else {
            uv + Ray::random_unit_vector()
        }
    }
}

impl Default for Material{
    fn default() -> Self {
        Material::new(Vec3::ONE, 0.0, 0.0, 1.0, Vec3::default())
    }
}