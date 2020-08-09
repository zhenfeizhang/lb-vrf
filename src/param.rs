/// P is the modulus for `B part`
pub const P: u32 = 2097169;

/// Q is the modulus for `A part`
pub const Q: u32 = 100679681;

/// R is a root s.t. (x^32+R) divides (x^256+1) mod P
pub const R: u32 = 852368;
