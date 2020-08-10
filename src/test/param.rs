use crate::lbvrf::LBVRF;
use crate::param::Param;
use crate::serde::Serdes;
use crate::VRF;

#[test]
fn test_param() {
    let mut rng = rand::thread_rng();
    let _param = Param::init(&mut rng);
}

#[test]
fn test_serdes_param() {
    let seed = [0u8; 32];
    // let mut rng = rand::thread_rng();
    // let param = Param::init(&mut rng);

    let param: Param = <LBVRF as VRF>::paramgen(seed).unwrap();
    let mut buf: Vec<u8> = vec![];
    assert!(param.serialize(&mut buf).is_ok());
    println!("{:02x?}", buf);
    let param2 = Param::deserialize(&mut buf[..].as_ref()).unwrap();
    assert_eq!(param, param2);
}
