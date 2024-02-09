use glam::f32::Vec3;

#[derive(Clone)]
pub struct Material{
    pub albedo: Vec3,
    pub roughness: f32,
    pub metalic: f32,
    pub emmision_power: Option<f32>,
}

impl Material{
    pub fn new(albedo: Vec3, roughness: f32, metalic: f32, emmision_power: Option<f32>) -> Self{
        Material { albedo: albedo, roughness: roughness, metalic: metalic, emmision_power: emmision_power }
    }
}

impl Default for Material{
    fn default() -> Self {
        Material::new(Vec3::new(1.0, 1.0, 1.0), 1.0, 0.0, None)
    }
}