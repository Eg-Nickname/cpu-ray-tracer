use std::mem::swap;

use glam::Vec3;

use crate::ray::Ray;
use super::Object;

// Axis aligned bounding box
pub struct AABB{
    material_id: usize,
    // Front bottom left corner
    min: Vec3,
    // Back top right corner
    max: Vec3
}

impl AABB {
    pub fn new(min: Vec3, max: Vec3, material_id: usize) -> Self{
        Self { 
            material_id: material_id,
            min: min, 
            max: max 
        }
    }
    
    pub fn cube(bottom_left: Vec3, side_length: f32, material_id: usize) -> Self{
        let top_right = Vec3::new(bottom_left.x+side_length, bottom_left.y-side_length, bottom_left.z + side_length);
        Self::new(bottom_left, top_right, material_id)
    }
}

impl Object for AABB{
    fn intersect(&self, ray: &Ray) -> f32 {
        let mut tx_min = (self.min.x - ray.orgin.x) / ray.direction.x;
        let mut tx_max = (self.max.x - ray.orgin.x) / ray.direction.x;
        if tx_min > tx_max { swap(&mut tx_min, &mut tx_max) };

        let mut ty_min = (self.min.y - ray.orgin.y) / ray.direction.y;
        let mut ty_max = (self.max.y - ray.orgin.y) / ray.direction.y;
        if ty_min > ty_max { swap(&mut ty_min, &mut ty_max) };

        if (tx_min > ty_max) || (ty_min > tx_max){
            return  -1.0;
        }

        tx_min = tx_min.max(ty_min);
        tx_max = tx_max.min(ty_max);

        let mut tz_min = (self.min.z - ray.orgin.z) / ray.direction.z;
        let mut tz_max = (self.max.z - ray.orgin.z) / ray.direction.z;
        if tz_min > tz_max { swap(&mut tz_min, &mut tz_max) };

        if (tx_min > tz_max) || (tz_min > tx_max){
            return -1.0
        }

        let t1 = tx_min.max(tz_min);
        let t2 = tx_max.min(tz_max);

        // t1 * t2 < 0 if both are opposite sign so we can return the higher one becouse we know that value is positive
        // t1 * t2 > 0 if they are same sign so we return the smaller one positive or negative becouse if function returns negative value renderer assumes the ray missed 
        if t1 * t2 < 0.0{
            return t1.max(t2);
        }else{
            return t1.min(t2);
        }
    }
    fn get_material_id(&self) -> usize {
        self.material_id
    }
    fn get_uv(&self, point: Vec3) -> Vec3 {
        if point.x == self.min.x{
            Vec3::new(-1.0, 0.0, 0.0)
        }else if point.x == self.max.x{
            Vec3::new(1.0, 0.0, 0.0)
        }else if point.y == self.min.y{
            Vec3::new(0.0, -1.0, 0.0)
        }else if point.y == self.max.y{
            Vec3::new(0.0, 1.0, 0.0)
        }else if point.z == self.min.z{
            Vec3::new(0.0, 0.0, -1.0)
        }else{ // if point.z == self.max.z
            Vec3::new(0.0, 0.0, 1.0)
        }
    }
}