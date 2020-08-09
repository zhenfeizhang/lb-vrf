use rand::Rng;
// #[derive(Clone, Debug)]
// pub struct Poly32 {
//     coeff: [u32; 32],
//     degree: usize,
//     modulus: u32,
// }

pub trait PolyArith {
    const DEGREE: usize = 0;
    const MODULUS: u32 = 0;

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

    // assign
    fn zero() -> Self;

    // random polynomials modulo Q
    fn rand_mod_q<R: Rng + ?Sized>(rng: &mut R) -> Self;

    // random polynomials modulus beta
    fn rand_mod_beta<R: Rng + ?Sized>(rng: &mut R, beta: u32) -> Self;

    fn rand_trinary<R: Rng + ?Sized>(rng: &mut R) -> Self;
}
