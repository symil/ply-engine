use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Rem, RemAssign, Sub, SubAssign};

use crate::layout::CornerRadius;

#[derive(Debug, Clone, Copy, PartialEq, Default)]
#[repr(C)]
pub struct Vector2 {
    pub x: f32,
    pub y: f32,
}

impl Vector2 {
    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }

    pub fn is_zero(&self) -> bool {
        self.x == 0. && self.y == 0.
    }
}

impl From<(f32, f32)> for Vector2 {
    fn from(value: (f32, f32)) -> Self {
        Self::new(value.0, value.1)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Default)]
#[repr(C)]
pub struct Dimensions {
    pub width: f32,
    pub height: f32,
}

impl Dimensions {
    pub fn new(width: f32, height: f32) -> Self {
        Self { width, height }
    }
}

impl From<(f32, f32)> for Dimensions {
    fn from(value: (f32, f32)) -> Self {
        Self::new(value.0, value.1)
    }
}

/// An axis-aligned rectangle defined by its top-left position and dimensions.
#[derive(Debug, Clone, Copy, PartialEq, Default)]
#[repr(C)]
pub struct BoundingBox {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
}

impl BoundingBox {
    pub fn new(x: f32, y: f32, width: f32, height: f32) -> Self {
        Self {
            x,
            y,
            width,
            height,
        }
    }
}

/// Classifies a rotation angle into common fast-path categories.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum AngleType {
    /// 0° (or 360°) — no rotation needed.
    Zero,
    /// 90° clockwise.
    Right90,
    /// 180°.
    Straight180,
    /// 270° clockwise (= 90° counter-clockwise).
    Right270,
    /// An angle that doesn't match any fast-path.
    Arbitrary(f32),
}

/// Classifies a rotation in radians into an [`AngleType`].
/// Normalises to `[0, 2π)` first, then checks within `EPS` of each cardinal.
pub fn classify_angle(radians: f32) -> AngleType {
    let normalized = radians.rem_euclid(std::f32::consts::TAU);
    const EPS: f32 = 0.001;
    if normalized < EPS || (std::f32::consts::TAU - normalized) < EPS {
        AngleType::Zero
    } else if (normalized - std::f32::consts::FRAC_PI_2).abs() < EPS {
        AngleType::Right90
    } else if (normalized - std::f32::consts::PI).abs() < EPS {
        AngleType::Straight180
    } else if (normalized - 3.0 * std::f32::consts::FRAC_PI_2).abs() < EPS {
        AngleType::Right270
    } else {
        AngleType::Arbitrary(normalized)
    }
}

/// Computes the axis-aligned bounding box of a rounded rectangle after rotation.
///
/// Uses the Minkowski-sum approach for equal corner radii:
///   `AABB_w = |(w-2r)·cosθ| + |(h-2r)·sinθ| + 2r`
///   `AABB_h = |(w-2r)·sinθ| + |(h-2r)·cosθ| + 2r`
///
/// For non-uniform radii, uses the maximum radius as a conservative approximation.
/// Returns `(effective_width, effective_height)`.
pub fn compute_rotated_aabb(
    width: f32,
    height: f32,
    corner_radius: &CornerRadius,
    rotation_radians: f32,
) -> (f32, f32) {
    let angle = classify_angle(rotation_radians);
    match angle {
        AngleType::Zero => (width, height),
        AngleType::Straight180 => (width, height),
        AngleType::Right90 | AngleType::Right270 => (height, width),
        AngleType::Arbitrary(theta) => {
            let r = corner_radius
                .top_left
                .max(corner_radius.top_right)
                .max(corner_radius.bottom_left)
                .max(corner_radius.bottom_right)
                .min(width / 2.0)
                .min(height / 2.0);

            let cos_t = theta.cos().abs();
            let sin_t = theta.sin().abs();
            let inner_w = (width - 2.0 * r).max(0.0);
            let inner_h = (height - 2.0 * r).max(0.0);

            let eff_w = inner_w * cos_t + inner_h * sin_t + 2.0 * r;
            let eff_h = inner_w * sin_t + inner_h * cos_t + 2.0 * r;
            (eff_w, eff_h)
        }
    }
}

impl Div for Vector2 {
    type Output = Self;
    #[inline]
    fn div(self, rhs: Self) -> Self {
        Self {
            x: self.x.div(rhs.x),
            y: self.y.div(rhs.y),
        }
    }
}

impl Div<&Self> for Vector2 {
    type Output = Self;
    #[inline]
    fn div(self, rhs: &Self) -> Self {
        self.div(*rhs)
    }
}

impl Div<&Vector2> for &Vector2 {
    type Output = Vector2;
    #[inline]
    fn div(self, rhs: &Vector2) -> Vector2 {
        (*self).div(*rhs)
    }
}

impl Div<Vector2> for &Vector2 {
    type Output = Vector2;
    #[inline]
    fn div(self, rhs: Vector2) -> Vector2 {
        (*self).div(rhs)
    }
}

impl DivAssign for Vector2 {
    #[inline]
    fn div_assign(&mut self, rhs: Self) {
        self.x.div_assign(rhs.x);
        self.y.div_assign(rhs.y);
    }
}

impl DivAssign<&Self> for Vector2 {
    #[inline]
    fn div_assign(&mut self, rhs: &Self) {
        self.div_assign(*rhs);
    }
}

impl Div<f32> for Vector2 {
    type Output = Self;
    #[inline]
    fn div(self, rhs: f32) -> Self {
        Self {
            x: self.x.div(rhs),
            y: self.y.div(rhs),
        }
    }
}

impl Div<&f32> for Vector2 {
    type Output = Self;
    #[inline]
    fn div(self, rhs: &f32) -> Self {
        self.div(*rhs)
    }
}

impl Div<&f32> for &Vector2 {
    type Output = Vector2;
    #[inline]
    fn div(self, rhs: &f32) -> Vector2 {
        (*self).div(*rhs)
    }
}

impl Div<f32> for &Vector2 {
    type Output = Vector2;
    #[inline]
    fn div(self, rhs: f32) -> Vector2 {
        (*self).div(rhs)
    }
}

impl DivAssign<f32> for Vector2 {
    #[inline]
    fn div_assign(&mut self, rhs: f32) {
        self.x.div_assign(rhs);
        self.y.div_assign(rhs);
    }
}

impl DivAssign<&f32> for Vector2 {
    #[inline]
    fn div_assign(&mut self, rhs: &f32) {
        self.div_assign(*rhs);
    }
}

impl Div<Vector2> for f32 {
    type Output = Vector2;
    #[inline]
    fn div(self, rhs: Vector2) -> Vector2 {
        Vector2 {
            x: self.div(rhs.x),
            y: self.div(rhs.y),
        }
    }
}

impl Div<&Vector2> for f32 {
    type Output = Vector2;
    #[inline]
    fn div(self, rhs: &Vector2) -> Vector2 {
        self.div(*rhs)
    }
}

impl Div<&Vector2> for &f32 {
    type Output = Vector2;
    #[inline]
    fn div(self, rhs: &Vector2) -> Vector2 {
        (*self).div(*rhs)
    }
}

impl Div<Vector2> for &f32 {
    type Output = Vector2;
    #[inline]
    fn div(self, rhs: Vector2) -> Vector2 {
        (*self).div(rhs)
    }
}

impl Mul for Vector2 {
    type Output = Self;
    #[inline]
    fn mul(self, rhs: Self) -> Self {
        Self {
            x: self.x.mul(rhs.x),
            y: self.y.mul(rhs.y),
        }
    }
}

impl Mul<&Self> for Vector2 {
    type Output = Self;
    #[inline]
    fn mul(self, rhs: &Self) -> Self {
        self.mul(*rhs)
    }
}

impl Mul<&Vector2> for &Vector2 {
    type Output = Vector2;
    #[inline]
    fn mul(self, rhs: &Vector2) -> Vector2 {
        (*self).mul(*rhs)
    }
}

impl Mul<Vector2> for &Vector2 {
    type Output = Vector2;
    #[inline]
    fn mul(self, rhs: Vector2) -> Vector2 {
        (*self).mul(rhs)
    }
}

impl MulAssign for Vector2 {
    #[inline]
    fn mul_assign(&mut self, rhs: Self) {
        self.x.mul_assign(rhs.x);
        self.y.mul_assign(rhs.y);
    }
}

impl MulAssign<&Self> for Vector2 {
    #[inline]
    fn mul_assign(&mut self, rhs: &Self) {
        self.mul_assign(*rhs);
    }
}

impl Mul<f32> for Vector2 {
    type Output = Self;
    #[inline]
    fn mul(self, rhs: f32) -> Self {
        Self {
            x: self.x.mul(rhs),
            y: self.y.mul(rhs),
        }
    }
}

impl Mul<&f32> for Vector2 {
    type Output = Self;
    #[inline]
    fn mul(self, rhs: &f32) -> Self {
        self.mul(*rhs)
    }
}

impl Mul<&f32> for &Vector2 {
    type Output = Vector2;
    #[inline]
    fn mul(self, rhs: &f32) -> Vector2 {
        (*self).mul(*rhs)
    }
}

impl Mul<f32> for &Vector2 {
    type Output = Vector2;
    #[inline]
    fn mul(self, rhs: f32) -> Vector2 {
        (*self).mul(rhs)
    }
}

impl MulAssign<f32> for Vector2 {
    #[inline]
    fn mul_assign(&mut self, rhs: f32) {
        self.x.mul_assign(rhs);
        self.y.mul_assign(rhs);
    }
}

impl MulAssign<&f32> for Vector2 {
    #[inline]
    fn mul_assign(&mut self, rhs: &f32) {
        self.mul_assign(*rhs);
    }
}

impl Mul<Vector2> for f32 {
    type Output = Vector2;
    #[inline]
    fn mul(self, rhs: Vector2) -> Vector2 {
        Vector2 {
            x: self.mul(rhs.x),
            y: self.mul(rhs.y),
        }
    }
}

impl Mul<&Vector2> for f32 {
    type Output = Vector2;
    #[inline]
    fn mul(self, rhs: &Vector2) -> Vector2 {
        self.mul(*rhs)
    }
}

impl Mul<&Vector2> for &f32 {
    type Output = Vector2;
    #[inline]
    fn mul(self, rhs: &Vector2) -> Vector2 {
        (*self).mul(*rhs)
    }
}

impl Mul<Vector2> for &f32 {
    type Output = Vector2;
    #[inline]
    fn mul(self, rhs: Vector2) -> Vector2 {
        (*self).mul(rhs)
    }
}

impl Add for Vector2 {
    type Output = Self;
    #[inline]
    fn add(self, rhs: Self) -> Self {
        Self {
            x: self.x.add(rhs.x),
            y: self.y.add(rhs.y),
        }
    }
}

impl Add<&Self> for Vector2 {
    type Output = Self;
    #[inline]
    fn add(self, rhs: &Self) -> Self {
        self.add(*rhs)
    }
}

impl Add<&Vector2> for &Vector2 {
    type Output = Vector2;
    #[inline]
    fn add(self, rhs: &Vector2) -> Vector2 {
        (*self).add(*rhs)
    }
}

impl Add<Vector2> for &Vector2 {
    type Output = Vector2;
    #[inline]
    fn add(self, rhs: Vector2) -> Vector2 {
        (*self).add(rhs)
    }
}

impl AddAssign for Vector2 {
    #[inline]
    fn add_assign(&mut self, rhs: Self) {
        self.x.add_assign(rhs.x);
        self.y.add_assign(rhs.y);
    }
}

impl AddAssign<&Self> for Vector2 {
    #[inline]
    fn add_assign(&mut self, rhs: &Self) {
        self.add_assign(*rhs);
    }
}

impl Add<f32> for Vector2 {
    type Output = Self;
    #[inline]
    fn add(self, rhs: f32) -> Self {
        Self {
            x: self.x.add(rhs),
            y: self.y.add(rhs),
        }
    }
}

impl Add<&f32> for Vector2 {
    type Output = Self;
    #[inline]
    fn add(self, rhs: &f32) -> Self {
        self.add(*rhs)
    }
}

impl Add<&f32> for &Vector2 {
    type Output = Vector2;
    #[inline]
    fn add(self, rhs: &f32) -> Vector2 {
        (*self).add(*rhs)
    }
}

impl Add<f32> for &Vector2 {
    type Output = Vector2;
    #[inline]
    fn add(self, rhs: f32) -> Vector2 {
        (*self).add(rhs)
    }
}

impl AddAssign<f32> for Vector2 {
    #[inline]
    fn add_assign(&mut self, rhs: f32) {
        self.x.add_assign(rhs);
        self.y.add_assign(rhs);
    }
}

impl AddAssign<&f32> for Vector2 {
    #[inline]
    fn add_assign(&mut self, rhs: &f32) {
        self.add_assign(*rhs);
    }
}

impl Add<Vector2> for f32 {
    type Output = Vector2;
    #[inline]
    fn add(self, rhs: Vector2) -> Vector2 {
        Vector2 {
            x: self.add(rhs.x),
            y: self.add(rhs.y),
        }
    }
}

impl Add<&Vector2> for f32 {
    type Output = Vector2;
    #[inline]
    fn add(self, rhs: &Vector2) -> Vector2 {
        self.add(*rhs)
    }
}

impl Add<&Vector2> for &f32 {
    type Output = Vector2;
    #[inline]
    fn add(self, rhs: &Vector2) -> Vector2 {
        (*self).add(*rhs)
    }
}

impl Add<Vector2> for &f32 {
    type Output = Vector2;
    #[inline]
    fn add(self, rhs: Vector2) -> Vector2 {
        (*self).add(rhs)
    }
}

impl Sub for Vector2 {
    type Output = Self;
    #[inline]
    fn sub(self, rhs: Self) -> Self {
        Self {
            x: self.x.sub(rhs.x),
            y: self.y.sub(rhs.y),
        }
    }
}

impl Sub<&Self> for Vector2 {
    type Output = Self;
    #[inline]
    fn sub(self, rhs: &Self) -> Self {
        self.sub(*rhs)
    }
}

impl Sub<&Vector2> for &Vector2 {
    type Output = Vector2;
    #[inline]
    fn sub(self, rhs: &Vector2) -> Vector2 {
        (*self).sub(*rhs)
    }
}

impl Sub<Vector2> for &Vector2 {
    type Output = Vector2;
    #[inline]
    fn sub(self, rhs: Vector2) -> Vector2 {
        (*self).sub(rhs)
    }
}

impl SubAssign for Vector2 {
    #[inline]
    fn sub_assign(&mut self, rhs: Self) {
        self.x.sub_assign(rhs.x);
        self.y.sub_assign(rhs.y);
    }
}

impl SubAssign<&Self> for Vector2 {
    #[inline]
    fn sub_assign(&mut self, rhs: &Self) {
        self.sub_assign(*rhs);
    }
}

impl Sub<f32> for Vector2 {
    type Output = Self;
    #[inline]
    fn sub(self, rhs: f32) -> Self {
        Self {
            x: self.x.sub(rhs),
            y: self.y.sub(rhs),
        }
    }
}

impl Sub<&f32> for Vector2 {
    type Output = Self;
    #[inline]
    fn sub(self, rhs: &f32) -> Self {
        self.sub(*rhs)
    }
}

impl Sub<&f32> for &Vector2 {
    type Output = Vector2;
    #[inline]
    fn sub(self, rhs: &f32) -> Vector2 {
        (*self).sub(*rhs)
    }
}

impl Sub<f32> for &Vector2 {
    type Output = Vector2;
    #[inline]
    fn sub(self, rhs: f32) -> Vector2 {
        (*self).sub(rhs)
    }
}

impl SubAssign<f32> for Vector2 {
    #[inline]
    fn sub_assign(&mut self, rhs: f32) {
        self.x.sub_assign(rhs);
        self.y.sub_assign(rhs);
    }
}

impl SubAssign<&f32> for Vector2 {
    #[inline]
    fn sub_assign(&mut self, rhs: &f32) {
        self.sub_assign(*rhs);
    }
}

impl Sub<Vector2> for f32 {
    type Output = Vector2;
    #[inline]
    fn sub(self, rhs: Vector2) -> Vector2 {
        Vector2 {
            x: self.sub(rhs.x),
            y: self.sub(rhs.y),
        }
    }
}

impl Sub<&Vector2> for f32 {
    type Output = Vector2;
    #[inline]
    fn sub(self, rhs: &Vector2) -> Vector2 {
        self.sub(*rhs)
    }
}

impl Sub<&Vector2> for &f32 {
    type Output = Vector2;
    #[inline]
    fn sub(self, rhs: &Vector2) -> Vector2 {
        (*self).sub(*rhs)
    }
}

impl Sub<Vector2> for &f32 {
    type Output = Vector2;
    #[inline]
    fn sub(self, rhs: Vector2) -> Vector2 {
        (*self).sub(rhs)
    }
}

impl Rem for Vector2 {
    type Output = Self;
    #[inline]
    fn rem(self, rhs: Self) -> Self {
        Self {
            x: self.x.rem(rhs.x),
            y: self.y.rem(rhs.y),
        }
    }
}

impl Rem<&Self> for Vector2 {
    type Output = Self;
    #[inline]
    fn rem(self, rhs: &Self) -> Self {
        self.rem(*rhs)
    }
}

impl Rem<&Vector2> for &Vector2 {
    type Output = Vector2;
    #[inline]
    fn rem(self, rhs: &Vector2) -> Vector2 {
        (*self).rem(*rhs)
    }
}

impl Rem<Vector2> for &Vector2 {
    type Output = Vector2;
    #[inline]
    fn rem(self, rhs: Vector2) -> Vector2 {
        (*self).rem(rhs)
    }
}

impl RemAssign for Vector2 {
    #[inline]
    fn rem_assign(&mut self, rhs: Self) {
        self.x.rem_assign(rhs.x);
        self.y.rem_assign(rhs.y);
    }
}

impl RemAssign<&Self> for Vector2 {
    #[inline]
    fn rem_assign(&mut self, rhs: &Self) {
        self.rem_assign(*rhs);
    }
}

impl Rem<f32> for Vector2 {
    type Output = Self;
    #[inline]
    fn rem(self, rhs: f32) -> Self {
        Self {
            x: self.x.rem(rhs),
            y: self.y.rem(rhs),
        }
    }
}

impl Rem<&f32> for Vector2 {
    type Output = Self;
    #[inline]
    fn rem(self, rhs: &f32) -> Self {
        self.rem(*rhs)
    }
}

impl Rem<&f32> for &Vector2 {
    type Output = Vector2;
    #[inline]
    fn rem(self, rhs: &f32) -> Vector2 {
        (*self).rem(*rhs)
    }
}

impl Rem<f32> for &Vector2 {
    type Output = Vector2;
    #[inline]
    fn rem(self, rhs: f32) -> Vector2 {
        (*self).rem(rhs)
    }
}

impl RemAssign<f32> for Vector2 {
    #[inline]
    fn rem_assign(&mut self, rhs: f32) {
        self.x.rem_assign(rhs);
        self.y.rem_assign(rhs);
    }
}

impl RemAssign<&f32> for Vector2 {
    #[inline]
    fn rem_assign(&mut self, rhs: &f32) {
        self.rem_assign(*rhs);
    }
}

impl Rem<Vector2> for f32 {
    type Output = Vector2;
    #[inline]
    fn rem(self, rhs: Vector2) -> Vector2 {
        Vector2 {
            x: self.rem(rhs.x),
            y: self.rem(rhs.y),
        }
    }
}

impl Rem<&Vector2> for f32 {
    type Output = Vector2;
    #[inline]
    fn rem(self, rhs: &Vector2) -> Vector2 {
        self.rem(*rhs)
    }
}

impl Rem<&Vector2> for &f32 {
    type Output = Vector2;
    #[inline]
    fn rem(self, rhs: &Vector2) -> Vector2 {
        (*self).rem(*rhs)
    }
}

impl Rem<Vector2> for &f32 {
    type Output = Vector2;
    #[inline]
    fn rem(self, rhs: Vector2) -> Vector2 {
        (*self).rem(rhs)
    }
}

impl AsRef<[f32; 2]> for Vector2 {
    #[inline]
    fn as_ref(&self) -> &[f32; 2] {
        unsafe { &*(self as *const Self as *const [f32; 2]) }
    }
}

impl AsMut<[f32; 2]> for Vector2 {
    #[inline]
    fn as_mut(&mut self) -> &mut [f32; 2] {
        unsafe { &mut *(self as *mut Self as *mut [f32; 2]) }
    }
}

impl Neg for Vector2 {
    type Output = Self;
    #[inline]
    fn neg(self) -> Self {
        Self {
            x: self.x.neg(),
            y: self.y.neg(),
        }
    }
}

impl Neg for &Vector2 {
    type Output = Vector2;
    #[inline]
    fn neg(self) -> Vector2 {
        (*self).neg()
    }
}