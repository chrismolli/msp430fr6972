#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Port 1 Input"]
    pub p1in: crate::Reg<p1in::P1IN_SPEC>,
    #[doc = "0x01 - Port 2 Input"]
    pub p2in: crate::Reg<p2in::P2IN_SPEC>,
    #[doc = "0x02 - Port 1 Output"]
    pub p1out: crate::Reg<p1out::P1OUT_SPEC>,
    #[doc = "0x03 - Port 2 Output"]
    pub p2out: crate::Reg<p2out::P2OUT_SPEC>,
    #[doc = "0x04 - Port 1 Direction"]
    pub p1dir: crate::Reg<p1dir::P1DIR_SPEC>,
    #[doc = "0x05 - Port 2 Direction"]
    pub p2dir: crate::Reg<p2dir::P2DIR_SPEC>,
    #[doc = "0x06 - Port 1 Resistor Enable"]
    pub p1ren: crate::Reg<p1ren::P1REN_SPEC>,
    #[doc = "0x07 - Port 2 Resistor Enable"]
    pub p2ren: crate::Reg<p2ren::P2REN_SPEC>,
    _reserved8: [u8; 2usize],
    #[doc = "0x0a - Port 1 Selection 0"]
    pub p1sel0: crate::Reg<p1sel0::P1SEL0_SPEC>,
    #[doc = "0x0b - Port 2 Selection 0"]
    pub p2sel0: crate::Reg<p2sel0::P2SEL0_SPEC>,
    #[doc = "0x0c - Port 1 Selection 1"]
    pub p1sel1: crate::Reg<p1sel1::P1SEL1_SPEC>,
    #[doc = "0x0d - Port 2 Selection 1"]
    pub p2sel1: crate::Reg<p2sel1::P2SEL1_SPEC>,
    #[doc = "0x0e - Port 1 Interrupt Vector Word"]
    pub p1iv: crate::Reg<p1iv::P1IV_SPEC>,
    _reserved13: [u8; 6usize],
    #[doc = "0x16 - Port 1 Complement Selection"]
    pub p1selc: crate::Reg<p1selc::P1SELC_SPEC>,
    #[doc = "0x17 - Port 2 Complement Selection"]
    pub p2selc: crate::Reg<p2selc::P2SELC_SPEC>,
    #[doc = "0x18 - Port 1 Interrupt Edge Select"]
    pub p1ies: crate::Reg<p1ies::P1IES_SPEC>,
    #[doc = "0x19 - Port 2 Interrupt Edge Select"]
    pub p2ies: crate::Reg<p2ies::P2IES_SPEC>,
    #[doc = "0x1a - Port 1 Interrupt Enable"]
    pub p1ie: crate::Reg<p1ie::P1IE_SPEC>,
    #[doc = "0x1b - Port 2 Interrupt Enable"]
    pub p2ie: crate::Reg<p2ie::P2IE_SPEC>,
    #[doc = "0x1c - Port 1 Interrupt Flag"]
    pub p1ifg: crate::Reg<p1ifg::P1IFG_SPEC>,
    #[doc = "0x1d - Port 2 Interrupt Flag"]
    pub p2ifg: crate::Reg<p2ifg::P2IFG_SPEC>,
    #[doc = "0x1e - Port 2 Interrupt Vector Word"]
    pub p2iv: crate::Reg<p2iv::P2IV_SPEC>,
}
#[doc = "P1IN register accessor: an alias for `Reg<P1IN_SPEC>`"]
pub type P1IN = crate::Reg<p1in::P1IN_SPEC>;
#[doc = "Port 1 Input"]
pub mod p1in;
#[doc = "P2IN register accessor: an alias for `Reg<P2IN_SPEC>`"]
pub type P2IN = crate::Reg<p2in::P2IN_SPEC>;
#[doc = "Port 2 Input"]
pub mod p2in;
#[doc = "P1OUT register accessor: an alias for `Reg<P1OUT_SPEC>`"]
pub type P1OUT = crate::Reg<p1out::P1OUT_SPEC>;
#[doc = "Port 1 Output"]
pub mod p1out;
#[doc = "P2OUT register accessor: an alias for `Reg<P2OUT_SPEC>`"]
pub type P2OUT = crate::Reg<p2out::P2OUT_SPEC>;
#[doc = "Port 2 Output"]
pub mod p2out;
#[doc = "P1DIR register accessor: an alias for `Reg<P1DIR_SPEC>`"]
pub type P1DIR = crate::Reg<p1dir::P1DIR_SPEC>;
#[doc = "Port 1 Direction"]
pub mod p1dir;
#[doc = "P2DIR register accessor: an alias for `Reg<P2DIR_SPEC>`"]
pub type P2DIR = crate::Reg<p2dir::P2DIR_SPEC>;
#[doc = "Port 2 Direction"]
pub mod p2dir;
#[doc = "P1REN register accessor: an alias for `Reg<P1REN_SPEC>`"]
pub type P1REN = crate::Reg<p1ren::P1REN_SPEC>;
#[doc = "Port 1 Resistor Enable"]
pub mod p1ren;
#[doc = "P2REN register accessor: an alias for `Reg<P2REN_SPEC>`"]
pub type P2REN = crate::Reg<p2ren::P2REN_SPEC>;
#[doc = "Port 2 Resistor Enable"]
pub mod p2ren;
#[doc = "P1SEL0 register accessor: an alias for `Reg<P1SEL0_SPEC>`"]
pub type P1SEL0 = crate::Reg<p1sel0::P1SEL0_SPEC>;
#[doc = "Port 1 Selection 0"]
pub mod p1sel0;
#[doc = "P2SEL0 register accessor: an alias for `Reg<P2SEL0_SPEC>`"]
pub type P2SEL0 = crate::Reg<p2sel0::P2SEL0_SPEC>;
#[doc = "Port 2 Selection 0"]
pub mod p2sel0;
#[doc = "P1SEL1 register accessor: an alias for `Reg<P1SEL1_SPEC>`"]
pub type P1SEL1 = crate::Reg<p1sel1::P1SEL1_SPEC>;
#[doc = "Port 1 Selection 1"]
pub mod p1sel1;
#[doc = "P2SEL1 register accessor: an alias for `Reg<P2SEL1_SPEC>`"]
pub type P2SEL1 = crate::Reg<p2sel1::P2SEL1_SPEC>;
#[doc = "Port 2 Selection 1"]
pub mod p2sel1;
#[doc = "P1SELC register accessor: an alias for `Reg<P1SELC_SPEC>`"]
pub type P1SELC = crate::Reg<p1selc::P1SELC_SPEC>;
#[doc = "Port 1 Complement Selection"]
pub mod p1selc;
#[doc = "P2SELC register accessor: an alias for `Reg<P2SELC_SPEC>`"]
pub type P2SELC = crate::Reg<p2selc::P2SELC_SPEC>;
#[doc = "Port 2 Complement Selection"]
pub mod p2selc;
#[doc = "P1IES register accessor: an alias for `Reg<P1IES_SPEC>`"]
pub type P1IES = crate::Reg<p1ies::P1IES_SPEC>;
#[doc = "Port 1 Interrupt Edge Select"]
pub mod p1ies;
#[doc = "P2IES register accessor: an alias for `Reg<P2IES_SPEC>`"]
pub type P2IES = crate::Reg<p2ies::P2IES_SPEC>;
#[doc = "Port 2 Interrupt Edge Select"]
pub mod p2ies;
#[doc = "P1IE register accessor: an alias for `Reg<P1IE_SPEC>`"]
pub type P1IE = crate::Reg<p1ie::P1IE_SPEC>;
#[doc = "Port 1 Interrupt Enable"]
pub mod p1ie;
#[doc = "P2IE register accessor: an alias for `Reg<P2IE_SPEC>`"]
pub type P2IE = crate::Reg<p2ie::P2IE_SPEC>;
#[doc = "Port 2 Interrupt Enable"]
pub mod p2ie;
#[doc = "P1IFG register accessor: an alias for `Reg<P1IFG_SPEC>`"]
pub type P1IFG = crate::Reg<p1ifg::P1IFG_SPEC>;
#[doc = "Port 1 Interrupt Flag"]
pub mod p1ifg;
#[doc = "P2IFG register accessor: an alias for `Reg<P2IFG_SPEC>`"]
pub type P2IFG = crate::Reg<p2ifg::P2IFG_SPEC>;
#[doc = "Port 2 Interrupt Flag"]
pub mod p2ifg;
#[doc = "P1IV register accessor: an alias for `Reg<P1IV_SPEC>`"]
pub type P1IV = crate::Reg<p1iv::P1IV_SPEC>;
#[doc = "Port 1 Interrupt Vector Word"]
pub mod p1iv;
#[doc = "P2IV register accessor: an alias for `Reg<P2IV_SPEC>`"]
pub type P2IV = crate::Reg<p2iv::P2IV_SPEC>;
#[doc = "Port 2 Interrupt Vector Word"]
pub mod p2iv;
