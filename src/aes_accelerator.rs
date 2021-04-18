#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - AES accelerator control register 0"]
    pub aesactl0: crate::Reg<aesactl0::AESACTL0_SPEC>,
    #[doc = "0x02 - AES accelerator control register 1"]
    pub aesactl1: crate::Reg<aesactl1::AESACTL1_SPEC>,
    #[doc = "0x04 - AES accelerator status register"]
    pub aesastat: crate::Reg<aesastat::AESASTAT_SPEC>,
    #[doc = "0x06 - AES accelerator key register"]
    pub aesakey: crate::Reg<aesakey::AESAKEY_SPEC>,
    #[doc = "0x08 - AES accelerator data in register"]
    pub aesadin: crate::Reg<aesadin::AESADIN_SPEC>,
    #[doc = "0x0a - AES accelerator data out register"]
    pub aesadout: crate::Reg<aesadout::AESADOUT_SPEC>,
    #[doc = "0x0c - AES accelerator XORed data in register"]
    pub aesaxdin: crate::Reg<aesaxdin::AESAXDIN_SPEC>,
    #[doc = "0x0e - AES accelerator XORed data in register (no trigger)"]
    pub aesaxin: crate::Reg<aesaxin::AESAXIN_SPEC>,
}
#[doc = "AESACTL0 register accessor: an alias for `Reg<AESACTL0_SPEC>`"]
pub type AESACTL0 = crate::Reg<aesactl0::AESACTL0_SPEC>;
#[doc = "AES accelerator control register 0"]
pub mod aesactl0;
#[doc = "AESACTL1 register accessor: an alias for `Reg<AESACTL1_SPEC>`"]
pub type AESACTL1 = crate::Reg<aesactl1::AESACTL1_SPEC>;
#[doc = "AES accelerator control register 1"]
pub mod aesactl1;
#[doc = "AESASTAT register accessor: an alias for `Reg<AESASTAT_SPEC>`"]
pub type AESASTAT = crate::Reg<aesastat::AESASTAT_SPEC>;
#[doc = "AES accelerator status register"]
pub mod aesastat;
#[doc = "AESAKEY register accessor: an alias for `Reg<AESAKEY_SPEC>`"]
pub type AESAKEY = crate::Reg<aesakey::AESAKEY_SPEC>;
#[doc = "AES accelerator key register"]
pub mod aesakey;
#[doc = "AESADIN register accessor: an alias for `Reg<AESADIN_SPEC>`"]
pub type AESADIN = crate::Reg<aesadin::AESADIN_SPEC>;
#[doc = "AES accelerator data in register"]
pub mod aesadin;
#[doc = "AESADOUT register accessor: an alias for `Reg<AESADOUT_SPEC>`"]
pub type AESADOUT = crate::Reg<aesadout::AESADOUT_SPEC>;
#[doc = "AES accelerator data out register"]
pub mod aesadout;
#[doc = "AESAXDIN register accessor: an alias for `Reg<AESAXDIN_SPEC>`"]
pub type AESAXDIN = crate::Reg<aesaxdin::AESAXDIN_SPEC>;
#[doc = "AES accelerator XORed data in register"]
pub mod aesaxdin;
#[doc = "AESAXIN register accessor: an alias for `Reg<AESAXIN_SPEC>`"]
pub type AESAXIN = crate::Reg<aesaxin::AESAXIN_SPEC>;
#[doc = "AES accelerator XORed data in register (no trigger)"]
pub mod aesaxin;
