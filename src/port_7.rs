#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Port 7 Input"]
    pub p7in: crate::Reg<p7in::P7IN_SPEC>,
    _reserved1: [u8; 1usize],
    #[doc = "0x02 - Port 7 Output"]
    pub p7out: crate::Reg<p7out::P7OUT_SPEC>,
    _reserved2: [u8; 1usize],
    #[doc = "0x04 - Port 7 Direction"]
    pub p7dir: crate::Reg<p7dir::P7DIR_SPEC>,
    _reserved3: [u8; 1usize],
    #[doc = "0x06 - Port 7 Resistor Enable"]
    pub p7ren: crate::Reg<p7ren::P7REN_SPEC>,
    _reserved4: [u8; 3usize],
    #[doc = "0x0a - Port 7 Selection0"]
    pub p7sel0: crate::Reg<p7sel0::P7SEL0_SPEC>,
    _reserved5: [u8; 1usize],
    #[doc = "0x0c - Port 7 Selection1"]
    pub p7sel1: crate::Reg<p7sel1::P7SEL1_SPEC>,
    _reserved6: [u8; 9usize],
    #[doc = "0x16 - Port 7 Complement Selection"]
    pub p7selc: crate::Reg<p7selc::P7SELC_SPEC>,
}
#[doc = "P7IN register accessor: an alias for `Reg<P7IN_SPEC>`"]
pub type P7IN = crate::Reg<p7in::P7IN_SPEC>;
#[doc = "Port 7 Input"]
pub mod p7in;
#[doc = "P7OUT register accessor: an alias for `Reg<P7OUT_SPEC>`"]
pub type P7OUT = crate::Reg<p7out::P7OUT_SPEC>;
#[doc = "Port 7 Output"]
pub mod p7out;
#[doc = "P7DIR register accessor: an alias for `Reg<P7DIR_SPEC>`"]
pub type P7DIR = crate::Reg<p7dir::P7DIR_SPEC>;
#[doc = "Port 7 Direction"]
pub mod p7dir;
#[doc = "P7REN register accessor: an alias for `Reg<P7REN_SPEC>`"]
pub type P7REN = crate::Reg<p7ren::P7REN_SPEC>;
#[doc = "Port 7 Resistor Enable"]
pub mod p7ren;
#[doc = "P7SEL0 register accessor: an alias for `Reg<P7SEL0_SPEC>`"]
pub type P7SEL0 = crate::Reg<p7sel0::P7SEL0_SPEC>;
#[doc = "Port 7 Selection0"]
pub mod p7sel0;
#[doc = "P7SEL1 register accessor: an alias for `Reg<P7SEL1_SPEC>`"]
pub type P7SEL1 = crate::Reg<p7sel1::P7SEL1_SPEC>;
#[doc = "Port 7 Selection1"]
pub mod p7sel1;
#[doc = "P7SELC register accessor: an alias for `Reg<P7SELC_SPEC>`"]
pub type P7SELC = crate::Reg<p7selc::P7SELC_SPEC>;
#[doc = "Port 7 Complement Selection"]
pub mod p7selc;
