// use crate::keypair::PublicKey;
use crate::param::*;
use crate::poly::PolyArith;
use crate::poly256::*;
use rand::{CryptoRng, RngCore};
// use crate::Poly32::*;
use crate::serde::Serdes;
use crate::VRF;
use rand_chacha::{rand_core::SeedableRng, ChaCha20Rng};
use sha2::{Digest, Sha512};
use std::convert::TryInto;

#[derive(PartialEq, Clone, Copy, Debug)]
pub struct Proof {
    pub(crate) z: [Poly256; 9],
    pub(crate) c: Poly256,
    pub(crate) v: VRFOutput,
}

pub type VRFOutput = Poly256;

pub struct LBVRF;

impl VRF for LBVRF {
    type PubParam = Param;
    type PublicKey = crate::keypair::PublicKey;
    type SecretKey = crate::keypair::SecretKey;
    type Proof = crate::lbvrf::Proof;
    type VrfOutput = crate::lbvrf::VRFOutput;

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
        let mut sk = Self::SecretKey {
            s: [Poly256::zero(); 9],
        };
        for e in sk.s.iter_mut() {
            *e = PolyArith::rand_trinary(&mut rng);
        }
        let mut pk = Self::PublicKey {
            t: [Poly256::zero(); 4],
        };
        for i in 0..4 {
            pk.t[i] = inner_product(&pp.matrix[i], &sk.s);
        }
        Ok((pk, sk))
    }

    /// input a message, a public parameter, a pair of keys
    /// generate a vrf proof
    fn prove<Blob: AsRef<[u8]>>(
        message: Blob,
        pp: Self::PubParam,
        pk: Self::PublicKey,
        sk: Self::SecretKey,
        seed: [u8; 32],
    ) -> Result<Self::Proof, String> {
        let mut rng = ChaCha20Rng::from_seed(seed);
        let mut y = [Poly256::zero(); 9];
        let mut rs = 0;

        // step 1: b = hash_to_new_basis (pp, pk, message)
        let mut hash_input: Vec<u8> = vec![];
        assert!(pp.serialize(&mut hash_input).is_ok());
        assert!(pk.serialize(&mut hash_input).is_ok());
        hash_input = [hash_input.as_ref(), message.as_ref()].concat();
        let mut hasher = Sha512::new();
        hasher.update(hash_input);
        let digest = hasher.finalize();
        let b = hash_to_new_basis(digest.as_ref());

        // step 2: v = <b, s>
        let v = inner_product(&b, &sk.s);

        // we start rejection sampling here
        loop {
            rs += 1;
            // step 3: sample y
            for e in y.iter_mut() {
                *e = Poly256::rand_mod_beta(&mut rng);
            }

            // step 4: w1 = Ay, w2 = b y
            let mut w1 = [Poly256::zero(); 4];
            for i in 0..4 {
                w1[i] = inner_product(&pp.matrix[i], &y);
            }
            let w2 = inner_product(&b, &y);

            // step 5: c = hash_to_challenge(pp, pk, message, w1, w2, v)
            let mut hash_input: Vec<u8> = vec![];
            for e in w1.iter() {
                assert!((*e).serialize(&mut hash_input).is_ok());
            }
            assert!(w2.serialize(&mut hash_input).is_ok());
            assert!(v.serialize(&mut hash_input).is_ok());
            let mut hasher = Sha512::new();
            hasher.update([digest.as_ref(), hash_input.as_ref()].concat());
            let digest = hasher.finalize();
            let c = hash_to_challenge(digest.as_ref());

            let mut z = y.clone();
            for i in 0..9 {
                z[i].add_assign(&PolyArith::mul(&c, &sk.s[i]));
            }
            for e in z.iter_mut() {
                (*e).centered();
            }

            println!("rejection sampling {} times", rs);
            println!("z:\n{:?}", z);
            if check_norm(&z) {
                return Ok(Proof { z, c, v });
            }
        }
    }

    /// input a message, a public parameter, the public key, and a proof
    /// generate an output if proof is valid
    fn verify<Blob: AsRef<[u8]>>(
        message: Blob,
        pp: Self::PubParam,
        pk: Self::PublicKey,
        proof: Self::Proof,
    ) -> Result<Option<Self::VrfOutput>, String> {
        // step 3: check the length of z
        if !check_norm(&proof.z) {
            return Ok(None);
        }

        // step 0: rebuild b
        let mut hash_input: Vec<u8> = vec![];
        assert!(pp.serialize(&mut hash_input).is_ok());
        assert!(pk.serialize(&mut hash_input).is_ok());
        hash_input = [hash_input.as_ref(), message.as_ref()].concat();
        let mut hasher = Sha512::new();
        hasher.update(hash_input);
        let digest = hasher.finalize();
        let b = hash_to_new_basis(digest.as_ref());

        // step 1: compute w1_prime = A z - c t
        let mut w1 = [Poly256::zero(); 4];
        for i in 0..4 {
            w1[i] = inner_product(&pp.matrix[i], &proof.z);
            w1[i].sub_assign(&Poly256::mul(&proof.c, &pk.t[i]));
        }

        // step 2: compute w2_prime = <b, z> - cv
        let mut w2 = inner_product(&b, &proof.z);
        w2.sub_assign(&Poly256::mul(&proof.c, &proof.v));

        // step 3: check length of z -- done already

        // step 4: check c = hash(A, t, u, w1_prime, w2_prime, v)
        let mut hash_input: Vec<u8> = vec![];
        for e in w1.iter() {
            assert!((*e).serialize(&mut hash_input).is_ok());
        }
        assert!(w2.serialize(&mut hash_input).is_ok());
        assert!(proof.v.serialize(&mut hash_input).is_ok());
        let mut hasher = Sha512::new();
        hasher.update([digest.as_ref(), hash_input.as_ref()].concat());
        let digest = hasher.finalize();
        let c = hash_to_challenge(digest.as_ref());
        if c == proof.c {
            Ok(Some(proof.v))
        } else {
            println!("verification failed");
            Ok(None)
        }
    }
}

pub(crate) fn hash_to_new_basis(input: &[u8]) -> [Poly256; 9] {
    let mut hasher = Sha512::new();
    hasher.update([input, "domain seperator: hash to basis".as_ref()].concat());
    let digest = hasher.finalize();
    let seed: [u8; 32] = digest.as_slice()[0..32].try_into().expect("Wrong length");
    let mut rng = ChaCha20Rng::from_seed(seed);
    let mut res = [Poly256::zero(); 9];
    for e in res.iter_mut() {
        *e = Poly256::rand_mod_q(&mut rng);
    }
    res
}

pub(crate) fn hash_to_challenge(input: &[u8]) -> Poly256 {
    let mut hasher = Sha512::new();
    hasher.update([input, "domain seperator: hash to challenge".as_ref()].concat());
    let digest = hasher.finalize();
    let mut res = [0i64; 256];
    let mut sign_pt = 0;
    let mut coeff_pt = 0;
    let mut ct = 0;
    let mut tmp = digest[63];
    for _i in 0..KAPPA {
        let coeff = tmp & 0b1;
        tmp >>= 1;
        ct += 1;
        if ct == 4 {
            ct = 0;
            sign_pt += 1;
            tmp = digest[64 - sign_pt - 1];
        }
        if res[digest[coeff_pt] as usize] != 0 {
            coeff_pt += 1;
        }
        res[digest[coeff_pt] as usize] = {
            if coeff == 1 {
                1
            } else {
                -1
            }
        };
        coeff_pt += 1;

        if coeff_pt + sign_pt == 64 {
            panic!(
                "used all entropy\n\
                 this should never happen in practise!!!\n\
                 (something may be wrong with prng)"
            )
        }
    }

    Poly256 { coeff: res }
}

fn check_norm(z: &[Poly256; 9]) -> bool {
    for e in z.iter() {
        for f in e.coeff.iter() {
            if *f > BETA_M_KAPPA || *f < -BETA_M_KAPPA {
                return false;
            }
        }
    }

    true
}
