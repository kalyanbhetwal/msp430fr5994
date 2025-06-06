#![feature(abi_msp430_interrupt)]
#![doc = "Peripheral access API for MSP430FR5994 microcontrollers (generated using svd2rust v0.36.1 ( ))\n\nYou can find an overview of the generated API [here].\n\nAPI features to be included in the [next] svd2rust release can be generated by cloning the svd2rust [repository], checking out the above commit, and running `cargo doc --open`.\n\n[here]: https://docs.rs/svd2rust/0.36.1/svd2rust/#peripheral-api\n[next]: https://github.com/rust-embedded/svd2rust/blob/master/CHANGELOG.md#unreleased\n[repository]: https://github.com/rust-embedded/svd2rust"]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![no_std]
#![cfg_attr(docsrs, feature(doc_auto_cfg))]
#[allow(unused_imports)]
use generic::*;
#[doc = "Common register and bit access and modify traits"]
pub mod generic;
#[cfg(feature = "rt")]
extern "msp430-interrupt" {
    fn LEA();
    fn PORT8();
    fn PORT7();
    fn EUSCI_B3();
    fn EUSCI_B2();
    fn EUSCI_B1();
    fn EUSCI_A3();
    fn EUSCI_A2();
    fn PORT6();
    fn PORT5();
    fn TIMER4_A1();
    fn TIMER4_A0();
    fn AES256();
    fn RTC_C();
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
    fn EUSCI_A1();
    fn TIMER0_A1();
    fn TIMER0_A0();
    fn ADC12_B();
    fn EUSCI_B0();
    fn EUSCI_A0();
    fn WDT();
    fn TIMER0_B1();
    fn TIMER0_B0();
    fn COMP_E();
    fn UNMI();
    fn SYSNMI();
}
#[doc(hidden)]
#[repr(C)]
pub union Vector {
    _handler: unsafe extern "msp430-interrupt" fn(),
    _reserved: u16,
}
#[cfg(feature = "rt")]
#[doc(hidden)]
#[link_section = ".vector_table.interrupts"]
#[no_mangle]
#[used]
pub static __INTERRUPTS: [Vector; 63] = [
    Vector {_reserved :  0}, //some _reserved area
    Vector {_reserved :  0},
    Vector {_reserved :  0},
    Vector {_reserved :  0},
    Vector {_reserved :  0},
    Vector {_reserved :  0},
    Vector {_reserved :  0},
    Vector {_reserved :  0},
    Vector {_reserved :  0},
    Vector {_reserved :  0},

    Vector {_reserved :  0}, //some _reserved area
    Vector {_reserved :  0},
    Vector {_reserved :  0},
    Vector {_reserved :  0},
    Vector {_reserved :  0},
    Vector {_reserved :  0},
    Vector {_reserved :  0},
    Vector {_reserved :  0},
    Vector {_reserved :  0},
    Vector {_reserved :  0},
    
    Vector {_reserved :  0}, //some _reserved area
    Vector {_reserved :  0},
    Vector {_reserved :  0},
    Vector {_reserved :  0},
    Vector {_reserved :  0},
    Vector {_reserved :  0},

    Vector { _handler: LEA },
    Vector { _handler: PORT8 },
    Vector { _handler: PORT7 },
    Vector { _handler: EUSCI_B3 },
    Vector { _handler: EUSCI_B2 },
    Vector { _handler: EUSCI_B1 },
    Vector { _handler: EUSCI_A3 },
    Vector { _handler: EUSCI_A2 },
    Vector { _handler: PORT6 },
    Vector { _handler: PORT5 },
    Vector { _handler: TIMER4_A1 },
    Vector { _handler: TIMER4_A0 },
    Vector { _handler: AES256 },
    Vector { _handler: RTC_C },
    Vector { _handler: PORT4 },
    Vector { _handler: PORT3 },
    Vector { _handler: TIMER3_A1 },
    Vector { _handler: TIMER3_A0 },
    Vector { _handler: PORT2 },
    Vector { _handler: TIMER2_A1 },
    Vector { _handler: TIMER2_A0 },
    Vector { _handler: PORT1 },
    Vector { _handler: TIMER1_A1 },
    Vector { _handler: TIMER1_A0 },
    Vector { _handler: DMA },
    Vector { _handler: EUSCI_A1 },
    Vector { _handler: TIMER0_A1 },
    Vector { _handler: TIMER0_A0 },
    Vector { _handler: ADC12_B },
    Vector { _handler: EUSCI_B0 },
    Vector { _handler: EUSCI_A0 },
    Vector { _handler: WDT },
    Vector { _handler: TIMER0_B1 },
    Vector { _handler: TIMER0_B0 },
    Vector { _handler: COMP_E },
    Vector { _handler: UNMI },
    Vector { _handler: SYSNMI },
];
#[doc = r"Enumeration of all the interrupts. This enum is seldom used in application or library crates. It is present primarily for documenting the device's implemented interrupts."]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[repr(u16)]
pub enum Interrupt {
    #[doc = "17 - 0xFFB4"]
    LEA = 17,
    #[doc = "18 - 0xFFB6"]
    PORT8 = 18,
    #[doc = "19 - 0xFFB8"]
    PORT7 = 19,
    #[doc = "20 - 0xFFBA"]
    EUSCI_B3 = 20,
    #[doc = "21 - 0xFFBC"]
    EUSCI_B2 = 21,
    #[doc = "22 - 0xFFBE"]
    EUSCI_B1 = 22,
    #[doc = "23 - 0xFFC0"]
    EUSCI_A3 = 23,
    #[doc = "24 - 0xFFC2"]
    EUSCI_A2 = 24,
    #[doc = "25 - 0xFFC4"]
    PORT6 = 25,
    #[doc = "26 - 0xFFC6"]
    PORT5 = 26,
    #[doc = "27 - 0xFFC8"]
    TIMER4_A1 = 27,
    #[doc = "28 - 0xFFCA"]
    TIMER4_A0 = 28,
    #[doc = "29 - 0xFFCC"]
    AES256 = 29,
    #[doc = "30 - 0xFFCE"]
    RTC_C = 30,
    #[doc = "31 - 0xFFD0"]
    PORT4 = 31,
    #[doc = "32 - 0xFFD2"]
    PORT3 = 32,
    #[doc = "33 - 0xFFD4"]
    TIMER3_A1 = 33,
    #[doc = "34 - 0xFFD6"]
    TIMER3_A0 = 34,
    #[doc = "35 - 0xFFD8"]
    PORT2 = 35,
    #[doc = "36 - 0xFFDA"]
    TIMER2_A1 = 36,
    #[doc = "37 - 0xFFDC"]
    TIMER2_A0 = 37,
    #[doc = "38 - 0xFFDE"]
    PORT1 = 38,
    #[doc = "39 - 0xFFE0"]
    TIMER1_A1 = 39,
    #[doc = "40 - 0xFFE2"]
    TIMER1_A0 = 40,
    #[doc = "41 - 0xFFE4"]
    DMA = 41,
    #[doc = "42 - 0xFFE6"]
    EUSCI_A1 = 42,
    #[doc = "43 - 0xFFE8"]
    TIMER0_A1 = 43,
    #[doc = "44 - 0xFFEA"]
    TIMER0_A0 = 44,
    #[doc = "45 - 0xFFEC"]
    ADC12_B = 45,
    #[doc = "46 - 0xFFEE"]
    EUSCI_B0 = 46,
    #[doc = "47 - 0xFFF0"]
    EUSCI_A0 = 47,
    #[doc = "48 - 0xFFF2"]
    WDT = 48,
    #[doc = "49 - 0xFFF4"]
    TIMER0_B1 = 49,
    #[doc = "50 - 0xFFF6"]
    TIMER0_B0 = 50,
    #[doc = "51 - 0xFFF8"]
    COMP_E = 51,
    #[doc = "52 - 0xFFFA"]
    UNMI = 52,
    #[doc = "53 - 0xFFFC"]
    SYSNMI = 53,
}
#[doc = "P1"]
pub type P1 = crate::Periph<p1::RegisterBlock, 0x0200>;
impl core::fmt::Debug for P1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("P1").finish()
    }
}
#[doc = "P1"]
pub mod p1;
#[doc = "P2"]
pub type P2 = crate::Periph<p2::RegisterBlock, 0x0200>;
impl core::fmt::Debug for P2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("P2").finish()
    }
}
#[doc = "P2"]
pub mod p2;
#[doc = "P3"]
pub type P3 = crate::Periph<p3::RegisterBlock, 0x0220>;
impl core::fmt::Debug for P3 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("P3").finish()
    }
}
#[doc = "P3"]
pub mod p3;
#[doc = "P4"]
pub type P4 = crate::Periph<p4::RegisterBlock, 0x0220>;
impl core::fmt::Debug for P4 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("P4").finish()
    }
}
#[doc = "P4"]
pub mod p4;
#[doc = "P5"]
pub type P5 = crate::Periph<p5::RegisterBlock, 0x0240>;
impl core::fmt::Debug for P5 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("P5").finish()
    }
}
#[doc = "P5"]
pub mod p5;
#[doc = "P6"]
pub type P6 = crate::Periph<p6::RegisterBlock, 0x0240>;
impl core::fmt::Debug for P6 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("P6").finish()
    }
}
#[doc = "P6"]
pub mod p6;
#[doc = "P7"]
pub type P7 = crate::Periph<p7::RegisterBlock, 0x0260>;
impl core::fmt::Debug for P7 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("P7").finish()
    }
}
#[doc = "P7"]
pub mod p7;
#[doc = "P8"]
pub type P8 = crate::Periph<p8::RegisterBlock, 0x0260>;
impl core::fmt::Debug for P8 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("P8").finish()
    }
}
#[doc = "P8"]
pub mod p8;
#[doc = "P9"]
pub type P9 = crate::Periph<p9::RegisterBlock, 0x0280>;
impl core::fmt::Debug for P9 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("P9").finish()
    }
}
#[doc = "P9"]
pub mod p9;
#[doc = "RTC_C"]
pub type RtcC = crate::Periph<rtc_c::RegisterBlock, 0x04a0>;
impl core::fmt::Debug for RtcC {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RtcC").finish()
    }
}
#[doc = "RTC_C"]
pub mod rtc_c;
#[doc = "SFR"]
pub type Sfr = crate::Periph<sfr::RegisterBlock, 0x0100>;
impl core::fmt::Debug for Sfr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sfr").finish()
    }
}
#[doc = "SFR"]
pub mod sfr;
#[doc = "PMM"]
pub type Pmm = crate::Periph<pmm::RegisterBlock, 0x0120>;
impl core::fmt::Debug for Pmm {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Pmm").finish()
    }
}
#[doc = "PMM"]
pub mod pmm;
#[doc = "FRCTL_A"]
pub type FrctlA = crate::Periph<frctl_a::RegisterBlock, 0x0140>;
impl core::fmt::Debug for FrctlA {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FrctlA").finish()
    }
}
#[doc = "FRCTL_A"]
pub mod frctl_a;
#[doc = "CRC"]
pub type Crc = crate::Periph<crc::RegisterBlock, 0x0150>;
impl core::fmt::Debug for Crc {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Crc").finish()
    }
}
#[doc = "CRC"]
pub mod crc;
#[doc = "RAMCTL"]
pub type Ramctl = crate::Periph<ramctl::RegisterBlock, 0x0158>;
impl core::fmt::Debug for Ramctl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ramctl").finish()
    }
}
#[doc = "RAMCTL"]
pub mod ramctl;
#[doc = "WDT_A"]
pub type WdtA = crate::Periph<wdt_a::RegisterBlock, 0x015c>;
impl core::fmt::Debug for WdtA {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("WdtA").finish()
    }
}
#[doc = "WDT_A"]
pub mod wdt_a;
#[doc = "CS"]
pub type Cs = crate::Periph<cs::RegisterBlock, 0x0160>;
impl core::fmt::Debug for Cs {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Cs").finish()
    }
}
#[doc = "CS"]
pub mod cs;
#[doc = "SYS"]
pub type Sys = crate::Periph<sys::RegisterBlock, 0x0180>;
impl core::fmt::Debug for Sys {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sys").finish()
    }
}
#[doc = "SYS"]
pub mod sys;
#[doc = "REF_A"]
pub type RefA = crate::Periph<ref_a::RegisterBlock, 0x01b0>;
impl core::fmt::Debug for RefA {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RefA").finish()
    }
}
#[doc = "REF_A"]
pub mod ref_a;
#[doc = "PJ"]
pub type Pj = crate::Periph<pj::RegisterBlock, 0x0320>;
impl core::fmt::Debug for Pj {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Pj").finish()
    }
}
#[doc = "PJ"]
pub mod pj;
#[doc = "TA0"]
pub type Ta0 = crate::Periph<ta0::RegisterBlock, 0x0340>;
impl core::fmt::Debug for Ta0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ta0").finish()
    }
}
#[doc = "TA0"]
pub mod ta0;
#[doc = "TA1"]
pub type Ta1 = crate::Periph<ta1::RegisterBlock, 0x0380>;
impl core::fmt::Debug for Ta1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ta1").finish()
    }
}
#[doc = "TA1"]
pub mod ta1;
#[doc = "TB0"]
pub type Tb0 = crate::Periph<tb0::RegisterBlock, 0x03c0>;
impl core::fmt::Debug for Tb0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Tb0").finish()
    }
}
#[doc = "TB0"]
pub mod tb0;
#[doc = "TA2"]
pub type Ta2 = crate::Periph<ta2::RegisterBlock, 0x0400>;
impl core::fmt::Debug for Ta2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ta2").finish()
    }
}
#[doc = "TA2"]
pub mod ta2;
#[doc = "CAPTIO0"]
pub type Captio0 = crate::Periph<captio0::RegisterBlock, 0x043e>;
impl core::fmt::Debug for Captio0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Captio0").finish()
    }
}
#[doc = "CAPTIO0"]
pub mod captio0;
#[doc = "TA3"]
pub type Ta3 = crate::Periph<ta3::RegisterBlock, 0x0440>;
impl core::fmt::Debug for Ta3 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ta3").finish()
    }
}
#[doc = "TA3"]
pub mod ta3;
#[doc = "CAPTIO1"]
pub type Captio1 = crate::Periph<captio1::RegisterBlock, 0x047e>;
impl core::fmt::Debug for Captio1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Captio1").finish()
    }
}
#[doc = "CAPTIO1"]
pub mod captio1;
#[doc = "MPY32"]
pub type Mpy32 = crate::Periph<mpy32::RegisterBlock, 0x04c0>;
impl core::fmt::Debug for Mpy32 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Mpy32").finish()
    }
}
#[doc = "MPY32"]
pub mod mpy32;
#[doc = "DMA"]
pub type Dma = crate::Periph<dma::RegisterBlock, 0x0500>;
impl core::fmt::Debug for Dma {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Dma").finish()
    }
}
#[doc = "DMA"]
pub mod dma;
#[doc = "MPU"]
pub type Mpu = crate::Periph<mpu::RegisterBlock, 0x05a0>;
impl core::fmt::Debug for Mpu {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Mpu").finish()
    }
}
#[doc = "MPU"]
pub mod mpu;
#[doc = "eUSCI_A0"]
pub type EUsciA0 = crate::Periph<e_usci_a0::RegisterBlock, 0x05c0>;
impl core::fmt::Debug for EUsciA0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("EUsciA0").finish()
    }
}
#[doc = "eUSCI_A0"]
pub mod e_usci_a0;
#[doc = "eUSCI_A1"]
pub type EUsciA1 = crate::Periph<e_usci_a1::RegisterBlock, 0x05e0>;
impl core::fmt::Debug for EUsciA1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("EUsciA1").finish()
    }
}
#[doc = "eUSCI_A1"]
pub mod e_usci_a1;
#[doc = "eUSCI_A2"]
pub type EUsciA2 = crate::Periph<e_usci_a2::RegisterBlock, 0x0600>;
impl core::fmt::Debug for EUsciA2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("EUsciA2").finish()
    }
}
#[doc = "eUSCI_A2"]
pub mod e_usci_a2;
#[doc = "eUSCI_A3"]
pub type EUsciA3 = crate::Periph<e_usci_a3::RegisterBlock, 0x0620>;
impl core::fmt::Debug for EUsciA3 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("EUsciA3").finish()
    }
}
#[doc = "eUSCI_A3"]
pub mod e_usci_a3;
#[doc = "eUSCI_B0"]
pub type EUsciB0 = crate::Periph<e_usci_b0::RegisterBlock, 0x0640>;
impl core::fmt::Debug for EUsciB0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("EUsciB0").finish()
    }
}
#[doc = "eUSCI_B0"]
pub mod e_usci_b0;
#[doc = "eUSCI_B1"]
pub type EUsciB1 = crate::Periph<e_usci_b1::RegisterBlock, 0x0680>;
impl core::fmt::Debug for EUsciB1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("EUsciB1").finish()
    }
}
#[doc = "eUSCI_B1"]
pub mod e_usci_b1;
#[doc = "eUSCI_B2"]
pub type EUsciB2 = crate::Periph<e_usci_b2::RegisterBlock, 0x06c0>;
impl core::fmt::Debug for EUsciB2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("EUsciB2").finish()
    }
}
#[doc = "eUSCI_B2"]
pub mod e_usci_b2;
#[doc = "eUSCI_B3"]
pub type EUsciB3 = crate::Periph<e_usci_b3::RegisterBlock, 0x0700>;
impl core::fmt::Debug for EUsciB3 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("EUsciB3").finish()
    }
}
#[doc = "eUSCI_B3"]
pub mod e_usci_b3;
#[doc = "TA4"]
pub type Ta4 = crate::Periph<ta4::RegisterBlock, 0x07c0>;
impl core::fmt::Debug for Ta4 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ta4").finish()
    }
}
#[doc = "TA4"]
pub mod ta4;
#[doc = "ADC12_B"]
pub type Adc12B = crate::Periph<adc12_b::RegisterBlock, 0x0800>;
impl core::fmt::Debug for Adc12B {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Adc12B").finish()
    }
}
#[doc = "ADC12_B"]
pub mod adc12_b;
#[doc = "COMP_E"]
pub type CompE = crate::Periph<comp_e::RegisterBlock, 0x08c0>;
impl core::fmt::Debug for CompE {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CompE").finish()
    }
}
#[doc = "COMP_E"]
pub mod comp_e;
#[doc = "CRC32"]
pub type Crc32 = crate::Periph<crc32::RegisterBlock, 0x0980>;
impl core::fmt::Debug for Crc32 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Crc32").finish()
    }
}
#[doc = "CRC32"]
pub mod crc32;
#[doc = "AES256"]
pub type Aes256 = crate::Periph<aes256::RegisterBlock, 0x09c0>;
impl core::fmt::Debug for Aes256 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Aes256").finish()
    }
}
#[doc = "AES256"]
pub mod aes256;
#[doc = "LEA"]
pub type Lea = crate::Periph<lea::RegisterBlock, 0x0a80>;
impl core::fmt::Debug for Lea {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Lea").finish()
    }
}
#[doc = "LEA"]
pub mod lea;
#[unsafe(no_mangle)]
static mut DEVICE_PERIPHERALS: bool = false;
#[doc = r" All the peripherals."]
#[allow(non_snake_case)]
pub struct Peripherals {
    #[doc = "P1"]
    pub p1: P1,
    #[doc = "P2"]
    pub p2: P2,
    #[doc = "P3"]
    pub p3: P3,
    #[doc = "P4"]
    pub p4: P4,
    #[doc = "P5"]
    pub p5: P5,
    #[doc = "P6"]
    pub p6: P6,
    #[doc = "P7"]
    pub p7: P7,
    #[doc = "P8"]
    pub p8: P8,
    #[doc = "P9"]
    pub p9: P9,
    #[doc = "RTC_C"]
    pub rtc_c: RtcC,
    #[doc = "SFR"]
    pub sfr: Sfr,
    #[doc = "PMM"]
    pub pmm: Pmm,
    #[doc = "FRCTL_A"]
    pub frctl_a: FrctlA,
    #[doc = "CRC"]
    pub crc: Crc,
    #[doc = "RAMCTL"]
    pub ramctl: Ramctl,
    #[doc = "WDT_A"]
    pub wdt_a: WdtA,
    #[doc = "CS"]
    pub cs: Cs,
    #[doc = "SYS"]
    pub sys: Sys,
    #[doc = "REF_A"]
    pub ref_a: RefA,
    #[doc = "PJ"]
    pub pj: Pj,
    #[doc = "TA0"]
    pub ta0: Ta0,
    #[doc = "TA1"]
    pub ta1: Ta1,
    #[doc = "TB0"]
    pub tb0: Tb0,
    #[doc = "TA2"]
    pub ta2: Ta2,
    #[doc = "CAPTIO0"]
    pub captio0: Captio0,
    #[doc = "TA3"]
    pub ta3: Ta3,
    #[doc = "CAPTIO1"]
    pub captio1: Captio1,
    #[doc = "MPY32"]
    pub mpy32: Mpy32,
    #[doc = "DMA"]
    pub dma: Dma,
    #[doc = "MPU"]
    pub mpu: Mpu,
    #[doc = "E_USCI_A0"]
    pub e_usci_a0: EUsciA0,
    #[doc = "E_USCI_A1"]
    pub e_usci_a1: EUsciA1,
    #[doc = "E_USCI_A2"]
    pub e_usci_a2: EUsciA2,
    #[doc = "E_USCI_A3"]
    pub e_usci_a3: EUsciA3,
    #[doc = "E_USCI_B0"]
    pub e_usci_b0: EUsciB0,
    #[doc = "E_USCI_B1"]
    pub e_usci_b1: EUsciB1,
    #[doc = "E_USCI_B2"]
    pub e_usci_b2: EUsciB2,
    #[doc = "E_USCI_B3"]
    pub e_usci_b3: EUsciB3,
    #[doc = "TA4"]
    pub ta4: Ta4,
    #[doc = "ADC12_B"]
    pub adc12_b: Adc12B,
    #[doc = "COMP_E"]
    pub comp_e: CompE,
    #[doc = "CRC32"]
    pub crc32: Crc32,
    #[doc = "AES256"]
    pub aes256: Aes256,
    #[doc = "LEA"]
    pub lea: Lea,
}
impl Peripherals {
    #[doc = r" Returns all the peripherals *once*."]
    #[cfg(feature = "critical-section")]
    #[inline]
    pub fn take() -> Option<Self> {
        critical_section::with(|_| {
            if unsafe { DEVICE_PERIPHERALS } {
                return None;
            }
            Some(unsafe { Peripherals::steal() })
        })
    }
    #[doc = r" Unchecked version of `Peripherals::take`."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Each of the returned peripherals must be used at most once."]
    #[inline]
    pub unsafe fn steal() -> Self {
        DEVICE_PERIPHERALS = true;
        Peripherals {
            p1: P1::steal(),
            p2: P2::steal(),
            p3: P3::steal(),
            p4: P4::steal(),
            p5: P5::steal(),
            p6: P6::steal(),
            p7: P7::steal(),
            p8: P8::steal(),
            p9: P9::steal(),
            rtc_c: RtcC::steal(),
            sfr: Sfr::steal(),
            pmm: Pmm::steal(),
            frctl_a: FrctlA::steal(),
            crc: Crc::steal(),
            ramctl: Ramctl::steal(),
            wdt_a: WdtA::steal(),
            cs: Cs::steal(),
            sys: Sys::steal(),
            ref_a: RefA::steal(),
            pj: Pj::steal(),
            ta0: Ta0::steal(),
            ta1: Ta1::steal(),
            tb0: Tb0::steal(),
            ta2: Ta2::steal(),
            captio0: Captio0::steal(),
            ta3: Ta3::steal(),
            captio1: Captio1::steal(),
            mpy32: Mpy32::steal(),
            dma: Dma::steal(),
            mpu: Mpu::steal(),
            e_usci_a0: EUsciA0::steal(),
            e_usci_a1: EUsciA1::steal(),
            e_usci_a2: EUsciA2::steal(),
            e_usci_a3: EUsciA3::steal(),
            e_usci_b0: EUsciB0::steal(),
            e_usci_b1: EUsciB1::steal(),
            e_usci_b2: EUsciB2::steal(),
            e_usci_b3: EUsciB3::steal(),
            ta4: Ta4::steal(),
            adc12_b: Adc12B::steal(),
            comp_e: CompE::steal(),
            crc32: Crc32::steal(),
            aes256: Aes256::steal(),
            lea: Lea::steal(),
        }
    }
}
