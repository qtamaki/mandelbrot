extern crate num;
extern crate image;
use num::Complex;

struct Mandelbrot {
    width: u32,
    height: u32,
    lower_complex: Complex<f64>,
    upper_complex: Complex<f64>,
    distance_re: f64,
    distance_im: f64,
}


impl Mandelbrot {
    fn new(width: u32, height: u32, lower_complex: Complex<f64>, upper_complex: Complex<f64>) -> Mandelbrot {
        Mandelbrot {
            width: width,
            height: height,
            lower_complex: lower_complex,
            upper_complex: upper_complex,
            distance_re: upper_complex.re  - lower_complex.re,
            distance_im: upper_complex.im - lower_complex.im
        }
    }

    fn diverge_time(&self, c: Complex<f64>, limit: u32) -> Option<u32> {
        let mut z = Complex {re: 0.0, im: 0.0 };
        for i in 0..limit {
            z = z * z + c;
            if z.norm_sqr() > 4.0 {
                return Some(i);
            }
        }
        None
    }

    fn pixel_to_point(&self, x: u32, y:u32) -> Complex<f64> {
        let xf = self.lower_complex.re + x as f64 * self.distance_re / self.width as f64;
        let yf = self.lower_complex.im + y as f64 * self.distance_im / self.height as f64;
        Complex{re: xf, im: yf}
    } 

    fn draw(&self) {
        let mut imgbuf = image::ImageBuffer::new(self.width, self.height);
        for(x,y,pixel) in imgbuf.enumerate_pixels_mut(){
            let c = self.pixel_to_point(x,y);
            let v = self.diverge_time(c, 255);
            match v {
                None => *pixel = image::Rgb([0,0,0]),
                Some(i) => *pixel = image::Rgb([255 - i as u8 ,255 - i as u8, 255-i as u8]),
            };
        }
        imgbuf.save("mandelbrot.png").unwrap();
    }
}

fn main() {
       //画像のサイズ
    let imgx = 1000;
    let imgy = 1000;

    let lower_complex = Complex{ re: -1.20, im: 0.35 };
    let upper_complex = Complex{ re: -1.0, im: 0.20 };

    let mand = Mandelbrot::new(imgx, imgy, lower_complex, upper_complex);
    mand.draw();
}