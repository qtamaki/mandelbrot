use mandelbrot::mandelbrot::Mandelbrot;
use num::Complex;

fn main() {
    //画像のサイズ
    let imgx = 1000;
    let imgy = 1000;

    let lower_complex = Complex{ re: -2.0, im: -2.0 };
    let upper_complex = Complex{ re: 2.0, im: 2.0 };
    let mut mand = Mandelbrot::new(imgx, imgy, lower_complex, upper_complex);

    mand.procp();
    mand.draw("png/mand_.png")

}