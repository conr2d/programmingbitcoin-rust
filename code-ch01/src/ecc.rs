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
    pub fn new<T:ToBigInt>(num: T, prime: T) -> FieldElement {
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

macro_rules! div_impl {
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
op_impl!(div_impl, Div, div, FieldElement, FieldElement);
