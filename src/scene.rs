use crate::objects::{material::Material, Object};

pub struct Scene{
    pub objects: Vec<Box<dyn Object + Sync + Send>>,
    pub materials: Vec<Material>
}

impl Scene{
    pub fn new() -> Self{
        Self { 
            objects: Vec::new(), 
            materials: Vec::new() 
        }
    }

    pub fn add_object(&mut self, object: Box<dyn Object + Sync + Send>){
        self.objects.push(object);
    }

    pub fn add_material(&mut self, material: Material){
        self.materials.push(material);
    }
}