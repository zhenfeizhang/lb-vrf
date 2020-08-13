use crate::keypair::{PublicKey, SecretKey};
use crate::lbvrf::{Proof, VRFOutput};
use crate::param::Param;
use crate::param::BETA;
use crate::poly::PolyArith;
use crate::poly256::Poly256;
use crate::poly32::Poly32;
use std::io::{Read, Result, Write};

pub trait Serdes {
    fn serialize<W: Write>(&self, writer: &mut W) -> Result<()>;

    fn deserialize<R: Read>(reader: &mut R) -> Result<Self>
    where
        Self: std::marker::Sized;
}

impl Serdes for Poly32 {
    // todo: use a more efficient way to (de)serialize poly256
    // todo: use a different endocing scheme for trinary secrets
    fn serialize<W: Write>(&self, writer: &mut W) -> Result<()> {
        pack_mod_p_poly(self, writer)
    }

    fn deserialize<R: Read>(reader: &mut R) -> Result<Self>
    where
        Self: std::marker::Sized,
    {
        let mut res = Poly32::zero();
        unpack_mod_p_poly(&mut res, reader)?;
        Ok(res)
    }
}

impl Serdes for Poly256 {
    // todo: use a more efficient way to (de)serialize poly256
    // todo: use a different endocing scheme for trinary secrets
    fn serialize<W: Write>(&self, writer: &mut W) -> Result<()> {
        for e in self.coeff.iter() {
            let tmp = *e as u32;
            let buf = tmp.to_be_bytes();
            writer.write_all(&buf)?;
        }
        Ok(())
    }

    fn deserialize<R: Read>(reader: &mut R) -> Result<Self>
    where
        Self: std::marker::Sized,
    {
        let mut coeff = [0i64; 256];
        let mut buf = [0u8; 4];
        for e in coeff.iter_mut() {
            reader.read_exact(&mut buf)?;
            *e = i32::from_be_bytes(buf) as i64;
        }
        Ok(Self { coeff })
    }
}

impl Serdes for Param {
    fn serialize<W: Write>(&self, writer: &mut W) -> Result<()> {
        for e in self.matrix.iter() {
            for f in e {
                pack_mod_q_poly(f, writer)?;
            }
        }
        Ok(())
    }

    fn deserialize<R: Read>(reader: &mut R) -> Result<Self>
    where
        Self: std::marker::Sized,
    {
        let mut res = [[Poly256::zero(); 9]; 4];
        for e in res.iter_mut() {
            for f in e.iter_mut() {
                unpack_mod_q_poly(f, reader)?;
            }
        }
        Ok(Param { matrix: res })
    }
}

impl Serdes for PublicKey {
    fn serialize<W: Write>(&self, writer: &mut W) -> Result<()> {
        for e in self.t.iter() {
            pack_mod_q_poly(e, writer)?;
        }
        Ok(())
    }

    fn deserialize<R: Read>(reader: &mut R) -> Result<Self>
    where
        Self: std::marker::Sized,
    {
        let mut res = [Poly256::zero(); 4];
        for e in res.iter_mut() {
            unpack_mod_q_poly(e, reader)?;
        }
        Ok(PublicKey { t: res })
    }
}

impl Serdes for SecretKey {
    fn serialize<W: Write>(&self, writer: &mut W) -> Result<()> {
        for e in self.s.iter() {
            pack_trinary(e, writer)?;
        }
        Ok(())
    }

    fn deserialize<R: Read>(reader: &mut R) -> Result<Self>
    where
        Self: std::marker::Sized,
    {
        let mut res = [Poly256::zero(); 9];
        for e in res.iter_mut() {
            unpack_trinary(e, reader)?;
        }
        Ok(SecretKey { s: res })
    }
}

impl Serdes for Proof {
    fn serialize<W: Write>(&self, writer: &mut W) -> Result<()> {
        for e in self.z.iter() {
            pack_mod_beta_poly(e, writer)?;
        }
        // todo: improve this packing method for challenges
        pack_trinary(&self.c, writer)?;
        self.v.serialize(writer)?;
        Ok(())
    }

    fn deserialize<R: Read>(reader: &mut R) -> Result<Self>
    where
        Self: std::marker::Sized,
    {
        let mut z = [Poly256::zero(); 9];
        for e in z.iter_mut() {
            unpack_mod_beta_poly(e, reader)?;
        }
        let mut c = Poly256::zero();
        unpack_trinary(&mut c, reader)?;
        let v = VRFOutput::deserialize(reader)?;
        Ok(Proof { z, c, v })
    }
}

fn pack_trinary<W: Write>(p: &Poly256, writer: &mut W) -> Result<()> {
    for i in 0..64 {
        let mut tmp = p.coeff[i * 4] + 1;
        tmp <<= 2;
        tmp += p.coeff[i * 4 + 1] + 1;
        tmp <<= 2;
        tmp += p.coeff[i * 4 + 2] + 1;
        tmp <<= 2;
        tmp += p.coeff[i * 4 + 3] + 1;
        writer.write_all(&[tmp as u8])?;
    }

    Ok(())
}

fn unpack_trinary<R: Read>(res: &mut Poly256, reader: &mut R) -> Result<()> {
    // let mut res = [0i64; 256];
    let mut buf = [0u8; 64];
    reader.read_exact(&mut buf)?;
    for (i, e) in buf.iter_mut().enumerate() {
        res.coeff[i * 4 + 3] = (*e & 0b11) as i64 - 1;
        (*e) >>= 2;
        res.coeff[i * 4 + 2] = (*e & 0b11) as i64 - 1;
        (*e) >>= 2;
        res.coeff[i * 4 + 1] = (*e & 0b11) as i64 - 1;
        (*e) >>= 2;
        res.coeff[i * 4] = (*e & 0b11) as i64 - 1;
    }
    Ok(())
    // Ok(Poly256 { coeff: res })
}

fn pack_mod_q_poly<W: Write>(p: &Poly256, writer: &mut W) -> Result<()> {
    for i in 0..64 {
        let mut tmp: u128 = p.coeff[i * 4] as u128;
        tmp <<= 27;
        tmp += p.coeff[i * 4 + 1] as u128;
        tmp <<= 27;
        tmp += p.coeff[i * 4 + 2] as u128;
        tmp <<= 27;
        tmp += p.coeff[i * 4 + 3] as u128;
        writer.write_all(&tmp.to_be_bytes()[2..16])?;
    }

    Ok(())
}

fn unpack_mod_q_poly<R: Read>(res: &mut Poly256, reader: &mut R) -> Result<()> {
    for i in 0..64 {
        let mut buf = [0u8; 16];
        reader.read_exact(&mut buf[2..16])?;

        let mut tmp = u128::from_be_bytes(buf);
        res.coeff[i * 4 + 3] = (tmp & 0x07FF_FFFF) as i64;
        tmp >>= 27;
        res.coeff[i * 4 + 2] = (tmp & 0x07FF_FFFF) as i64;
        tmp >>= 27;
        res.coeff[i * 4 + 1] = (tmp & 0x07FF_FFFF) as i64;
        tmp >>= 27;
        res.coeff[i * 4] = tmp as i64;
    }
    Ok(())
}

fn pack_mod_beta_poly<W: Write>(p: &Poly256, writer: &mut W) -> Result<()> {
    for i in 0..64 {
        let mut tmp: u128 = (p.coeff[i * 4] + BETA) as u128;
        tmp <<= 18;
        tmp += (p.coeff[i * 4 + 1] + BETA) as u128;
        tmp <<= 18;
        tmp += (p.coeff[i * 4 + 2] + BETA) as u128;
        tmp <<= 18;
        tmp += (p.coeff[i * 4 + 3] + BETA) as u128;

        writer.write_all(&tmp.to_be_bytes()[7..16])?;
    }
    Ok(())
}

fn unpack_mod_beta_poly<R: Read>(res: &mut Poly256, reader: &mut R) -> Result<()> {
    println!();
    for i in 0..64 {
        let mut buf = [0u8; 16];
        reader.read_exact(&mut buf[7..16])?;
        let mut tmp = u128::from_be_bytes(buf);
        res.coeff[i * 4 + 3] = (tmp & 0x0003_FFFF) as i64 - BETA;
        tmp >>= 18;
        res.coeff[i * 4 + 2] = (tmp & 0x0003_FFFF) as i64 - BETA;
        tmp >>= 18;
        res.coeff[i * 4 + 1] = (tmp & 0x0003_FFFF) as i64 - BETA;
        tmp >>= 18;
        res.coeff[i * 4] = tmp as i64 - BETA;
    }
    Ok(())
}

fn pack_mod_p_poly<W: Write>(p: &Poly32, writer: &mut W) -> Result<()> {
    for i in 0..8 {
        let mut tmp: u128 = p.coeff[i * 4] as u128;
        tmp <<= 22;
        tmp += p.coeff[i * 4 + 1] as u128;
        tmp <<= 22;
        tmp += p.coeff[i * 4 + 2] as u128;
        tmp <<= 22;
        tmp += p.coeff[i * 4 + 3] as u128;
        writer.write_all(&tmp.to_be_bytes()[5..16])?;
    }

    Ok(())
}

fn unpack_mod_p_poly<R: Read>(res: &mut Poly32, reader: &mut R) -> Result<()> {
    for i in 0..8 {
        let mut buf = [0u8; 16];
        reader.read_exact(&mut buf[5..16])?;

        let mut tmp = u128::from_be_bytes(buf);
        res.coeff[i * 4 + 3] = (tmp & 0x003F_FFFF) as i64;
        tmp >>= 22;
        res.coeff[i * 4 + 2] = (tmp & 0x003F_FFFF) as i64;
        tmp >>= 22;
        res.coeff[i * 4 + 1] = (tmp & 0x003F_FFFF) as i64;
        tmp >>= 22;
        res.coeff[i * 4] = tmp as i64;
    }
    Ok(())
}

#[test]
fn test_packing() {
    let mut rng = rand::thread_rng();
    let p = Poly256::uniform_random(&mut rng);
    let mut v: Vec<u8> = vec![];
    pack_mod_q_poly(&p, &mut v).unwrap();
    let mut p2 = Poly256::zero();
    unpack_mod_q_poly(&mut p2, &mut v[..].as_ref()).unwrap();
    assert_eq!(p, p2);

    let p = Poly256::rand_mod_beta(&mut rng);
    let mut v: Vec<u8> = vec![];
    pack_mod_beta_poly(&p, &mut v).unwrap();
    let mut p2 = Poly256::zero();
    unpack_mod_beta_poly(&mut p2, &mut v[..].as_ref()).unwrap();
    assert_eq!(p, p2);
    // assert!(false);
}
