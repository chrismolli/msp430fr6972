#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - CS Control Register 0"]
    pub csctl0: crate::Reg<csctl0::CSCTL0_SPEC>,
    #[doc = "0x02 - CS Control Register 1"]
    pub csctl1: crate::Reg<csctl1::CSCTL1_SPEC>,
    #[doc = "0x04 - CS Control Register 2"]
    pub csctl2: crate::Reg<csctl2::CSCTL2_SPEC>,
    #[doc = "0x06 - CS Control Register 3"]
    pub csctl3: crate::Reg<csctl3::CSCTL3_SPEC>,
    #[doc = "0x08 - CS Control Register 4"]
    pub csctl4: crate::Reg<csctl4::CSCTL4_SPEC>,
    #[doc = "0x0a - CS Control Register 5"]
    pub csctl5: crate::Reg<csctl5::CSCTL5_SPEC>,
    #[doc = "0x0c - CS Control Register 6"]
    pub csctl6: crate::Reg<csctl6::CSCTL6_SPEC>,
}
#[doc = "CSCTL0 register accessor: an alias for `Reg<CSCTL0_SPEC>`"]
pub type CSCTL0 = crate::Reg<csctl0::CSCTL0_SPEC>;
#[doc = "CS Control Register 0"]
pub mod csctl0;
#[doc = "CSCTL1 register accessor: an alias for `Reg<CSCTL1_SPEC>`"]
pub type CSCTL1 = crate::Reg<csctl1::CSCTL1_SPEC>;
#[doc = "CS Control Register 1"]
pub mod csctl1;
#[doc = "CSCTL2 register accessor: an alias for `Reg<CSCTL2_SPEC>`"]
pub type CSCTL2 = crate::Reg<csctl2::CSCTL2_SPEC>;
#[doc = "CS Control Register 2"]
pub mod csctl2;
#[doc = "CSCTL3 register accessor: an alias for `Reg<CSCTL3_SPEC>`"]
pub type CSCTL3 = crate::Reg<csctl3::CSCTL3_SPEC>;
#[doc = "CS Control Register 3"]
pub mod csctl3;
#[doc = "CSCTL4 register accessor: an alias for `Reg<CSCTL4_SPEC>`"]
pub type CSCTL4 = crate::Reg<csctl4::CSCTL4_SPEC>;
#[doc = "CS Control Register 4"]
pub mod csctl4;
#[doc = "CSCTL5 register accessor: an alias for `Reg<CSCTL5_SPEC>`"]
pub type CSCTL5 = crate::Reg<csctl5::CSCTL5_SPEC>;
#[doc = "CS Control Register 5"]
pub mod csctl5;
#[doc = "CSCTL6 register accessor: an alias for `Reg<CSCTL6_SPEC>`"]
pub type CSCTL6 = crate::Reg<csctl6::CSCTL6_SPEC>;
#[doc = "CS Control Register 6"]
pub mod csctl6;
