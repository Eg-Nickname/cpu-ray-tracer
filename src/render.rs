use std::sync::Arc;
use std::sync::RwLock;
use glam::f32::{Vec2,Vec3};
use rayon::prelude::*;

use crate::{objects::Object, ray::Ray};
use crate::canvas::Canvas;

const BOUNCES: usize = 5;
pub struct Renderer{
    image_width: usize,
    image_height: usize,
    objects: Vec<Arc<dyn Object + Sync + Send>>
}

struct HitObject{
    object: Arc<dyn Object + Sync + Send>,
    t: (f32, f32),
    min_non_negative_hit_poitn: f32
}

impl HitObject{
    pub fn new(object: Arc<dyn Object + Sync + Send>, t: (f32, f32), min_non_negative_hit_poitn: f32) -> Self{
        HitObject {object: object, t: t, min_non_negative_hit_poitn: min_non_negative_hit_poitn}
    }
}

impl Renderer{
    pub fn new(image_width: usize, image_height: usize, objects: Vec<Arc<dyn Object + Sync + Send>>) -> Self{
        Renderer{
            image_width: image_width,
            image_height: image_height,
            objects: objects
        }
    }
    pub fn render(&self){
        let mut canvas = Canvas::new(self.image_width, self.image_height);
        canvas.data.par_iter_mut().enumerate().for_each(|(x, collumn)|{
            collumn.iter_mut().enumerate().for_each(|(y, value)|{
                let mut  coord = Vec2::new((x as f32)/self.image_width as f32, (y as f32)/self.image_height as f32);
                coord = (coord * 2.0) - 1.0;
                let color = self.per_pixel(coord);

                *value = color;
            });
        });
        canvas.to_png()
    }

    fn per_pixel(&self, coord: Vec2) -> Vec3{
        let mut  color = Vec3::default();
        for _ in 0..20{
            let mut ray = Ray::new(Vec3::new(0.0, 0.0, -3.0), Vec3::new(coord.x, coord.y, 1.0), Vec3::new(0.0, 0.0, 0.0));
            self.trace_ray(&mut ray, BOUNCES);
            color += ray.energy;
        }
        color / 20.0
    }

    // Mutates ray energy
    fn trace_ray(&self, ray: &mut Ray, depth: usize){
        // let hitobjects: Vec<HitObject> = Vec::new();
        let mut closest_hit: Option<HitObject> = None;

        for object_ref in &self.objects{
            let object = (*object_ref).clone();
            if let Some(hit) = object.intersect(ray){
                if hit.0 > 0.0 || hit.1 > 0.0{
                    let min_non_negative_hit_point = if hit.0 < 0.0{
                        hit.1 
                    }else if hit.1 < 0.0{
                        hit.0
                    }else{
                        hit.0.min(hit.1)
                    };

                    match &closest_hit{
                        None => closest_hit = Some(HitObject::new(object.clone(), hit, min_non_negative_hit_point)),
                        Some(close_hit) => {
                            if close_hit.t.0 > hit.0 {
                                closest_hit = Some(HitObject::new(object.clone(), hit, min_non_negative_hit_point));
                            }
                        }
                    }
                }
            }
        }

        match closest_hit {
            None => {
                // ray.energy = Vec3::new(0.1, 0.2, 0.7);
                },
            Some(hit_object) => {
                // Spawn new ray
                let hit_point = ray.orgin + ray.direction * hit_object.min_non_negative_hit_poitn;
                let uv = hit_object.object.get_uv(hit_point);
                if depth != 0 {
                    ray.orgin = hit_point;
                    ray.direction = (Ray::random_in_unit_sphere() + uv).normalize();
                    self.trace_ray(ray, depth-1);
                }
                
                // Do things with ray from spawned rays
                let hit_object_material = hit_object.object.get_material();
                ray.energy *= hit_object_material.albedo;
                ray.energy += hit_object_material.get_emmision();
            }
        }
    }
}