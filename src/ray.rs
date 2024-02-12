use glam::f32::Vec3;

pub struct Ray{
    pub orgin: Vec3,
    pub direction: Vec3,
}

impl Ray{
    pub fn new(origin: Vec3, direction: Vec3) -> Self{
        Self { 
            orgin: origin,
            direction: direction
        }
    }

    pub fn get_origin(&self) -> Vec3{
        self.orgin.clone()
    }

    pub fn get_direction(&self) -> Vec3{
        self.direction.clone()
    }

    pub fn position(&self, t: f32) -> Vec3{
        self.orgin.clone() + self.direction.clone() * t
    }

    pub fn random_in_unit_sphere() -> Vec3{
        loop{
            let point = Vec3::new(rand::random::<f32>(), rand::random::<f32>(),rand::random::<f32>()) * 2.0 - Vec3::ONE;
            if point.length_squared() < 1.0{
                return point.normalize()
            }
        }
        // ().normalize()
    }
}

#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn ray_creation(){
        let ray = Ray::new(Vec3::new(1.0, 2.0, 3.0), Vec3::new(4.0, 5.0, 6.0));

        assert_eq!(ray.get_origin(), Vec3::new(1.0, 2.0, 3.0));
        assert_eq!(ray.get_direction(), Vec3::new(4.0, 5.0, 6.0));
    }

    #[test]
    fn point_from_dsitance(){
        let ray = Ray::new(Vec3::new(2.0, 3.0, 4.0), Vec3::new(1.0, 0.0, 0.0));
        assert_eq!(ray.position(1.0), Vec3::new(3.0, 3.0, 4.0));
        assert_eq!(ray.position(-1.0), Vec3::new(1.0, 3.0, 4.0));
        assert_eq!(ray.position(2.5), Vec3::new(4.5, 3.0, 4.0));

    }
}