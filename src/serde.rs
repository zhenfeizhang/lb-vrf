use crate::keypair::{PublicKey, SecretKey};
use crate::lbvrf::{Proof, VRFOutput};
use crate::param::Param;
use crate::poly::PolyArith;
use crate::poly256::Poly256;
use std::io::{Error, ErrorKind, Read, Result, Write};

pub trait Serdes {
    fn serialize<W: Write>(&self, writer: &mut W) -> Result<()>;

    fn deserialize<R: Read>(reader: &mut R) -> Result<Self>
    where
        Self: std::marker::Sized;
}

impl Serdes for Poly256 {
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
        for i in 0..256 {
            reader.read_exact(&mut buf)?;
            coeff[i] = i32::from_be_bytes(buf) as i64;
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
                *f = Poly256::deserialize(reader)?;
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
            e.serialize(writer)?;
        }
        Ok(())
    }

    fn deserialize<R: Read>(reader: &mut R) -> Result<Self>
    where
        Self: std::marker::Sized,
    {
        let mut res = [Poly256::zero(); 9];
        for e in res.iter_mut() {
            *e = Poly256::deserialize(reader)?;
        }
        Ok(SecretKey { s: res })
    }
}

impl Serdes for VRFOutput {
    fn serialize<W: Write>(&self, writer: &mut W) -> Result<()> {
        for e in self.iter() {
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
        Ok(res)
    }
}

impl Serdes for Proof {
    fn serialize<W: Write>(&self, writer: &mut W) -> Result<()> {
        for e in self.z.iter() {
            e.serialize(writer)?;
        }
        self.c.serialize(writer)?;
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
        let c = Poly256::deserialize(reader)?;
        let v = VRFOutput::deserialize(reader)?;
        Ok(Proof { z, c, v })
    }
}
