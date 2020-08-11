use crate::param::BETA;
use crate::param::Q;
use crate::poly::PolyArith;
use crate::poly256::poly256_inner_product;
use crate::poly256::Poly256;
use crate::serde::Serdes;

#[test]
fn test_rand_mod_q() {
    let mut rng = rand::thread_rng();
    let a: Poly256 = PolyArith::uniform_random(&mut rng);
    for e in a.coeff.iter() {
        assert!(*e < Poly256::MODULUS, "coefficient greater than Q")
    }
}

#[test]
fn test_rand_mod_beta() {
    let mut rng = rand::thread_rng();
    let a: Poly256 = PolyArith::rand_mod_beta(&mut rng);
    for e in a.coeff.iter() {
        assert!(
            *e <= BETA as i64 && *e >= -(BETA as i64),
            "coefficient greater than beta"
        )
    }
}

#[test]
fn test_rand_trinary() {
    let mut rng = rand::thread_rng();
    let a: Poly256 = PolyArith::rand_trinary(&mut rng);
    for e in a.coeff.iter() {
        assert!(*e <= 1 && *e >= -1, "coefficient not trinary")
    }
}

#[test]
fn test_poly256_mul() {
    // todo: add a lot more test cases
    let mut rng = rand::thread_rng();

    // zero
    let a = Poly256::zero();
    let b = Poly256::uniform_random(&mut rng);
    let c = Poly256::mul(&a, &b);
    for e in c.coeff.iter() {
        assert!(*e == 0, "coefficient not zero")
    }

    // associative

    let a = Poly256::uniform_random(&mut rng);
    let b = Poly256::uniform_random(&mut rng);
    let c = Poly256::mul(&a, &b);
    let d = Poly256::mul(&b, &a);
    assert!(c == d, "coefficient not zero");

    // (x+1) * (x+1) = x^2 + 2x + 1
    let mut a = Poly256::zero();
    a.coeff[0] = 1;
    a.coeff[1] = 1;
    let b = Poly256::mul(&a, &a);
    assert!(b.coeff[0] == 1);
    assert!(b.coeff[1] == 2);
    assert!(b.coeff[2] == 1);
    for i in 3..Poly256::DEGREE {
        assert!(b.coeff[i] == 0)
    }
}

#[test]
fn test_poly256_inner_prod() {
    let mut a = Poly256::zero();
    a.coeff[0] = 1;
    a.coeff[1] = -1;
    let vec_a = [a; 4];
    let vec_b = [a; 4];
    let c = poly256_inner_product(vec_a.as_ref(), &vec_b.as_ref());
    assert!(c.coeff[0] == 4);
    assert!(c.coeff[1] == Q - 8);
    assert!(c.coeff[2] == 4);
    for i in 3..Poly256::DEGREE {
        assert!(c.coeff[i] == 0)
    }
    println!(
        "{:?}",
        a.coeff.iter().map(|x| *x as i32).collect::<Vec<i32>>()
    );
}

#[test]
fn test_poly256_serdes() {
    // zero poly
    let a = Poly256::zero();
    let mut buf: Vec<u8> = vec![];
    assert!(a.serialize(&mut buf).is_ok());
    let b = Poly256::deserialize(&mut buf[..].as_ref()).unwrap();
    assert_eq!(a, b);

    // random poly
    let mut rng = rand::thread_rng();
    let a: Poly256 = PolyArith::uniform_random(&mut rng);
    let mut buf: Vec<u8> = vec![];
    assert!(a.serialize(&mut buf).is_ok());
    let b = Poly256::deserialize(&mut buf[..].as_ref()).unwrap();
    assert_eq!(a, b);

    // trinary poly
    let a: Poly256 = PolyArith::rand_trinary(&mut rng);
    let mut buf: Vec<u8> = vec![];
    assert!(a.serialize(&mut buf).is_ok());
    let b = Poly256::deserialize(&mut buf[..].as_ref()).unwrap();
    assert_eq!(a, b);
}
