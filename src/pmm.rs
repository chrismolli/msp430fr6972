#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - PMM Control 0"]
    pub pmmctl0: crate::Reg<pmmctl0::PMMCTL0_SPEC>,
    _reserved1: [u8; 8usize],
    #[doc = "0x0a - PMM Interrupt Flag"]
    pub pmmifg: crate::Reg<pmmifg::PMMIFG_SPEC>,
    _reserved2: [u8; 4usize],
    #[doc = "0x10 - PMM Power Mode 5 Control Register 0"]
    pub pm5ctl0: crate::Reg<pm5ctl0::PM5CTL0_SPEC>,
}
#[doc = "PMMCTL0 register accessor: an alias for `Reg<PMMCTL0_SPEC>`"]
pub type PMMCTL0 = crate::Reg<pmmctl0::PMMCTL0_SPEC>;
#[doc = "PMM Control 0"]
pub mod pmmctl0;
#[doc = "PMMIFG register accessor: an alias for `Reg<PMMIFG_SPEC>`"]
pub type PMMIFG = crate::Reg<pmmifg::PMMIFG_SPEC>;
#[doc = "PMM Interrupt Flag"]
pub mod pmmifg;
#[doc = "PM5CTL0 register accessor: an alias for `Reg<PM5CTL0_SPEC>`"]
pub type PM5CTL0 = crate::Reg<pm5ctl0::PM5CTL0_SPEC>;
#[doc = "PMM Power Mode 5 Control Register 0"]
pub mod pm5ctl0;
