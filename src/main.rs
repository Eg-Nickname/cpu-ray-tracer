use std::sync::Arc;
use glam::Vec3;
use std::time::Instant;

use cpu_ray_tracer::objects::Object;
use cpu_ray_tracer::objects::material;
use cpu_ray_tracer::objects::sphere::Sphere;
use cpu_ray_tracer::render;

fn main(){
    let timer = Instant::now();

    let mut objects: Vec<Arc<dyn Object + Sync + Send>> = Vec::new();
    objects.push(Arc::new(Sphere::new(Vec3::new(1.1, 0.0, 1.0), 1.0, material::Material::default())));

    let mut red_sphere = Sphere::new(Vec3::new(-1.1, 0.0, -0.2), 1.0, material::Material::default());
    red_sphere.material.emmision_color = Vec3::new(1.0, 0.0, 0.0);
    red_sphere.material.emmision_power = 20.0;
    objects.push(Arc::new(red_sphere));

    let mut green_sphere = Sphere::new(Vec3::new(0.0, 30.0, 4.0), 28.0, material::Material::default());
    green_sphere.material.emmision_color = Vec3::new(0.1, 0.8, 0.0);
    green_sphere.material.emmision_power = 1.0;
    objects.push(Arc::new(green_sphere));

    let mut white_sphere = Sphere::new(Vec3::new(0.0, -4.0, 6.0), 3.0, material::Material::default());
    white_sphere.material.emmision_color = Vec3::new(0.8, 0.8, 0.8);
    white_sphere.material.albedo = Vec3::new(0.8, 0.8, 0.8);
    white_sphere.material.emmision_power = 0.4;
    objects.push(Arc::new(white_sphere));



    let render = render::Renderer::new(1000, 1000, objects);
    render.render();

    
    println!("Render time: {}", timer.elapsed().as_secs_f64());
}