#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Port 3 Input"]
    pub p3in: crate::Reg<p3in::P3IN_SPEC>,
    #[doc = "0x01 - Port 4 Input"]
    pub p4in: crate::Reg<p4in::P4IN_SPEC>,
    #[doc = "0x02 - Port 3 Output"]
    pub p3out: crate::Reg<p3out::P3OUT_SPEC>,
    #[doc = "0x03 - Port 4 Output"]
    pub p4out: crate::Reg<p4out::P4OUT_SPEC>,
    #[doc = "0x04 - Port 3 Direction"]
    pub p3dir: crate::Reg<p3dir::P3DIR_SPEC>,
    #[doc = "0x05 - Port 4 Direction"]
    pub p4dir: crate::Reg<p4dir::P4DIR_SPEC>,
    #[doc = "0x06 - Port 3 Resistor Enable"]
    pub p3ren: crate::Reg<p3ren::P3REN_SPEC>,
    #[doc = "0x07 - Port 4 Resistor Enable"]
    pub p4ren: crate::Reg<p4ren::P4REN_SPEC>,
    _reserved8: [u8; 2usize],
    #[doc = "0x0a - Port 3 Selection 0"]
    pub p3sel0: crate::Reg<p3sel0::P3SEL0_SPEC>,
    #[doc = "0x0b - Port 4 Selection 0"]
    pub p4sel0: crate::Reg<p4sel0::P4SEL0_SPEC>,
    #[doc = "0x0c - Port 3 Selection 1"]
    pub p3sel1: crate::Reg<p3sel1::P3SEL1_SPEC>,
    #[doc = "0x0d - Port 4 Selection 1"]
    pub p4sel1: crate::Reg<p4sel1::P4SEL1_SPEC>,
    #[doc = "0x0e - Port 3 Interrupt Vector Word"]
    pub p3iv: crate::Reg<p3iv::P3IV_SPEC>,
    _reserved13: [u8; 6usize],
    #[doc = "0x16 - Port 3 Complement Selection"]
    pub p3selc: crate::Reg<p3selc::P3SELC_SPEC>,
    #[doc = "0x17 - Port 4 Complement Selection"]
    pub p4selc: crate::Reg<p4selc::P4SELC_SPEC>,
    #[doc = "0x18 - Port 3 Interrupt Edge Select"]
    pub p3ies: crate::Reg<p3ies::P3IES_SPEC>,
    #[doc = "0x19 - Port 4 Interrupt Edge Select"]
    pub p4ies: crate::Reg<p4ies::P4IES_SPEC>,
    #[doc = "0x1a - Port 3 Interrupt Enable"]
    pub p3ie: crate::Reg<p3ie::P3IE_SPEC>,
    #[doc = "0x1b - Port 4 Interrupt Enable"]
    pub p4ie: crate::Reg<p4ie::P4IE_SPEC>,
    #[doc = "0x1c - Port 3 Interrupt Flag"]
    pub p3ifg: crate::Reg<p3ifg::P3IFG_SPEC>,
    #[doc = "0x1d - Port 4 Interrupt Flag"]
    pub p4ifg: crate::Reg<p4ifg::P4IFG_SPEC>,
    #[doc = "0x1e - Port 4 Interrupt Vector Word"]
    pub p4iv: crate::Reg<p4iv::P4IV_SPEC>,
}
#[doc = "P3IN register accessor: an alias for `Reg<P3IN_SPEC>`"]
pub type P3IN = crate::Reg<p3in::P3IN_SPEC>;
#[doc = "Port 3 Input"]
pub mod p3in;
#[doc = "P4IN register accessor: an alias for `Reg<P4IN_SPEC>`"]
pub type P4IN = crate::Reg<p4in::P4IN_SPEC>;
#[doc = "Port 4 Input"]
pub mod p4in;
#[doc = "P3OUT register accessor: an alias for `Reg<P3OUT_SPEC>`"]
pub type P3OUT = crate::Reg<p3out::P3OUT_SPEC>;
#[doc = "Port 3 Output"]
pub mod p3out;
#[doc = "P4OUT register accessor: an alias for `Reg<P4OUT_SPEC>`"]
pub type P4OUT = crate::Reg<p4out::P4OUT_SPEC>;
#[doc = "Port 4 Output"]
pub mod p4out;
#[doc = "P3DIR register accessor: an alias for `Reg<P3DIR_SPEC>`"]
pub type P3DIR = crate::Reg<p3dir::P3DIR_SPEC>;
#[doc = "Port 3 Direction"]
pub mod p3dir;
#[doc = "P4DIR register accessor: an alias for `Reg<P4DIR_SPEC>`"]
pub type P4DIR = crate::Reg<p4dir::P4DIR_SPEC>;
#[doc = "Port 4 Direction"]
pub mod p4dir;
#[doc = "P3REN register accessor: an alias for `Reg<P3REN_SPEC>`"]
pub type P3REN = crate::Reg<p3ren::P3REN_SPEC>;
#[doc = "Port 3 Resistor Enable"]
pub mod p3ren;
#[doc = "P4REN register accessor: an alias for `Reg<P4REN_SPEC>`"]
pub type P4REN = crate::Reg<p4ren::P4REN_SPEC>;
#[doc = "Port 4 Resistor Enable"]
pub mod p4ren;
#[doc = "P3SEL0 register accessor: an alias for `Reg<P3SEL0_SPEC>`"]
pub type P3SEL0 = crate::Reg<p3sel0::P3SEL0_SPEC>;
#[doc = "Port 3 Selection 0"]
pub mod p3sel0;
#[doc = "P4SEL0 register accessor: an alias for `Reg<P4SEL0_SPEC>`"]
pub type P4SEL0 = crate::Reg<p4sel0::P4SEL0_SPEC>;
#[doc = "Port 4 Selection 0"]
pub mod p4sel0;
#[doc = "P3SEL1 register accessor: an alias for `Reg<P3SEL1_SPEC>`"]
pub type P3SEL1 = crate::Reg<p3sel1::P3SEL1_SPEC>;
#[doc = "Port 3 Selection 1"]
pub mod p3sel1;
#[doc = "P4SEL1 register accessor: an alias for `Reg<P4SEL1_SPEC>`"]
pub type P4SEL1 = crate::Reg<p4sel1::P4SEL1_SPEC>;
#[doc = "Port 4 Selection 1"]
pub mod p4sel1;
#[doc = "P3SELC register accessor: an alias for `Reg<P3SELC_SPEC>`"]
pub type P3SELC = crate::Reg<p3selc::P3SELC_SPEC>;
#[doc = "Port 3 Complement Selection"]
pub mod p3selc;
#[doc = "P4SELC register accessor: an alias for `Reg<P4SELC_SPEC>`"]
pub type P4SELC = crate::Reg<p4selc::P4SELC_SPEC>;
#[doc = "Port 4 Complement Selection"]
pub mod p4selc;
#[doc = "P3IES register accessor: an alias for `Reg<P3IES_SPEC>`"]
pub type P3IES = crate::Reg<p3ies::P3IES_SPEC>;
#[doc = "Port 3 Interrupt Edge Select"]
pub mod p3ies;
#[doc = "P4IES register accessor: an alias for `Reg<P4IES_SPEC>`"]
pub type P4IES = crate::Reg<p4ies::P4IES_SPEC>;
#[doc = "Port 4 Interrupt Edge Select"]
pub mod p4ies;
#[doc = "P3IE register accessor: an alias for `Reg<P3IE_SPEC>`"]
pub type P3IE = crate::Reg<p3ie::P3IE_SPEC>;
#[doc = "Port 3 Interrupt Enable"]
pub mod p3ie;
#[doc = "P4IE register accessor: an alias for `Reg<P4IE_SPEC>`"]
pub type P4IE = crate::Reg<p4ie::P4IE_SPEC>;
#[doc = "Port 4 Interrupt Enable"]
pub mod p4ie;
#[doc = "P3IFG register accessor: an alias for `Reg<P3IFG_SPEC>`"]
pub type P3IFG = crate::Reg<p3ifg::P3IFG_SPEC>;
#[doc = "Port 3 Interrupt Flag"]
pub mod p3ifg;
#[doc = "P4IFG register accessor: an alias for `Reg<P4IFG_SPEC>`"]
pub type P4IFG = crate::Reg<p4ifg::P4IFG_SPEC>;
#[doc = "Port 4 Interrupt Flag"]
pub mod p4ifg;
#[doc = "P3IV register accessor: an alias for `Reg<P3IV_SPEC>`"]
pub type P3IV = crate::Reg<p3iv::P3IV_SPEC>;
#[doc = "Port 3 Interrupt Vector Word"]
pub mod p3iv;
#[doc = "P4IV register accessor: an alias for `Reg<P4IV_SPEC>`"]
pub type P4IV = crate::Reg<p4iv::P4IV_SPEC>;
#[doc = "Port 4 Interrupt Vector Word"]
pub mod p4iv;
