use std::sync::Arc;
use std::sync::RwLock;
use glam::f32::Vec3;
use glam::f64::DVec3;
#[allow(unused_imports)]
use rayon::prelude::*;

use crate::camera::Camera;
use crate::scene::Scene;
use crate::ray::Ray;
use crate::canvas::Canvas;

pub struct Renderer{
    image_width: usize,
    image_height: usize,
    max_ray_bounces: usize,
    samples_per_pixel: usize,
    camera: Camera,
    scene: Arc<RwLock<Scene>>,
}

struct HitObject{
    object_id: usize,
    hit_distance: f32
}

impl HitObject{
    pub fn new(object_id: usize, hit_distance: f32) -> Self{
        HitObject {object_id: object_id, hit_distance: hit_distance}
    }
}

impl Renderer{
    pub fn new(camera: Camera, scene: Scene, samples_per_pixel: usize, max_ray_bounces: usize) -> Self{
        Renderer{
            image_width: camera.image_width,
            image_height: camera.image_height,
            samples_per_pixel: samples_per_pixel,
            max_ray_bounces: max_ray_bounces,
            camera: camera,
            scene: Arc::new(RwLock::new(scene)),
        }
    }

    pub fn render(&self){
        let mut canvas = Canvas::new(self.image_width, self.image_height);
        canvas.data.par_iter_mut().enumerate().for_each(|(x, collumn)|{
            collumn.iter_mut().enumerate().for_each(|(y, value)|{
                let ray_dir = (self.camera.top_left + (self.camera.pixel_delta_w * x as f32) + (self.camera.pixel_delta_h * y as f32)) - self.camera.look_from;
                let color = Renderer::per_pixel(
                    Arc::clone(&self.scene),
                    self.camera.look_from,
                    ray_dir, self.camera.pixel_delta_w,
                    self.camera.pixel_delta_h,
                    self.samples_per_pixel,
                    self.max_ray_bounces
                );

                *value = color;
            });
        });
        canvas.to_png()
    }

    fn per_pixel(scene: Arc<RwLock<Scene>>, ray_origin: Vec3, ray_dir: Vec3, delta_w: Vec3, delta_h: Vec3, spp: usize, max_ray_bounces: usize) -> Vec3{
        let mut  color = DVec3::default();
        for _ in 0..spp{
            let ray_dir_jitter = delta_w * (rand::random::<f32>() - 0.5) + delta_h * (rand::random::<f32>() - 0.5);

            let mut ray = Ray::new(ray_origin, (ray_dir + ray_dir_jitter).normalize());

            // Nothing should have lock on scene after render start
            let rw_scene: std::sync::RwLockReadGuard<'_, Scene> = (*scene).read().unwrap();

            color += Renderer::trace_ray(&rw_scene, &mut ray, max_ray_bounces).as_dvec3();
        }
        (color / spp as f64).as_vec3()
    }

    // Returns energy traced from that ray
    // Recursive version
    fn trace_ray(read_scene: &std::sync::RwLockReadGuard<'_, Scene>, ray: &mut Ray, depth: usize) -> Vec3{
        let mut ray_energy = Vec3::ZERO;
        let mut closest_hit: Option<HitObject> = None;
        
        // Iter over scene objects
        for (object_id, object) in (*read_scene).objects.iter().enumerate(){
            let hit = object.intersect(&ray);
            // Adding epsilon prevents floating point calc error
            if hit > 0.0+f32::EPSILON{
                match &closest_hit{
                    None => closest_hit = Some(HitObject::new(object_id, hit)),
                    Some(close_hit) => {
                        if close_hit.hit_distance > hit {
                            closest_hit = Some(HitObject::new(object_id, hit));
                        }
                    }
                }
            }
        }

        if let Some(hit) = closest_hit{
            let material_id = read_scene.objects[hit.object_id].get_material_id();
            let object_albedo = read_scene.materials[material_id].albedo;
            let object_emmision = read_scene.materials[material_id].get_emmision();
            let object_opacity = read_scene.materials[material_id].opacity;

            // Spawn new ray
            if depth != 0 {
                let hit_point = ray.orgin + ray.direction * hit.hit_distance;
                let mut uv = read_scene.objects[hit.object_id].get_uv(hit_point);
                let front_face = ray.direction.dot(uv) < 0.0;

                // Needed for transparent objects becouse ray can be inside and then uv should point opposite way
                if !front_face{
                    uv = -uv;
                }


                let ray_dir = ray.direction;
                
                // Reflected ray
                ray.orgin = hit_point;
                ray.direction = read_scene.materials[material_id].get_scattered_ray_dir(ray_dir, uv);

                ray_energy += Self::trace_ray(read_scene, ray, depth-1) * object_opacity;

                // Refracted ray
                if object_opacity < 1.0{
                    ray.orgin = hit_point;
                    ray.direction = ray_dir;
                    ray_energy += Self::trace_ray(read_scene, ray, depth-1) * (1.0 - object_opacity);
                }
            }
            
            ray_energy *= object_albedo;
            ray_energy += object_emmision;
        }

        ray_energy
    }
}