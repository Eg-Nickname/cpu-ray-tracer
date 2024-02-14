use cpu_ray_tracer::scene::Scene;
use glam::Vec3;
use std::time::Instant;

use cpu_ray_tracer::objects::material;
use cpu_ray_tracer::objects::sphere::Sphere;
use cpu_ray_tracer::render;

fn main(){
    let timer = Instant::now();

    _mate_sphere();

    
    println!("Render time: {}", timer.elapsed().as_secs_f64());
}

fn _mate_sphere(){
    let mut scene = Scene::new();
    // Default 0 material
    scene.add_material(material::Material::default());

    // White emmisive sphere material
    scene.add_material(material::Material::default());
    scene.materials[1].albedo = Vec3::ONE;
    scene.materials[1].emmision_color = Vec3::ONE;
    scene.materials[1].emmision_power = 1.0;

    // Purple mate sphere
    scene.add_material(material::Material::default());
    scene.materials[2].albedo = Vec3::new(0.8, 0.0, 1.0);

    // White emissive sphere
    scene.add_object(Box::new(Sphere::new(Vec3::new(0.0, 0.0, -13.5), 10.0, 1)));
    // Purple spheres
    scene.add_object(Box::new(Sphere::new(Vec3::new(0.0, 0.0, -1.0), 0.5, 2)));
    scene.add_object(Box::new(Sphere::new(Vec3::new(0.0, 100.5, -1.0), 100.0, 2)));


    let render = render::Renderer::new(1000, 1000, scene);
    render.render();
}

fn _emmisive_spheres_scene(){
    let mut scene = Scene::new();
    // Default 0 material
    scene.add_material(material::Material::default());

    // Red sphere material
    scene.add_material(material::Material::default());
    scene.materials[1].emmision_color = Vec3::new(1.0, 0.0, 0.0);
    scene.materials[1].emmision_power = 20.0;

    // Green sphere material
    scene.add_material(material::Material::default());
    scene.materials[2].emmision_color = Vec3::new(0.1, 0.8, 0.0);
    scene.materials[2].emmision_power = 1.0;

    // White sphere material
    scene.add_material(material::Material::default());
    scene.materials[3].albedo = Vec3::new(0.8, 0.8, 0.8);
    scene.materials[3].emmision_color = Vec3::new(0.8, 0.8, 0.8);
    scene.materials[3].emmision_power = 0.4;

    // White non emissive sphere
    scene.add_object(Box::new(Sphere::new(Vec3::new(1.1, 0.0, 1.0), 1.0, 0)));
    // Red sphere
    scene.add_object(Box::new(Sphere::new(Vec3::new(-1.1, 0.0, -0.2), 1.0, 1)));
    // Green sphere
    scene.add_object(Box::new(Sphere::new(Vec3::new(0.0, 30.0, 4.0), 28.0, 2)));
    scene.add_object(Box::new(Sphere::new(Vec3::new(0.0, -4.0, 6.0), 3.0, 3)));

    let render = render::Renderer::new(1000, 1000, scene);
    render.render();
}