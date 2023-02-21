use std::error::Error;

use image::GenericImageView;

#[derive(Debug)]
#[cfg_attr(test, derive(PartialEq))]
pub struct Pixel(u8, u8, u8);

#[derive(Debug)]
pub struct Image {
    pub pixels: Vec<Pixel>,
    pub width: u32,
    pub height: u32,
}

impl Image {
    pub fn new_from_path(path: &str) -> Result<Image, Box<dyn Error>> {
        let img = image::open(path)?;
        let (width, height) = img.dimensions();
        let pixels = img
            .pixels()
            .map(|pixel| {
                let rgb = pixel.2;
                Pixel(rgb[0], rgb[1], rgb[2])
            })
            .collect::<Vec<_>>();
        Ok(Image {
            pixels,
            width,
            height,
        })
    }

    pub fn save_image(self, path: &str) -> Result<(), Box<dyn Error>> {
        let mut imgbuf = image::ImageBuffer::new(self.width, self.height);
        for (index, (_, _, pixel)) in imgbuf.enumerate_pixels_mut().enumerate() {
            let p = &self.pixels[index];
            *pixel = image::Rgb([p.0, p.1, p.2]);
        }

        imgbuf.save(path)?;
        Ok(())
    }

    pub fn negative(&mut self) {
        for pixel in &mut self.pixels {
            pixel.0 = 255 - pixel.0;
            pixel.1 = 255 - pixel.1;
            pixel.2 = 255 - pixel.2;
        }
    }

    pub fn blur(&mut self, radius: f64) {
        // TODO: Remove this new vector and blur in place
        let mut new_pixels = Vec::new();
        for (index, _pixel) in self.pixels.iter().enumerate() {
            let mut red = 0;
            let mut green = 0;
            let mut blue = 0;
            let mut count = 0;
            for i in -radius as i32..radius as i32 {
                for j in -radius as i32..radius as i32 {
                    let width = (index as i32 % self.width as i32) + i;
                    let height = (index as i32 / self.width as i32) + j;
                    if width >= 0
                        && width < self.width as i32
                        && height >= 0
                        && height < self.height as i32
                        && (f64::from(i)).powi(2) + (f64::from(j)).powi(2) <= radius.powi(2)
                    {
                        let p = &self.pixels[(height * self.width as i32 + width) as usize];
                        red += i32::from(p.0);
                        green += i32::from(p.1);
                        blue += i32::from(p.2);
                        count += 1;
                    }
                }
            }
            new_pixels.push(Pixel(
                (red / count) as u8,
                (green / count) as u8,
                (blue / count) as u8,
            ));
        }
        self.pixels = new_pixels;
    }
}

#[cfg(test)]

mod tests {
    use super::*;

    #[test]
    fn test_new_from_path() {
        let img = Image::new_from_path("imgs/rgby.png").unwrap();
        assert_eq!(img.width, 2);
        assert_eq!(img.height, 2);
        assert_eq!(img.pixels.len(), 4);
        assert_eq!(img.pixels[0], Pixel(255, 0, 0));
        assert_eq!(img.pixels[1], Pixel(0, 255, 0));
        assert_eq!(img.pixels[2], Pixel(0, 0, 255));
        assert_eq!(img.pixels[3], Pixel(255, 255, 0));
    }

    #[test]
    fn test_negative() {
        let mut img = Image::new_from_path("imgs/rgby.png").unwrap();
        img.negative();
        assert_eq!(img.width, 2);
        assert_eq!(img.height, 2);
        assert_eq!(img.pixels.len(), 4);
        assert_eq!(img.pixels[0], Pixel(0, 255, 255));
        assert_eq!(img.pixels[1], Pixel(255, 0, 255));
        assert_eq!(img.pixels[2], Pixel(255, 255, 0));
        assert_eq!(img.pixels[3], Pixel(0, 0, 255));
    }

    #[test]
    fn test_blur() {
        let mut img = Image::new_from_path("imgs/rgby.png").unwrap();
        img.blur(5.0);
        assert_eq!(img.width, 2);
        assert_eq!(img.height, 2);
        assert_eq!(img.pixels.len(), 4);
        assert_eq!(img.pixels[0], Pixel(127, 127, 63));
        assert_eq!(img.pixels[1], Pixel(127, 127, 63));
        assert_eq!(img.pixels[2], Pixel(127, 127, 63));
        assert_eq!(img.pixels[3], Pixel(127, 127, 63));
    }

    // #[bench]
    // fn bench_new_from_path(b: &mut Bencher) {
    //     b.iter(|| Image::new_from_path("imgs/rgby.png"));
    // }

    // #[bench]
    // fn bench_negative(b: &mut Bencher) {
    //     let mut img = Image::new_from_path("imgs/rgby.png").unwrap();
    //     b.iter(|| img.negative());
    // }
}
