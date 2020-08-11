use rand::{CryptoRng, RngCore};

// #[derive(Clone, Debug)]
// pub struct Poly32 {
//     coeff: [u32; 32],
//     degree: usize,
//     modulus: u32,
// }

pub trait PolyArith {
    const DEGREE: usize = 0;
    const MODULUS: i64 = 0;

    // arith
    fn add(a: &Self, b: &Self) -> Self;
    fn add_assign(&mut self, b: &Self)
    where
        Self: std::marker::Sized,
    {
        *self = Self::add(self, b);
    }

    fn sub(a: &Self, b: &Self) -> Self;
    fn sub_assign(&mut self, b: &Self)
    where
        Self: std::marker::Sized,
    {
        *self = Self::sub(self, b);
    }

    fn mul(a: &Self, b: &Self) -> Self;
    fn mul_assign(&mut self, b: &Self)
    where
        Self: std::marker::Sized,
    {
        *self = Self::mul(self, b);
    }

    // lift the coefficients to [0, q-1)
    fn normalized(&mut self);

    // lift the coefficients to [-q/2, q/2)
    fn centered(&mut self);

    // assign
    fn zero() -> Self;

    // random polynomials modulo Q
    fn rand_mod_q<R: RngCore + CryptoRng + ?Sized>(rng: &mut R) -> Self;

    // random polynomials modulus beta
    fn rand_mod_beta<R: RngCore + CryptoRng + ?Sized>(rng: &mut R) -> Self;

    fn rand_trinary<R: RngCore + CryptoRng + ?Sized>(rng: &mut R) -> Self;
}
