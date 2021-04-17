#[cfg(feature = "rt")]
global_asm!(
    "
                DH_TRAMPOLINE:
                    br #DEFAULT_HANDLER
                "
);
#[cfg(feature = "rt")]
global_asm ! ( "\n.weak AES256\nAES256 = DH_TRAMPOLINE\n.weak RTC\nRTC = DH_TRAMPOLINE\n.weak LCD_C\nLCD_C = DH_TRAMPOLINE\n.weak PORT4\nPORT4 = DH_TRAMPOLINE\n.weak PORT3\nPORT3 = DH_TRAMPOLINE\n.weak TIMER3_A1\nTIMER3_A1 = DH_TRAMPOLINE\n.weak TIMER3_A0\nTIMER3_A0 = DH_TRAMPOLINE\n.weak PORT2\nPORT2 = DH_TRAMPOLINE\n.weak TIMER2_A1\nTIMER2_A1 = DH_TRAMPOLINE\n.weak TIMER2_A0\nTIMER2_A0 = DH_TRAMPOLINE\n.weak PORT1\nPORT1 = DH_TRAMPOLINE\n.weak TIMER1_A1\nTIMER1_A1 = DH_TRAMPOLINE\n.weak TIMER1_A0\nTIMER1_A0 = DH_TRAMPOLINE\n.weak DMA\nDMA = DH_TRAMPOLINE\n.weak USCI_B1\nUSCI_B1 = DH_TRAMPOLINE\n.weak USCI_A1\nUSCI_A1 = DH_TRAMPOLINE\n.weak TIMER0_A1\nTIMER0_A1 = DH_TRAMPOLINE\n.weak TIMER0_A0\nTIMER0_A0 = DH_TRAMPOLINE\n.weak ADC12\nADC12 = DH_TRAMPOLINE\n.weak USCI_B0\nUSCI_B0 = DH_TRAMPOLINE\n.weak USCI_A0\nUSCI_A0 = DH_TRAMPOLINE\n.weak WDT\nWDT = DH_TRAMPOLINE\n.weak TIMER0_B1\nTIMER0_B1 = DH_TRAMPOLINE\n.weak TIMER0_B0\nTIMER0_B0 = DH_TRAMPOLINE\n.weak COMP_E\nCOMP_E = DH_TRAMPOLINE\n.weak UNMI\nUNMI = DH_TRAMPOLINE\n.weak SYSNMI\nSYSNMI = DH_TRAMPOLINE" ) ;
#[cfg(feature = "rt")]
extern "msp430-interrupt" {
    fn AES256();
    fn RTC();
    fn LCD_C();
    fn PORT4();
    fn PORT3();
    fn TIMER3_A1();
    fn TIMER3_A0();
    fn PORT2();
    fn TIMER2_A1();
    fn TIMER2_A0();
    fn PORT1();
    fn TIMER1_A1();
    fn TIMER1_A0();
    fn DMA();
    fn USCI_B1();
    fn USCI_A1();
    fn TIMER0_A1();
    fn TIMER0_A0();
    fn ADC12();
    fn USCI_B0();
    fn USCI_A0();
    fn WDT();
    fn TIMER0_B1();
    fn TIMER0_B0();
    fn COMP_E();
    fn UNMI();
    fn SYSNMI();
}
#[doc(hidden)]
pub union Vector {
    _handler: unsafe extern "msp430-interrupt" fn(),
    _reserved: u16,
}
#[cfg(feature = "rt")]
#[doc(hidden)]
#[link_section = ".vector_table.interrupts"]
#[no_mangle]
#[used]
pub static INTERRUPTS: [Vector; 55] = [
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _handler: AES256 },
    Vector { _handler: RTC },
    Vector { _handler: LCD_C },
    Vector { _handler: PORT4 },
    Vector { _handler: PORT3 },
    Vector {
        _handler: TIMER3_A1,
    },
    Vector {
        _handler: TIMER3_A0,
    },
    Vector { _handler: PORT2 },
    Vector {
        _handler: TIMER2_A1,
    },
    Vector {
        _handler: TIMER2_A0,
    },
    Vector { _handler: PORT1 },
    Vector {
        _handler: TIMER1_A1,
    },
    Vector {
        _handler: TIMER1_A0,
    },
    Vector { _handler: DMA },
    Vector { _handler: USCI_B1 },
    Vector { _handler: USCI_A1 },
    Vector {
        _handler: TIMER0_A1,
    },
    Vector {
        _handler: TIMER0_A0,
    },
    Vector { _handler: ADC12 },
    Vector { _handler: USCI_B0 },
    Vector { _handler: USCI_A0 },
    Vector { _reserved: 0 },
    Vector { _handler: WDT },
    Vector {
        _handler: TIMER0_B1,
    },
    Vector {
        _handler: TIMER0_B0,
    },
    Vector { _handler: COMP_E },
    Vector { _handler: UNMI },
    Vector { _handler: SYSNMI },
];
#[doc = r"Enumeration of all the interrupts"]
#[derive(Copy, Clone, Debug)]
#[repr(u8)]
pub enum Interrupt {
    #[doc = "27 - 0xFFC6 AES256"]
    AES256 = 27,
    #[doc = "28 - 0xFFC8 RTC"]
    RTC = 28,
    #[doc = "29 - 0xFFCA LCD C"]
    LCD_C = 29,
    #[doc = "30 - 0xFFCC Port 4"]
    PORT4 = 30,
    #[doc = "31 - 0xFFCE Port 3"]
    PORT3 = 31,
    #[doc = "32 - 0xFFD0 Timer3_A2 CC1, TA"]
    TIMER3_A1 = 32,
    #[doc = "33 - 0xFFD2 Timer3_A2 CC0"]
    TIMER3_A0 = 33,
    #[doc = "34 - 0xFFD4 Port 2"]
    PORT2 = 34,
    #[doc = "35 - 0xFFD6 Timer2_A3 CC1, TA"]
    TIMER2_A1 = 35,
    #[doc = "36 - 0xFFD8 Timer2_A3 CC0"]
    TIMER2_A0 = 36,
    #[doc = "37 - 0xFFDA Port 1"]
    PORT1 = 37,
    #[doc = "38 - 0xFFDC Timer1_A3 CC1-2, TA1"]
    TIMER1_A1 = 38,
    #[doc = "39 - 0xFFDE Timer1_A3 CC0"]
    TIMER1_A0 = 39,
    #[doc = "40 - 0xFFE0 DMA"]
    DMA = 40,
    #[doc = "41 - 0xFFE2 USCI B1 Receive/Transmit"]
    USCI_B1 = 41,
    #[doc = "42 - 0xFFE4 USCI A1 Receive/Transmit"]
    USCI_A1 = 42,
    #[doc = "43 - 0xFFE6 Timer0_A5 CC1-4, TA"]
    TIMER0_A1 = 43,
    #[doc = "44 - 0xFFE8 Timer0_A5 CC0"]
    TIMER0_A0 = 44,
    #[doc = "45 - 0xFFEA ADC"]
    ADC12 = 45,
    #[doc = "46 - 0xFFEC USCI B0 Receive/Transmit"]
    USCI_B0 = 46,
    #[doc = "47 - 0xFFEE USCI A0 Receive/Transmit"]
    USCI_A0 = 47,
    #[doc = "49 - 0xFFF2 Watchdog Timer"]
    WDT = 49,
    #[doc = "50 - 0xFFF4 Timer0_B3 CC1-2, TB"]
    TIMER0_B1 = 50,
    #[doc = "51 - 0xFFF6 Timer0_B3 CC0"]
    TIMER0_B0 = 51,
    #[doc = "52 - 0xFFF8 Comparator E"]
    COMP_E = 52,
    #[doc = "53 - 0xFFFA User Non-maskable"]
    UNMI = 53,
    #[doc = "54 - 0xFFFC System Non-maskable"]
    SYSNMI = 54,
}
unsafe impl bare_metal::Nr for Interrupt {
    #[inline(always)]
    fn nr(&self) -> u8 {
        *self as u8
    }
}
#[derive(Debug, Copy, Clone)]
pub struct TryFromInterruptError(());
impl Interrupt {
    #[inline]
    pub fn try_from(value: u8) -> Result<Self, TryFromInterruptError> {
        match value {
            27 => Ok(Interrupt::AES256),
            28 => Ok(Interrupt::RTC),
            29 => Ok(Interrupt::LCD_C),
            30 => Ok(Interrupt::PORT4),
            31 => Ok(Interrupt::PORT3),
            32 => Ok(Interrupt::TIMER3_A1),
            33 => Ok(Interrupt::TIMER3_A0),
            34 => Ok(Interrupt::PORT2),
            35 => Ok(Interrupt::TIMER2_A1),
            36 => Ok(Interrupt::TIMER2_A0),
            37 => Ok(Interrupt::PORT1),
            38 => Ok(Interrupt::TIMER1_A1),
            39 => Ok(Interrupt::TIMER1_A0),
            40 => Ok(Interrupt::DMA),
            41 => Ok(Interrupt::USCI_B1),
            42 => Ok(Interrupt::USCI_A1),
            43 => Ok(Interrupt::TIMER0_A1),
            44 => Ok(Interrupt::TIMER0_A0),
            45 => Ok(Interrupt::ADC12),
            46 => Ok(Interrupt::USCI_B0),
            47 => Ok(Interrupt::USCI_A0),
            49 => Ok(Interrupt::WDT),
            50 => Ok(Interrupt::TIMER0_B1),
            51 => Ok(Interrupt::TIMER0_B0),
            52 => Ok(Interrupt::COMP_E),
            53 => Ok(Interrupt::UNMI),
            54 => Ok(Interrupt::SYSNMI),
            _ => Err(TryFromInterruptError(())),
        }
    }
}
#[cfg(feature = "rt")]
#[macro_export]
#[doc = r" Assigns a handler to an interrupt"]
#[doc = r""]
#[doc = r" This macro takes two arguments: the name of an interrupt and the path to the"]
#[doc = r" function that will be used as the handler of that interrupt. That function"]
#[doc = r" must have signature `fn()`."]
#[doc = r""]
#[doc = r" Optionally, a third argument may be used to declare interrupt local data."]
#[doc = r" The handler will have exclusive access to these *local* variables on each"]
#[doc = r" invocation. If the third argument is used then the signature of the handler"]
#[doc = r" function must be `fn(&mut $NAME::Locals)` where `$NAME` is the first argument"]
#[doc = r" passed to the macro."]
#[doc = r""]
#[doc = r" # Example"]
#[doc = r""]
#[doc = r" ``` ignore"]
#[doc = r" interrupt!(TIM2, periodic);"]
#[doc = r""]
#[doc = r" fn periodic() {"]
#[doc = r#"     print!(".");"#]
#[doc = r" }"]
#[doc = r""]
#[doc = r" interrupt!(TIM3, tick, locals: {"]
#[doc = r"     tick: bool = false;"]
#[doc = r" });"]
#[doc = r""]
#[doc = r" fn tick(locals: &mut TIM3::Locals) {"]
#[doc = r"     locals.tick = !locals.tick;"]
#[doc = r""]
#[doc = r"     if locals.tick {"]
#[doc = r#"         println!("Tick");"#]
#[doc = r"     } else {"]
#[doc = r#"         println!("Tock");"#]
#[doc = r"     }"]
#[doc = r" }"]
#[doc = r" ```"]
macro_rules ! interrupt { ( $ NAME : ident , $ path : path , locals : { $ ( $ lvar : ident : $ lty : ty = $ lval : expr ; ) * } ) => { # [ allow ( non_snake_case ) ] mod $ NAME { pub struct Locals { $ ( pub $ lvar : $ lty , ) * } } # [ allow ( non_snake_case ) ] # [ no_mangle ] pub extern "msp430-interrupt" fn $ NAME ( ) { let _ = $ crate :: interrupt :: Interrupt :: $ NAME ; static mut LOCALS : self :: $ NAME :: Locals = self :: $ NAME :: Locals { $ ( $ lvar : $ lval , ) * } ; let f : fn ( & mut self :: $ NAME :: Locals ) = $ path ; f ( unsafe { & mut LOCALS } ) ; } } ; ( $ NAME : ident , $ path : path ) => { # [ allow ( non_snake_case ) ] # [ no_mangle ] pub extern "msp430-interrupt" fn $ NAME ( ) { let _ = $ crate :: interrupt :: Interrupt :: $ NAME ; let f : fn ( ) = $ path ; f ( ) ; } } }
