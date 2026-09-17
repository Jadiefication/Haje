use num_traits::real::Real;

use crate::complex::Complex;

pub trait Sqrt {
    fn sqrt(&self) -> Self;
}

impl<T: Real> Sqrt for Complex<T> {
    fn sqrt(&self) -> Self {
        let two = T::one() + T::one();
        Complex {
            re: ((self.re + self.mag()) / two).sqrt(),
            im: (self.im / self.im.abs()) * ((self.mag() - self.re) / two).sqrt(),
        }
    }
}

impl<T: Real> Sqrt for T {
    fn sqrt(&self) -> Self {
        T::sqrt(*self)
    }
}
