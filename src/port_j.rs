#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Port J Input"]
    pub pjin: crate::Reg<pjin::PJIN_SPEC>,
    #[doc = "0x02 - Port J Output"]
    pub pjout: crate::Reg<pjout::PJOUT_SPEC>,
    #[doc = "0x04 - Port J Direction"]
    pub pjdir: crate::Reg<pjdir::PJDIR_SPEC>,
    #[doc = "0x06 - Port J Resistor Enable"]
    pub pjren: crate::Reg<pjren::PJREN_SPEC>,
    _reserved4: [u8; 2usize],
    #[doc = "0x0a - Port J Selection 0"]
    pub pjsel0: crate::Reg<pjsel0::PJSEL0_SPEC>,
    #[doc = "0x0c - Port J Selection 1"]
    pub pjsel1: crate::Reg<pjsel1::PJSEL1_SPEC>,
    _reserved6: [u8; 8usize],
    #[doc = "0x16 - Port J Complement Selection"]
    pub pjselc: crate::Reg<pjselc::PJSELC_SPEC>,
}
#[doc = "PJIN register accessor: an alias for `Reg<PJIN_SPEC>`"]
pub type PJIN = crate::Reg<pjin::PJIN_SPEC>;
#[doc = "Port J Input"]
pub mod pjin;
#[doc = "PJOUT register accessor: an alias for `Reg<PJOUT_SPEC>`"]
pub type PJOUT = crate::Reg<pjout::PJOUT_SPEC>;
#[doc = "Port J Output"]
pub mod pjout;
#[doc = "PJDIR register accessor: an alias for `Reg<PJDIR_SPEC>`"]
pub type PJDIR = crate::Reg<pjdir::PJDIR_SPEC>;
#[doc = "Port J Direction"]
pub mod pjdir;
#[doc = "PJREN register accessor: an alias for `Reg<PJREN_SPEC>`"]
pub type PJREN = crate::Reg<pjren::PJREN_SPEC>;
#[doc = "Port J Resistor Enable"]
pub mod pjren;
#[doc = "PJSEL0 register accessor: an alias for `Reg<PJSEL0_SPEC>`"]
pub type PJSEL0 = crate::Reg<pjsel0::PJSEL0_SPEC>;
#[doc = "Port J Selection 0"]
pub mod pjsel0;
#[doc = "PJSEL1 register accessor: an alias for `Reg<PJSEL1_SPEC>`"]
pub type PJSEL1 = crate::Reg<pjsel1::PJSEL1_SPEC>;
#[doc = "Port J Selection 1"]
pub mod pjsel1;
#[doc = "PJSELC register accessor: an alias for `Reg<PJSELC_SPEC>`"]
pub type PJSELC = crate::Reg<pjselc::PJSELC_SPEC>;
#[doc = "Port J Complement Selection"]
pub mod pjselc;
