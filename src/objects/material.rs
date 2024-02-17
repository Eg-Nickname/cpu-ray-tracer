use glam::f32::Vec3;

use crate::ray::Ray;

#[derive(Clone)]
pub struct Material{
    pub albedo: Vec3,
    // fuzzines
    pub roughness: f32,
    pub metalic: f32,
    pub refractive_index: f32,
    pub opacity: f32,
    pub emmision_power: f32,
    pub emmision_color: Vec3,
}

impl Material{
    pub fn new(albedo: Vec3, roughness: f32, metalic: f32, emmision_power: f32, emmision_color: Vec3) -> Self{
        Material { albedo: albedo,
            roughness: roughness,
            metalic: metalic,
            emmision_power: emmision_power,
            refractive_index: 1.0,
            opacity: 1.0,
            emmision_color: emmision_color }
    }
    
    pub fn get_emmision(&self) -> Vec3{
        self.emmision_color * self.emmision_power
    }

    pub fn get_scattered_ray_dir(&self, ray_dir: Vec3,  uv: Vec3) -> Vec3{
        if self.metalic == 1.0{
            ((ray_dir - 2.0 * ray_dir.dot(uv) * uv) + self.roughness * Ray::random_unit_vector()).normalize()
        }else {
            uv + Ray::random_unit_vector()
        }
    }

    pub fn get_refracted_ray(ray_dir: Vec3,  uv: Vec3, refraction_ratio: f32) -> Vec3{
        let cos_theta = 1.0f32.min((-ray_dir).dot(uv));
        let out_perpendicular = refraction_ratio * (uv + cos_theta * ray_dir);
        let out_parallel = -(1.0 - out_perpendicular.length_squared()).abs().sqrt() * ray_dir;

        return out_perpendicular + out_parallel;
    }
}

impl Default for Material{
    fn default() -> Self {
        Material::new(Vec3::ONE, 0.0, 0.0, 1.0, Vec3::default())
    }
}