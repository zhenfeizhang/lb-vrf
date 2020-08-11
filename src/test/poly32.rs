use crate::param::P;
use crate::poly::PolyArith;
use crate::poly32::poly32_inner_product;
use crate::poly32::Poly32;
use crate::serde::Serdes;

#[test]
fn test_rand_mod_p() {
    let mut rng = rand::thread_rng();
    let a: Poly32 = PolyArith::uniform_random(&mut rng);
    for e in a.coeff.iter() {
        assert!(*e < Poly32::MODULUS, "coefficient greater than Q")
    }
}

#[test]
fn test_poly32_mul() {
    // todo: add a lot more test cases
    let mut rng = rand::thread_rng();

    // zero
    let a = Poly32::zero();
    let b = Poly32::uniform_random(&mut rng);
    let c = Poly32::mul(&a, &b);
    for e in c.coeff.iter() {
        assert!(*e == 0, "coefficient not zero")
    }

    // associative

    let a = Poly32::uniform_random(&mut rng);
    let b = Poly32::uniform_random(&mut rng);
    let c = Poly32::mul(&a, &b);
    let d = Poly32::mul(&b, &a);
    assert!(c == d, "coefficient not zero");

    // (x+1) * (x+1) = x^2 + 2x + 1
    let mut a = Poly32::zero();
    a.coeff[0] = 1;
    a.coeff[1] = 1;
    let b = Poly32::mul(&a, &a);
    assert!(b.coeff[0] == 1);
    assert!(b.coeff[1] == 2);
    assert!(b.coeff[2] == 1);
    for i in 3..Poly32::DEGREE {
        assert!(b.coeff[i] == 0)
    }
}

#[test]
fn test_poly32_inner_prod() {
    let mut a = Poly32::zero();
    a.coeff[0] = 1;
    a.coeff[1] = -1;
    let vec_a = [a; 4];
    let vec_b = [a; 4];
    let c = poly32_inner_product(vec_a.as_ref(), &vec_b.as_ref());
    println!("{:?}", c);
    assert!(c.coeff[0] == 4);
    assert!(c.coeff[1] == P - 8);
    assert!(c.coeff[2] == 4);
    for i in 3..Poly32::DEGREE {
        assert!(c.coeff[i] == 0)
    }
    println!(
        "{:?}",
        a.coeff.iter().map(|x| *x as i32).collect::<Vec<i32>>()
    );
}

#[test]
fn test_poly32_serdes() {
    // zero poly
    let a = Poly32::zero();
    let mut buf: Vec<u8> = vec![];
    assert!(a.serialize(&mut buf).is_ok());
    let b = Poly32::deserialize(&mut buf[..].as_ref()).unwrap();
    assert_eq!(a, b);

    // random poly
    let mut rng = rand::thread_rng();
    let a: Poly32 = PolyArith::uniform_random(&mut rng);
    let mut buf: Vec<u8> = vec![];
    assert!(a.serialize(&mut buf).is_ok());
    let b = Poly32::deserialize(&mut buf[..].as_ref()).unwrap();
    assert_eq!(a, b);
}
