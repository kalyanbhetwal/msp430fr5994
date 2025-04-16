#![no_main]
#![no_std]
#![feature(asm_experimental_arch)]
#![feature(abi_msp430_interrupt)]
//extern crate panic_msp430; // For now, we only have an infinitely-looping panic handler.

use msp430::{interrupt as mspint, critical_section as mspcs};
use msp430fr5962 as _;
// use msp430::asm;
// use msp430_rt::entry;
use core::arch::asm;
// use core::ptr;
use core:: panic::PanicInfo;
// use msp430::interrupt; 
// use msp430_rt::interrupt;

// Vector table: 48 entries, addresses 0xFF80 to 0xFFFE
#[repr(C)]
pub union Vector {
    reserved: u16,
    handler: unsafe extern "C" fn(),
}

#[used]
#[link_section = ".vector_table.exceptions"]
#[no_mangle]
pub static EXCEPTIONS: [Vector; 62] = [
    Vector {reserved :  0}, //some reserved area
    Vector {reserved :  0},
    Vector {reserved :  0},
    Vector {reserved :  0},
    Vector {reserved :  0},
    Vector {reserved :  0},
    Vector {reserved :  0},
    Vector {reserved :  0},
    Vector { handler:TIMER0_A1 },      // 0
    Vector {handler: TIMER0_A1},      //  1
    Vector {handler: TIMER0_A1},       //  2
    Vector {handler: TIMER0_A1},       //  3
    Vector {handler: TIMER0_A1},       //  4
    Vector {handler: TIMER0_A1},       //  5
    Vector {handler: TIMER0_A1},       //  6
    Vector {handler: TIMER0_A1},       //  7
    Vector {handler: TIMER0_A1},       //  8
    Vector {handler: TIMER0_A1},       //  9
    Vector {handler: TIMER0_A1},       // 10
    Vector {handler: TIMER0_A1},       // 11
    Vector {handler: TIMER0_A1},       // 12
    Vector {handler: TIMER0_A1},       // 13
    Vector {handler: TIMER0_A1},       // 14
    Vector {handler: TIMER0_A1},       // 15
    Vector {handler: TIMER0_A1},       // 16
    Vector {handler: TIMER0_A1},       // 17 - LEA
    Vector {handler: TIMER0_A1},       // 18 - PORT8
    Vector {handler: TIMER0_A1},       // 19 - PORT7
    Vector {handler: TIMER0_A1},       // 20 - EUSCI_B3
    Vector {handler: TIMER0_A1},       // 21 - EUSCI_B2
    Vector {handler: TIMER0_A1},       // 22 - EUSCI_B1
    Vector {handler: TIMER0_A1},       // 23 - EUSCI_A3
    Vector {handler: TIMER0_A1},       // 24 - EUSCI_A2
    Vector {handler: TIMER0_A1},       // 25 - PORT6
    Vector {handler: TIMER0_A1},       // 26 - PORT5
    Vector {handler: TIMER0_A1},       // 27 - TIMER4_A1
    Vector {handler: TIMER0_A1},             // 28 - TIMER4_A0 
    Vector {handler: TIMER0_A1},       // 29 - AES256
    Vector {handler: TIMER0_A1},       // 30 - RTC_C
    Vector {handler: TIMER0_A1},       // 31 - PORT4
    Vector {handler: TIMER0_A1},       // 32 - PORT3
    Vector {handler: TIMER0_A1},       // 33 - TIMER3_A1
    Vector {handler: TIMER0_A1},       // 34 - TIMER3_A0
    Vector {handler: TIMER0_A1},       // 35 - PORT2
    Vector {handler: TIMER0_A1},       // 36 - TIMER2_A1
    Vector {handler: TIMER0_A1},       // 37 - TIMER2_A0
    Vector {handler: TIMER0_A1},       // 38 - PORT1
    Vector {handler: TIMER0_A1},       // 39 - TIMER1_A1
    Vector {handler: TIMER0_A0},       // 40 - TIMER1_A0
    Vector {handler: TIMER0_A1},       // 41 - DMA
    Vector {handler: TIMER0_A1},       // 42 - EUSCI_A1
    Vector {handler: TIMER0_A1},       // 43 - TIMER0_A1
    Vector {handler: TIMER0_A1},         // 44 - TIMER0_A0
    Vector {handler: TIMER0_A1},       // 45 - ADC12_B
    Vector {handler: TIMER0_A1},       // 46 - EUSCI_B0
    Vector {handler: TIMER0_A1},       // 47 - EUSCI_A0
    Vector {handler: TIMER0_A1},       // 48 - WDT
    Vector {handler: TIMER0_A1},       // 49 - TIMER0_B1
    Vector {handler: TIMER0_A1},       // 50 - TIMER0_B0
    Vector {handler: TIMER0_A1},       // 51 - COMP_E
    Vector {handler: TIMER0_A1},       // 52 - UNMI
    Vector {handler: _start},       // 53 - SYSNMI
];

/// Register addresses for MSP430FR5994
const WDTCTL: *mut u16 = 0x015C as *mut u16;
const PM5CTL0: *mut u16 = 0x0130 as *mut u16;
const P1OUT: *mut u8 = 0x0202 as *mut u8;
const P1DIR: *mut u8 = 0x0204 as *mut u8;

/// Watchdog Timer password and hold bit
const WDTPW: u16 = 0x5A00;
const WDTHOLD: u16 = 0x0080;

/// Bit masks
const BIT0: u8 = 0b00000011;
const LOCKLPM5: u16 = 0x0001;



// Constants from msp430 headers
const TA0CCTL0: *mut u16 = 0x0342 as *mut u16;
const TA0CCR0: *mut u16 = 0x0352 as *mut u16;
const TA0CTL:   *mut u16 = 0x0340 as *mut u16;

// Bitfields
const CCIE: u16 = 0x0010;
const TASSEL__SMCLK: u16 = 0x0200;
const MC__CONTINUOUS: u16 = 0x0002;
const GIE: u16 = 0x0008;
const LPM0_bits: u16 = 0x0010;

#[no_mangle]
pub extern "C" fn _start() -> (){
    

    
    unsafe {
        // Stop watchdog timer
        *WDTCTL = WDTPW | WDTHOLD;

        // Clear P1.0 output latch
        *P1OUT &= !BIT0;

        // Set P1.0 as output
        *P1DIR |= BIT0;

        // Disable high-impedance mode
        *PM5CTL0 &= !LOCKLPM5;

       // *P1OUT |= BIT0; 

        *TA0CCTL0 = CCIE ; // Enable TACCR0 interrupt
        *TA0CCR0 = 50000;
        *TA0CTL =  0x0220 ;//TASSEL__SMCLK | MC__CONTINUOUS ;

       // Enter LPM0 with interrupts enabled
      // msp430::asm::bis_sr(LPM0_bits | GIE);
    
        
       //msp430::asm::nop();
    }
   // unsafe {asm!("bis.w #0x0018, SR");}

   unsafe {
    asm!("NOP");
    core::arch::asm!(
        "bis #0x0018, sr", // 0x0010 = LPM0_bits, 0x0008 = GIE
        options(nomem, nostack, preserves_flags)
    );
    asm!("NOP");
    asm!("NOP");
    
}
  
    loop{
        
    }
    // loop {
    //     // Do nothing, stay in low power mode
    //     // unsafe { *P1OUT ^= BIT0; }
    //     // delay();
    // }

}


#[panic_handler]
fn panic(_panic: &PanicInfo<'_>) -> ! {
    loop {
        msp430::asm::barrier();
    }
}

#[no_mangle]
pub extern "C" fn TIMER0_A1() {
    // Handle the interrupt for Timer0_A0
    // For example, toggle an LED or increment a counter
    // unsafe {asm!("mov #0x0015, r6");}
    // unsafe {asm!("mov #0x0123, r7");}
    // unsafe {asm!("mov #0x0045, r8");}
    unsafe{
    *P1OUT ^= 1;
    asm!("sub.w #0x3cb0, &0x0352")
    }
}


#[no_mangle]
pub extern "C" fn TIMER0_A0() {
    // Handle the interrupt for Timer0_A0
    // For example, toggle an LED or increment a counter
    // unsafe {asm!("mov #0x0015, r6");}
    // unsafe {asm!("mov #0x0123, r7");}
    // unsafe {asm!("mov #0x0045, r8");}
    unsafe{
    *P1OUT ^= 1;
    asm!("sub.w #0x3cb0, &0x0352")
    }
}


// #[no_mangle]
// pub extern "C" fn TIMER0_A0() {
//     // Handle the interrupt for Timer0_A0
//     // For example, toggle an LED or increment a counter
//     // unsafe {asm!("mov #0x0015, r6");}
//     // unsafe {asm!("mov #0x0123, r7");}
//     // unsafe {asm!("mov #0x0045, r8");}
//     unsafe { *P1OUT ^= BIT0; }
//     delay();
// }



#[no_mangle]
pub extern "C" fn DefaultHandler() {
    loop {}
}

#[no_mangle]
pub extern  "C" fn abort(){
    loop{
        unsafe{ asm!("NOP");}
    }
}


// fn delay() {
//     // Crude software delay loop
//     for _ in 0..50_000 {
//         unsafe { core::arch::asm!("nop") }
//     }
// }