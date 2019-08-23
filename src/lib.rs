mod vec;

use vec::Vec3f;


pub fn blank_screen(width: usize, height: usize) -> Vec<u32> {
    let mut buffer: Vec<u32> = vec![0; width * height];

    for i in buffer.iter_mut() {
        *i = 0xff_ffff;
    }

    buffer
}

pub fn gradient(width: usize, height: usize) -> Vec<u32> {
    let mut buffer: Vec<u32> = vec![0; width * height];

    let height_f64 = height as f64;
    let width_f64 = width as f64;

    for y in 0..height {
        for x in 0..width {
            let color_vec = Vec3f::new(y as f64 / height_f64, x as f64 / width_f64, 0.0);

            let cr: u32 = ((color_vec.x() * 255.0) as u32) << 16;
            let cg: u32 = ((color_vec.y() * 255.0) as u32) << 8;
            let color = (cr + cg) as u32;

            buffer[(y * width) + x] = color;
        }
    }

    buffer
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
