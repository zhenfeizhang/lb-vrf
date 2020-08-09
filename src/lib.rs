// #![cfg_attr(feature = "cargo-clippy", deny(warnings))]

extern crate rand;
extern crate rand_chacha;

pub mod keypair;
pub mod lbvrf;
pub mod param;
pub mod poly;
pub mod poly256;

#[cfg(test)]
mod test;

pub trait VRF {
    type PubParam;
    type PublicKey;
    type SecretKey;
    type Proof;
    type VrfOutput;

    /// input some seed, generate public parameters
    fn paramgen(seed: [u8; 32]) -> Result<Self::PubParam, String>;
    /// input a seed and a parameter output a pair of keys
    fn keygen(
        seed: [u8; 32],
        pp: Self::PubParam,
    ) -> Result<(Self::PublicKey, Self::SecretKey), String>;

    /// input a message, a public parameter, a pair of keys
    /// generate a vrf proof
    fn proof<Blob: AsRef<[u8]>>(
        message: Blob,
        pp: Self::PubParam,
        pk: Self::PublicKey,
        sk: Self::SecretKey,
    ) -> Result<Self::Proof, String>;

    /// input a message, a public parameter, the public key, and a proof
    /// generate an output if proof is valid
    fn verify<Blob: AsRef<[u8]>>(
        message: Blob,
        pp: Self::PubParam,
        pk: Self::PublicKey,
        proof: Self::Proof,
    ) -> Result<Option<Self::VrfOutput>, String>;
}
