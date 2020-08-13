// this file implements neccessary arithmetics over Z_p[x]/(x^32 + R)

use crate::param::{P, P_RS_RANGE, R, R_BASE};
use crate::poly::PolyArith;
use crate::poly256::Poly256;
use rand::{CryptoRng, RngCore};
use std::convert::From;
// use std::fmt;
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Poly32 {
    pub coeff: [i64; 32],
}

impl From<Poly256> for Poly32 {
    // converting a ring element over Z_q[x]/(x^256+1)
    // into a ring elemetn over Z_p[x]/(x^32+R)
    fn from(a: Poly256) -> Self {
        let mut res = [0i64; 32];
        res.copy_from_slice(&a.coeff[0..32]);

        for (i, e) in res.iter_mut().enumerate() {
            for (j, r) in R_BASE.iter().enumerate().skip(1) {
                *e += a.coeff[i + (j << 5)] * (*r);
            }
            *e %= P;
        }

        Self { coeff: res }
    }
}

impl PolyArith for Poly32 {
    const DEGREE: usize = 32;
    const MODULUS: i64 = P;

    fn add(a: &Self, b: &Self) -> Self {
        let mut res = [0i64; Self::DEGREE];
        for (i, e) in res.iter_mut().enumerate() {
            *e = (a.coeff[i] + b.coeff[i]) % Self::MODULUS;
        }
        Poly32 { coeff: res }
    }

    fn sub(a: &Self, b: &Self) -> Self {
        let mut res = [0i64; 32];
        for (i, e) in res.iter_mut().enumerate() {
            *e = (a.coeff[i] + (Self::MODULUS << 1) - b.coeff[i]) % Self::MODULUS;
        }
        Poly32 { coeff: res }
    }

    fn mul(a: &Self, b: &Self) -> Self {
        // todo: implement NTT
        school_book(a, b)
    }

    // assign
    fn zero() -> Self {
        Poly32 {
            coeff: [0i64; Self::DEGREE],
        }
    }

    fn normalized(&mut self) {
        for e in self.coeff.iter_mut() {
            (*e) = (*e + (Self::MODULUS << 1)) % Self::MODULUS;
        }
    }

    fn centered(&mut self) {
        self.normalized();
        for e in self.coeff.iter_mut() {
            if *e << 1 > Self::MODULUS {
                *e -= Self::MODULUS;
            }
        }
    }
    // random polynomials modulo Q
    fn uniform_random<R: RngCore + CryptoRng + ?Sized>(rng: &mut R) -> Self {
        let mut coeff = [0i64; Self::DEGREE];
        for e in &mut coeff.iter_mut() {
            let mut tmp = rng.next_u32();
            while tmp > P_RS_RANGE {
                tmp = rng.next_u32();
            }
            *e = (tmp % Self::MODULUS as u32) as i64;
        }
        Poly32 { coeff }
    }

    // random polynomials modulus beta
    fn rand_mod_beta<R: RngCore + CryptoRng + ?Sized>(_rng: &mut R) -> Self {
        // we will never need to sample a uniform mod beta element
        // over this ring
        unimplemented!();
    }

    fn rand_trinary<R: RngCore + CryptoRng + ?Sized>(_rng: &mut R) -> Self {
        // we will never need to sample a trinary element
        // over this ring
        unimplemented!();
    }
}

pub(crate) fn poly32_inner_product(a: &[Poly32], b: &[Poly32]) -> Poly32 {
    if a.len() != b.len() {
        panic!("inner product: length do not match");
    }
    let mut res = Poly32::zero();
    for i in 0..a.len() {
        res.add_assign(&Poly32::mul(&a[i], &b[i]));
    }
    res.normalized();
    res
}

fn school_book(a: &Poly32, b: &Poly32) -> Poly32 {
    let mut res = [0i64; Poly32::DEGREE << 1];
    let mut array = [0; Poly32::DEGREE];
    for i in 0..Poly32::DEGREE {
        for j in 0..Poly32::DEGREE {
            res[i + j] += (a.coeff[i] as i64) * (b.coeff[j] as i64);
        }
    }

    for i in 0..Poly32::DEGREE {
        array[i] = (res[i] + (P << 2) - R * (res[i + Poly32::DEGREE] % P)) % P;
    }
    Poly32 { coeff: array }
}
