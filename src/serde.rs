// use crate::param::Param;
// use serde::ser::{Serialize, SerializeSeq, Serializer};
// use serde::{Deserialize, Serialize};
//
// impl Serialize for Param {
//     fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
//     where
//         S: Serializer,
//     {
//         let mut seq = serializer.serialize_seq(Some(self.len()))?;
//         for e in self.matrix.iter() {
//             for f in e {
//                 seq.serialize_element(f)?;
//             }
//         }
//         seq.end()
//     }
// }
