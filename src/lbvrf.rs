// use crate::keypair::PublicKey;

use crate::param::*;
use crate::poly::PolyArith;
use crate::poly256::*;
use crate::VRF;
use rand::CryptoRng;
use rand_chacha::{rand_core::SeedableRng, ChaCha20Rng};
pub struct PublicKey {
    t: [Poly256; 4],
}

pub struct SecretKey {
    s: [Poly256; 9],
}

pub struct Proof {
    z: [Poly256; 9],
    c: Poly256,
    v: Output,
}

pub type Output = [Poly256; 4];

pub struct LBVRF;
impl VRF for LBVRF {
    type PubParam = Param;
    type PublicKey = crate::lbvrf::PublicKey;
    type SecretKey = crate::lbvrf::SecretKey;
    type Proof = crate::lbvrf::Proof;
    type VrfOutput = crate::lbvrf::Output;

    /// input some seed, generate public parameters
    fn paramgen(seed: [u8; 32]) -> Result<Self::PubParam, String> {
        let mut rng = ChaCha20Rng::from_seed(seed);
        Ok(Param::init(&mut rng))
    }
    /// input a seed and a parameter output a pair of keys
    fn keygen(
        seed: [u8; 32],
        pp: Self::PubParam,
    ) -> Result<(Self::PublicKey, Self::SecretKey), String> {
        let mut rng = ChaCha20Rng::from_seed(seed);
        let mut sk = SecretKey {
            s: [Poly256::zero(); 9],
        };
        for e in sk.s.iter_mut() {
            *e = PolyArith::rand_trinary(&mut rng);
        }
        let mut pk = PublicKey {
            t: [Poly256::zero(); 4],
        };
        for i in 0..4 {
            pk.t[i] = inner_product(&pp.matrix[i], &sk.s);
        }
        Ok((pk, sk))
    }

    /// input a message, a public parameter, a pair of keys
    /// generate a vrf proof
    fn proof<Blob: AsRef<[u8]>>(
        message: Blob,
        pp: Self::PubParam,
        pk: Self::PublicKey,
        sk: Self::SecretKey,
    ) -> Result<Self::Proof, String> {
        todo!()
    }

    /// input a message, a public parameter, the public key, and a proof
    /// generate an output if proof is valid
    fn verify<Blob: AsRef<[u8]>>(
        message: Blob,
        pp: Self::PubParam,
        pk: Self::PublicKey,
        proof: Self::Proof,
    ) -> Result<Option<Self::VrfOutput>, String> {
        todo!()
    }
}
