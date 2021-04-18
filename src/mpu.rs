#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - MPU Control Register 0"]
    pub mpuctl0: crate::Reg<mpuctl0::MPUCTL0_SPEC>,
    #[doc = "0x02 - MPU Control Register 1"]
    pub mpuctl1: crate::Reg<mpuctl1::MPUCTL1_SPEC>,
    #[doc = "0x04 - MPU Segmentation Border 2 Register"]
    pub mpusegb2: crate::Reg<mpusegb2::MPUSEGB2_SPEC>,
    #[doc = "0x06 - MPU Segmentation Border 1 Register"]
    pub mpusegb1: crate::Reg<mpusegb1::MPUSEGB1_SPEC>,
    #[doc = "0x08 - MPU Access Management Register"]
    pub mpusam: crate::Reg<mpusam::MPUSAM_SPEC>,
    #[doc = "0x0a - MPU IP Control 0 Register"]
    pub mpuipc0: crate::Reg<mpuipc0::MPUIPC0_SPEC>,
    #[doc = "0x0c - MPU IP Segment Border 2 Register"]
    pub mpuipsegb2: crate::Reg<mpuipsegb2::MPUIPSEGB2_SPEC>,
    #[doc = "0x0e - MPU IP Segment Border 1 Register"]
    pub mpuipsegb1: crate::Reg<mpuipsegb1::MPUIPSEGB1_SPEC>,
}
#[doc = "MPUCTL0 register accessor: an alias for `Reg<MPUCTL0_SPEC>`"]
pub type MPUCTL0 = crate::Reg<mpuctl0::MPUCTL0_SPEC>;
#[doc = "MPU Control Register 0"]
pub mod mpuctl0;
#[doc = "MPUCTL1 register accessor: an alias for `Reg<MPUCTL1_SPEC>`"]
pub type MPUCTL1 = crate::Reg<mpuctl1::MPUCTL1_SPEC>;
#[doc = "MPU Control Register 1"]
pub mod mpuctl1;
#[doc = "MPUSEGB2 register accessor: an alias for `Reg<MPUSEGB2_SPEC>`"]
pub type MPUSEGB2 = crate::Reg<mpusegb2::MPUSEGB2_SPEC>;
#[doc = "MPU Segmentation Border 2 Register"]
pub mod mpusegb2;
#[doc = "MPUSEGB1 register accessor: an alias for `Reg<MPUSEGB1_SPEC>`"]
pub type MPUSEGB1 = crate::Reg<mpusegb1::MPUSEGB1_SPEC>;
#[doc = "MPU Segmentation Border 1 Register"]
pub mod mpusegb1;
#[doc = "MPUSAM register accessor: an alias for `Reg<MPUSAM_SPEC>`"]
pub type MPUSAM = crate::Reg<mpusam::MPUSAM_SPEC>;
#[doc = "MPU Access Management Register"]
pub mod mpusam;
#[doc = "MPUIPC0 register accessor: an alias for `Reg<MPUIPC0_SPEC>`"]
pub type MPUIPC0 = crate::Reg<mpuipc0::MPUIPC0_SPEC>;
#[doc = "MPU IP Control 0 Register"]
pub mod mpuipc0;
#[doc = "MPUIPSEGB2 register accessor: an alias for `Reg<MPUIPSEGB2_SPEC>`"]
pub type MPUIPSEGB2 = crate::Reg<mpuipsegb2::MPUIPSEGB2_SPEC>;
#[doc = "MPU IP Segment Border 2 Register"]
pub mod mpuipsegb2;
#[doc = "MPUIPSEGB1 register accessor: an alias for `Reg<MPUIPSEGB1_SPEC>`"]
pub type MPUIPSEGB1 = crate::Reg<mpuipsegb1::MPUIPSEGB1_SPEC>;
#[doc = "MPU IP Segment Border 1 Register"]
pub mod mpuipsegb1;
