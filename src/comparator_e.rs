#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Comparator E Control Register 0"]
    pub cectl0: crate::Reg<cectl0::CECTL0_SPEC>,
    #[doc = "0x02 - Comparator E Control Register 1"]
    pub cectl1: crate::Reg<cectl1::CECTL1_SPEC>,
    #[doc = "0x04 - Comparator E Control Register 2"]
    pub cectl2: crate::Reg<cectl2::CECTL2_SPEC>,
    #[doc = "0x06 - Comparator E Control Register 3"]
    pub cectl3: crate::Reg<cectl3::CECTL3_SPEC>,
    _reserved4: [u8; 4usize],
    #[doc = "0x0c - Comparator E Interrupt Register"]
    pub ceint: crate::Reg<ceint::CEINT_SPEC>,
    #[doc = "0x0e - Comparator E Interrupt Vector Word"]
    pub ceiv: crate::Reg<ceiv::CEIV_SPEC>,
}
#[doc = "CECTL0 register accessor: an alias for `Reg<CECTL0_SPEC>`"]
pub type CECTL0 = crate::Reg<cectl0::CECTL0_SPEC>;
#[doc = "Comparator E Control Register 0"]
pub mod cectl0;
#[doc = "CECTL1 register accessor: an alias for `Reg<CECTL1_SPEC>`"]
pub type CECTL1 = crate::Reg<cectl1::CECTL1_SPEC>;
#[doc = "Comparator E Control Register 1"]
pub mod cectl1;
#[doc = "CECTL2 register accessor: an alias for `Reg<CECTL2_SPEC>`"]
pub type CECTL2 = crate::Reg<cectl2::CECTL2_SPEC>;
#[doc = "Comparator E Control Register 2"]
pub mod cectl2;
#[doc = "CECTL3 register accessor: an alias for `Reg<CECTL3_SPEC>`"]
pub type CECTL3 = crate::Reg<cectl3::CECTL3_SPEC>;
#[doc = "Comparator E Control Register 3"]
pub mod cectl3;
#[doc = "CEINT register accessor: an alias for `Reg<CEINT_SPEC>`"]
pub type CEINT = crate::Reg<ceint::CEINT_SPEC>;
#[doc = "Comparator E Interrupt Register"]
pub mod ceint;
#[doc = "CEIV register accessor: an alias for `Reg<CEIV_SPEC>`"]
pub type CEIV = crate::Reg<ceiv::CEIV_SPEC>;
#[doc = "Comparator E Interrupt Vector Word"]
pub mod ceiv;
