#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Capacitive_Touch_IO 1 control register"]
    pub captio1ctl: crate::Reg<captio1ctl::CAPTIO1CTL_SPEC>,
}
#[doc = "CAPTIO1CTL register accessor: an alias for `Reg<CAPTIO1CTL_SPEC>`"]
pub type CAPTIO1CTL = crate::Reg<captio1ctl::CAPTIO1CTL_SPEC>;
#[doc = "Capacitive_Touch_IO 1 control register"]
pub mod captio1ctl;
