use glam::f32::Vec3;

#[derive(Clone)]
pub struct Material{
    pub albedo: Vec3,
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
}

impl Default for Material{
    fn default() -> Self {
        Material::new(Vec3::new(0.8, 0.8, 0.8), 1.0, 0.0, 1.0, Vec3::default())
    }
}