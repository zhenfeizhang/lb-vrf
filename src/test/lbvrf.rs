use crate::lbvrf::*;
use crate::VRF;

#[test]
fn test_param_gen() {
    let p = LBVRF::paramgen([0; 32]);
    println!("{:?}", p);
}
