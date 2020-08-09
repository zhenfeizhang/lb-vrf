use crate::param::Q;
use crate::poly::PolyArith;
use rand::Rng;
#[derive(Clone)]
pub struct Poly256 {
    coeff: [u32; 256],
}

impl PolyArith for Poly256 {
    const DEGREE: usize = 256;
    const MODULUS: u32 = Q;

    fn add(a: &Self, b: &Self) -> Self {
        let mut res = [0u32; Self::DEGREE];
        for i in 0..Self::DEGREE {
            res[i] = (a.coeff[i] + b.coeff[i]) % Self::MODULUS;
        }
        Poly256 { coeff: res }
    }

    fn sub(a: &Self, b: &Self) -> Self {
        let mut res = [0u32; 256];
        for i in 0..Self::DEGREE {
            res[i] = (a.coeff[i] + (Self::MODULUS << 1) - b.coeff[i]) % Self::MODULUS;
        }
        Poly256 { coeff: res }
    }

    fn mul(_a: &Self, _b: &Self) -> Self {
        todo!()
    }

    // assign
    fn zero() -> Self {
        Poly256 { coeff: [0; 256] }
    }

    // random polynomials modulo Q
    fn rand_mod_q<R: Rng + ?Sized>(_rng: &mut R) -> Self {
        todo!()
    }

    // random polynomials modulus beta
    fn rand_mod_beta<R: Rng + ?Sized>(_rng: &mut R, _beta: u32) -> Self {
        todo!()
    }

    fn rand_trinary<R: Rng + ?Sized>(_rng: &mut R) -> Self {
        todo!()
    }
}
