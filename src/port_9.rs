#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Port 9 Input"]
    pub p9in: crate::Reg<p9in::P9IN_SPEC>,
    _reserved1: [u8; 1usize],
    #[doc = "0x02 - Port 9 Output"]
    pub p9out: crate::Reg<p9out::P9OUT_SPEC>,
    _reserved2: [u8; 1usize],
    #[doc = "0x04 - Port 9 Direction"]
    pub p9dir: crate::Reg<p9dir::P9DIR_SPEC>,
    _reserved3: [u8; 1usize],
    #[doc = "0x06 - Port 9 Resistor Enable"]
    pub p9ren: crate::Reg<p9ren::P9REN_SPEC>,
    _reserved4: [u8; 3usize],
    #[doc = "0x0a - Port 9 Selection0"]
    pub p9sel0: crate::Reg<p9sel0::P9SEL0_SPEC>,
    _reserved5: [u8; 1usize],
    #[doc = "0x0c - Port 9 Selection1"]
    pub p9sel1: crate::Reg<p9sel1::P9SEL1_SPEC>,
    _reserved6: [u8; 9usize],
    #[doc = "0x16 - Port 9 Complement Selection"]
    pub p9selc: crate::Reg<p9selc::P9SELC_SPEC>,
}
#[doc = "P9IN register accessor: an alias for `Reg<P9IN_SPEC>`"]
pub type P9IN = crate::Reg<p9in::P9IN_SPEC>;
#[doc = "Port 9 Input"]
pub mod p9in;
#[doc = "P9OUT register accessor: an alias for `Reg<P9OUT_SPEC>`"]
pub type P9OUT = crate::Reg<p9out::P9OUT_SPEC>;
#[doc = "Port 9 Output"]
pub mod p9out;
#[doc = "P9DIR register accessor: an alias for `Reg<P9DIR_SPEC>`"]
pub type P9DIR = crate::Reg<p9dir::P9DIR_SPEC>;
#[doc = "Port 9 Direction"]
pub mod p9dir;
#[doc = "P9REN register accessor: an alias for `Reg<P9REN_SPEC>`"]
pub type P9REN = crate::Reg<p9ren::P9REN_SPEC>;
#[doc = "Port 9 Resistor Enable"]
pub mod p9ren;
#[doc = "P9SEL0 register accessor: an alias for `Reg<P9SEL0_SPEC>`"]
pub type P9SEL0 = crate::Reg<p9sel0::P9SEL0_SPEC>;
#[doc = "Port 9 Selection0"]
pub mod p9sel0;
#[doc = "P9SEL1 register accessor: an alias for `Reg<P9SEL1_SPEC>`"]
pub type P9SEL1 = crate::Reg<p9sel1::P9SEL1_SPEC>;
#[doc = "Port 9 Selection1"]
pub mod p9sel1;
#[doc = "P9SELC register accessor: an alias for `Reg<P9SELC_SPEC>`"]
pub type P9SELC = crate::Reg<p9selc::P9SELC_SPEC>;
#[doc = "Port 9 Complement Selection"]
pub mod p9selc;
