use num_enum::TryFromPrimitive;
use serde::{Deserialize, Serialize};

#[repr(u16)]
#[derive(Debug, Eq, PartialEq, TryFromPrimitive, Serialize, Deserialize, Copy, Clone, Ord, PartialOrd)]
pub enum SMBDialect {
    V2_0_2 = 0x202,
    V2_1_0 = 0x210,
    V3_0_0 = 0x300,
    V3_0_2 = 0x302,
    V3_1_1 = 0x311,
    V2_X_X = 0x2FF
}