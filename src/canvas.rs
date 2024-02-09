use glam::f32::Vec3;

pub struct Canvas{
    width: usize,
    height: usize,
    data: Vec<Vec<Vec3>>
}

impl Canvas{
    pub fn new(width: usize, height: usize) -> Canvas{
        let mut row_vec = Vec::with_capacity(width+1);
        for _x in 0..width{
            let mut collumn_vec = Vec::with_capacity(height+1);
            for _y in 0..height{
                collumn_vec.push(Vec3::default())
            }
            row_vec.push(collumn_vec)
        }
        Canvas{
            width: width,
            height: height,
            data: row_vec
        }
    }
    pub fn get_color(&mut self, x: usize, y: usize) -> Vec3{
        self.data[x][y].clone()
    }

    pub fn set_color(&mut self, x: usize, y: usize, color: Vec3){
        self.data[x][y] = color;
    }

    // Creates png file from canvas
    pub fn to_png(self){
        use std::path::Path;
        use std::fs::File;
        use std::io::BufWriter;
        
        let path = Path::new(r"output.png");
        let file = File::create(path).unwrap();
        let ref mut w = BufWriter::new(file);
        
        let mut encoder = png::Encoder::new(w, self.width as u32, self.height as u32); // Width is 2 pixels and height is 1.
        encoder.set_color(png::ColorType::Rgba);
        encoder.set_depth(png::BitDepth::Eight);
        encoder.set_source_gamma(png::ScaledFloat::from_scaled(45455)); // 1.0 / 2.2, scaled by 100000
        encoder.set_source_gamma(png::ScaledFloat::new(1.0 / 2.2));     // 1.0 / 2.2, unscaled, but rounded
        let source_chromaticities = png::SourceChromaticities::new(     // Using unscaled instantiation here
            (0.31270, 0.32900),
            (0.64000, 0.33000),
            (0.30000, 0.60000),
            (0.15000, 0.06000)
        );
        encoder.set_source_chromaticities(source_chromaticities);
        let mut writer = encoder.write_header().unwrap();
        
        let mut data = Vec::with_capacity(self.width*self.height + 1); // An array containing a RGBA sequence. First pixel is red and second pixel is black.
        for y in 0..self.height{
            for x in 0..self.width{
                let rgba = self.data[x][y];
                data.push((rgba.x * 255.0) as u8);
                data.push((rgba.y * 255.0) as u8);
                data.push((rgba.z * 255.0) as u8);
                data.push(255);
            }
        }

        writer.write_image_data(data.as_slice()).unwrap(); // Save
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn canvas_creation() {
        let canvas = Canvas::new(100, 100);
        assert_eq!(canvas.data.len(), 100);
        assert_eq!(canvas.data[0].len(), 100);
    }

    #[test]
    fn get_set_canvas_pixel(){
        let mut canvas = Canvas::new(100, 100);
        canvas.set_color(10, 10, Vec3::new(0.1, 0.2, 0.3));
        assert_eq!(canvas.data[10][10], Vec3::new(0.1, 0.2, 0.3));
        assert_eq!(canvas.get_color(10, 10), Vec3::new(0.1, 0.2, 0.3));
        canvas.set_color(10, 1, Vec3::new(1.1, 0.2, 0.3));
        canvas.to_png();
    }
}