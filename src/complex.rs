pub struct Complex {
    pub re: f64,
    pub im: f64,
}

impl Complex {
    pub fn new(re: f64, im: f64) -> Self {
        Self { re, im }
    }
    pub fn add(&self, other: &Self) -> Self {
        Self {
            re: self.re + other.re,
            im: self.im + other.im,
        }
    }
    #[allow(dead_code)]
    pub fn mul(&self, other: &Self) -> Self {
        Self {
            re: (self.re * other.re) - (self.im * other.im),
            im: (self.re * other.im) - (self.im * other.re),
        }
    }
    pub fn square(&self) -> Self {
        Self {
            re: self.re * self.re - self.im * self.im,
            im: 2.0 * self.re * self.im,
        }
    }
    pub fn magnitude_squared(&self) -> f64 {
        (self.re * self.re) + (self.im * self.im)
    }

    #[allow(clippy::too_many_arguments)]
    pub fn pixel_to_complex(
        x: u32,
        y: u32,
        width: u32,
        height: u32,
        min_re: f64,
        max_re: f64,
        min_im: f64,
        max_im: f64,
    ) -> Self {
        Self {
            re: min_re + (x as f64 / width as f64) * (max_re - min_re),
            im: max_im - (y as f64 / height as f64) * (max_im - min_im),
        }
    }
}
