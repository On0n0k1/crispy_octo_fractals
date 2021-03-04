pub fn run(fractal_x: f64, fractal_y: f64, iterations: u32) -> u32{
    let c = num::complex::Complex::new(fractal_x, fractal_y);
    let mut z = num::complex::Complex::new(0.0, 0.0);

    let mut n: u32 = 0;
    while n < iterations{
        z = (z*z) + c;
        if z.norm() >= 2.0 {
            break;
        };
        n = n + 1;
    }
    n
}
