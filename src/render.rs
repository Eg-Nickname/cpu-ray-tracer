use std::sync::Arc;
use std::sync::RwLock;
use glam::f32::Vec3;

#[allow(unused_imports)]
use rayon::prelude::*;

use crate::camera::Camera;
use crate::scene::Scene;
use crate::ray::Ray;
use crate::canvas::Canvas;

const BOUNCES: usize = 5;
pub struct Renderer{
    image_width: usize,
    image_height: usize,
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

// struct EnergyStack{
//     contribiution: f32,
//     color: Vec3,
//     emmision: Vec3,
// }

// struct RayStack{
//     ray: Ray,
//     depth: usize,
//     contribiution: f32
// }

impl Renderer{
    pub fn new(camera: Camera, scene: Scene) -> Self{
        Renderer{
            image_width: camera.image_width,
            image_height: camera.image_height,
            camera: camera,
            scene: Arc::new(RwLock::new(scene)),
        }
    }

    pub fn render(&self){
        let mut canvas = Canvas::new(self.image_width, self.image_height);
        canvas.data.par_iter_mut().enumerate().for_each(|(x, collumn)|{
            collumn.iter_mut().enumerate().for_each(|(y, value)|{
                let ray_dir = (self.camera.top_left + (self.camera.pixel_delta_w * x as f32) + (self.camera.pixel_delta_h * y as f32)) - self.camera.look_from;
                let color = Renderer::per_pixel(Arc::clone(&self.scene), self.camera.look_from, ray_dir.normalize());

                *value = color;
            });
        });
        canvas.to_png()
    }

    fn per_pixel(scene: Arc<RwLock<Scene>>, ray_origin: Vec3, ray_dir: Vec3) -> Vec3{
        let mut  color = Vec3::default();
        for _ in 0..120{
            let mut ray = Ray::new(ray_origin, ray_dir);

            // Nothing should have lock on scene after render start
            let rw_scene: std::sync::RwLockReadGuard<'_, Scene> = (*scene).read().unwrap();

            color += Renderer::trace_ray(&rw_scene, &mut ray, BOUNCES);
        }
        color / 120.0
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

    // fn trace_ray_recursive(read_scene: std::sync::RwLockReadGuard<'_, Scene>, start_ray: Ray, max_depth: usize) -> Vec3{
    //     let mut ray_stack: Vec<RayStack> = Vec::with_capacity(max_depth*2);
    //     let ray_stack_object = RayStack{ray: start_ray, depth: max_depth, contribiution: 1.0};
    //     ray_stack.push(ray_stack_object);
    //
    //     let mut energy_stack: Vec<EnergyStack> = Vec::with_capacity(max_depth*2);
    //
    //     while let Some(ray_object) = ray_stack.pop(){
    //         let mut closest_hit: Option<HitObject> = None;
    //         // Iter over scene objects
    //         for (object_id, object) in (*read_scene).objects.iter().enumerate(){
    //             if let Some(hit) = object.intersect(&ray_object.ray){
    //                 if hit.0 > 0.0 || hit.1 > 0.0{
    //                     let min_non_negative_hit_point = if hit.0 < 0.0{
    //                         hit.1 
    //                     }else if hit.1 < 0.0{
    //                         hit.0
    //                     }else{
    //                         hit.0.min(hit.1)
    //                     };
    //
    //                     match &closest_hit{
    //                         None => closest_hit = Some(HitObject::new(object_id, hit, min_non_negative_hit_point)),
    //                         Some(close_hit) => {
    //                             if close_hit.t.0 > hit.0 {
    //                                 closest_hit = Some(HitObject::new(object_id, hit, min_non_negative_hit_point));
    //                             }
    //                         }
    //                     }
    //                 }
    //             }
    //         }
    //
    //         match closest_hit {
    //             None => {},
    //             Some(hit_object) => {
    //                 // Spawn new ray
    //                 if ray_object.depth != 0 {
    //                     let hit_point = ray_object.ray.orgin + ray_object.ray.direction * hit_object.min_non_negative_hit_point;
    //                     let uv = read_scene.objects[hit_object.object_id].get_uv(hit_point);
    //
    //                     ray_stack.push(
    //                         RayStack{
    //                             ray: Ray::new(hit_point, (Ray::random_in_unit_sphere() + uv).normalize()),
    //                             depth: ray_object.depth-1,
    //                             contribiution: 1.0,
    //                         }
    //                     );
    //                 }
    //
    //                 let material_id = read_scene.objects[hit_object.object_id].get_material_id();
    //                 let object_albedo = read_scene.materials[material_id].albedo;
    //                 let object_emmision = read_scene.materials[material_id].get_emmision();
    //
    //                 energy_stack.push(EnergyStack{
    //                     contribiution: ray_object.contribiution,
    //                     color: object_albedo,
    //                     emmision: object_emmision
    //                 });
    //             }
    //         }
    //     }
    //
    //     let mut color = Vec3::ZERO;
    //     while let Some(energy_object) = energy_stack.pop(){
    //         color *= energy_object.color * energy_object.contribiution;
    //         color += energy_object.emmision * energy_object.contribiution;
    //     }
    //
    //     color
    // }
}