#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Timer3_A5 Control"]
    pub ta3ctl: crate::Reg<ta3ctl::TA3CTL_SPEC>,
    #[doc = "0x02 - Timer3_A5 Capture/Compare Control 0"]
    pub ta3cctl0: crate::Reg<ta3cctl0::TA3CCTL0_SPEC>,
    #[doc = "0x04 - Timer3_A5 Capture/Compare Control 1"]
    pub ta3cctl1: crate::Reg<ta3cctl1::TA3CCTL1_SPEC>,
    #[doc = "0x06 - Timer3_A5 Capture/Compare Control 2"]
    pub ta3cctl2: crate::Reg<ta3cctl2::TA3CCTL2_SPEC>,
    #[doc = "0x08 - Timer3_A5 Capture/Compare Control 3"]
    pub ta3cctl3: crate::Reg<ta3cctl3::TA3CCTL3_SPEC>,
    #[doc = "0x0a - Timer3_A5 Capture/Compare Control 4"]
    pub ta3cctl4: crate::Reg<ta3cctl4::TA3CCTL4_SPEC>,
    _reserved6: [u8; 4usize],
    #[doc = "0x10 - Timer3_A5"]
    pub ta3r: crate::Reg<ta3r::TA3R_SPEC>,
    #[doc = "0x12 - Timer3_A5 Capture/Compare 0"]
    pub ta3ccr0: crate::Reg<ta3ccr0::TA3CCR0_SPEC>,
    #[doc = "0x14 - Timer3_A5 Capture/Compare 1"]
    pub ta3ccr1: crate::Reg<ta3ccr1::TA3CCR1_SPEC>,
    #[doc = "0x16 - Timer3_A5 Capture/Compare 2"]
    pub ta3ccr2: crate::Reg<ta3ccr2::TA3CCR2_SPEC>,
    #[doc = "0x18 - Timer3_A5 Capture/Compare 3"]
    pub ta3ccr3: crate::Reg<ta3ccr3::TA3CCR3_SPEC>,
    #[doc = "0x1a - Timer3_A5 Capture/Compare 4"]
    pub ta3ccr4: crate::Reg<ta3ccr4::TA3CCR4_SPEC>,
    _reserved12: [u8; 4usize],
    #[doc = "0x20 - Timer3_A5 Expansion Register 0"]
    pub ta3ex0: crate::Reg<ta3ex0::TA3EX0_SPEC>,
    _reserved13: [u8; 12usize],
    #[doc = "0x2e - Timer3_A5 Interrupt Vector Word"]
    pub ta3iv: crate::Reg<ta3iv::TA3IV_SPEC>,
}
#[doc = "TA3CTL register accessor: an alias for `Reg<TA3CTL_SPEC>`"]
pub type TA3CTL = crate::Reg<ta3ctl::TA3CTL_SPEC>;
#[doc = "Timer3_A5 Control"]
pub mod ta3ctl;
#[doc = "TA3CCTL0 register accessor: an alias for `Reg<TA3CCTL0_SPEC>`"]
pub type TA3CCTL0 = crate::Reg<ta3cctl0::TA3CCTL0_SPEC>;
#[doc = "Timer3_A5 Capture/Compare Control 0"]
pub mod ta3cctl0;
#[doc = "TA3CCTL1 register accessor: an alias for `Reg<TA3CCTL1_SPEC>`"]
pub type TA3CCTL1 = crate::Reg<ta3cctl1::TA3CCTL1_SPEC>;
#[doc = "Timer3_A5 Capture/Compare Control 1"]
pub mod ta3cctl1;
#[doc = "TA3CCTL2 register accessor: an alias for `Reg<TA3CCTL2_SPEC>`"]
pub type TA3CCTL2 = crate::Reg<ta3cctl2::TA3CCTL2_SPEC>;
#[doc = "Timer3_A5 Capture/Compare Control 2"]
pub mod ta3cctl2;
#[doc = "TA3CCTL3 register accessor: an alias for `Reg<TA3CCTL3_SPEC>`"]
pub type TA3CCTL3 = crate::Reg<ta3cctl3::TA3CCTL3_SPEC>;
#[doc = "Timer3_A5 Capture/Compare Control 3"]
pub mod ta3cctl3;
#[doc = "TA3CCTL4 register accessor: an alias for `Reg<TA3CCTL4_SPEC>`"]
pub type TA3CCTL4 = crate::Reg<ta3cctl4::TA3CCTL4_SPEC>;
#[doc = "Timer3_A5 Capture/Compare Control 4"]
pub mod ta3cctl4;
#[doc = "TA3R register accessor: an alias for `Reg<TA3R_SPEC>`"]
pub type TA3R = crate::Reg<ta3r::TA3R_SPEC>;
#[doc = "Timer3_A5"]
pub mod ta3r;
#[doc = "TA3CCR0 register accessor: an alias for `Reg<TA3CCR0_SPEC>`"]
pub type TA3CCR0 = crate::Reg<ta3ccr0::TA3CCR0_SPEC>;
#[doc = "Timer3_A5 Capture/Compare 0"]
pub mod ta3ccr0;
#[doc = "TA3CCR1 register accessor: an alias for `Reg<TA3CCR1_SPEC>`"]
pub type TA3CCR1 = crate::Reg<ta3ccr1::TA3CCR1_SPEC>;
#[doc = "Timer3_A5 Capture/Compare 1"]
pub mod ta3ccr1;
#[doc = "TA3CCR2 register accessor: an alias for `Reg<TA3CCR2_SPEC>`"]
pub type TA3CCR2 = crate::Reg<ta3ccr2::TA3CCR2_SPEC>;
#[doc = "Timer3_A5 Capture/Compare 2"]
pub mod ta3ccr2;
#[doc = "TA3CCR3 register accessor: an alias for `Reg<TA3CCR3_SPEC>`"]
pub type TA3CCR3 = crate::Reg<ta3ccr3::TA3CCR3_SPEC>;
#[doc = "Timer3_A5 Capture/Compare 3"]
pub mod ta3ccr3;
#[doc = "TA3CCR4 register accessor: an alias for `Reg<TA3CCR4_SPEC>`"]
pub type TA3CCR4 = crate::Reg<ta3ccr4::TA3CCR4_SPEC>;
#[doc = "Timer3_A5 Capture/Compare 4"]
pub mod ta3ccr4;
#[doc = "TA3EX0 register accessor: an alias for `Reg<TA3EX0_SPEC>`"]
pub type TA3EX0 = crate::Reg<ta3ex0::TA3EX0_SPEC>;
#[doc = "Timer3_A5 Expansion Register 0"]
pub mod ta3ex0;
#[doc = "TA3IV register accessor: an alias for `Reg<TA3IV_SPEC>`"]
pub type TA3IV = crate::Reg<ta3iv::TA3IV_SPEC>;
#[doc = "Timer3_A5 Interrupt Vector Word"]
pub mod ta3iv;
