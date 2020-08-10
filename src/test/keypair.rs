use crate::keypair::{PublicKey, SecretKey};
use crate::lbvrf::LBVRF;
use crate::param::Param;
use crate::serde::Serdes;
use crate::VRF;

#[test]
fn test_keygen() {
    let seed = [0u8; 32];
    // let mut rng = rand::thread_rng();
    // let param = Param::init(&mut rng);

    let param: Param = <LBVRF as VRF>::paramgen(seed).unwrap();
    let (_pk, _sk) = <LBVRF as VRF>::keygen(seed, param).unwrap();
}

#[test]
fn test_serdes_keygen() {
    let seed = [0u8; 32];
    // let mut rng = rand::thread_rng();
    // let param = Param::init(&mut rng);

    let param: Param = <LBVRF as VRF>::paramgen(seed).unwrap();
    let (pk, sk) = <LBVRF as VRF>::keygen(seed, param).unwrap();

    let mut buf: Vec<u8> = vec![];
    assert!(pk.serialize(&mut buf).is_ok());
    println!("{:?}", buf);
    let pk2 = <LBVRF as VRF>::PublicKey::deserialize(&mut buf[..].as_ref()).unwrap();
    assert_eq!(pk, pk2);

    let mut buf: Vec<u8> = vec![];
    assert!(sk.serialize(&mut buf).is_ok());
    println!("{:?}", buf);
    let sk2 = <LBVRF as VRF>::SecretKey::deserialize(&mut buf[..].as_ref()).unwrap();
    assert_eq!(sk, sk2);
}
