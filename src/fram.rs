#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - FRAM Controller Control 0"]
    pub frctl0: crate::Reg<frctl0::FRCTL0_SPEC>,
    _reserved1: [u8; 2usize],
    #[doc = "0x04 - General Control 0"]
    pub gcctl0: crate::Reg<gcctl0::GCCTL0_SPEC>,
    #[doc = "0x06 - General Control 1"]
    pub gcctl1: crate::Reg<gcctl1::GCCTL1_SPEC>,
}
#[doc = "FRCTL0 register accessor: an alias for `Reg<FRCTL0_SPEC>`"]
pub type FRCTL0 = crate::Reg<frctl0::FRCTL0_SPEC>;
#[doc = "FRAM Controller Control 0"]
pub mod frctl0;
#[doc = "GCCTL0 register accessor: an alias for `Reg<GCCTL0_SPEC>`"]
pub type GCCTL0 = crate::Reg<gcctl0::GCCTL0_SPEC>;
#[doc = "General Control 0"]
pub mod gcctl0;
#[doc = "GCCTL1 register accessor: an alias for `Reg<GCCTL1_SPEC>`"]
pub type GCCTL1 = crate::Reg<gcctl1::GCCTL1_SPEC>;
#[doc = "General Control 1"]
pub mod gcctl1;
