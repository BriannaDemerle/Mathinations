use std::fmt::Debug;

use super::*;

/// A complex number a+bi
pub struct Complex<N: Arithmetic> {
    real: N,
    imag: N,
}

impl<N: Arithmetic> Complex<N> {
    /// Creates a new complex number with real and imaginary parts.
    pub fn new(real: N, imag: N) -> Self {
        Self { real, imag }
    }
}

impl<N: Arithmetic + Clone> Complex<N> {
    /// Gets the real component of the complex number.
    pub fn real(&self) -> N {
        self.real.clone()
    }

    /// Gets the imaginary component of the complex number.
    pub fn imag(&self) -> N {
        self.imag.clone()
    }

    /// Returns the complex conjugate of `self`.
    pub fn conj(&self) -> Self {
        Self::new(self.real(), -self.imag())
    }

    /// Returns the magnitude of the complex number squared.
    pub fn magnitude_squared(&self) -> N {
        self.real() * self.real() + self.imag() * self.imag()
    }
}

impl<N: Arithmetic + AddIdentity + MulIdentity> Complex<N> {
    pub fn imaginary() -> Self {
        Self::new(N::ZERO, N::ONE)
    }
}

impl<N: Arithmetic + Clone> Clone for Complex<N> {
    fn clone(&self) -> Self {
        Self::new(self.real(), self.imag())
    }
}

impl<N: Arithmetic + Copy> Copy for Complex<N> {}

impl<N: Debug + Arithmetic + Clone> Debug for Complex<N> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?} + {:?}i", self.real(), self.imag())
    }
}

impl<N: Arithmetic + AddIdentity> From<N> for Complex<N> {
    fn from(value: N) -> Self {
        Self::new(value, N::ZERO)
    }
}

impl<N: Arithmetic> Add<N> for Complex<N> {
    type Output = Complex<N>;

    fn add(self, rhs: N) -> Self::Output {
        Self::Output::new(
            self.real + rhs,
            self.imag
        )
    }
}

impl<N: Arithmetic> Add for Complex<N> {
    type Output = Complex<N>;

    fn add(self, rhs: Self) -> Self::Output {
        Self::Output::new(
            self.real + rhs.real,
            self.imag + rhs.imag
        )
    }
}

impl<N: Arithmetic + Clone> AddAssign<N> for Complex<N> {
    fn add_assign(&mut self, rhs: N) {
        *self = self.clone() + rhs;
    }
}

impl<N: Arithmetic + Clone> AddAssign for Complex<N> {
    fn add_assign(&mut self, rhs: Self) {
        *self = self.clone() + rhs;
    }
}

impl<N: Arithmetic> Sub<N> for Complex<N> {
    type Output = Complex<N>;

    fn sub(self, rhs: N) -> Self::Output {
        Self::Output::new(
            self.real - rhs,
            self.imag
        )
    }
}

impl<N: Arithmetic> Sub for Complex<N> {
    type Output = Complex<N>;

    fn sub(self, rhs: Self) -> Self::Output {
        Self::Output::new(
            self.real - rhs.real,
            self.imag - rhs.imag
        )
    }
}

impl<N: Arithmetic + Clone> SubAssign<N> for Complex<N> {
    fn sub_assign(&mut self, rhs: N) {
        *self = self.clone() - rhs;
    }
}

impl<N: Arithmetic + Clone> SubAssign for Complex<N> {
    fn sub_assign(&mut self, rhs: Self) {
        *self = self.clone() - rhs;
    }
}

impl<N: Arithmetic + Clone> Mul<N> for Complex<N> {
    type Output = Complex<N>;

    fn mul(self, rhs: N) -> Self::Output {
        Self::Output::new(
            self.real * rhs.clone(),
            self.imag * rhs
        )
    }
}

impl<N: Arithmetic + Clone> Mul for Complex<N> {
    type Output = Complex<N>;

    fn mul(self, rhs: Self) -> Self::Output {
        Self::Output::new(
            self.real() * rhs.real() - self.imag() * rhs.imag(),
            self.real() * rhs.imag() + self.imag() * rhs.real(),
        )
    }
}

impl<N: Arithmetic + Clone> MulAssign<N> for Complex<N> {
    fn mul_assign(&mut self, rhs: N) {
        *self = self.clone() * rhs;
    }
}

impl<N: Arithmetic + Clone> MulAssign for Complex<N> {
    fn mul_assign(&mut self, rhs: Self) {
        *self = self.clone() * rhs;
    }
}

impl<N: Arithmetic + Clone> Div<N> for Complex<N> {
    type Output = Complex<N>;

    fn div(self, rhs: N) -> Self::Output {
        Self::new(
            self.real / rhs.clone(),
            self.imag / rhs,
        )
    }
}

impl<N: Arithmetic + Clone> Div for Complex<N> {
    type Output = Complex<N>;

    fn div(self, rhs: Self) -> Self::Output {
        (self.clone() * rhs.conj()) / rhs.magnitude_squared()
    }
}

impl<N: Arithmetic + Clone> DivAssign<N> for Complex<N> {
    fn div_assign(&mut self, rhs: N) {
        *self = self.clone() / rhs;
    }
}

impl<N: Arithmetic + Clone> DivAssign for Complex<N> {
    fn div_assign(&mut self, rhs: Self) {
        *self = self.clone() / rhs;
    }
}

impl<N: Arithmetic + Clone> Neg for Complex<N> {
    type Output = Complex<N>;

    fn neg(self) -> Self::Output {
        Self::Output::new(
            -self.real(),            
            -self.imag(),            
        )
    }
}

impl<N: Arithmetic + Clone + PartialEq> PartialEq for Complex<N> {
    fn eq(&self, other: &Self) -> bool {
        self.real() == other.real() && self.imag() == other.imag()
    }
}

impl<N: Arithmetic + Clone + Eq> Eq for Complex<N> { }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn components() {
        let z = Complex::new(3_i32, -4_i32);

        assert_eq!(z.real(), 3);
        assert_eq!(z.imag(), -4);
    }

    #[test]
    fn conjugate() {
        let z = Complex::new(3_i32, -4_i32);

        assert_eq!(z.conj(), Complex::new(3_i32, 4_i32));
    }

    #[test]
    fn magnitude_squared() {
        let z = Complex::new(3_i32, -4_i32);
        let w = Complex::new(0_i32, 3_i32);

        assert_eq!(z.magnitude_squared(), 25);
        assert_eq!(w.magnitude_squared(), 9);
    }

    #[test]
    fn addition() {
        let a = Complex::new(3_i32, -4_i32);
        let b = Complex::new(1_i32, 2_i32);

        assert_eq!(a + b, Complex::new(4, -2));
        assert_eq!(a + 4, Complex::new(7, -4));
        assert_eq!(b + b, Complex::new(2, 4));

        let mut z = Complex::new(1_i32, 1_i32);
        z += Complex::new(2_i32, -1_i32);
        z += 4;

        assert_eq!(z, Complex::new(7, 0));
    }

    #[test]
    fn subtraction() {
        let a = Complex::new(3_i32, -4_i32);
        let b = Complex::new(1_i32, 2_i32);

        assert_eq!(a - b, Complex::new(2, -6));
        assert_eq!(a - 4, Complex::new(-1, -4));
        assert_eq!(b - b, Complex::new(0, 0));

        let mut z = Complex::new(1_i32, 1_i32);
        z -= Complex::new(2_i32, -1_i32);
        z -= 4;

        assert_eq!(z, Complex::new(-5, 2));
    }

    #[test]
    fn multiply() {
        let a = Complex::new(3_i32, -4_i32);
        let b = Complex::new(1_i32, 2_i32);

        assert_eq!(a * b, Complex::new(11, 2));
        assert_eq!(a * 2, Complex::new(6, -8));
        assert_eq!(b * b, Complex::new(-3, 4));

        let mut z = Complex::new(1_i32, 1_i32);
        z *= Complex::new(2_i32, -1_i32);
        z *= 4;

        assert_eq!(z, Complex::new(12, 4));
    }

    #[test]
    fn divide() {
        let a = Complex::new(3_i32, -4_i32);
        let b = Complex::new(1_i32, 2_i32);

        assert_eq!(a / b, Complex::new(-1, -2));
        assert_eq!(a / 2, Complex::new(1, -2));
        assert_eq!(b / b, Complex::new(1, 0));

        let mut z = Complex::new(18_i32, 15_i32);
        z /= Complex::new(2_i32, -1_i32);
        z /= 4;

        assert_eq!(z, Complex::new(1, 2));
    }

    #[test]
    fn negate() {
        let z = Complex::new(3_i32, -4_i32);

        assert_eq!(-z, Complex::new(-3, 4));
        assert_eq!(-Complex::new(0_i32, 0_i32), Complex::new(0_i32, 0_i32));
    }
}