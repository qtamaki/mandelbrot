extern crate num;
extern crate image;
use num::Complex;
use std::cell::Cell;
use std::cell::RefCell;

struct Mandelbrot {
    width: u32,
    height: u32,
    lower_complex: RefCell<Complex<f64>>,
    upper_complex: RefCell<Complex<f64>>,
    distance_re: Cell<f64>,
    distance_im: Cell<f64>,
}


impl Mandelbrot {
    fn new(width: u32, height: u32, lower_complex: Complex<f64>, upper_complex: Complex<f64>) -> Mandelbrot {
        Mandelbrot {
            width: width,
            height: height,
            lower_complex: RefCell::new(lower_complex),
            upper_complex: RefCell::new(upper_complex),
            distance_re: Cell::new(upper_complex.re - lower_complex.re),
            distance_im: Cell::new(upper_complex.im - lower_complex.im)
        }
    }

    fn enlarge(&self, lower_complex: Complex<f64>, upper_complex: Complex<f64>){
        *self.lower_complex.borrow_mut() = lower_complex;
        *self.upper_complex.borrow_mut() = upper_complex;
        self.distance_re.set(upper_complex.re  - lower_complex.re);
        self.distance_im.set(upper_complex.im - lower_complex.im);
    }

    fn resize(&self){
        let lower_complex = Complex{re: self.lower_complex.borrow().re + self.distance_re.get() * 0.19,
                                    im: self.lower_complex.borrow().im + self.distance_im.get() * 0.13 }; 
        let upper_complex = Complex{re: self.upper_complex.borrow().re - self.distance_re.get() * 0.21,
                                    im: self.upper_complex.borrow().im - self.distance_im.get() * 0.27 };    
        self.enlarge(lower_complex, upper_complex);                      
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
        let xf = self.lower_complex.borrow().re + x as f64 * self.distance_re.get() / self.width as f64;
        let yf = self.lower_complex.borrow().im + y as f64 * self.distance_im.get() / self.height as f64;
        Complex{re: xf, im: yf}
    } 

    fn draw(&self, image_name: &str) {
        let mut imgbuf = image::ImageBuffer::new(self.width, self.height);
        for(x,y,pixel) in imgbuf.enumerate_pixels_mut(){
            let c = self.pixel_to_point(x,y);
            let v = self.diverge_time(c, 255);
            match v {
                None => *pixel = image::Rgb([0,0,0]),
                Some(i) => *pixel = image::Rgb([255 - i as u8 ,255 - i as u8, 255-i as u8]),
            };
        }
        imgbuf.save(image_name).unwrap();
    }
}

fn main() {
    //画像のサイズ
    let imgx = 1000;
    let imgy = 1000;

    let lower_complex = Complex{ re: -2.0, im: -2.0 };
    let upper_complex = Complex{ re: 2.0, im: 2.0 };
    let mand = Mandelbrot::new(imgx, imgy, lower_complex, upper_complex);

    for i in 0..10 {
        println!("{}",i);
        let s = format!("png/mand_{:04}.png", i);
        mand.draw(&s);
        mand.resize();
    }
}