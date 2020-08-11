use crate::lbvrf::*;
use crate::param::*;
use crate::serde::Serdes;
use crate::VRF;
#[test]
fn test_param_gen() {
    let p = LBVRF::paramgen([0; 32]);
    println!("{:?}", p);
}

#[test]
fn test_hash_to_challenge() {
    let input = "this is a random input for testing";
    let c = hash_to_challenge(input.as_ref());
    let mut sum = 0;
    for e in c.coeff.iter() {
        assert!(*e <= 1 || *e >= -1, "coefficients out of range {}", *e);
        if *e != 0 {
            sum += 1;
        }
    }
    assert_eq!(sum, KAPPA)
}

#[test]
fn test_lbvrf() {
    let seed = [0u8; 32];
    // let mut rng = rand::thread_rng();
    // let param = Param::init(&mut rng);

    let param: Param = <LBVRF as VRF>::paramgen(seed).unwrap();
    let (pk, sk) = <LBVRF as VRF>::keygen(seed, param).unwrap();
    let message = "this is a message that vrf signs";
    let seed = [0u8; 32];
    let proof = <LBVRF as VRF>::prove(message, param, pk, sk, seed).unwrap();

    let mut buf: Vec<u8> = vec![];
    assert!(proof.serialize(&mut buf).is_ok());
    println!("{:?}", buf);
    let proof2 = <LBVRF as VRF>::Proof::deserialize(&mut buf[..].as_ref()).unwrap();
    assert_eq!(proof, proof2);

    let res = <LBVRF as VRF>::verify(message, param, pk, proof).unwrap();
    assert!(res.is_some());
}
