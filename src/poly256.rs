// this file implements neccessary arithmetics over Z_q[x]/(x^256 + 1)

use crate::param::{BETA, BETA_M2_P1, BETA_RS_RANGE};
use crate::param::{Q, Q_RS_RANGE};
use crate::poly::PolyArith;
use rand::{CryptoRng, RngCore};
use std::fmt;
#[derive(Clone, Copy)]
pub struct Poly256 {
    pub coeff: [i64; 256],
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

        // the following code uses karatsuba -- it is somehow slower than school_book
        // let mut c = [0i64; Self::DEGREE << 1];
        // karatsuba(&a.coeff, &b.coeff, &mut c, Self::DEGREE);
        // let mut res = [0i64; Self::DEGREE];
        // res.copy_from_slice(
        //     &(0..Self::DEGREE)
        //         .map(|i| c[i] - c[Self::DEGREE + i])
        //         .collect::<Vec<i64>>(),
        // );
        // let mut rt = Self { coeff: res };
        // rt.normalized();
        // rt
    }

    // assign
    fn zero() -> Self {
        Poly256 {
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

pub(crate) fn poly256_inner_product(a: &[Poly256], b: &[Poly256]) -> Poly256 {
    if a.len() != b.len() {
        panic!("inner product: length do not match");
    }
    let mut res = Poly256::zero();
    for i in 0..a.len() {
        res.add_assign(&Poly256::mul(&a[i], &b[i]));
    }
    res.normalized();
    res
}
#[allow(dead_code)]
pub(crate) fn school_book(a: &Poly256, b: &Poly256) -> Poly256 {
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
#[allow(dead_code)]
pub(crate) fn karatsuba(a: &[i64], b: &[i64], c: &mut [i64], n: usize) {
    if n <= 32 {
        school_book_without_reduction(a, b, c, n);
        return;
    }

    let size = n / 2;
    // f(0) = a0 * b0
    let mut zero = vec![0i64; n];
    // f(1) = (a0 + a1)(b0 + b1)
    let mut one = vec![0i64; n];
    // f(infty) = a1 * b1
    let mut infinity = vec![0i64; n];
    karatsuba(&a[0..size], &b[0..size], &mut zero, size);
    karatsuba(&a[size..n], &b[size..n], &mut infinity, size);
    let a1a2: Vec<i64> = (0..size).map(|i| a[i] + a[i + size]).collect();
    let b1b2: Vec<i64> = (0..size).map(|i| b[i] + b[i + size]).collect();
    karatsuba(a1a2.as_ref(), b1b2.as_ref(), &mut one, size);

    // a0b1 + a1b0 = f(1) - f(0) - f(infty)
    let a0b1_p_a1b0: Vec<i64> = (0..n).map(|i| one[i] - zero[i] - infinity[i]).collect();
    // c = a0b0 + (a0b1 + a1b0)x + a1b1 x^2
    for i in 0..size {
        c[i] = zero[i] % Q;
        c[i + size] = (zero[i + size] + a0b1_p_a1b0[i]) % Q;
        c[i + n] = (a0b1_p_a1b0[i + size] + infinity[i]) % Q;
        c[i + size * 3] = infinity[i + size] % Q;
    }
}
#[allow(dead_code)]
pub(crate) fn school_book_without_reduction(a: &[i64], b: &[i64], c: &mut [i64], n: usize) {
    for i in 0..n {
        for j in 0..n {
            c[i + j] += a[i] * b[j];
        }
    }
}

#[test]
fn test_karatsuba() {
    let mut rng = rand::thread_rng();
    let mut a = [0i64; 64];
    for e in a.iter_mut() {
        *e = rng.next_u32() as i64 % Q;
    }
    let mut b = [0i64; 64];
    for e in b.iter_mut() {
        *e = rng.next_u32() as i64 % Q;
    }
    let mut c1 = [0i64; 128];
    let mut c2 = c1.clone();

    karatsuba(&a, &b, &mut c1, 64);
    school_book_without_reduction(&a, &b, &mut c2, 64);
    println!("{:?}", &c1[0..32]);
    for (i, e) in c1.iter().enumerate() {
        assert_eq!(*e % Q, c2[i] % Q)
    }
    // assert!(false);
}
