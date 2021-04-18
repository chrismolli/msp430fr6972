#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Port 5 Input"]
    pub p5in: crate::Reg<p5in::P5IN_SPEC>,
    #[doc = "0x01 - Port 6 Input"]
    pub p6in: crate::Reg<p6in::P6IN_SPEC>,
    #[doc = "0x02 - Port 5 Output"]
    pub p5out: crate::Reg<p5out::P5OUT_SPEC>,
    #[doc = "0x03 - Port 6 Output"]
    pub p6out: crate::Reg<p6out::P6OUT_SPEC>,
    #[doc = "0x04 - Port 5 Direction"]
    pub p5dir: crate::Reg<p5dir::P5DIR_SPEC>,
    #[doc = "0x05 - Port 6 Direction"]
    pub p6dir: crate::Reg<p6dir::P6DIR_SPEC>,
    #[doc = "0x06 - Port 5 Resistor Enable"]
    pub p5ren: crate::Reg<p5ren::P5REN_SPEC>,
    #[doc = "0x07 - Port 6 Resistor Enable"]
    pub p6ren: crate::Reg<p6ren::P6REN_SPEC>,
    _reserved8: [u8; 2usize],
    #[doc = "0x0a - Port 5 Selection 0"]
    pub p5sel0: crate::Reg<p5sel0::P5SEL0_SPEC>,
    #[doc = "0x0b - Port 6 Selection 0"]
    pub p6sel0: crate::Reg<p6sel0::P6SEL0_SPEC>,
    #[doc = "0x0c - Port 5 Selection 1"]
    pub p5sel1: crate::Reg<p5sel1::P5SEL1_SPEC>,
    #[doc = "0x0d - Port 6 Selection 1"]
    pub p6sel1: crate::Reg<p6sel1::P6SEL1_SPEC>,
    _reserved12: [u8; 8usize],
    #[doc = "0x16 - Port 5 Complement Selection"]
    pub p5selc: crate::Reg<p5selc::P5SELC_SPEC>,
    #[doc = "0x17 - Port 6 Complement Selection"]
    pub p6selc: crate::Reg<p6selc::P6SELC_SPEC>,
}
#[doc = "P5IN register accessor: an alias for `Reg<P5IN_SPEC>`"]
pub type P5IN = crate::Reg<p5in::P5IN_SPEC>;
#[doc = "Port 5 Input"]
pub mod p5in;
#[doc = "P6IN register accessor: an alias for `Reg<P6IN_SPEC>`"]
pub type P6IN = crate::Reg<p6in::P6IN_SPEC>;
#[doc = "Port 6 Input"]
pub mod p6in;
#[doc = "P5OUT register accessor: an alias for `Reg<P5OUT_SPEC>`"]
pub type P5OUT = crate::Reg<p5out::P5OUT_SPEC>;
#[doc = "Port 5 Output"]
pub mod p5out;
#[doc = "P6OUT register accessor: an alias for `Reg<P6OUT_SPEC>`"]
pub type P6OUT = crate::Reg<p6out::P6OUT_SPEC>;
#[doc = "Port 6 Output"]
pub mod p6out;
#[doc = "P5DIR register accessor: an alias for `Reg<P5DIR_SPEC>`"]
pub type P5DIR = crate::Reg<p5dir::P5DIR_SPEC>;
#[doc = "Port 5 Direction"]
pub mod p5dir;
#[doc = "P6DIR register accessor: an alias for `Reg<P6DIR_SPEC>`"]
pub type P6DIR = crate::Reg<p6dir::P6DIR_SPEC>;
#[doc = "Port 6 Direction"]
pub mod p6dir;
#[doc = "P5REN register accessor: an alias for `Reg<P5REN_SPEC>`"]
pub type P5REN = crate::Reg<p5ren::P5REN_SPEC>;
#[doc = "Port 5 Resistor Enable"]
pub mod p5ren;
#[doc = "P6REN register accessor: an alias for `Reg<P6REN_SPEC>`"]
pub type P6REN = crate::Reg<p6ren::P6REN_SPEC>;
#[doc = "Port 6 Resistor Enable"]
pub mod p6ren;
#[doc = "P5SEL0 register accessor: an alias for `Reg<P5SEL0_SPEC>`"]
pub type P5SEL0 = crate::Reg<p5sel0::P5SEL0_SPEC>;
#[doc = "Port 5 Selection 0"]
pub mod p5sel0;
#[doc = "P6SEL0 register accessor: an alias for `Reg<P6SEL0_SPEC>`"]
pub type P6SEL0 = crate::Reg<p6sel0::P6SEL0_SPEC>;
#[doc = "Port 6 Selection 0"]
pub mod p6sel0;
#[doc = "P5SEL1 register accessor: an alias for `Reg<P5SEL1_SPEC>`"]
pub type P5SEL1 = crate::Reg<p5sel1::P5SEL1_SPEC>;
#[doc = "Port 5 Selection 1"]
pub mod p5sel1;
#[doc = "P6SEL1 register accessor: an alias for `Reg<P6SEL1_SPEC>`"]
pub type P6SEL1 = crate::Reg<p6sel1::P6SEL1_SPEC>;
#[doc = "Port 6 Selection 1"]
pub mod p6sel1;
#[doc = "P5SELC register accessor: an alias for `Reg<P5SELC_SPEC>`"]
pub type P5SELC = crate::Reg<p5selc::P5SELC_SPEC>;
#[doc = "Port 5 Complement Selection"]
pub mod p5selc;
#[doc = "P6SELC register accessor: an alias for `Reg<P6SELC_SPEC>`"]
pub type P6SELC = crate::Reg<p6selc::P6SELC_SPEC>;
#[doc = "Port 6 Complement Selection"]
pub mod p6selc;
