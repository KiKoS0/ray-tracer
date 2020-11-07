use num::Num;
use rand::distributions::Distribution;
use rand::distributions::Standard;
use rand::random;
use std::convert::Into;
use std::fmt::Debug;
use std::ops::Add;
use std::ops::AddAssign;
use std::ops::Div;
use std::ops::DivAssign;
use std::ops::Index;
use std::ops::Mul;
use std::ops::MulAssign;
use std::ops::Neg;
use std::ops::Sub;

#[repr(C, packed)]
#[derive(Clone, Copy, PartialEq)]
pub struct Vec3<T>
where
    T: Num + Copy + Into<f64> + Debug,
{
    e: [T; 3],
}

impl<T: Num + Copy + Into<f64> + Debug> Debug for Vec3<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        write!(f, "({:?},{:?},{:?})", self.e[0], self.e[1], self.e[2])
    }
}

impl<T: Add<Output = T> + Num + Copy + Into<f64> + Debug + Default> Add for Vec3<T> {
    type Output = Self;
    #[inline]
    fn add(self, rhs: Self) -> Self {
        Vec3::with_values(
            self.e[0] + rhs.e[0],
            self.e[1] + rhs.e[1],
            self.e[2] + rhs.e[2],
        )
    }
}

impl<T: Sub<Output = T> + Num + Copy + Into<f64> + Debug + Default> Sub for Vec3<T> {
    type Output = Self;
    #[inline]
    fn sub(self, rhs: Self) -> Self {
        Vec3::with_values(
            self.e[0] - rhs.e[0],
            self.e[1] - rhs.e[1],
            self.e[2] - rhs.e[2],
        )
    }
}

impl<T: Mul<Output = T> + Num + Copy + Into<f64> + Debug + Default> Mul for Vec3<T> {
    type Output = Self;
    #[inline]
    fn mul(self, rhs: Self) -> Self {
        Vec3::with_values(
            self.e[0] * rhs.e[0],
            self.e[1] * rhs.e[1],
            self.e[2] * rhs.e[2],
        )
    }
}

impl<T: Mul<Output = T> + Num + Copy + Into<f64> + Debug + Default> Mul<T> for Vec3<T> {
    type Output = Self;
    #[inline]
    fn mul(self, t: T) -> Self {
        Vec3::with_values(self.e[0] * t, self.e[1] * t, self.e[2] * t)
    }
}

impl<T: Neg<Output = T> + Num + Copy + Into<f64> + Debug + Default> Neg for Vec3<T> {
    type Output = Self;
    #[inline]
    fn neg(self) -> <Self as std::ops::Neg>::Output {
        Vec3::with_values(-self.e[0], -self.e[1], -self.e[2])
    }
}

impl<T: Neg<Output = T> + Num + Copy + Into<f64> + Debug + Default> Neg for &Vec3<T> {
    type Output = Vec3<T>;
    #[inline]
    fn neg(self) -> Self::Output {
        Vec3::with_values(-self.e[0], -self.e[1], -self.e[2])
    }
}

impl<T: Num + Copy + Into<f64> + Debug> AddAssign<T> for Vec3<T> {
    #[inline]
    fn add_assign(&mut self, rhs: T) {
        self.e[0] = self.e[0] + rhs;
        self.e[1] = self.e[1] + rhs;
        self.e[2] = self.e[2] + rhs;
    }
}

impl<T: Num + Copy + Into<f64> + Debug> AddAssign<&Vec3<T>> for Vec3<T> {
    #[inline]
    fn add_assign(&mut self, rhs: &Vec3<T>) {
        self.e[0] = self.e[0] + rhs.e[0];
        self.e[1] = self.e[1] + rhs.e[1];
        self.e[2] = self.e[2] + rhs.e[2];
    }
}

impl<T: Num + Copy + Into<f64> + Debug> MulAssign<T> for Vec3<T> {
    #[inline]
    fn mul_assign(&mut self, rhs: T) {
        self.e[0] = self.e[0] * rhs;
        self.e[1] = self.e[1] * rhs;
        self.e[2] = self.e[2] * rhs;
    }
}

impl Div<f64> for Vec3<f64> {
    type Output = Self;
    #[inline]
    fn div(self, t: f64) -> Self {
        Vec3::with_values(self.e[0] / t, self.e[1] / t, self.e[2] / t)
    }
}

impl Div<f64> for Vec3<f32> {
    type Output = Self;
    #[inline]
    fn div(self, t: f64) -> Self {
        let t = t as f32;
        Vec3::with_values(self.e[0] / t, self.e[1] / t, self.e[2] / t)
    }
}

impl<T: Num + Copy + Into<f64> + Debug> DivAssign<T> for Vec3<T> {
    #[inline]
    fn div_assign(&mut self, rhs: T) {
        self.e[0] = self.e[0] / rhs;
        self.e[1] = self.e[1] / rhs;
        self.e[2] = self.e[2] / rhs;
    }
}

impl<T: Num + Copy + Into<f64> + Debug> Index<usize> for Vec3<T> {
    type Output = T;
    fn index(&self, index: usize) -> &<Self as std::ops::Index<usize>>::Output {
        &self.e[index]
    }
}

impl<T: Num + Copy + Debug + Into<f64>> AsRef<Vec3<T>> for Vec3<T> {
    fn as_ref(&self) -> &Vec3<T> {
        self
    }
}

impl<T: Num + Copy + Into<f64> + Debug + Default + Sized> Vec3<T> {
    #[inline]
    pub fn length(&self) -> f64 {
        self.length_squared().into().sqrt()
    }
    #[inline]
    pub fn length_squared(&self) -> T {
        self.e[0] * self.e[0] + self.e[1] * self.e[1] + self.e[2] * self.e[2]
    }
    #[inline]
    pub fn x(&self) -> T {
        self.e[0]
    }
    #[inline]
    pub fn y(&self) -> T {
        self.e[1]
    }
    #[inline]
    pub fn z(&self) -> T {
        self.e[2]
    }
    #[inline]
    pub fn with_values(x: T, y: T, z: T) -> Vec3<T> {
        Vec3 { e: [x, y, z] }
    }
    #[inline]
    pub fn new() -> Vec3<T> {
        Vec3 {
            e: [T::default(), T::default(), T::default()],
        }
    }
    #[inline]
    pub fn as_std_vec(&self) -> Vec<T> {
        vec![self.x(), self.y(), self.z()]
    }

    #[inline]
    pub fn dot<A: AsRef<Vec3<T>>>(&self, v: A) -> f64 {
        let m = v.as_ref();
        (self.e[0] * m.e[0] + self.e[1] * m.e[1] + self.e[2] * m.e[2]).into()
    }

    #[inline]
    pub fn cross(&self, v: &Self) -> Self {
        Vec3::with_values(
            self.e[1] * v.e[2] - self.e[2] * v.e[1],
            self.e[2] * v.e[0] - self.e[0] * v.e[2],
            self.e[0] * v.e[1] - self.e[1] * v.e[0],
        )
    }
}

impl Vec3<f64> {
    pub fn unit_vec(&self) -> Self {
        *self / self.length()
    }
}
impl Vec3<f32> {
    pub fn unit_vec(&self) -> Self {
        *self / self.length()
    }
}

/// Generic implementation to generate
/// random vectors3 from T default generators
impl<T> Distribution<Vec3<T>> for Standard
where
    T: Num + Copy + Into<f64> + Debug + Default,
    Standard: Distribution<T>,
{
    fn sample<R: rand::Rng + ?Sized>(&self, _: &mut R) -> Vec3<T>
    where
        R: rand::Rng,
    {
        Vec3::with_values(random::<T>(), random::<T>(), random::<T>())
    }
}

#[cfg(test)]
mod tests {
    use crate::math::vec3::Vec3;
    #[test]
    fn with_values() {
        let e = [1.0, 4.0, 99.5];
        assert_eq!(Vec3 { e }, Vec3::with_values(1.0, 4.0, 99.5));
    }

    #[test]
    fn break_equality() {
        assert_ne!(
            Vec3::with_values(5.0, 9., 33.7),
            Vec3::with_values(1.0, 4.0, 99.5)
        );
    }

    #[test]
    fn xyz() {
        let v = Vec3::with_values(5.0, 9., 33.7);
        assert_eq!(v.x(), 5.0);
        assert_eq!(v.y(), 9.0);
        assert_eq!(v.z(), 33.7);
    }

    #[test]
    fn length() {
        let v = Vec3::with_values(2.0, 4.0, 4.0);
        assert_eq!(v.length_squared(), 36.);
        assert_eq!(v.length(), 6.);
    }

    #[test]
    fn add() {
        let v = Vec3::with_values(2.0, 4.0, 4.0);
        let w = Vec3::with_values(1.0, 3.0, 7.0);
        assert_eq!(v + w, Vec3::with_values(3.0, 7.0, 11.0));
    }
    #[test]
    fn neg() {
        let v = Vec3::with_values(2.0, 4.0, 4.0);
        assert_eq!(-v, Vec3::with_values(-2.0, -4.0, -4.0));
    }
    #[test]
    fn add_assign() {
        let mut v = Vec3::with_values(2.0, 4.0, 3.0);
        v += 5.;
        assert_eq!(v, Vec3::with_values(7.0, 9.0, 8.0));
    }

    #[test]
    fn mul_assign() {
        let mut v = Vec3::with_values(2.0, 4.0, 5.0);
        v *= 2.;
        assert_eq!(v, Vec3::with_values(4.0, 8.0, 10.0));
    }

    #[test]
    fn new() {
        let v = Vec3::new();
        let w = Vec3::new();
        assert_eq!(v, Vec3::with_values(0, 0, 0));
        assert_eq!(w, Vec3::with_values(0., 0., 0.));
    }

    #[test]
    fn index() {
        let v = Vec3::with_values(2.0, 4.0, 5.0);
        assert_eq!(v[0], 2.0);
        assert_eq!(v[1], 4.0);
        assert_eq!(v[2], 5.0);
    }

    #[test]
    #[should_panic]
    fn index_out_of_bounds() {
        let v = Vec3::with_values(2.0, 4.0, 5.0);
        let x = v[5];
    }

    #[test]
    fn dot() {
        let u = Vec3::with_values(2.0, 4.0, 5.0);
        let v = Vec3::with_values(1.0, 5.0, 2.0);
        assert_eq!(u.dot(v), 32.);
    }

    #[test]
    fn cross() {
        let u = Vec3::with_values(2.0, 4.0, 5.0);
        let v = Vec3::with_values(1.0, 5.0, 2.0);
        assert_eq!(u.cross(&v), Vec3::with_values(-17., 1., 6.));
    }

    #[test]
    fn unit() {
        let u: Vec3<f64> = Vec3::with_values(6.0, 4.0, 5.0);
        let a = u.unit_vec();
        let n = (77f64).sqrt();
        assert_eq!(a, Vec3::with_values(6.0 / n, 4.0 / n, 5.0 / n));
    }

    #[test]
    fn self_dot_equals_length_squared() {
        let u: Vec3<f64> = Vec3::with_values(6.0, 4.0, 5.0);
        assert_eq!(u.dot(u), u.length_squared());
    }
}
