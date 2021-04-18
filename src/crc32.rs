#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - CRC32 Data In"]
    pub crc32diw0: crate::Reg<crc32diw0::CRC32DIW0_SPEC>,
    #[doc = "0x02 - CRC32 Data In"]
    pub crc32diw1: crate::Reg<crc32diw1::CRC32DIW1_SPEC>,
    #[doc = "0x04 - CRC32 Data In Reversed Bit"]
    pub crc32dirbw1: crate::Reg<crc32dirbw1::CRC32DIRBW1_SPEC>,
    #[doc = "0x06 - CRC32 Data In Reversed Bit"]
    pub crc32dirbw0: crate::Reg<crc32dirbw0::CRC32DIRBW0_SPEC>,
    #[doc = "0x08 - CRC32 Initialization and Result"]
    pub crc32iniresw0: crate::Reg<crc32iniresw0::CRC32INIRESW0_SPEC>,
    #[doc = "0x0a - CRC32 Initialization and Result"]
    pub crc32iniresw1: crate::Reg<crc32iniresw1::CRC32INIRESW1_SPEC>,
    #[doc = "0x0c - CRC32 Result Reverse"]
    pub crc32resrw1: crate::Reg<crc32resrw1::CRC32RESRW1_SPEC>,
    #[doc = "0x0e - CRC32 Result Reverse"]
    pub crc32resrw0: crate::Reg<crc32resrw0::CRC32RESRW0_SPEC>,
    #[doc = "0x10 - CRC16 Data Input"]
    pub crc16diw0: crate::Reg<crc16diw0::CRC16DIW0_SPEC>,
    #[doc = "0x12 - CRC16 Data Input"]
    pub crc16diw1: crate::Reg<crc16diw1::CRC16DIW1_SPEC>,
    #[doc = "0x14 - CRC16 Data In Reverse"]
    pub crc16dirbw1: crate::Reg<crc16dirbw1::CRC16DIRBW1_SPEC>,
    #[doc = "0x16 - CRC16 Data In Reverse"]
    pub crc16dirbw0: crate::Reg<crc16dirbw0::CRC16DIRBW0_SPEC>,
    #[doc = "0x18 - CRC16 Init and Result"]
    pub crc16iniresw0: crate::Reg<crc16iniresw0::CRC16INIRESW0_SPEC>,
    _reserved13: [u8; 2usize],
    #[doc = "0x1c - CRC16 Result Reverse"]
    pub crc16resrw1: crate::Reg<crc16resrw1::CRC16RESRW1_SPEC>,
    #[doc = "0x1e - CRC16 Result Reverse"]
    pub crc16resrw0: crate::Reg<crc16resrw0::CRC16RESRW0_SPEC>,
}
#[doc = "CRC32DIW0 register accessor: an alias for `Reg<CRC32DIW0_SPEC>`"]
pub type CRC32DIW0 = crate::Reg<crc32diw0::CRC32DIW0_SPEC>;
#[doc = "CRC32 Data In"]
pub mod crc32diw0;
#[doc = "CRC32DIW1 register accessor: an alias for `Reg<CRC32DIW1_SPEC>`"]
pub type CRC32DIW1 = crate::Reg<crc32diw1::CRC32DIW1_SPEC>;
#[doc = "CRC32 Data In"]
pub mod crc32diw1;
#[doc = "CRC32DIRBW1 register accessor: an alias for `Reg<CRC32DIRBW1_SPEC>`"]
pub type CRC32DIRBW1 = crate::Reg<crc32dirbw1::CRC32DIRBW1_SPEC>;
#[doc = "CRC32 Data In Reversed Bit"]
pub mod crc32dirbw1;
#[doc = "CRC32DIRBW0 register accessor: an alias for `Reg<CRC32DIRBW0_SPEC>`"]
pub type CRC32DIRBW0 = crate::Reg<crc32dirbw0::CRC32DIRBW0_SPEC>;
#[doc = "CRC32 Data In Reversed Bit"]
pub mod crc32dirbw0;
#[doc = "CRC32INIRESW0 register accessor: an alias for `Reg<CRC32INIRESW0_SPEC>`"]
pub type CRC32INIRESW0 = crate::Reg<crc32iniresw0::CRC32INIRESW0_SPEC>;
#[doc = "CRC32 Initialization and Result"]
pub mod crc32iniresw0;
#[doc = "CRC32INIRESW1 register accessor: an alias for `Reg<CRC32INIRESW1_SPEC>`"]
pub type CRC32INIRESW1 = crate::Reg<crc32iniresw1::CRC32INIRESW1_SPEC>;
#[doc = "CRC32 Initialization and Result"]
pub mod crc32iniresw1;
#[doc = "CRC32RESRW1 register accessor: an alias for `Reg<CRC32RESRW1_SPEC>`"]
pub type CRC32RESRW1 = crate::Reg<crc32resrw1::CRC32RESRW1_SPEC>;
#[doc = "CRC32 Result Reverse"]
pub mod crc32resrw1;
#[doc = "CRC32RESRW0 register accessor: an alias for `Reg<CRC32RESRW0_SPEC>`"]
pub type CRC32RESRW0 = crate::Reg<crc32resrw0::CRC32RESRW0_SPEC>;
#[doc = "CRC32 Result Reverse"]
pub mod crc32resrw0;
#[doc = "CRC16DIW0 register accessor: an alias for `Reg<CRC16DIW0_SPEC>`"]
pub type CRC16DIW0 = crate::Reg<crc16diw0::CRC16DIW0_SPEC>;
#[doc = "CRC16 Data Input"]
pub mod crc16diw0;
#[doc = "CRC16DIW1 register accessor: an alias for `Reg<CRC16DIW1_SPEC>`"]
pub type CRC16DIW1 = crate::Reg<crc16diw1::CRC16DIW1_SPEC>;
#[doc = "CRC16 Data Input"]
pub mod crc16diw1;
#[doc = "CRC16DIRBW1 register accessor: an alias for `Reg<CRC16DIRBW1_SPEC>`"]
pub type CRC16DIRBW1 = crate::Reg<crc16dirbw1::CRC16DIRBW1_SPEC>;
#[doc = "CRC16 Data In Reverse"]
pub mod crc16dirbw1;
#[doc = "CRC16DIRBW0 register accessor: an alias for `Reg<CRC16DIRBW0_SPEC>`"]
pub type CRC16DIRBW0 = crate::Reg<crc16dirbw0::CRC16DIRBW0_SPEC>;
#[doc = "CRC16 Data In Reverse"]
pub mod crc16dirbw0;
#[doc = "CRC16INIRESW0 register accessor: an alias for `Reg<CRC16INIRESW0_SPEC>`"]
pub type CRC16INIRESW0 = crate::Reg<crc16iniresw0::CRC16INIRESW0_SPEC>;
#[doc = "CRC16 Init and Result"]
pub mod crc16iniresw0;
#[doc = "CRC16RESRW1 register accessor: an alias for `Reg<CRC16RESRW1_SPEC>`"]
pub type CRC16RESRW1 = crate::Reg<crc16resrw1::CRC16RESRW1_SPEC>;
#[doc = "CRC16 Result Reverse"]
pub mod crc16resrw1;
#[doc = "CRC16RESRW0 register accessor: an alias for `Reg<CRC16RESRW0_SPEC>`"]
pub type CRC16RESRW0 = crate::Reg<crc16resrw0::CRC16RESRW0_SPEC>;
#[doc = "CRC16 Result Reverse"]
pub mod crc16resrw0;
