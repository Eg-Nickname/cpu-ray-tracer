use cpu_ray_tracer::camera::Camera;
use cpu_ray_tracer::objects::aabb::AABB;
use cpu_ray_tracer::scene::Scene;
use glam::Vec3;
use std::time::Instant;

use cpu_ray_tracer::objects::material;
use cpu_ray_tracer::objects::sphere::Sphere;
use cpu_ray_tracer::render;

fn main(){
    let setup_timer = Instant::now();
    let scene = _rgb_cubes();
    
    let mut camera = Camera::default();
    camera.image_width = 900;
    camera.image_height = 600;
    camera.update_look_from(Vec3::new(-6.0, -4.0, 6.1));
    camera.update_look_at(Vec3::new(-8.0, -2.0, 8.1));
    camera.recalculate_viewport();
    println!("Setup time: {}", setup_timer.elapsed().as_micros());

    let render_timer = Instant::now();
    let render = render::Renderer::new(camera, scene, 10, 120);
    render.render();
    println!("Render time: {}", render_timer.elapsed().as_secs_f64());
}

fn _rgb_cubes() -> Scene{
    let mut scene = Scene::new();
    // Default 0 material
    scene.add_material(material::Material::default());

    // White emmisive material
    scene.add_material(material::Material::default());
    scene.materials[1].albedo = Vec3::ONE;
    scene.materials[1].emmision_color = Vec3::ONE;
    scene.materials[1].emmision_power = 1.0;

    // Red emmisive material
    scene.add_material(material::Material::default());
    scene.materials[2].albedo = Vec3::new(0.5, 0.5, 0.5);
    scene.materials[2].emmision_color = Vec3::new(1.0, 0.0, 0.0);
    scene.materials[2].emmision_power = 1.5;

    // Green emmisive material
    scene.add_material(material::Material::default());
    scene.materials[3].albedo = Vec3::new(0.5, 0.5, 0.5);
    scene.materials[3].emmision_color = Vec3::new(0.0, 1.0, 0.0);
    scene.materials[3].emmision_power = 1.5;

    // Blue emmisive material
    scene.add_material(material::Material::default());
    scene.materials[4].albedo = Vec3::new(0.5, 0.5, 0.5);
    scene.materials[4].emmision_color = Vec3::new(0.0, 0.0, 1.0);
    scene.materials[4].emmision_power = 1.5;

    
    // Floor
    scene.add_object(Box::new(AABB::new(Vec3::new(-10.0, 1.0, -10.0), Vec3::new(10.0, 0.0, 10.0), 0)));

    // Celling
    scene.add_object(Box::new(AABB::new(Vec3::new(-10.0, -10.0, -10.0), Vec3::new(10.0, -11.0, 10.0), 0)));

    // Walls
    scene.add_object(Box::new(AABB::new(Vec3::new(-10.0, 10.0, -10.0), Vec3::new(-10.0, -11.0, 10.0), 0)));
    scene.add_object(Box::new(AABB::new(Vec3::new(-10.0, 10.0, 10.0), Vec3::new(10.0, -10.0, 11.0), 0)));

    scene.add_object(Box::new(AABB::new(Vec3::new(10.0, 10.0, -10.0), Vec3::new(11.0, -10.0, 10.0), 0)));
    scene.add_object(Box::new(AABB::new(Vec3::new(-10.0, 10.0, -10.0), Vec3::new(10.0, -10.0, -11.0), 0)));

    // Red emmisive cube
    scene.add_object(Box::new(AABB::cube(Vec3::new(-7.9, -1.0, 8.1), 1.0,2)));
    // Green emmisive cube
    scene.add_object(Box::new(AABB::cube(Vec3::new(-9.0, -1.0, 7.0), 1.0,3)));
    // Blue emmisive cube
    scene.add_object(Box::new(AABB::cube(Vec3::new(-9.0, -2.1, 8.1), 1.0,4)));

    scene
}

fn _test_cubes() -> Scene{
    let mut scene = Scene::new();
    // Default 0 material
    scene.add_material(material::Material::default());

    // White emmisive material
    scene.add_material(material::Material::default());
    scene.materials[1].albedo = Vec3::ONE;
    scene.materials[1].emmision_color = Vec3::ONE;
    scene.materials[1].emmision_power = 1.0;

    // White mate material
    scene.add_material(material::Material::default());
    scene.materials[2].albedo = Vec3::new(0.8, 0.8, 0.8);

    // Yellow mate material
    scene.add_material(material::Material::default());
    scene.materials[3].albedo = Vec3::new(0.8, 0.8, 0.0);

    // Pink metalic material
    scene.add_material(material::Material::default());
    scene.materials[4].albedo = Vec3::new(0.8, 0.2, 0.9);
    scene.materials[4].metalic = 1.0;
    scene.materials[4].roughness = 0.0;


    // Cyan transparent material
    scene.add_material(material::Material::default());
    scene.materials[5].albedo = Vec3::new(0.0, 0.8, 0.8);
    scene.materials[5].metalic = 1.0;
    scene.materials[5].opacity = 0.1;

    // White emissive sphere
    scene.add_object(Box::new(Sphere::new(Vec3::new(0.0, -11.0, -13.5), 10.0, 1)));

    // Mate sphere and cubes
    scene.add_object(Box::new(Sphere::new(Vec3::new(0.0, 100.5, -1.0), 100.0, 2)));
    scene.add_object(Box::new(Sphere::new(Vec3::new(0.0, 0.0, 102.5), 100.0, 2)));

    // scene.add_object(Box::new(AABB::cube(Vec3::new(-3.8, 0.0, -1.0), 1.0, 4)));
    // scene.add_object(Box::new(AABB::cube(Vec3::new(-0.6, 0.0, -1.0), 1.0, 3)));
    // scene.add_object(Box::new(AABB::cube(Vec3::new(0.6, 0.0, -1.0), 1.0, 5)));
    scene.add_object(Box::new(AABB::cube(Vec3::new(-3.0, -1.6, 0.0), 1.0, 5)));
    scene.add_object(Box::new(AABB::new(Vec3::new(-3.0, -0.5, 0.0), Vec3::new(-1.0, -1.5, 1.0), 1)));

    scene
}

fn _transparent_spheres() -> Scene{
    let mut scene = Scene::new();
    // Default 0 material
    scene.add_material(material::Material::default());

    // White emmisive sphere material
    scene.add_material(material::Material::default());
    scene.materials[1].albedo = Vec3::ONE;
    scene.materials[1].emmision_color = Vec3::ONE;
    scene.materials[1].emmision_power = 1.0;

    // White mate material
    scene.add_material(material::Material::default());
    scene.materials[2].albedo = Vec3::new(0.8, 0.8, 0.8);

    // Yellow transparent material
    scene.add_material(material::Material::default());
    scene.materials[3].albedo = Vec3::new(0.8, 0.8, 0.0);
    scene.materials[3].metalic = 1.0;
    scene.materials[3].opacity = 0.1;
    scene.materials[3].refractive_index = 1.458;

    // Yellow mate material
    scene.add_material(material::Material::default());
    scene.materials[4].albedo = Vec3::new(0.8, 0.8, 0.0);

    // Cyan transparent material
    scene.add_material(material::Material::default());
    scene.materials[5].albedo = Vec3::new(0.0, 0.8, 0.8);
    scene.materials[5].metalic = 1.0;
    scene.materials[5].opacity = 0.1;
    scene.materials[5].refractive_index = 1.458;

    // White emissive sphere (scene light source)
    scene.add_object(Box::new(Sphere::new(Vec3::new(0.0, -11.0, -13.5), 10.0, 1)));

    // White mate spheres
    scene.add_object(Box::new(Sphere::new(Vec3::new(0.0, 100.5, -1.0), 100.0, 2)));
    scene.add_object(Box::new(Sphere::new(Vec3::new(0.0, 0.0, 102.5), 100.0, 2)));

    // Yellow transparent sphere
    scene.add_object(Box::new(Sphere::new(Vec3::new(0.6, 0.0, -1.0), 0.5, 3)));
    // Cyan transparent sphere
    scene.add_object(Box::new(Sphere::new(Vec3::new(-0.6, 0.0, -1.0), 0.5, 5)));
    // Yellow mate sphere
    scene.add_object(Box::new(Sphere::new(Vec3::new(0.6, 0.0, 1.0), 0.5, 4)));
    // White emmisive sphere
    scene.add_object(Box::new(Sphere::new(Vec3::new(-0.6, 0.0, 1.0), 0.5, 1)));

    scene
}

fn _metal_sphere() -> Scene{
    let mut scene = Scene::new();
    // Default 0 material
    scene.add_material(material::Material::default());

    // White emmisive material
    scene.add_material(material::Material::default());
    scene.materials[1].albedo = Vec3::ONE;
    scene.materials[1].emmision_color = Vec3::ONE;
    scene.materials[1].emmision_power = 1.0;

    // White mate material
    scene.add_material(material::Material::default());
    scene.materials[2].albedo = Vec3::new(0.8, 0.8, 0.8);

    // Yellow metalic material
    scene.add_material(material::Material::default());
    scene.materials[3].albedo = Vec3::new(0.8, 0.8, 0.0);
    scene.materials[3].metalic = 1.0;

    // Pink metalic fuzzy material
    scene.add_material(material::Material::default());
    scene.materials[4].albedo = Vec3::new(0.8, 0.2, 0.9);
    scene.materials[4].metalic = 1.0;
    scene.materials[4].roughness = 0.3;
    
    // White emissive sphere
    scene.add_object(Box::new(Sphere::new(Vec3::new(0.0, -11.0, -13.5), 10.0, 1)));

    // White mate sphere
    scene.add_object(Box::new(Sphere::new(Vec3::new(0.0, 100.5, -1.0), 100.0, 2)));
    scene.add_object(Box::new(Sphere::new(Vec3::new(0.0, 0.0, -1.0), 0.5, 2)));
    // Yellow metalic sphere
    scene.add_object(Box::new(Sphere::new(Vec3::new(-1.2, 0.0, -1.0), 0.5, 3)));
    // Purple metalic fuzzy sphere
    scene.add_object(Box::new(Sphere::new(Vec3::new(1.2, 0.0, -1.0), 0.5, 4)));

    scene
}

fn _mate_sphere() -> Scene{
    let mut scene = Scene::new();
    // Default 0 material
    scene.add_material(material::Material::default());

    // White emmisive material
    scene.add_material(material::Material::default());
    scene.materials[1].albedo = Vec3::ONE;
    scene.materials[1].emmision_color = Vec3::ONE;
    scene.materials[1].emmision_power = 1.0;

    // Purple mate material
    scene.add_material(material::Material::default());
    scene.materials[2].albedo = Vec3::new(0.8, 0.0, 1.0);

    // White emissive sphere
    scene.add_object(Box::new(Sphere::new(Vec3::new(0.0, 0.0, -13.5), 10.0, 1)));
    // Purple mate spheres
    scene.add_object(Box::new(Sphere::new(Vec3::new(0.0, 0.0, -1.0), 0.5, 2)));
    scene.add_object(Box::new(Sphere::new(Vec3::new(0.0, 100.5, -1.0), 100.0, 2)));

    scene
}

fn _emmisive_spheres_scene() -> Scene{
    let mut scene = Scene::new();
    // Default 0 material
    scene.add_material(material::Material::default());

    // Red emmisive material
    scene.add_material(material::Material::default());
    scene.materials[1].emmision_color = Vec3::new(1.0, 0.0, 0.0);
    scene.materials[1].emmision_power = 20.0;

    // Green emmisive material
    scene.add_material(material::Material::default());
    scene.materials[2].emmision_color = Vec3::new(0.1, 0.8, 0.0);
    scene.materials[2].emmision_power = 1.0;

    // White emmisive material
    scene.add_material(material::Material::default());
    scene.materials[3].albedo = Vec3::new(0.8, 0.8, 0.8);
    scene.materials[3].emmision_color = Vec3::new(0.8, 0.8, 0.8);
    scene.materials[3].emmision_power = 0.4;

    // White mate sphere
    scene.add_object(Box::new(Sphere::new(Vec3::new(1.1, 0.0, 1.0), 1.0, 0)));
    // Red emmisive sphere
    scene.add_object(Box::new(Sphere::new(Vec3::new(-1.1, 0.0, -0.2), 1.0, 1)));
    // Green emmisive sphere
    scene.add_object(Box::new(Sphere::new(Vec3::new(0.0, 30.0, 4.0), 28.0, 2)));
    // White emmisive sphere
    scene.add_object(Box::new(Sphere::new(Vec3::new(0.0, -4.0, 6.0), 3.0, 3)));

    scene
}