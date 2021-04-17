#![feature(abi_msp430_interrupt)]
#![cfg_attr(feature = "rt", feature(global_asm))]
#![cfg_attr(feature = "rt", feature(use_extern_macros))]
#![cfg_attr(feature = "rt", feature(used))]
#![doc = "Peripheral access API for MSP430FR6972 microcontrollers (generated using svd2rust v0.17.0)\n\nYou can find an overview of the API [here].\n\n[here]: https://docs.rs/svd2rust/0.17.0/svd2rust/#peripheral-api"]
#![deny(const_err)]
#![deny(dead_code)]
#![deny(improper_ctypes)]
#![deny(legacy_directory_ownership)]
#![deny(missing_docs)]
#![deny(no_mangle_generic_items)]
#![deny(non_shorthand_field_patterns)]
#![deny(overflowing_literals)]
#![deny(path_statements)]
#![deny(patterns_in_fns_without_body)]
#![deny(plugin_as_library)]
#![deny(private_in_public)]
#![deny(safe_extern_statics)]
#![deny(unconditional_recursion)]
#![deny(unions_with_drop_fields)]
#![deny(unused_allocation)]
#![deny(unused_comparisons)]
#![deny(unused_parens)]
#![deny(while_true)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![no_std]
extern crate msp430;
#[cfg(feature = "rt")]
extern crate msp430_rt;
#[cfg(feature = "rt")]
pub use msp430_rt::default_handler;
extern crate bare_metal;
extern crate vcell;
use core::marker::PhantomData;
use core::ops::Deref;
#[doc(hidden)]
pub mod interrupt;
pub use self::interrupt::Interrupt;
#[allow(unused_imports)]
use generic::*;
#[doc = r"Common register and bit access and modify traits"]
pub mod generic;
#[doc = "Port 1/2"]
pub struct PORT_1_2 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for PORT_1_2 {}
impl PORT_1_2 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const port_1_2::RegisterBlock {
        0x0200 as *const _
    }
}
impl Deref for PORT_1_2 {
    type Target = port_1_2::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*PORT_1_2::ptr() }
    }
}
#[doc = "Port 1/2"]
pub mod port_1_2;
#[doc = "Port 3/4"]
pub struct PORT_3_4 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for PORT_3_4 {}
impl PORT_3_4 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const port_3_4::RegisterBlock {
        0x0220 as *const _
    }
}
impl Deref for PORT_3_4 {
    type Target = port_3_4::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*PORT_3_4::ptr() }
    }
}
#[doc = "Port 3/4"]
pub mod port_3_4;
#[doc = "Port 5/6"]
pub struct PORT_5_6 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for PORT_5_6 {}
impl PORT_5_6 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const port_5_6::RegisterBlock {
        0x0240 as *const _
    }
}
impl Deref for PORT_5_6 {
    type Target = port_5_6::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*PORT_5_6::ptr() }
    }
}
#[doc = "Port 5/6"]
pub mod port_5_6;
#[doc = "Port 7"]
pub struct PORT_7 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for PORT_7 {}
impl PORT_7 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const port_7::RegisterBlock {
        0x0260 as *const _
    }
}
impl Deref for PORT_7 {
    type Target = port_7::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*PORT_7::ptr() }
    }
}
#[doc = "Port 7"]
pub mod port_7;
#[doc = "Port 9"]
pub struct PORT_9 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for PORT_9 {}
impl PORT_9 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const port_9::RegisterBlock {
        0x0280 as *const _
    }
}
impl Deref for PORT_9 {
    type Target = port_9::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*PORT_9::ptr() }
    }
}
#[doc = "Port 9"]
pub mod port_9;
#[doc = "RTC_C Real Time Clock"]
pub struct RTC_C_REAL_TIME_CLOCK {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for RTC_C_REAL_TIME_CLOCK {}
impl RTC_C_REAL_TIME_CLOCK {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const rtc_c_real_time_clock::RegisterBlock {
        0x04a0 as *const _
    }
}
impl Deref for RTC_C_REAL_TIME_CLOCK {
    type Target = rtc_c_real_time_clock::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*RTC_C_REAL_TIME_CLOCK::ptr() }
    }
}
#[doc = "RTC_C Real Time Clock"]
pub mod rtc_c_real_time_clock;
#[doc = "USCI_A0 UART Mode"]
pub struct USCI_A0_UART_MODE {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for USCI_A0_UART_MODE {}
impl USCI_A0_UART_MODE {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const usci_a0_uart_mode::RegisterBlock {
        0x05c0 as *const _
    }
}
impl Deref for USCI_A0_UART_MODE {
    type Target = usci_a0_uart_mode::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*USCI_A0_UART_MODE::ptr() }
    }
}
#[doc = "USCI_A0 UART Mode"]
pub mod usci_a0_uart_mode;
#[doc = "USCI_A0 SPI Mode"]
pub struct USCI_A0_SPI_MODE {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for USCI_A0_SPI_MODE {}
impl USCI_A0_SPI_MODE {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const usci_a0_spi_mode::RegisterBlock {
        0x05c0 as *const _
    }
}
impl Deref for USCI_A0_SPI_MODE {
    type Target = usci_a0_spi_mode::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*USCI_A0_SPI_MODE::ptr() }
    }
}
#[doc = "USCI_A0 SPI Mode"]
pub mod usci_a0_spi_mode;
#[doc = "USCI_A1 UART Mode"]
pub struct USCI_A1_UART_MODE {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for USCI_A1_UART_MODE {}
impl USCI_A1_UART_MODE {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const usci_a1_uart_mode::RegisterBlock {
        0x05e0 as *const _
    }
}
impl Deref for USCI_A1_UART_MODE {
    type Target = usci_a1_uart_mode::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*USCI_A1_UART_MODE::ptr() }
    }
}
#[doc = "USCI_A1 UART Mode"]
pub mod usci_a1_uart_mode;
#[doc = "USCI_A1 SPI Mode"]
pub struct USCI_A1_SPI_MODE {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for USCI_A1_SPI_MODE {}
impl USCI_A1_SPI_MODE {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const usci_a1_spi_mode::RegisterBlock {
        0x05e0 as *const _
    }
}
impl Deref for USCI_A1_SPI_MODE {
    type Target = usci_a1_spi_mode::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*USCI_A1_SPI_MODE::ptr() }
    }
}
#[doc = "USCI_A1 SPI Mode"]
pub mod usci_a1_spi_mode;
#[doc = "USCI_B0 I2C Mode"]
pub struct USCI_B0_I2C_MODE {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for USCI_B0_I2C_MODE {}
impl USCI_B0_I2C_MODE {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const usci_b0_i2c_mode::RegisterBlock {
        0x0640 as *const _
    }
}
impl Deref for USCI_B0_I2C_MODE {
    type Target = usci_b0_i2c_mode::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*USCI_B0_I2C_MODE::ptr() }
    }
}
#[doc = "USCI_B0 I2C Mode"]
pub mod usci_b0_i2c_mode;
#[doc = "USCI_B0 SPI Mode"]
pub struct USCI_B0_SPI_MODE {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for USCI_B0_SPI_MODE {}
impl USCI_B0_SPI_MODE {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const usci_b0_spi_mode::RegisterBlock {
        0x0640 as *const _
    }
}
impl Deref for USCI_B0_SPI_MODE {
    type Target = usci_b0_spi_mode::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*USCI_B0_SPI_MODE::ptr() }
    }
}
#[doc = "USCI_B0 SPI Mode"]
pub mod usci_b0_spi_mode;
#[doc = "USCI_B1 I2C Mode"]
pub struct USCI_B1_I2C_MODE {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for USCI_B1_I2C_MODE {}
impl USCI_B1_I2C_MODE {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const usci_b1_i2c_mode::RegisterBlock {
        0x0680 as *const _
    }
}
impl Deref for USCI_B1_I2C_MODE {
    type Target = usci_b1_i2c_mode::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*USCI_B1_I2C_MODE::ptr() }
    }
}
#[doc = "USCI_B1 I2C Mode"]
pub mod usci_b1_i2c_mode;
#[doc = "USCI_B1 SPI Mode"]
pub struct USCI_B1_SPI_MODE {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for USCI_B1_SPI_MODE {}
impl USCI_B1_SPI_MODE {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const usci_b1_spi_mode::RegisterBlock {
        0x0680 as *const _
    }
}
impl Deref for USCI_B1_SPI_MODE {
    type Target = usci_b1_spi_mode::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*USCI_B1_SPI_MODE::ptr() }
    }
}
#[doc = "USCI_B1 SPI Mode"]
pub mod usci_b1_spi_mode;
#[doc = "LCD_C"]
pub struct LCD_C {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for LCD_C {}
impl LCD_C {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const lcd_c::RegisterBlock {
        0x0a00 as *const _
    }
}
impl Deref for LCD_C {
    type Target = lcd_c::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*LCD_C::ptr() }
    }
}
#[doc = "LCD_C"]
pub mod lcd_c;
#[doc = "SFR Special Function Registers"]
pub struct SFR {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SFR {}
impl SFR {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const sfr::RegisterBlock {
        0x0100 as *const _
    }
}
impl Deref for SFR {
    type Target = sfr::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*SFR::ptr() }
    }
}
#[doc = "SFR Special Function Registers"]
pub mod sfr;
#[doc = "PMM Power Management System"]
pub struct PMM {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for PMM {}
impl PMM {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const pmm::RegisterBlock {
        0x0120 as *const _
    }
}
impl Deref for PMM {
    type Target = pmm::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*PMM::ptr() }
    }
}
#[doc = "PMM Power Management System"]
pub mod pmm;
#[doc = "FRAM"]
pub struct FRAM {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for FRAM {}
impl FRAM {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const fram::RegisterBlock {
        0x0140 as *const _
    }
}
impl Deref for FRAM {
    type Target = fram::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*FRAM::ptr() }
    }
}
#[doc = "FRAM"]
pub mod fram;
#[doc = "CRC16"]
pub struct CRC16 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for CRC16 {}
impl CRC16 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const crc16::RegisterBlock {
        0x0150 as *const _
    }
}
impl Deref for CRC16 {
    type Target = crc16::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*CRC16::ptr() }
    }
}
#[doc = "CRC16"]
pub mod crc16;
#[doc = "RC RAM Control Module (FRAM)"]
pub struct RC_FRAM {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for RC_FRAM {}
impl RC_FRAM {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const rc_fram::RegisterBlock {
        0x0158 as *const _
    }
}
impl Deref for RC_FRAM {
    type Target = rc_fram::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*RC_FRAM::ptr() }
    }
}
#[doc = "RC RAM Control Module (FRAM)"]
pub mod rc_fram;
#[doc = "Watchdog Timer"]
pub struct WATCHDOG_TIMER {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for WATCHDOG_TIMER {}
impl WATCHDOG_TIMER {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const watchdog_timer::RegisterBlock {
        0x015c as *const _
    }
}
impl Deref for WATCHDOG_TIMER {
    type Target = watchdog_timer::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*WATCHDOG_TIMER::ptr() }
    }
}
#[doc = "Watchdog Timer"]
pub mod watchdog_timer;
#[doc = "CS Clock System"]
pub struct CS {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for CS {}
impl CS {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const cs::RegisterBlock {
        0x0160 as *const _
    }
}
impl Deref for CS {
    type Target = cs::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*CS::ptr() }
    }
}
#[doc = "CS Clock System"]
pub mod cs;
#[doc = "SYS System Module"]
pub struct SYS {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SYS {}
impl SYS {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const sys::RegisterBlock {
        0x0180 as *const _
    }
}
impl Deref for SYS {
    type Target = sys::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*SYS::ptr() }
    }
}
#[doc = "SYS System Module"]
pub mod sys;
#[doc = "Shared Reference"]
pub struct SHARED_REFERENCE {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SHARED_REFERENCE {}
impl SHARED_REFERENCE {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const shared_reference::RegisterBlock {
        0x01b0 as *const _
    }
}
impl Deref for SHARED_REFERENCE {
    type Target = shared_reference::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*SHARED_REFERENCE::ptr() }
    }
}
#[doc = "Shared Reference"]
pub mod shared_reference;
#[doc = "Port J"]
pub struct PORT_J {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for PORT_J {}
impl PORT_J {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const port_j::RegisterBlock {
        0x0320 as *const _
    }
}
impl Deref for PORT_J {
    type Target = port_j::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*PORT_J::ptr() }
    }
}
#[doc = "Port J"]
pub mod port_j;
#[doc = "Timer0_A3"]
pub struct TIMER_0_A3 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for TIMER_0_A3 {}
impl TIMER_0_A3 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const timer_0_a3::RegisterBlock {
        0x0340 as *const _
    }
}
impl Deref for TIMER_0_A3 {
    type Target = timer_0_a3::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*TIMER_0_A3::ptr() }
    }
}
#[doc = "Timer0_A3"]
pub mod timer_0_a3;
#[doc = "Timer1_A3"]
pub struct TIMER_1_A3 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for TIMER_1_A3 {}
impl TIMER_1_A3 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const timer_1_a3::RegisterBlock {
        0x0380 as *const _
    }
}
impl Deref for TIMER_1_A3 {
    type Target = timer_1_a3::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*TIMER_1_A3::ptr() }
    }
}
#[doc = "Timer1_A3"]
pub mod timer_1_a3;
#[doc = "Timer0_B7"]
pub struct TIMER_0_B7 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for TIMER_0_B7 {}
impl TIMER_0_B7 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const timer_0_b7::RegisterBlock {
        0x03c0 as *const _
    }
}
impl Deref for TIMER_0_B7 {
    type Target = timer_0_b7::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*TIMER_0_B7::ptr() }
    }
}
#[doc = "Timer0_B7"]
pub mod timer_0_b7;
#[doc = "Timer2_A2"]
pub struct TIMER_2_A2 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for TIMER_2_A2 {}
impl TIMER_2_A2 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const timer_2_a2::RegisterBlock {
        0x0400 as *const _
    }
}
impl Deref for TIMER_2_A2 {
    type Target = timer_2_a2::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*TIMER_2_A2::ptr() }
    }
}
#[doc = "Timer2_A2"]
pub mod timer_2_a2;
#[doc = "Capacitive_Touch_IO 0"]
pub struct CAPACITIVE_TOUCH_IO_0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for CAPACITIVE_TOUCH_IO_0 {}
impl CAPACITIVE_TOUCH_IO_0 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const capacitive_touch_io_0::RegisterBlock {
        0x043e as *const _
    }
}
impl Deref for CAPACITIVE_TOUCH_IO_0 {
    type Target = capacitive_touch_io_0::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*CAPACITIVE_TOUCH_IO_0::ptr() }
    }
}
#[doc = "Capacitive_Touch_IO 0"]
pub mod capacitive_touch_io_0;
#[doc = "Timer3_A5"]
pub struct TIMER_3_A5 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for TIMER_3_A5 {}
impl TIMER_3_A5 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const timer_3_a5::RegisterBlock {
        0x0440 as *const _
    }
}
impl Deref for TIMER_3_A5 {
    type Target = timer_3_a5::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*TIMER_3_A5::ptr() }
    }
}
#[doc = "Timer3_A5"]
pub mod timer_3_a5;
#[doc = "Capacitive_Touch_IO 1"]
pub struct CAPACITIVE_TOUCH_IO_1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for CAPACITIVE_TOUCH_IO_1 {}
impl CAPACITIVE_TOUCH_IO_1 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const capacitive_touch_io_1::RegisterBlock {
        0x047e as *const _
    }
}
impl Deref for CAPACITIVE_TOUCH_IO_1 {
    type Target = capacitive_touch_io_1::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*CAPACITIVE_TOUCH_IO_1::ptr() }
    }
}
#[doc = "Capacitive_Touch_IO 1"]
pub mod capacitive_touch_io_1;
#[doc = "MPY 16 Multiplier 16 Bit Mode"]
pub struct MPY_16 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for MPY_16 {}
impl MPY_16 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const mpy_16::RegisterBlock {
        0x04c0 as *const _
    }
}
impl Deref for MPY_16 {
    type Target = mpy_16::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*MPY_16::ptr() }
    }
}
#[doc = "MPY 16 Multiplier 16 Bit Mode"]
pub mod mpy_16;
#[doc = "MPY 32 Multiplier 32 Bit Mode"]
pub struct MPY_32 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for MPY_32 {}
impl MPY_32 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const mpy_32::RegisterBlock {
        0x04d0 as *const _
    }
}
impl Deref for MPY_32 {
    type Target = mpy_32::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*MPY_32::ptr() }
    }
}
#[doc = "MPY 32 Multiplier 32 Bit Mode"]
pub mod mpy_32;
#[doc = "DMA"]
pub struct DMA {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for DMA {}
impl DMA {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const dma::RegisterBlock {
        0x0500 as *const _
    }
}
impl Deref for DMA {
    type Target = dma::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*DMA::ptr() }
    }
}
#[doc = "DMA"]
pub mod dma;
#[doc = "MPU"]
pub struct MPU {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for MPU {}
impl MPU {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const mpu::RegisterBlock {
        0x05a0 as *const _
    }
}
impl Deref for MPU {
    type Target = mpu::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*MPU::ptr() }
    }
}
#[doc = "MPU"]
pub mod mpu;
#[doc = "ADC12"]
pub struct ADC12 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for ADC12 {}
impl ADC12 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const adc12::RegisterBlock {
        0x0800 as *const _
    }
}
impl Deref for ADC12 {
    type Target = adc12::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*ADC12::ptr() }
    }
}
#[doc = "ADC12"]
pub mod adc12;
#[doc = "Comparator E"]
pub struct COMPARATOR_E {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for COMPARATOR_E {}
impl COMPARATOR_E {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const comparator_e::RegisterBlock {
        0x08c0 as *const _
    }
}
impl Deref for COMPARATOR_E {
    type Target = comparator_e::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*COMPARATOR_E::ptr() }
    }
}
#[doc = "Comparator E"]
pub mod comparator_e;
#[doc = "CRC32"]
pub struct CRC32 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for CRC32 {}
impl CRC32 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const crc32::RegisterBlock {
        0x0980 as *const _
    }
}
impl Deref for CRC32 {
    type Target = crc32::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*CRC32::ptr() }
    }
}
#[doc = "CRC32"]
pub mod crc32;
#[doc = "AES Accelerator"]
pub struct AES_ACCELERATOR {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for AES_ACCELERATOR {}
impl AES_ACCELERATOR {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const aes_accelerator::RegisterBlock {
        0x09c0 as *const _
    }
}
impl Deref for AES_ACCELERATOR {
    type Target = aes_accelerator::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*AES_ACCELERATOR::ptr() }
    }
}
#[doc = "AES Accelerator"]
pub mod aes_accelerator;
#[no_mangle]
static mut DEVICE_PERIPHERALS: bool = false;
#[doc = r"All the peripherals"]
#[allow(non_snake_case)]
pub struct Peripherals {
    #[doc = "PORT_1_2"]
    pub PORT_1_2: PORT_1_2,
    #[doc = "PORT_3_4"]
    pub PORT_3_4: PORT_3_4,
    #[doc = "PORT_5_6"]
    pub PORT_5_6: PORT_5_6,
    #[doc = "PORT_7"]
    pub PORT_7: PORT_7,
    #[doc = "PORT_9"]
    pub PORT_9: PORT_9,
    #[doc = "RTC_C_REAL_TIME_CLOCK"]
    pub RTC_C_REAL_TIME_CLOCK: RTC_C_REAL_TIME_CLOCK,
    #[doc = "USCI_A0_UART_MODE"]
    pub USCI_A0_UART_MODE: USCI_A0_UART_MODE,
    #[doc = "USCI_A0_SPI_MODE"]
    pub USCI_A0_SPI_MODE: USCI_A0_SPI_MODE,
    #[doc = "USCI_A1_UART_MODE"]
    pub USCI_A1_UART_MODE: USCI_A1_UART_MODE,
    #[doc = "USCI_A1_SPI_MODE"]
    pub USCI_A1_SPI_MODE: USCI_A1_SPI_MODE,
    #[doc = "USCI_B0_I2C_MODE"]
    pub USCI_B0_I2C_MODE: USCI_B0_I2C_MODE,
    #[doc = "USCI_B0_SPI_MODE"]
    pub USCI_B0_SPI_MODE: USCI_B0_SPI_MODE,
    #[doc = "USCI_B1_I2C_MODE"]
    pub USCI_B1_I2C_MODE: USCI_B1_I2C_MODE,
    #[doc = "USCI_B1_SPI_MODE"]
    pub USCI_B1_SPI_MODE: USCI_B1_SPI_MODE,
    #[doc = "LCD_C"]
    pub LCD_C: LCD_C,
    #[doc = "SFR"]
    pub SFR: SFR,
    #[doc = "PMM"]
    pub PMM: PMM,
    #[doc = "FRAM"]
    pub FRAM: FRAM,
    #[doc = "CRC16"]
    pub CRC16: CRC16,
    #[doc = "RC_FRAM"]
    pub RC_FRAM: RC_FRAM,
    #[doc = "WATCHDOG_TIMER"]
    pub WATCHDOG_TIMER: WATCHDOG_TIMER,
    #[doc = "CS"]
    pub CS: CS,
    #[doc = "SYS"]
    pub SYS: SYS,
    #[doc = "SHARED_REFERENCE"]
    pub SHARED_REFERENCE: SHARED_REFERENCE,
    #[doc = "PORT_J"]
    pub PORT_J: PORT_J,
    #[doc = "TIMER_0_A3"]
    pub TIMER_0_A3: TIMER_0_A3,
    #[doc = "TIMER_1_A3"]
    pub TIMER_1_A3: TIMER_1_A3,
    #[doc = "TIMER_0_B7"]
    pub TIMER_0_B7: TIMER_0_B7,
    #[doc = "TIMER_2_A2"]
    pub TIMER_2_A2: TIMER_2_A2,
    #[doc = "CAPACITIVE_TOUCH_IO_0"]
    pub CAPACITIVE_TOUCH_IO_0: CAPACITIVE_TOUCH_IO_0,
    #[doc = "TIMER_3_A5"]
    pub TIMER_3_A5: TIMER_3_A5,
    #[doc = "CAPACITIVE_TOUCH_IO_1"]
    pub CAPACITIVE_TOUCH_IO_1: CAPACITIVE_TOUCH_IO_1,
    #[doc = "MPY_16"]
    pub MPY_16: MPY_16,
    #[doc = "MPY_32"]
    pub MPY_32: MPY_32,
    #[doc = "DMA"]
    pub DMA: DMA,
    #[doc = "MPU"]
    pub MPU: MPU,
    #[doc = "ADC12"]
    pub ADC12: ADC12,
    #[doc = "COMPARATOR_E"]
    pub COMPARATOR_E: COMPARATOR_E,
    #[doc = "CRC32"]
    pub CRC32: CRC32,
    #[doc = "AES_ACCELERATOR"]
    pub AES_ACCELERATOR: AES_ACCELERATOR,
}
impl Peripherals {
    #[doc = r"Returns all the peripherals *once*"]
    #[inline]
    pub fn take() -> Option<Self> {
        msp430::interrupt::free(|_| {
            if unsafe { DEVICE_PERIPHERALS } {
                None
            } else {
                Some(unsafe { Peripherals::steal() })
            }
        })
    }
    #[doc = r"Unchecked version of `Peripherals::take`"]
    #[inline]
    pub unsafe fn steal() -> Self {
        DEVICE_PERIPHERALS = true;
        Peripherals {
            PORT_1_2: PORT_1_2 {
                _marker: PhantomData,
            },
            PORT_3_4: PORT_3_4 {
                _marker: PhantomData,
            },
            PORT_5_6: PORT_5_6 {
                _marker: PhantomData,
            },
            PORT_7: PORT_7 {
                _marker: PhantomData,
            },
            PORT_9: PORT_9 {
                _marker: PhantomData,
            },
            RTC_C_REAL_TIME_CLOCK: RTC_C_REAL_TIME_CLOCK {
                _marker: PhantomData,
            },
            USCI_A0_UART_MODE: USCI_A0_UART_MODE {
                _marker: PhantomData,
            },
            USCI_A0_SPI_MODE: USCI_A0_SPI_MODE {
                _marker: PhantomData,
            },
            USCI_A1_UART_MODE: USCI_A1_UART_MODE {
                _marker: PhantomData,
            },
            USCI_A1_SPI_MODE: USCI_A1_SPI_MODE {
                _marker: PhantomData,
            },
            USCI_B0_I2C_MODE: USCI_B0_I2C_MODE {
                _marker: PhantomData,
            },
            USCI_B0_SPI_MODE: USCI_B0_SPI_MODE {
                _marker: PhantomData,
            },
            USCI_B1_I2C_MODE: USCI_B1_I2C_MODE {
                _marker: PhantomData,
            },
            USCI_B1_SPI_MODE: USCI_B1_SPI_MODE {
                _marker: PhantomData,
            },
            LCD_C: LCD_C {
                _marker: PhantomData,
            },
            SFR: SFR {
                _marker: PhantomData,
            },
            PMM: PMM {
                _marker: PhantomData,
            },
            FRAM: FRAM {
                _marker: PhantomData,
            },
            CRC16: CRC16 {
                _marker: PhantomData,
            },
            RC_FRAM: RC_FRAM {
                _marker: PhantomData,
            },
            WATCHDOG_TIMER: WATCHDOG_TIMER {
                _marker: PhantomData,
            },
            CS: CS {
                _marker: PhantomData,
            },
            SYS: SYS {
                _marker: PhantomData,
            },
            SHARED_REFERENCE: SHARED_REFERENCE {
                _marker: PhantomData,
            },
            PORT_J: PORT_J {
                _marker: PhantomData,
            },
            TIMER_0_A3: TIMER_0_A3 {
                _marker: PhantomData,
            },
            TIMER_1_A3: TIMER_1_A3 {
                _marker: PhantomData,
            },
            TIMER_0_B7: TIMER_0_B7 {
                _marker: PhantomData,
            },
            TIMER_2_A2: TIMER_2_A2 {
                _marker: PhantomData,
            },
            CAPACITIVE_TOUCH_IO_0: CAPACITIVE_TOUCH_IO_0 {
                _marker: PhantomData,
            },
            TIMER_3_A5: TIMER_3_A5 {
                _marker: PhantomData,
            },
            CAPACITIVE_TOUCH_IO_1: CAPACITIVE_TOUCH_IO_1 {
                _marker: PhantomData,
            },
            MPY_16: MPY_16 {
                _marker: PhantomData,
            },
            MPY_32: MPY_32 {
                _marker: PhantomData,
            },
            DMA: DMA {
                _marker: PhantomData,
            },
            MPU: MPU {
                _marker: PhantomData,
            },
            ADC12: ADC12 {
                _marker: PhantomData,
            },
            COMPARATOR_E: COMPARATOR_E {
                _marker: PhantomData,
            },
            CRC32: CRC32 {
                _marker: PhantomData,
            },
            AES_ACCELERATOR: AES_ACCELERATOR {
                _marker: PhantomData,
            },
        }
    }
}
