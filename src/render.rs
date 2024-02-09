use std::sync::Arc;
use std::sync::RwLock;
use glam::f32::{Vec2,Vec3};
use rayon::prelude::*;

use crate::objects;
use crate::scene;
use crate::scene::Scene;
use crate::{objects::Object, ray::Ray};
use crate::canvas::Canvas;

const BOUNCES: usize = 5;
pub struct Renderer{
    image_width: usize,
    image_height: usize,
    scene: Arc<RwLock<Scene>>,
    // objects: Vec<Arc<dyn Object + Sync + Send>>
}

struct HitObject{
    object_id: usize,
    t: (f32, f32),
    min_non_negative_hit_poitn: f32
}

impl HitObject{
    pub fn new(object_id: usize, t: (f32, f32), min_non_negative_hit_poitn: f32) -> Self{
        HitObject {object_id: object_id, t: t, min_non_negative_hit_poitn: min_non_negative_hit_poitn}
    }
}

impl Renderer{
    pub fn new(image_width: usize, image_height: usize, scene: Scene) -> Self{
        Renderer{
            image_width: image_width,
            image_height: image_height,
            scene: Arc::new(RwLock::new(scene)),
        }
    }
    pub fn render(&self){
        let mut canvas = Canvas::new(self.image_width, self.image_height);
        let scene = Arc::clone(&self.scene);
        canvas.data.par_iter_mut().enumerate().for_each(|(x, collumn)|{
            collumn.iter_mut().enumerate().for_each(|(y, value)|{
                let mut  coord = Vec2::new((x as f32)/self.image_width as f32, (y as f32)/self.image_height as f32);
                coord = (coord * 2.0) - 1.0;
                let color = Renderer::per_pixel(Arc::clone(&scene), coord);

                *value = color;
            });
        });
        canvas.to_png()
    }

    fn per_pixel(scene: Arc<RwLock<Scene>>, coord: Vec2) -> Vec3{
        let mut  color = Vec3::default();
        for _ in 0..120{
            let mut ray = Ray::new(Vec3::new(0.0, 0.0, -3.0), Vec3::new(coord.x, coord.y, 1.0), Vec3::new(0.0, 0.0, 0.0));
            Renderer::trace_ray(Arc::clone(&scene), &mut ray, BOUNCES);
            color += ray.energy;
        }
        color / 120.0
    }

    // Mutates ray energy
    fn trace_ray(scene_ptr: Arc<RwLock<Scene>>, ray: &mut Ray, depth: usize){
        // Nothing should have lock on scene after render start
        let temp_scene_ptr = Arc::clone(&scene_ptr);
        let read_scene = (*temp_scene_ptr).read().unwrap();

        let mut closest_hit: Option<HitObject> = None;
        // Iter over scene objects
        for (object_id, object) in (*read_scene).objects.iter().enumerate(){
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
                        None => closest_hit = Some(HitObject::new(object_id, hit, min_non_negative_hit_point)),
                        Some(close_hit) => {
                            if close_hit.t.0 > hit.0 {
                                closest_hit = Some(HitObject::new(object_id, hit, min_non_negative_hit_point));
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
                let uv = (*read_scene).objects[hit_object.object_id].get_uv(hit_point);
                if depth != 0 {
                    ray.orgin = hit_point;
                    ray.direction = (Ray::random_in_unit_sphere() + uv).normalize();
                    Renderer::trace_ray(scene_ptr, ray, depth-1);
                }
                
                // Do things with ray from spawned rays
                // let hit_object_material_id = scene.objects[hit_object.object_id].get_material_id();
                ray.energy *= (*read_scene).materials[(*read_scene).objects[hit_object.object_id].get_material_id()].albedo;
                ray.energy += (*read_scene).materials[(*read_scene).objects[hit_object.object_id].get_material_id()].get_emmision();
            }
        }
    }
}