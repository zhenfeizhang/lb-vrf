use crate::keypair::{PublicKey, SecretKey};
use crate::lbvrf::{Proof, VRFOutput};
use crate::param::Param;
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
        let mut coeff = [0i64; 32];
        let mut buf = [0u8; 4];
        for e in coeff.iter_mut() {
            reader.read_exact(&mut buf)?;
            *e = i32::from_be_bytes(buf) as i64;
        }
        Ok(Self { coeff })
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
                f.serialize(writer)?;
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
                *f = Poly256::deserialize::<R>(reader)?;
            }
        }
        Ok(Param { matrix: res })
    }
}

impl Serdes for PublicKey {
    fn serialize<W: Write>(&self, writer: &mut W) -> Result<()> {
        for e in self.t.iter() {
            e.serialize(writer)?;
        }
        Ok(())
    }

    fn deserialize<R: Read>(reader: &mut R) -> Result<Self>
    where
        Self: std::marker::Sized,
    {
        let mut res = [Poly256::zero(); 4];
        for e in res.iter_mut() {
            *e = Poly256::deserialize(reader)?;
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
            e.serialize(writer)?;
        }
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
            *e = Poly256::deserialize(reader)?;
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
