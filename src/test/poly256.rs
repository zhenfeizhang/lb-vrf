use crate::param::BETA;
use crate::param::Q;
use crate::poly::PolyArith;
use crate::poly256::poly256_inner_product;
use crate::poly256::Poly256;
use crate::poly32::Poly32;
use crate::serde::Serdes;
use std::convert::Into;

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

#[test]
fn test_poly256_to_poly32() {
    let a: Poly256 = Poly256 {
        coeff: [
            0, 1, 1, 0, 1, 0, 2, 1, 1, 0, 1, 2, 1, 1, 1, 1, 1, 1, 0, 2, 1, 1, 1, 1, 2, 0, 1, 0, 1,
            0, 2, 1, 2, 1, 1, 1, 1, 2, 1, 2, 1, 1, 1, 1, 1, 1, 1, 0, 0, 2, 2, 1, 1, 1, 1, 1, 2, 0,
            0, 0, 0, 1, 0, 0, 2, 0, 0, 2, 1, 1, 0, 0, 2, 2, 2, 0, 0, 2, 1, 0, 0, 2, 2, 2, 1, 2, 2,
            1, 0, 0, 1, 2, 2, 0, 1, 0, 1, 1, 2, 0, 1, 1, 1, 0, 1, 1, 0, 0, 0, 1, 0, 1, 0, 0, 2, 0,
            1, 0, 1, 0, 0, 1, 0, 1, 1, 1, 0, 1, 2, 0, 0, 2, 1, 1, 2, 0, 1, 1, 2, 1, 1, 2, 0, 1, 1,
            2, 2, 2, 1, 2, 2, 2, 0, 2, 0, 2, 1, 0, 0, 1, 2, 2, 0, 2, 2, 1, 0, 1, 2, 1, 2, 0, 2, 0,
            0, 0, 1, 1, 0, 0, 1, 0, 0, 0, 2, 1, 2, 1, 0, 1, 1, 2, 1, 0, 2, 1, 0, 0, 2, 2, 2, 1, 2,
            2, 1, 2, 0, 0, 1, 0, 1, 2, 1, 0, 0, 1, 1, 2, 1, 1, 0, 0, 2, 2, 2, 0, 2, 1, 1, 2, 0, 1,
            0, 0, 1, 0, 2, 0, 1, 1, 1, 0, 0, 2, 1, 0, 1, 2, 1, 1, 0, 0, 0, 2, 0, 1,
        ],
    };
    let b: Poly32 = a.into();
    assert_eq!(
        b,
        Poly32 {
            coeff: [
                630345, 42056, 638005, 1013356, 1673055, 1412743, 1673289, 1473226, 1177611,
                1981827, 1289186, 1256202, 2038640, 700275, 1318891, 2026268, 264705, 1453195,
                529423, 864121, 931761, 1288440, 1757797, 26213, 117352, 957620, 774985, 196767,
                1541244, 146123, 33692, 417834
            ]
        }
    );

    let a: Poly256 = Poly256 {
        coeff: [
            1543767, 418200, 660641, 483072, 1420147, 798945, 1409425, 543594, 1658424, 1915191,
            2087822, 919225, 1966731, 1689454, 783050, 1522806, 685864, 701835, 172480, 1305406,
            457726, 614888, 422826, 592740, 1910486, 1451462, 881194, 1320548, 2045853, 1206818,
            1365764, 1650345, 911421, 807027, 1167896, 985581, 1536281, 1704210, 1162327, 279102,
            111805, 741149, 1642251, 625116, 1891849, 1899668, 785942, 697161, 937107, 1642332,
            1463255, 814834, 502026, 651909, 1751657, 1297308, 830987, 1202219, 657322, 1182296,
            1555822, 1729883, 987108, 721280, 258120, 846510, 1575878, 1550161, 1856321, 1996694,
            1227176, 284265, 1549913, 884754, 2019980, 499890, 1125653, 765079, 1035073, 608716,
            239223, 905709, 1129391, 230080, 1585528, 1468355, 1451629, 495898, 1575278, 1386810,
            72855, 1071661, 2003471, 1607371, 2033720, 199360, 1985204, 1938279, 1677071, 683393,
            1058284, 335248, 389929, 1317940, 1515205, 238005, 1960484, 1262641, 2077729, 638764,
            764684, 1017493, 1363606, 1105168, 704647, 1239951, 335024, 960231, 2042722, 364891,
            36365, 1889311, 1064771, 1540096, 1096182, 1081754, 191890, 1407750, 403154, 2074207,
            80117, 337302, 1730525, 1331853, 1051689, 1660268, 1788639, 545596, 1848240, 123850,
            296057, 340164, 645597, 1119471, 1847451, 625187, 390448, 660830, 814309, 541366,
            1222243, 576219, 889278, 1570893, 1140291, 934780, 773981, 360548, 549008, 536591,
            976774, 1104336, 527219, 183376, 16464, 111911, 1878802, 533463, 1144914, 291862,
            855199, 2035236, 1228398, 1582956, 94113, 1344643, 1198915, 827203, 1702710, 296085,
            258609, 64039, 875239, 1717573, 825440, 1666544, 872131, 1389028, 687915, 528667,
            976921, 1492826, 582604, 1977599, 1628442, 1290199, 127643, 922678, 733876, 882094,
            394844, 142050, 1067224, 1428351, 612273, 179007, 1653969, 311158, 1880983, 191308,
            1600293, 788186, 95810, 42302, 1340218, 1115093, 1098495, 497348, 118844, 1186097,
            1291212, 1145695, 2031000, 317240, 691492, 566726, 154117, 804422, 1746013, 160334,
            102096, 1672265, 1462833, 1368978, 208102, 1195653, 2063005, 720078, 709569, 813851,
            1440087, 962455, 2063443, 769020, 743219, 1489331, 1772366, 1509545, 1825276, 1155829,
            111979, 909010, 1543760, 276096, 2062686, 720873,
        ],
    };
    let b: Poly32 = a.into();
    assert_eq!(
        b,
        Poly32 {
            coeff: [
                1773376, 2068275, 1434311, 658477, 1569031, 957210, 1403770, 1595308, 101633,
                1554344, 84443, 516658, 68301, 1053749, 241022, 1503900, 2047842, 1959259, 560423,
                1396941, 2061564, 1785086, 175570, 1146466, 1239275, 2001886, 1536473, 493469,
                1781752, 787868, 511863, 1339564
            ]
        }
    );
}
