pub mod canvas;
pub mod ray;
pub mod objects;
pub mod render;
pub mod scene;
pub mod camera;

// mod tests{
//     use glam::Vec3;

//     #[allow(unused_imports)]
//     use super::*;
//     #[allow(unused_imports)]
//     use self::objects::Object;
//     #[allow(unused_imports)]
//     use std::sync::Arc;
//     use crate::objects::material;
//     #[allow(unused_imports)]
//     use crate::objects::sphere::Sphere;
//     #[test]
//     fn renderer_test(){
//         let mut objects: Vec<Arc<dyn Object>> = Vec::new();
//         objects.push(Arc::new(Sphere::new(Vec3::new(1.1, 0.0, 0.0), 1.0, material::Material::default())));
//         objects.push(Arc::new(Sphere::new(Vec3::new(0.0, 30.0, 3.0), 28.0, material::Material::default())));
//         let mut glowing_sphere = Sphere::new(Vec3::new(-1.1, 0.0, -0.2), 1.0, material::Material::default());
//         glowing_sphere.material.emmision_color = Vec3::new(1.0, 0.0, 0.0);
//         glowing_sphere.material.emmision_power = 20.0;
//         objects.push(Arc::new(glowing_sphere));
//         let render = render::Renderer::new(100, 100, objects);
//         render.render();
//         assert_eq!((), ());
//     }
// }