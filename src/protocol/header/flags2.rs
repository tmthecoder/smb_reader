use bitflags::bitflags;
use serde::{Serialize, Deserialize};

bitflags! {
    #[derive(Default, Serialize, Deserialize)]
    pub struct LegacySMBFlags2: u16 {
        const UNICODE_STRINGS    = 0b1000000000000000;
        const ERROR_CODE_STATUS  = 0b100000000000000; // 32_BIT_STATUS
        const READ_IF_EXECUTE    = 0b10000000000000;
        const DFS_PATHNAME       = 0b1000000000000;
        const EXTENDED_SECURITY  = 0b100000000000;
        const RESERVED_01        = 0b10000000000;
        const RESERVED_02        = 0b1000000000;
        const RESERVED_03        = 0b100000000;
        const RESERVED_04        = 0b10000000;
        const IS_LONG_NAME       = 0b1000000;
        const RESERVED_05        = 0b100000;
        const RESERVED_06        = 0b1000;
        const SECURITY_SIGNATURE = 0b100;
        const EAS                = 0b10;
        const KNOWS_LONG_NAMES   = 0b1;
    }
}

impl LegacySMBFlags2 {
    pub fn clear(&mut self) {
        self.bits = 0;
    }
}