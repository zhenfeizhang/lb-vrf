use crate::poly::PolyArith;
use crate::poly256::Poly256;

#[derive(PartialEq, Clone, Copy, Debug)]
pub struct PublicKey {
    pub(crate) t: [Poly256; 4],
}

#[derive(PartialEq, Clone, Copy, Debug)]
pub struct SecretKey {
    pub(crate) s: [Poly256; 9],
}
