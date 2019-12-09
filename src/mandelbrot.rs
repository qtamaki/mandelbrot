extern crate num;
extern crate image;
use num::Complex;
use std::sync::{Arc, Mutex};
use std::thread;

pub struct Mandelbrot {
    width: u32,
    height: u32,
    lower_complex: Complex<f64>,    
    distance_re: f64,
    distance_im: f64,
    data: Vec<u32>
}

impl Mandelbrot {
    pub fn new(width: u32, height: u32, lower_complex: Complex<f64>, upper_complex: Complex<f64>) -> Mandelbrot {
        let s = (width*height) as usize;
        let mut data = Vec::with_capacity(s);
        data.resize(s, 0);
        Mandelbrot {
            width: width,
            height: height,
            lower_complex: lower_complex,
            distance_re: upper_complex.re - lower_complex.re,
            distance_im: upper_complex.im - lower_complex.im,
            data: data
        }
    }

    fn diverge_time(c: Complex<f64>, limit: u32) -> u32 {
        let mut z = Complex {re: 0.0, im: 0.0 };
        for i in 0..limit {
            z = z * z + c;
            if z.norm_sqr() > 4.0 {
                return i;
            }
        }
        limit
    }

    fn pixel_to_point(&self, x: u32, y: u32) -> Complex<f64> {
        let xf = self.lower_complex.re + x as f64 * self.distance_re / self.width as f64;
        let yf = self.lower_complex.im + y as f64 * self.distance_im / self.height as f64;
        Complex{re: xf, im: yf}
    } 

    pub fn proc(&mut self) {
        for x in 0..self.width {
            for y in 0..self.height {
                let p = y * self.width + x;
                let c = self.pixel_to_point(x,y);
                let v = Mandelbrot::diverge_time(c, 255);
                self.data[p as usize] = v;
            }
        }
    }

    // parallel
    pub fn procp(&mut self) {
        let s = (self.width*self.height) as usize;
        let data: Vec<u32> = Vec::with_capacity(s);
        let d = Arc::new(Mutex::new(data));
        let mut handles = Vec::with_capacity(s);
        for x in 0..self.width {
            for y in 0..self.height {
                let dd = d.clone();
                let p = y * self.width + x;
                let c = self.pixel_to_point(x,y);
                let h = thread::spawn(move || {
                    let mut ddd = dd.lock().unwrap();
                    let v = Mandelbrot::diverge_time(c, 255);
                    ddd[p as usize] = v;
                });
                handles.push(h);
            }
        }
        for h in handles {
            let _ = h.join();
        }
        let dd = d.lock().unwrap();
        for i in 0..s {
            self.data[i] = dd[i];
        }
    }

    pub fn draw(&self, image_name: &str) {
        let mut imgbuf = image::ImageBuffer::new(self.width, self.height);
        for(x,y,pixel) in imgbuf.enumerate_pixels_mut(){
            let p = y * self.width + x;
            let i = self.data[p as usize];
            *pixel = image::Rgb([255 - i as u8 ,255 - i as u8, 255-i as u8]);
        }
        imgbuf.save(image_name).unwrap();
    }
}
