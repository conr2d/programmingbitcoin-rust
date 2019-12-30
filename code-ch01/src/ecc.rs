extern crate num_bigint;
extern crate num_traits;

use num_bigint::{ BigInt, ToBigInt };
use num_traits::{ Zero, One };
use std::ops::{ Add, Sub, Mul, Div };
use std::fmt;

#[derive(Debug, PartialEq)]
pub struct FieldElement {
    num: BigInt,
    prime: BigInt,
}

impl FieldElement {
    pub fn new<T: ToBigInt>(num: T, prime: T) -> FieldElement {
        let n: BigInt = num.to_bigint().unwrap();
        let p: BigInt = prime.to_bigint().unwrap();

        if n >= p || n < Zero::zero() {
            panic!("Num {} not in field range 0 to {}", n, p - 1);
        }

        FieldElement { 
            num: n,
            prime: p,
        }
    }

    pub fn pow<T: ToBigInt>(&self, exponent: T) -> FieldElement {
        let e: BigInt = exponent
            .to_bigint()
            .unwrap()
            .modpow(&One::one(), &(&self.prime - 1));

        Self::new((&self.num).modpow(&e, &self.prime), self.prime.clone())
    }
}

impl fmt::Display for FieldElement {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[{}, {}]", self.num, self.prime)
    }
}

macro_rules! op_common_impl {
    (impl $imp: ident, $method: ident for $t: ty, $u: ty) => {
        impl $imp<$u> for $t {
            type Output = FieldElement;

            #[inline]
            fn $method(self, other: $u) -> Self::Output {
                if self.prime != other.prime {
                    panic!("Cannot $method two numbers in different Fields");
                }

                let n: BigInt = (&self.num)
                    .$method(&other.num)
                    .modpow(&One::one(), &self.prime);

                Self::Output::new(n, self.prime.clone())
            }
        }
    }
}

macro_rules! op_div_impl {
    (impl $imp: ident, $method: ident for $t: ty, $u: ty) => {
        impl $imp<$u> for $t {
            type Output = FieldElement;

            #[inline]
            fn $method(self, other: $u) -> Self::Output {
                self * other.pow(-1)
            }
        }
    }
}

macro_rules! op_impl {
    ($m: ident, $imp: ident, $method: ident, $t: ty, $u: ty) => {
        $m! { impl $imp, $method for $t, $u }
        $m! { impl $imp, $method for $t, &$u }
        $m! { impl $imp, $method for &$t, $u }
        $m! { impl $imp, $method for &$t, &$u }
    }
}

op_impl!(op_common_impl, Add, add, FieldElement, FieldElement);
op_impl!(op_common_impl, Sub, sub, FieldElement, FieldElement);
op_impl!(op_common_impl, Mul, mul, FieldElement, FieldElement);
op_impl!(op_div_impl, Div, div, FieldElement, FieldElement);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ne() {
        let a = FieldElement::new(2, 31);
        let b = FieldElement::new(2, 31);
        let c = FieldElement::new(15, 31);
        assert_eq!(a, b);
        assert_true!(a != c);
        assert_false!(a != b);
    }

    #[test]
    fn test_add() {
        let mut a = FieldElement::new(2, 31);
        let mut b = FieldElement::new(15, 31);
        assert_eq!(a + b, FieldElement::new(17, 31));
        a = FieldElement::new(17, 31);
        b = FieldElement::new(21, 31);
        assert_eq!(a + b, FieldElement::new(7, 31));
    }

    #[test]
    fn test_sub() {
        let mut a = FieldElement::new(29, 31);
        let mut b = FieldElement::new(4, 31);
        assert_eq!(a - b, FieldElement::new(25, 31));
        a = FieldElement::new(15, 31);
        b = FieldElement::new(30, 31);
        assert_eq!(a - b, FieldElement::new(16, 31));
    }

    #[test]
    fn test_mul() {
        let a = FieldElement::new(24, 31);
        let b = FieldElement::new(19, 31);
        assert_eq!(a * b, FieldElement::new(22, 31));
    }

    #[test]
    fn test_pow() {
        let mut a = FieldElement::new(17, 31);
        assert_eq!(a.pow(3), FieldElement::new(15, 31));
        a = FieldElement::new(5, 31);
        let b = FieldElement::new(18, 31);
        assert_eq!(a.pow(5) * b, FieldElement::new(16, 31));
    }

    #[test]
    fn test_div() {
        let mut a = FieldElement::new(3, 31);
        let mut b = FieldElement::new(24, 31);
        assert_eq!(a / b, FieldElement::new(4, 31));
        a = FieldElement::new(17, 31);
        assert_eq!(a.pow(-3), FieldElement::new(29, 31));
        a = FieldElement::new(4, 31);
        b = FieldElement::new(11, 31);
        assert_eq!(a.pow(-4) * b, FieldElement::new(13, 31));
    }
}
