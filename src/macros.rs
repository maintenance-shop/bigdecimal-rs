// \file src/macros.rs
//! macros for
/*
macro_rules! forward_val_val_binop {
    (impl $imp:ident for $res:ty, $method:ident) => {
        impl $imp<$res> for $res {
            type Output = $res;

            #[inline]
            fn $method(self, other: $res) -> $res {
                // forward to val-ref
                $imp::$method(self, &other)
            }
        }
    };
}
*/

macro_rules! forward_ref_val_binop {
    (impl $imp:ident for $res:ty, $method:ident) => {
        impl<'a> $imp<$res> for &'a $res {
            type Output = $res;

            #[inline]
            fn $method(self, other: $res) -> $res {
                // forward to ref-ref
                $imp::$method(self, &other)
            }
        }
    };
}

/*
macro_rules! forward_val_ref_binop {
    (impl $imp:ident for $res:ty, $method:ident) => {
        impl<'a> $imp<&'a $res> for $res {
            type Output = $res;

            #[inline]
            fn $method(self, other: &$res) -> $res {
                // forward to ref-ref
                $imp::$method(&self, other)
            }
        }
    };
}

// Forward everything to ref-ref, when reusing storage is not helpful
macro_rules! forward_all_binop_to_ref_ref {
    (impl $imp:ident for $res:ty, $method:ident) => {
        forward_val_val_binop!(impl $imp for $res, $method);
        forward_val_ref_binop!(impl $imp for $res, $method);
        forward_ref_val_binop!(impl $imp for $res, $method);
    };
}
*/

macro_rules! forward_val_assignop {
    (impl $imp:ident for $res:ty, $method:ident) => {
        impl $imp<$res> for $res {
            #[inline]
            fn $method(&mut self, other: $res) {
                // forward to mutref-ref
                $imp::$method(self, &other)
            }
        }
    };
}

macro_rules! impl_div_for_uint_primitive {
    // (impl $imp:ident for $res:ty, $method:ident) => {
    ($res:ty) => {
        impl<'a> Div<$res> for &'a BigDecimal {
            type Output = BigDecimal;

            #[inline]
            fn div(self, den: $res) -> Self::Output {
                if den == 1 {
                    self.clone()
                } else if den == 2 {
                    self.half()
                } else {
                    self / BigDecimal::from(den)
                }
            }
        }

        impl<'a> Div<&'a BigDecimal> for $res {
            type Output = BigDecimal;

            #[inline(always)]
            fn div(self, den: &'a BigDecimal) -> Self::Output {
                BigDecimal::from(self) / den
            }
        }

        impl Div<BigDecimal> for $res {
            type Output = BigDecimal;

            #[inline(always)]
            fn div(self, den: BigDecimal) -> Self::Output {
                BigDecimal::from(self) / den
            }
        }
    };
}

macro_rules! impl_div_for_int_primitive {
    // (impl $imp:ident for $res:ty, $method:ident) => {
    ($res:ty) => {
        impl<'a> Div<$res> for BigDecimal {
            type Output = BigDecimal;

            #[inline(always)]
            fn div(self, den: $res) -> Self::Output {
                if den < 0 {
                    -Div::div(self, -den)
                } else if den == 1 {
                    self
                } else if den == 2 {
                    self.half()
                } else {
                    self / BigDecimal::from(den)
                }
            }
        }

        impl<'a> Div<$res> for &'a BigDecimal {
            type Output = BigDecimal;

            #[inline(always)]
            fn div(self, den: $res) -> Self::Output {
                if den < 0 {
                    -Div::div(self, -den)
                } else if den == 1 {
                    self.clone()
                } else if den == 2 {
                    self.half()
                } else {
                    self / BigDecimal::from(den)
                }
            }
        }

        impl<'a> Div<&'a BigDecimal> for $res {
            type Output = BigDecimal;

            #[inline(always)]
            fn div(self, den: &'a BigDecimal) -> Self::Output {
                match (self < 0, den.is_negative()) {
                    (true, true) => -self / -den,
                    (true, false) => (-self / den).neg(),
                    (false, true) => (-self / den.abs()),
                    (false, false) => BigDecimal::from(self) / den,
                }
            }
        }

        impl Div<BigDecimal> for $res {
            type Output = BigDecimal;

            #[inline(always)]
            fn div(self, den: BigDecimal) -> Self::Output {
                match (self < 0, den.is_negative()) {
                    (true, true) => -self / -den,
                    (true, false) => (-self / den).neg(),
                    (false, true) => (-self / den.abs()),
                    (false, false) => BigDecimal::from(self) / den,
                }
            }
        }
    };
}

macro_rules! impl_div_for_float_primitive {
    // (impl $imp:ident for $res:ty, $method:ident) => {
    ($res:ty) => {
        impl<'a> Div<$res> for &'a BigDecimal {
            type Output = BigDecimal;

            #[inline]
            #[allow(clippy::float_cmp)]
            fn div(self, den: $res) -> Self::Output {
                if den.is_nan() {
                    BigDecimal::zero()
                } else if den == 1.0 {
                    self.clone()
                } else if den == 0.5 {
                    self.double()
                } else if den == 2.0 {
                    self.half()
                } else if den == -1.0 {
                    -self
                } else if den < 0.0 && self.is_positive() {
                    -(self / -den)
                } else {
                    // Unwrap is safe, because `is_nan` checked above
                    self / BigDecimal::try_from(den).unwrap()
                }
            }
        }

        impl<'a> Div<&'a BigDecimal> for $res {
            type Output = BigDecimal;
            #[inline(always)]
            fn div(self, den: &'a BigDecimal) -> Self::Output {
                if self.is_nan() {
                    BigDecimal::zero()
                } else {
                    BigDecimal::try_from(self).unwrap() / den
                }
            }
        }

        impl Div<BigDecimal> for $res {
            type Output = BigDecimal;
            #[inline(always)]
            fn div(self, den: BigDecimal) -> Self::Output {
                if self.is_nan() {
                    BigDecimal::zero()
                } else {
                    BigDecimal::try_from(self).unwrap() / den
                }
            }
        }
    };
}

macro_rules! forward_primitive_types {
    (floats) => {
        crate::macros::impl_div_for_float_primitive!(f32);
        crate::macros::impl_div_for_float_primitive!(f64);
    };
    (ints) => {
        crate::macros::impl_div_for_int_primitive!(i8);
        crate::macros::impl_div_for_int_primitive!(i16);
        crate::macros::impl_div_for_int_primitive!(i32);
        crate::macros::impl_div_for_int_primitive!(i64);
    };
    (uints) => {
        crate::macros::impl_div_for_uint_primitive!(u8);
        crate::macros::impl_div_for_uint_primitive!(u16);
        crate::macros::impl_div_for_uint_primitive!(u32);
        crate::macros::impl_div_for_uint_primitive!(u64);
    };
}

pub(crate) use forward_primitive_types;
pub(crate) use forward_ref_val_binop;
pub(crate) use forward_val_assignop;
pub(crate) use impl_div_for_float_primitive;
pub(crate) use impl_div_for_int_primitive;
pub(crate) use impl_div_for_uint_primitive;
