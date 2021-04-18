#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - ADC12 B Control 0"]
    pub adc12ctl0: crate::Reg<adc12ctl0::ADC12CTL0_SPEC>,
    #[doc = "0x02 - ADC12 B Control 1"]
    pub adc12ctl1: crate::Reg<adc12ctl1::ADC12CTL1_SPEC>,
    #[doc = "0x04 - ADC12 B Control 2"]
    pub adc12ctl2: crate::Reg<adc12ctl2::ADC12CTL2_SPEC>,
    #[doc = "0x06 - ADC12 B Control 3"]
    pub adc12ctl3: crate::Reg<adc12ctl3::ADC12CTL3_SPEC>,
    #[doc = "0x08 - ADC12 B Window Comparator High Threshold"]
    pub adc12lo: crate::Reg<adc12lo::ADC12LO_SPEC>,
    #[doc = "0x0a - ADC12 B Window Comparator High Threshold"]
    pub adc12hi: crate::Reg<adc12hi::ADC12HI_SPEC>,
    #[doc = "0x0c - ADC12 B Interrupt Flag 0"]
    pub adc12ifgr0: crate::Reg<adc12ifgr0::ADC12IFGR0_SPEC>,
    #[doc = "0x0e - ADC12 B Interrupt Flag 1"]
    pub adc12ifgr1: crate::Reg<adc12ifgr1::ADC12IFGR1_SPEC>,
    #[doc = "0x10 - ADC12 B Interrupt Flag 2"]
    pub adc12ifgr2: crate::Reg<adc12ifgr2::ADC12IFGR2_SPEC>,
    #[doc = "0x12 - ADC12 B Interrupt Enable 0"]
    pub adc12ier0: crate::Reg<adc12ier0::ADC12IER0_SPEC>,
    #[doc = "0x14 - ADC12 B Interrupt Enable 1"]
    pub adc12ier1: crate::Reg<adc12ier1::ADC12IER1_SPEC>,
    #[doc = "0x16 - ADC12 B Interrupt Enable 2"]
    pub adc12ier2: crate::Reg<adc12ier2::ADC12IER2_SPEC>,
    #[doc = "0x18 - ADC12 B Interrupt Vector Word"]
    pub adc12iv: crate::Reg<adc12iv::ADC12IV_SPEC>,
    _reserved13: [u8; 6usize],
    #[doc = "0x20 - ADC12 Memory Control 0"]
    pub adc12mctl0: crate::Reg<adc12mctl0::ADC12MCTL0_SPEC>,
    #[doc = "0x22 - ADC12 Memory Control 1"]
    pub adc12mctl1: crate::Reg<adc12mctl1::ADC12MCTL1_SPEC>,
    #[doc = "0x24 - ADC12 Memory Control 2"]
    pub adc12mctl2: crate::Reg<adc12mctl2::ADC12MCTL2_SPEC>,
    #[doc = "0x26 - ADC12 Memory Control 3"]
    pub adc12mctl3: crate::Reg<adc12mctl3::ADC12MCTL3_SPEC>,
    #[doc = "0x28 - ADC12 Memory Control 4"]
    pub adc12mctl4: crate::Reg<adc12mctl4::ADC12MCTL4_SPEC>,
    #[doc = "0x2a - ADC12 Memory Control 5"]
    pub adc12mctl5: crate::Reg<adc12mctl5::ADC12MCTL5_SPEC>,
    #[doc = "0x2c - ADC12 Memory Control 6"]
    pub adc12mctl6: crate::Reg<adc12mctl6::ADC12MCTL6_SPEC>,
    #[doc = "0x2e - ADC12 Memory Control 7"]
    pub adc12mctl7: crate::Reg<adc12mctl7::ADC12MCTL7_SPEC>,
    #[doc = "0x30 - ADC12 Memory Control 8"]
    pub adc12mctl8: crate::Reg<adc12mctl8::ADC12MCTL8_SPEC>,
    #[doc = "0x32 - ADC12 Memory Control 9"]
    pub adc12mctl9: crate::Reg<adc12mctl9::ADC12MCTL9_SPEC>,
    #[doc = "0x34 - ADC12 Memory Control 10"]
    pub adc12mctl10: crate::Reg<adc12mctl10::ADC12MCTL10_SPEC>,
    #[doc = "0x36 - ADC12 Memory Control 11"]
    pub adc12mctl11: crate::Reg<adc12mctl11::ADC12MCTL11_SPEC>,
    #[doc = "0x38 - ADC12 Memory Control 12"]
    pub adc12mctl12: crate::Reg<adc12mctl12::ADC12MCTL12_SPEC>,
    #[doc = "0x3a - ADC12 Memory Control 13"]
    pub adc12mctl13: crate::Reg<adc12mctl13::ADC12MCTL13_SPEC>,
    #[doc = "0x3c - ADC12 Memory Control 14"]
    pub adc12mctl14: crate::Reg<adc12mctl14::ADC12MCTL14_SPEC>,
    #[doc = "0x3e - ADC12 Memory Control 15"]
    pub adc12mctl15: crate::Reg<adc12mctl15::ADC12MCTL15_SPEC>,
    #[doc = "0x40 - ADC12 Memory Control 16"]
    pub adc12mctl16: crate::Reg<adc12mctl16::ADC12MCTL16_SPEC>,
    #[doc = "0x42 - ADC12 Memory Control 17"]
    pub adc12mctl17: crate::Reg<adc12mctl17::ADC12MCTL17_SPEC>,
    #[doc = "0x44 - ADC12 Memory Control 18"]
    pub adc12mctl18: crate::Reg<adc12mctl18::ADC12MCTL18_SPEC>,
    #[doc = "0x46 - ADC12 Memory Control 19"]
    pub adc12mctl19: crate::Reg<adc12mctl19::ADC12MCTL19_SPEC>,
    #[doc = "0x48 - ADC12 Memory Control 20"]
    pub adc12mctl20: crate::Reg<adc12mctl20::ADC12MCTL20_SPEC>,
    #[doc = "0x4a - ADC12 Memory Control 21"]
    pub adc12mctl21: crate::Reg<adc12mctl21::ADC12MCTL21_SPEC>,
    #[doc = "0x4c - ADC12 Memory Control 22"]
    pub adc12mctl22: crate::Reg<adc12mctl22::ADC12MCTL22_SPEC>,
    #[doc = "0x4e - ADC12 Memory Control 23"]
    pub adc12mctl23: crate::Reg<adc12mctl23::ADC12MCTL23_SPEC>,
    #[doc = "0x50 - ADC12 Memory Control 24"]
    pub adc12mctl24: crate::Reg<adc12mctl24::ADC12MCTL24_SPEC>,
    #[doc = "0x52 - ADC12 Memory Control 25"]
    pub adc12mctl25: crate::Reg<adc12mctl25::ADC12MCTL25_SPEC>,
    #[doc = "0x54 - ADC12 Memory Control 26"]
    pub adc12mctl26: crate::Reg<adc12mctl26::ADC12MCTL26_SPEC>,
    #[doc = "0x56 - ADC12 Memory Control 27"]
    pub adc12mctl27: crate::Reg<adc12mctl27::ADC12MCTL27_SPEC>,
    #[doc = "0x58 - ADC12 Memory Control 28"]
    pub adc12mctl28: crate::Reg<adc12mctl28::ADC12MCTL28_SPEC>,
    #[doc = "0x5a - ADC12 Memory Control 29"]
    pub adc12mctl29: crate::Reg<adc12mctl29::ADC12MCTL29_SPEC>,
    #[doc = "0x5c - ADC12 Memory Control 30"]
    pub adc12mctl30: crate::Reg<adc12mctl30::ADC12MCTL30_SPEC>,
    #[doc = "0x5e - ADC12 Memory Control 31"]
    pub adc12mctl31: crate::Reg<adc12mctl31::ADC12MCTL31_SPEC>,
    #[doc = "0x60 - ADC12 Conversion Memory 0"]
    pub adc12mem0: crate::Reg<adc12mem0::ADC12MEM0_SPEC>,
    #[doc = "0x62 - ADC12 Conversion Memory 1"]
    pub adc12mem1: crate::Reg<adc12mem1::ADC12MEM1_SPEC>,
    #[doc = "0x64 - ADC12 Conversion Memory 2"]
    pub adc12mem2: crate::Reg<adc12mem2::ADC12MEM2_SPEC>,
    #[doc = "0x66 - ADC12 Conversion Memory 3"]
    pub adc12mem3: crate::Reg<adc12mem3::ADC12MEM3_SPEC>,
    #[doc = "0x68 - ADC12 Conversion Memory 4"]
    pub adc12mem4: crate::Reg<adc12mem4::ADC12MEM4_SPEC>,
    #[doc = "0x6a - ADC12 Conversion Memory 5"]
    pub adc12mem5: crate::Reg<adc12mem5::ADC12MEM5_SPEC>,
    #[doc = "0x6c - ADC12 Conversion Memory 6"]
    pub adc12mem6: crate::Reg<adc12mem6::ADC12MEM6_SPEC>,
    #[doc = "0x6e - ADC12 Conversion Memory 7"]
    pub adc12mem7: crate::Reg<adc12mem7::ADC12MEM7_SPEC>,
    #[doc = "0x70 - ADC12 Conversion Memory 8"]
    pub adc12mem8: crate::Reg<adc12mem8::ADC12MEM8_SPEC>,
    #[doc = "0x72 - ADC12 Conversion Memory 9"]
    pub adc12mem9: crate::Reg<adc12mem9::ADC12MEM9_SPEC>,
    #[doc = "0x74 - ADC12 Conversion Memory 10"]
    pub adc12mem10: crate::Reg<adc12mem10::ADC12MEM10_SPEC>,
    #[doc = "0x76 - ADC12 Conversion Memory 11"]
    pub adc12mem11: crate::Reg<adc12mem11::ADC12MEM11_SPEC>,
    #[doc = "0x78 - ADC12 Conversion Memory 12"]
    pub adc12mem12: crate::Reg<adc12mem12::ADC12MEM12_SPEC>,
    #[doc = "0x7a - ADC12 Conversion Memory 13"]
    pub adc12mem13: crate::Reg<adc12mem13::ADC12MEM13_SPEC>,
    #[doc = "0x7c - ADC12 Conversion Memory 14"]
    pub adc12mem14: crate::Reg<adc12mem14::ADC12MEM14_SPEC>,
    #[doc = "0x7e - ADC12 Conversion Memory 15"]
    pub adc12mem15: crate::Reg<adc12mem15::ADC12MEM15_SPEC>,
    #[doc = "0x80 - ADC12 Conversion Memory 16"]
    pub adc12mem16: crate::Reg<adc12mem16::ADC12MEM16_SPEC>,
    #[doc = "0x82 - ADC12 Conversion Memory 17"]
    pub adc12mem17: crate::Reg<adc12mem17::ADC12MEM17_SPEC>,
    #[doc = "0x84 - ADC12 Conversion Memory 18"]
    pub adc12mem18: crate::Reg<adc12mem18::ADC12MEM18_SPEC>,
    #[doc = "0x86 - ADC12 Conversion Memory 19"]
    pub adc12mem19: crate::Reg<adc12mem19::ADC12MEM19_SPEC>,
    #[doc = "0x88 - ADC12 Conversion Memory 20"]
    pub adc12mem20: crate::Reg<adc12mem20::ADC12MEM20_SPEC>,
    #[doc = "0x8a - ADC12 Conversion Memory 21"]
    pub adc12mem21: crate::Reg<adc12mem21::ADC12MEM21_SPEC>,
    #[doc = "0x8c - ADC12 Conversion Memory 22"]
    pub adc12mem22: crate::Reg<adc12mem22::ADC12MEM22_SPEC>,
    #[doc = "0x8e - ADC12 Conversion Memory 23"]
    pub adc12mem23: crate::Reg<adc12mem23::ADC12MEM23_SPEC>,
    #[doc = "0x90 - ADC12 Conversion Memory 24"]
    pub adc12mem24: crate::Reg<adc12mem24::ADC12MEM24_SPEC>,
    #[doc = "0x92 - ADC12 Conversion Memory 25"]
    pub adc12mem25: crate::Reg<adc12mem25::ADC12MEM25_SPEC>,
    #[doc = "0x94 - ADC12 Conversion Memory 26"]
    pub adc12mem26: crate::Reg<adc12mem26::ADC12MEM26_SPEC>,
    #[doc = "0x96 - ADC12 Conversion Memory 27"]
    pub adc12mem27: crate::Reg<adc12mem27::ADC12MEM27_SPEC>,
    #[doc = "0x98 - ADC12 Conversion Memory 28"]
    pub adc12mem28: crate::Reg<adc12mem28::ADC12MEM28_SPEC>,
    #[doc = "0x9a - ADC12 Conversion Memory 29"]
    pub adc12mem29: crate::Reg<adc12mem29::ADC12MEM29_SPEC>,
    #[doc = "0x9c - ADC12 Conversion Memory 30"]
    pub adc12mem30: crate::Reg<adc12mem30::ADC12MEM30_SPEC>,
    #[doc = "0x9e - ADC12 Conversion Memory 31"]
    pub adc12mem31: crate::Reg<adc12mem31::ADC12MEM31_SPEC>,
}
#[doc = "ADC12CTL0 register accessor: an alias for `Reg<ADC12CTL0_SPEC>`"]
pub type ADC12CTL0 = crate::Reg<adc12ctl0::ADC12CTL0_SPEC>;
#[doc = "ADC12 B Control 0"]
pub mod adc12ctl0;
#[doc = "ADC12CTL1 register accessor: an alias for `Reg<ADC12CTL1_SPEC>`"]
pub type ADC12CTL1 = crate::Reg<adc12ctl1::ADC12CTL1_SPEC>;
#[doc = "ADC12 B Control 1"]
pub mod adc12ctl1;
#[doc = "ADC12CTL2 register accessor: an alias for `Reg<ADC12CTL2_SPEC>`"]
pub type ADC12CTL2 = crate::Reg<adc12ctl2::ADC12CTL2_SPEC>;
#[doc = "ADC12 B Control 2"]
pub mod adc12ctl2;
#[doc = "ADC12CTL3 register accessor: an alias for `Reg<ADC12CTL3_SPEC>`"]
pub type ADC12CTL3 = crate::Reg<adc12ctl3::ADC12CTL3_SPEC>;
#[doc = "ADC12 B Control 3"]
pub mod adc12ctl3;
#[doc = "ADC12LO register accessor: an alias for `Reg<ADC12LO_SPEC>`"]
pub type ADC12LO = crate::Reg<adc12lo::ADC12LO_SPEC>;
#[doc = "ADC12 B Window Comparator High Threshold"]
pub mod adc12lo;
#[doc = "ADC12HI register accessor: an alias for `Reg<ADC12HI_SPEC>`"]
pub type ADC12HI = crate::Reg<adc12hi::ADC12HI_SPEC>;
#[doc = "ADC12 B Window Comparator High Threshold"]
pub mod adc12hi;
#[doc = "ADC12IFGR0 register accessor: an alias for `Reg<ADC12IFGR0_SPEC>`"]
pub type ADC12IFGR0 = crate::Reg<adc12ifgr0::ADC12IFGR0_SPEC>;
#[doc = "ADC12 B Interrupt Flag 0"]
pub mod adc12ifgr0;
#[doc = "ADC12IFGR1 register accessor: an alias for `Reg<ADC12IFGR1_SPEC>`"]
pub type ADC12IFGR1 = crate::Reg<adc12ifgr1::ADC12IFGR1_SPEC>;
#[doc = "ADC12 B Interrupt Flag 1"]
pub mod adc12ifgr1;
#[doc = "ADC12IFGR2 register accessor: an alias for `Reg<ADC12IFGR2_SPEC>`"]
pub type ADC12IFGR2 = crate::Reg<adc12ifgr2::ADC12IFGR2_SPEC>;
#[doc = "ADC12 B Interrupt Flag 2"]
pub mod adc12ifgr2;
#[doc = "ADC12IER0 register accessor: an alias for `Reg<ADC12IER0_SPEC>`"]
pub type ADC12IER0 = crate::Reg<adc12ier0::ADC12IER0_SPEC>;
#[doc = "ADC12 B Interrupt Enable 0"]
pub mod adc12ier0;
#[doc = "ADC12IER1 register accessor: an alias for `Reg<ADC12IER1_SPEC>`"]
pub type ADC12IER1 = crate::Reg<adc12ier1::ADC12IER1_SPEC>;
#[doc = "ADC12 B Interrupt Enable 1"]
pub mod adc12ier1;
#[doc = "ADC12IER2 register accessor: an alias for `Reg<ADC12IER2_SPEC>`"]
pub type ADC12IER2 = crate::Reg<adc12ier2::ADC12IER2_SPEC>;
#[doc = "ADC12 B Interrupt Enable 2"]
pub mod adc12ier2;
#[doc = "ADC12IV register accessor: an alias for `Reg<ADC12IV_SPEC>`"]
pub type ADC12IV = crate::Reg<adc12iv::ADC12IV_SPEC>;
#[doc = "ADC12 B Interrupt Vector Word"]
pub mod adc12iv;
#[doc = "ADC12MCTL0 register accessor: an alias for `Reg<ADC12MCTL0_SPEC>`"]
pub type ADC12MCTL0 = crate::Reg<adc12mctl0::ADC12MCTL0_SPEC>;
#[doc = "ADC12 Memory Control 0"]
pub mod adc12mctl0;
#[doc = "ADC12MCTL1 register accessor: an alias for `Reg<ADC12MCTL1_SPEC>`"]
pub type ADC12MCTL1 = crate::Reg<adc12mctl1::ADC12MCTL1_SPEC>;
#[doc = "ADC12 Memory Control 1"]
pub mod adc12mctl1;
#[doc = "ADC12MCTL2 register accessor: an alias for `Reg<ADC12MCTL2_SPEC>`"]
pub type ADC12MCTL2 = crate::Reg<adc12mctl2::ADC12MCTL2_SPEC>;
#[doc = "ADC12 Memory Control 2"]
pub mod adc12mctl2;
#[doc = "ADC12MCTL3 register accessor: an alias for `Reg<ADC12MCTL3_SPEC>`"]
pub type ADC12MCTL3 = crate::Reg<adc12mctl3::ADC12MCTL3_SPEC>;
#[doc = "ADC12 Memory Control 3"]
pub mod adc12mctl3;
#[doc = "ADC12MCTL4 register accessor: an alias for `Reg<ADC12MCTL4_SPEC>`"]
pub type ADC12MCTL4 = crate::Reg<adc12mctl4::ADC12MCTL4_SPEC>;
#[doc = "ADC12 Memory Control 4"]
pub mod adc12mctl4;
#[doc = "ADC12MCTL5 register accessor: an alias for `Reg<ADC12MCTL5_SPEC>`"]
pub type ADC12MCTL5 = crate::Reg<adc12mctl5::ADC12MCTL5_SPEC>;
#[doc = "ADC12 Memory Control 5"]
pub mod adc12mctl5;
#[doc = "ADC12MCTL6 register accessor: an alias for `Reg<ADC12MCTL6_SPEC>`"]
pub type ADC12MCTL6 = crate::Reg<adc12mctl6::ADC12MCTL6_SPEC>;
#[doc = "ADC12 Memory Control 6"]
pub mod adc12mctl6;
#[doc = "ADC12MCTL7 register accessor: an alias for `Reg<ADC12MCTL7_SPEC>`"]
pub type ADC12MCTL7 = crate::Reg<adc12mctl7::ADC12MCTL7_SPEC>;
#[doc = "ADC12 Memory Control 7"]
pub mod adc12mctl7;
#[doc = "ADC12MCTL8 register accessor: an alias for `Reg<ADC12MCTL8_SPEC>`"]
pub type ADC12MCTL8 = crate::Reg<adc12mctl8::ADC12MCTL8_SPEC>;
#[doc = "ADC12 Memory Control 8"]
pub mod adc12mctl8;
#[doc = "ADC12MCTL9 register accessor: an alias for `Reg<ADC12MCTL9_SPEC>`"]
pub type ADC12MCTL9 = crate::Reg<adc12mctl9::ADC12MCTL9_SPEC>;
#[doc = "ADC12 Memory Control 9"]
pub mod adc12mctl9;
#[doc = "ADC12MCTL10 register accessor: an alias for `Reg<ADC12MCTL10_SPEC>`"]
pub type ADC12MCTL10 = crate::Reg<adc12mctl10::ADC12MCTL10_SPEC>;
#[doc = "ADC12 Memory Control 10"]
pub mod adc12mctl10;
#[doc = "ADC12MCTL11 register accessor: an alias for `Reg<ADC12MCTL11_SPEC>`"]
pub type ADC12MCTL11 = crate::Reg<adc12mctl11::ADC12MCTL11_SPEC>;
#[doc = "ADC12 Memory Control 11"]
pub mod adc12mctl11;
#[doc = "ADC12MCTL12 register accessor: an alias for `Reg<ADC12MCTL12_SPEC>`"]
pub type ADC12MCTL12 = crate::Reg<adc12mctl12::ADC12MCTL12_SPEC>;
#[doc = "ADC12 Memory Control 12"]
pub mod adc12mctl12;
#[doc = "ADC12MCTL13 register accessor: an alias for `Reg<ADC12MCTL13_SPEC>`"]
pub type ADC12MCTL13 = crate::Reg<adc12mctl13::ADC12MCTL13_SPEC>;
#[doc = "ADC12 Memory Control 13"]
pub mod adc12mctl13;
#[doc = "ADC12MCTL14 register accessor: an alias for `Reg<ADC12MCTL14_SPEC>`"]
pub type ADC12MCTL14 = crate::Reg<adc12mctl14::ADC12MCTL14_SPEC>;
#[doc = "ADC12 Memory Control 14"]
pub mod adc12mctl14;
#[doc = "ADC12MCTL15 register accessor: an alias for `Reg<ADC12MCTL15_SPEC>`"]
pub type ADC12MCTL15 = crate::Reg<adc12mctl15::ADC12MCTL15_SPEC>;
#[doc = "ADC12 Memory Control 15"]
pub mod adc12mctl15;
#[doc = "ADC12MCTL16 register accessor: an alias for `Reg<ADC12MCTL16_SPEC>`"]
pub type ADC12MCTL16 = crate::Reg<adc12mctl16::ADC12MCTL16_SPEC>;
#[doc = "ADC12 Memory Control 16"]
pub mod adc12mctl16;
#[doc = "ADC12MCTL17 register accessor: an alias for `Reg<ADC12MCTL17_SPEC>`"]
pub type ADC12MCTL17 = crate::Reg<adc12mctl17::ADC12MCTL17_SPEC>;
#[doc = "ADC12 Memory Control 17"]
pub mod adc12mctl17;
#[doc = "ADC12MCTL18 register accessor: an alias for `Reg<ADC12MCTL18_SPEC>`"]
pub type ADC12MCTL18 = crate::Reg<adc12mctl18::ADC12MCTL18_SPEC>;
#[doc = "ADC12 Memory Control 18"]
pub mod adc12mctl18;
#[doc = "ADC12MCTL19 register accessor: an alias for `Reg<ADC12MCTL19_SPEC>`"]
pub type ADC12MCTL19 = crate::Reg<adc12mctl19::ADC12MCTL19_SPEC>;
#[doc = "ADC12 Memory Control 19"]
pub mod adc12mctl19;
#[doc = "ADC12MCTL20 register accessor: an alias for `Reg<ADC12MCTL20_SPEC>`"]
pub type ADC12MCTL20 = crate::Reg<adc12mctl20::ADC12MCTL20_SPEC>;
#[doc = "ADC12 Memory Control 20"]
pub mod adc12mctl20;
#[doc = "ADC12MCTL21 register accessor: an alias for `Reg<ADC12MCTL21_SPEC>`"]
pub type ADC12MCTL21 = crate::Reg<adc12mctl21::ADC12MCTL21_SPEC>;
#[doc = "ADC12 Memory Control 21"]
pub mod adc12mctl21;
#[doc = "ADC12MCTL22 register accessor: an alias for `Reg<ADC12MCTL22_SPEC>`"]
pub type ADC12MCTL22 = crate::Reg<adc12mctl22::ADC12MCTL22_SPEC>;
#[doc = "ADC12 Memory Control 22"]
pub mod adc12mctl22;
#[doc = "ADC12MCTL23 register accessor: an alias for `Reg<ADC12MCTL23_SPEC>`"]
pub type ADC12MCTL23 = crate::Reg<adc12mctl23::ADC12MCTL23_SPEC>;
#[doc = "ADC12 Memory Control 23"]
pub mod adc12mctl23;
#[doc = "ADC12MCTL24 register accessor: an alias for `Reg<ADC12MCTL24_SPEC>`"]
pub type ADC12MCTL24 = crate::Reg<adc12mctl24::ADC12MCTL24_SPEC>;
#[doc = "ADC12 Memory Control 24"]
pub mod adc12mctl24;
#[doc = "ADC12MCTL25 register accessor: an alias for `Reg<ADC12MCTL25_SPEC>`"]
pub type ADC12MCTL25 = crate::Reg<adc12mctl25::ADC12MCTL25_SPEC>;
#[doc = "ADC12 Memory Control 25"]
pub mod adc12mctl25;
#[doc = "ADC12MCTL26 register accessor: an alias for `Reg<ADC12MCTL26_SPEC>`"]
pub type ADC12MCTL26 = crate::Reg<adc12mctl26::ADC12MCTL26_SPEC>;
#[doc = "ADC12 Memory Control 26"]
pub mod adc12mctl26;
#[doc = "ADC12MCTL27 register accessor: an alias for `Reg<ADC12MCTL27_SPEC>`"]
pub type ADC12MCTL27 = crate::Reg<adc12mctl27::ADC12MCTL27_SPEC>;
#[doc = "ADC12 Memory Control 27"]
pub mod adc12mctl27;
#[doc = "ADC12MCTL28 register accessor: an alias for `Reg<ADC12MCTL28_SPEC>`"]
pub type ADC12MCTL28 = crate::Reg<adc12mctl28::ADC12MCTL28_SPEC>;
#[doc = "ADC12 Memory Control 28"]
pub mod adc12mctl28;
#[doc = "ADC12MCTL29 register accessor: an alias for `Reg<ADC12MCTL29_SPEC>`"]
pub type ADC12MCTL29 = crate::Reg<adc12mctl29::ADC12MCTL29_SPEC>;
#[doc = "ADC12 Memory Control 29"]
pub mod adc12mctl29;
#[doc = "ADC12MCTL30 register accessor: an alias for `Reg<ADC12MCTL30_SPEC>`"]
pub type ADC12MCTL30 = crate::Reg<adc12mctl30::ADC12MCTL30_SPEC>;
#[doc = "ADC12 Memory Control 30"]
pub mod adc12mctl30;
#[doc = "ADC12MCTL31 register accessor: an alias for `Reg<ADC12MCTL31_SPEC>`"]
pub type ADC12MCTL31 = crate::Reg<adc12mctl31::ADC12MCTL31_SPEC>;
#[doc = "ADC12 Memory Control 31"]
pub mod adc12mctl31;
#[doc = "ADC12MEM0 register accessor: an alias for `Reg<ADC12MEM0_SPEC>`"]
pub type ADC12MEM0 = crate::Reg<adc12mem0::ADC12MEM0_SPEC>;
#[doc = "ADC12 Conversion Memory 0"]
pub mod adc12mem0;
#[doc = "ADC12MEM1 register accessor: an alias for `Reg<ADC12MEM1_SPEC>`"]
pub type ADC12MEM1 = crate::Reg<adc12mem1::ADC12MEM1_SPEC>;
#[doc = "ADC12 Conversion Memory 1"]
pub mod adc12mem1;
#[doc = "ADC12MEM2 register accessor: an alias for `Reg<ADC12MEM2_SPEC>`"]
pub type ADC12MEM2 = crate::Reg<adc12mem2::ADC12MEM2_SPEC>;
#[doc = "ADC12 Conversion Memory 2"]
pub mod adc12mem2;
#[doc = "ADC12MEM3 register accessor: an alias for `Reg<ADC12MEM3_SPEC>`"]
pub type ADC12MEM3 = crate::Reg<adc12mem3::ADC12MEM3_SPEC>;
#[doc = "ADC12 Conversion Memory 3"]
pub mod adc12mem3;
#[doc = "ADC12MEM4 register accessor: an alias for `Reg<ADC12MEM4_SPEC>`"]
pub type ADC12MEM4 = crate::Reg<adc12mem4::ADC12MEM4_SPEC>;
#[doc = "ADC12 Conversion Memory 4"]
pub mod adc12mem4;
#[doc = "ADC12MEM5 register accessor: an alias for `Reg<ADC12MEM5_SPEC>`"]
pub type ADC12MEM5 = crate::Reg<adc12mem5::ADC12MEM5_SPEC>;
#[doc = "ADC12 Conversion Memory 5"]
pub mod adc12mem5;
#[doc = "ADC12MEM6 register accessor: an alias for `Reg<ADC12MEM6_SPEC>`"]
pub type ADC12MEM6 = crate::Reg<adc12mem6::ADC12MEM6_SPEC>;
#[doc = "ADC12 Conversion Memory 6"]
pub mod adc12mem6;
#[doc = "ADC12MEM7 register accessor: an alias for `Reg<ADC12MEM7_SPEC>`"]
pub type ADC12MEM7 = crate::Reg<adc12mem7::ADC12MEM7_SPEC>;
#[doc = "ADC12 Conversion Memory 7"]
pub mod adc12mem7;
#[doc = "ADC12MEM8 register accessor: an alias for `Reg<ADC12MEM8_SPEC>`"]
pub type ADC12MEM8 = crate::Reg<adc12mem8::ADC12MEM8_SPEC>;
#[doc = "ADC12 Conversion Memory 8"]
pub mod adc12mem8;
#[doc = "ADC12MEM9 register accessor: an alias for `Reg<ADC12MEM9_SPEC>`"]
pub type ADC12MEM9 = crate::Reg<adc12mem9::ADC12MEM9_SPEC>;
#[doc = "ADC12 Conversion Memory 9"]
pub mod adc12mem9;
#[doc = "ADC12MEM10 register accessor: an alias for `Reg<ADC12MEM10_SPEC>`"]
pub type ADC12MEM10 = crate::Reg<adc12mem10::ADC12MEM10_SPEC>;
#[doc = "ADC12 Conversion Memory 10"]
pub mod adc12mem10;
#[doc = "ADC12MEM11 register accessor: an alias for `Reg<ADC12MEM11_SPEC>`"]
pub type ADC12MEM11 = crate::Reg<adc12mem11::ADC12MEM11_SPEC>;
#[doc = "ADC12 Conversion Memory 11"]
pub mod adc12mem11;
#[doc = "ADC12MEM12 register accessor: an alias for `Reg<ADC12MEM12_SPEC>`"]
pub type ADC12MEM12 = crate::Reg<adc12mem12::ADC12MEM12_SPEC>;
#[doc = "ADC12 Conversion Memory 12"]
pub mod adc12mem12;
#[doc = "ADC12MEM13 register accessor: an alias for `Reg<ADC12MEM13_SPEC>`"]
pub type ADC12MEM13 = crate::Reg<adc12mem13::ADC12MEM13_SPEC>;
#[doc = "ADC12 Conversion Memory 13"]
pub mod adc12mem13;
#[doc = "ADC12MEM14 register accessor: an alias for `Reg<ADC12MEM14_SPEC>`"]
pub type ADC12MEM14 = crate::Reg<adc12mem14::ADC12MEM14_SPEC>;
#[doc = "ADC12 Conversion Memory 14"]
pub mod adc12mem14;
#[doc = "ADC12MEM15 register accessor: an alias for `Reg<ADC12MEM15_SPEC>`"]
pub type ADC12MEM15 = crate::Reg<adc12mem15::ADC12MEM15_SPEC>;
#[doc = "ADC12 Conversion Memory 15"]
pub mod adc12mem15;
#[doc = "ADC12MEM16 register accessor: an alias for `Reg<ADC12MEM16_SPEC>`"]
pub type ADC12MEM16 = crate::Reg<adc12mem16::ADC12MEM16_SPEC>;
#[doc = "ADC12 Conversion Memory 16"]
pub mod adc12mem16;
#[doc = "ADC12MEM17 register accessor: an alias for `Reg<ADC12MEM17_SPEC>`"]
pub type ADC12MEM17 = crate::Reg<adc12mem17::ADC12MEM17_SPEC>;
#[doc = "ADC12 Conversion Memory 17"]
pub mod adc12mem17;
#[doc = "ADC12MEM18 register accessor: an alias for `Reg<ADC12MEM18_SPEC>`"]
pub type ADC12MEM18 = crate::Reg<adc12mem18::ADC12MEM18_SPEC>;
#[doc = "ADC12 Conversion Memory 18"]
pub mod adc12mem18;
#[doc = "ADC12MEM19 register accessor: an alias for `Reg<ADC12MEM19_SPEC>`"]
pub type ADC12MEM19 = crate::Reg<adc12mem19::ADC12MEM19_SPEC>;
#[doc = "ADC12 Conversion Memory 19"]
pub mod adc12mem19;
#[doc = "ADC12MEM20 register accessor: an alias for `Reg<ADC12MEM20_SPEC>`"]
pub type ADC12MEM20 = crate::Reg<adc12mem20::ADC12MEM20_SPEC>;
#[doc = "ADC12 Conversion Memory 20"]
pub mod adc12mem20;
#[doc = "ADC12MEM21 register accessor: an alias for `Reg<ADC12MEM21_SPEC>`"]
pub type ADC12MEM21 = crate::Reg<adc12mem21::ADC12MEM21_SPEC>;
#[doc = "ADC12 Conversion Memory 21"]
pub mod adc12mem21;
#[doc = "ADC12MEM22 register accessor: an alias for `Reg<ADC12MEM22_SPEC>`"]
pub type ADC12MEM22 = crate::Reg<adc12mem22::ADC12MEM22_SPEC>;
#[doc = "ADC12 Conversion Memory 22"]
pub mod adc12mem22;
#[doc = "ADC12MEM23 register accessor: an alias for `Reg<ADC12MEM23_SPEC>`"]
pub type ADC12MEM23 = crate::Reg<adc12mem23::ADC12MEM23_SPEC>;
#[doc = "ADC12 Conversion Memory 23"]
pub mod adc12mem23;
#[doc = "ADC12MEM24 register accessor: an alias for `Reg<ADC12MEM24_SPEC>`"]
pub type ADC12MEM24 = crate::Reg<adc12mem24::ADC12MEM24_SPEC>;
#[doc = "ADC12 Conversion Memory 24"]
pub mod adc12mem24;
#[doc = "ADC12MEM25 register accessor: an alias for `Reg<ADC12MEM25_SPEC>`"]
pub type ADC12MEM25 = crate::Reg<adc12mem25::ADC12MEM25_SPEC>;
#[doc = "ADC12 Conversion Memory 25"]
pub mod adc12mem25;
#[doc = "ADC12MEM26 register accessor: an alias for `Reg<ADC12MEM26_SPEC>`"]
pub type ADC12MEM26 = crate::Reg<adc12mem26::ADC12MEM26_SPEC>;
#[doc = "ADC12 Conversion Memory 26"]
pub mod adc12mem26;
#[doc = "ADC12MEM27 register accessor: an alias for `Reg<ADC12MEM27_SPEC>`"]
pub type ADC12MEM27 = crate::Reg<adc12mem27::ADC12MEM27_SPEC>;
#[doc = "ADC12 Conversion Memory 27"]
pub mod adc12mem27;
#[doc = "ADC12MEM28 register accessor: an alias for `Reg<ADC12MEM28_SPEC>`"]
pub type ADC12MEM28 = crate::Reg<adc12mem28::ADC12MEM28_SPEC>;
#[doc = "ADC12 Conversion Memory 28"]
pub mod adc12mem28;
#[doc = "ADC12MEM29 register accessor: an alias for `Reg<ADC12MEM29_SPEC>`"]
pub type ADC12MEM29 = crate::Reg<adc12mem29::ADC12MEM29_SPEC>;
#[doc = "ADC12 Conversion Memory 29"]
pub mod adc12mem29;
#[doc = "ADC12MEM30 register accessor: an alias for `Reg<ADC12MEM30_SPEC>`"]
pub type ADC12MEM30 = crate::Reg<adc12mem30::ADC12MEM30_SPEC>;
#[doc = "ADC12 Conversion Memory 30"]
pub mod adc12mem30;
#[doc = "ADC12MEM31 register accessor: an alias for `Reg<ADC12MEM31_SPEC>`"]
pub type ADC12MEM31 = crate::Reg<adc12mem31::ADC12MEM31_SPEC>;
#[doc = "ADC12 Conversion Memory 31"]
pub mod adc12mem31;
