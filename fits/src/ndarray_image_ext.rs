use ndarray::{Array, ArrayView, Ix2, Ix3};

pub trait NDArrayExt {
    fn create_rgba_image(&self) -> image::RgbaImage;
    fn create_r_image(&self) -> image::GrayImage;
    fn create_g_image(&self) -> image::GrayImage;
    fn create_b_image(&self) -> image::GrayImage;
    fn create_gray_image(&self) -> image::GrayImage;
}

impl<'a> NDArrayExt for ArrayView<'a, f32, Ix2> {
    fn create_rgba_image(&self) -> image::RgbaImage {
        todo!()
    }

    fn create_r_image(&self) -> image::GrayImage {
        todo!()
    }

    fn create_g_image(&self) -> image::GrayImage {
        todo!()
    }

    fn create_b_image(&self) -> image::GrayImage {
        todo!()
    }

    fn create_gray_image(&self) -> image::GrayImage {
        let params = StretchParams {
            scale: 255.0 / (self.mean().unwrap() * 2.0)
        };
        let shape = self.shape();
        let mut buff: image::GrayImage = image::ImageBuffer::new(shape[0] as u32, shape[1] as u32);
        for (x, y, pixel) in buff.enumerate_pixels_mut() {
            let px = [self[[x as usize, y as usize]]];
            *pixel = image::Luma(px.map(|v| {
                (((params.scale * v) as f32).clamp(0.0, 255.0) + 0.5).round() as u8
            }));
        }
        return buff;
    }
}

impl NDArrayExt for Array<f32, Ix3> {
    fn create_rgba_image(self: &Array<f32, Ix3>) -> image::RgbaImage {
        let params = StretchParams {
            scale: 255.0 / (self.mean().unwrap() * 2.0)
        };
        let shape = self.shape();
        let mut buff: image::RgbaImage = image::ImageBuffer::new(shape[0] as u32, shape[1] as u32);
        for (x, y, pixel) in buff.enumerate_pixels_mut() {
            let d = self.slice(ndarray::s![x as usize, y as usize, ..]);
            let px: [u8; 3] = [&d[0], &d[1], &d[2]]
                .map(|v| -> u8 {
                    (((params.scale * v) as f32).clamp(0.0, 255.0) + 0.5).round() as u8
                });
            *pixel = image::Rgba([px[0], px[1], px[2], 255]);
        }
        return buff;
    }

    fn create_r_image(&self) -> image::GrayImage {
        let r_image = self.slice(ndarray::s![.., .., 0]);
        return r_image.create_gray_image();
    }

    fn create_g_image(&self) -> image::GrayImage {
        let r_image = self.slice(ndarray::s![.., .., 1]);
        return r_image.create_gray_image();
    }

    fn create_b_image(&self) -> image::GrayImage {
        let r_image = self.slice(ndarray::s![.., .., 2]);
        return r_image.create_gray_image();
    }

    fn create_gray_image(&self) -> image::GrayImage {
        todo!()
    }
}


struct StretchParams {
    pub scale: f32
}
