pub mod canvas;
pub mod ray;
pub mod objects;
pub mod render;

mod tests{
    #[allow(unused_imports)]
    use super::*;
    #[allow(unused_imports)]
    use self::objects::Object;
    #[allow(unused_imports)]
    use std::sync::Arc;
    #[allow(unused_imports)]
    use crate::objects::sphere::Sphere;
    #[test]
    fn renderer_test(){
        let mut objects: Vec<Arc<dyn Object>> = Vec::new();
        objects.push(Arc::new(Sphere::default()));
        let render = render::Renderer::new(1000, 1000, objects);
        render.render();
        assert_eq!((), ());
    }
}