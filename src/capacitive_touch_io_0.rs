#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Capacitive_Touch_IO 0 control register"]
    pub captio0ctl: crate::Reg<captio0ctl::CAPTIO0CTL_SPEC>,
}
#[doc = "CAPTIO0CTL register accessor: an alias for `Reg<CAPTIO0CTL_SPEC>`"]
pub type CAPTIO0CTL = crate::Reg<captio0ctl::CAPTIO0CTL_SPEC>;
#[doc = "Capacitive_Touch_IO 0 control register"]
pub mod captio0ctl;
