use std::sync::Arc;
use glam::f32::{Vec2,Vec3};

use crate::{objects::Object, ray::Ray};
use crate::canvas::Canvas;
pub struct Renderer{
    image_width: usize,
    image_height: usize,
    objects: Vec<Arc<dyn Object>>
}

struct HitObject{
    object: Arc<dyn Object>,
    t: (f32, f32)    
}

impl HitObject{
    pub fn new(object: Arc<dyn Object>, t: (f32, f32)) -> Self{
        HitObject {object: object, t: t}
    }
}

impl Renderer{
    pub fn new(image_width: usize, image_height: usize, objects: Vec<Arc<dyn Object>>) -> Self{
        Renderer{
            image_width: image_width,
            image_height: image_height,
            objects: objects
        }
    }
    pub fn render(&self){
        let mut canvas = Canvas::new(self.image_width, self.image_height);
        for i in 0..(self.image_width*self.image_height) {
            let x = (i % self.image_width) as f32;
            let y = (i / self.image_height) as f32;

            let mut  coord = Vec2::new(x/self.image_width as f32, y/self.image_height as f32);
            coord = (coord * 2.0) - 1.0;

            let color = self.per_pixel(coord);
            canvas.set_color(i % self.image_width, i / self.image_height, color);
        }
        canvas.to_png()
    }

    fn per_pixel(&self, coord: Vec2) -> Vec3{
        let ray = Ray::new(Vec3::new(0.0, 0.0, -3.0), Vec3::new(coord.x, coord.y, -1.0));
        self.trace_ray(&ray)
    }

    fn trace_ray(&self, ray: &Ray) -> Vec3 {
        // let hitobjects: Vec<HitObject> = Vec::new();
        let mut closest_hit: Option<HitObject> = None;
        for object_ref in &self.objects{
            let object = (*object_ref).clone();
            if let Some(hit) = object.intersect(ray){
                match &closest_hit{
                    None => closest_hit = Some(HitObject::new(object.clone(), hit)),
                    Some(close_hit) => {
                        if close_hit.t.1 > hit.1{
                            closest_hit = Some(HitObject::new(object.clone(), hit));
                        }
                    }
                }
            }
        }
        match closest_hit {
            None => Vec3::new(0.0, 0.0, 0.0),
            Some(hit) => Vec3::new(0.5, 0.5, 0.5)
        }
    }
}