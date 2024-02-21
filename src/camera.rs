use glam::Vec3;

pub struct Camera{
    pub look_from: Vec3,
    pub look_at: Vec3,
    up_direction: Vec3,
    aspect_ratio: f32,
    vfov_radians: f32,

    pub image_width: usize,
    pub image_height: usize,

    // Data for renderer
    pub top_left: Vec3,
    pub pixel_delta_w: Vec3,
    pub pixel_delta_h: Vec3,
}

impl Camera{
    pub fn new(image_width: usize, image_height: usize, look_from: Vec3, look_at: Vec3, vfov: f32) -> Self{
        let aspect_ratio: f32 = image_width as f32 / image_height as f32;
        // Vector up
        let up_direction = Vec3::new(0.0, -1.0, 0.0);
        let vfov_radians = vfov.to_radians();

        let mut base_camera = Self { 
            look_from: look_from,
            look_at: look_at,
            up_direction: up_direction,
            aspect_ratio: aspect_ratio,
            vfov_radians: vfov_radians,
            image_width: image_width,
            image_height: image_height,
            top_left: Vec3::default(), 
            pixel_delta_w: Vec3::default(),
            pixel_delta_h: Vec3::default()
        };
        base_camera.calculate_viewport();
        base_camera
    }

    fn calculate_viewport(&mut self){
        // Viewport dimensions
        let focal_length = (self.look_from - self.look_at).length();
        let viewport_height = 2.0 * (self.vfov_radians / 2.0).tan() * focal_length;
        let viewport_width = viewport_height * self.aspect_ratio;


        let w = (self.look_from - self.look_at).normalize();
        let u = self.up_direction.cross(w).normalize();
        let v = w.cross(u);

        // Vieport edges
        let viewport_h = viewport_height * -v;
        let viewport_w = viewport_width * u;

        // Ray change deppending on image position
        self.pixel_delta_h =  viewport_h / self.image_height as f32;
        self.pixel_delta_w =  viewport_w / self.image_width as f32;

        // Top left pixel location
        self.top_left = self.look_from - (focal_length * w) - viewport_w/2.0 - viewport_h/2.0 + 0.5 * (self.pixel_delta_h + self.pixel_delta_w);
    }

    pub fn update_vfov(&mut self, vfov: f32){
        self.vfov_radians = vfov.to_radians();
        self.calculate_viewport();
    }

    pub fn update_look_at(&mut self, look_at: Vec3){
        self.look_at = look_at;
        self.calculate_viewport();
    }

    pub fn update_look_from(&mut self, look_from: Vec3){
        self.look_from = look_from;
        self.calculate_viewport();
    }
}

impl Default for Camera{
    fn default() -> Self {
        Camera::new(1000, 1000, Vec3::new(0.0, 0.0, -3.0), Vec3::ZERO, 90.0)
    }
}