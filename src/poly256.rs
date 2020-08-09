use crate::param::{BETA, BETA_M2_P1, BETA_RS_RANGE};
use crate::param::{Q, Q_RS_RANGE};
use crate::poly::PolyArith;
use rand::{CryptoRng, RngCore};
use std::fmt;
#[derive(Clone, Copy)]
pub struct Poly256 {
    coeff: [i64; 256],
}

impl PolyArith for Poly256 {
    const DEGREE: usize = 256;
    const MODULUS: i64 = Q;

    fn add(a: &Self, b: &Self) -> Self {
        let mut res = [0i64; Self::DEGREE];
        for (i, e) in res.iter_mut().enumerate() {
            *e = (a.coeff[i] + b.coeff[i]) % Self::MODULUS;
        }
        Poly256 { coeff: res }
    }

    fn sub(a: &Self, b: &Self) -> Self {
        let mut res = [0i64; 256];
        for (i, e) in res.iter_mut().enumerate() {
            *e = (a.coeff[i] + (Self::MODULUS << 1) - b.coeff[i]) % Self::MODULUS;
        }
        Poly256 { coeff: res }
    }

    fn mul(a: &Self, b: &Self) -> Self {
        // todo: implement NTT
        school_book(a, b)
    }

    // assign
    fn zero() -> Self {
        Poly256 {
            coeff: [0i64; Self::DEGREE],
        }
    }

    // random polynomials modulo Q
    fn rand_mod_q<R: RngCore + CryptoRng + ?Sized>(rng: &mut R) -> Self {
        let mut coeff = [0i64; Self::DEGREE];
        for e in &mut coeff.iter_mut() {
            let mut tmp = rng.next_u32();
            while tmp > Q_RS_RANGE {
                tmp = rng.next_u32();
            }
            *e = (tmp % Self::MODULUS as u32) as i64;
        }
        Poly256 { coeff }
    }

    // random polynomials modulus beta
    fn rand_mod_beta<R: RngCore + CryptoRng + ?Sized>(rng: &mut R) -> Self {
        let mut coeff = [0i64; Self::DEGREE];
        for e in &mut coeff.iter_mut() {
            let mut tmp = rng.next_u32();
            while tmp > BETA_RS_RANGE {
                tmp = rng.next_u32();
            }
            tmp %= BETA_M2_P1;
            *e = (tmp as i32 - (BETA as i32)) as i64;
        }
        Poly256 { coeff }
    }

    fn rand_trinary<R: RngCore + CryptoRng + ?Sized>(rng: &mut R) -> Self {
        let mut coeff = [0i64; Self::DEGREE];
        let mut tmp = rng.next_u64();
        let mut ct = 0;
        let mut cur;
        for e in coeff.iter_mut() {
            loop {
                cur = tmp & 0b11;
                tmp >>= 2;
                ct += 1;
                if ct == 32 {
                    tmp = rng.next_u64();
                    ct = 0;
                }
                if cur != 3 {
                    break;
                }
            }
            *e = cur as i64 - 1;
        }
        Poly256 { coeff }
    }
}

fn school_book(a: &Poly256, b: &Poly256) -> Poly256 {
    let mut res = [0i64; Poly256::DEGREE << 1];
    let mut array = [0; Poly256::DEGREE];
    for i in 0..Poly256::DEGREE {
        for j in 0..Poly256::DEGREE {
            res[i + j] += (a.coeff[i] as i64) * (b.coeff[j] as i64);
        }
    }
    for i in 0..Poly256::DEGREE {
        array[i] = (res[i] + Q - res[i + Poly256::DEGREE]) % Q;
    }
    Poly256 { coeff: array }
}

/// convenient function to output a secret key object
impl fmt::Debug for Poly256 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for i in 0..7 {
            writeln!(f, "{:?}", self.coeff[i * 32..(i + 1) * 32].as_ref())?;
        }
        writeln!(f, "{:?}", self.coeff[7 * 32..8 * 32].as_ref())
    }
}

impl PartialEq for Poly256 {
    fn eq(&self, other: &Self) -> bool {
        self.coeff
            .iter()
            .zip(other.coeff.iter())
            .all(|(a, b)| a == b)
    }
}

#[test]
fn test_rand_mod_q() {
    let mut rng = rand::thread_rng();
    let a: Poly256 = PolyArith::rand_mod_q(&mut rng);
    for e in a.coeff.iter() {
        assert!(*e < Poly256::MODULUS, "coefficient greater than Q")
    }
}

#[test]
fn test_rand_mod_beta() {
    let mut rng = rand::thread_rng();
    let a: Poly256 = PolyArith::rand_mod_beta(&mut rng);
    for e in a.coeff.iter() {
        assert!(
            *e <= BETA as i64 && *e >= -(BETA as i64),
            "coefficient greater than beta"
        )
    }
}

#[test]
fn test_rand_trinary() {
    let mut rng = rand::thread_rng();
    let a: Poly256 = PolyArith::rand_trinary(&mut rng);
    for e in a.coeff.iter() {
        assert!(*e <= 1 && *e >= -1, "coefficient not trinary")
    }
}

#[test]
fn test_poly256_mul() {
    // todo: add a lot more test cases
    let mut rng = rand::thread_rng();

    // zero
    let a = Poly256::zero();
    let b = Poly256::rand_mod_q(&mut rng);
    let c = Poly256::mul(&a, &b);
    for e in c.coeff.iter() {
        assert!(*e == 0, "coefficient not zero")
    }

    // associative

    let a = Poly256::rand_mod_q(&mut rng);
    let b = Poly256::rand_mod_q(&mut rng);
    let c = Poly256::mul(&a, &b);
    let d = Poly256::mul(&b, &a);
    assert!(c == d, "coefficient not zero");

    // (x+1) * (x+1) = x^2 + 2x + 1
    let mut a = Poly256::zero();
    a.coeff[0] = 1;
    a.coeff[1] = 1;
    let b = Poly256::mul(&a, &a);
    assert!(b.coeff[0] == 1);
    assert!(b.coeff[1] == 2);
    assert!(b.coeff[2] == 1);
    for i in 3..Poly256::DEGREE {
        assert!(b.coeff[i] == 0)
    }
}
