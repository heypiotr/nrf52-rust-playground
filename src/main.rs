#![no_std]
#![no_main]

use cortex_m::peripheral::syst;
use cortex_m_rt::{entry, exception};
use nrf52840_pac::interrupt;
use panic_halt as _;
use rtt_target::{rprintln, rtt_init_print};

#[entry]
fn main() -> ! {
    rtt_init_print!();
    rprintln!("hello world");

    let mut c = cortex_m::Peripherals::take().unwrap();
    c.SYST.set_clock_source(syst::SystClkSource::External);
    // c.SYST.enable_interrupt();

    let p = nrf52840_pac::Peripherals::take().unwrap();

    // D8 = 1.03 TXD
    // D7 = 1.12 RXD
    // D6 = 1.11 RTS
    // D5 = 1.10 CTS

    // 0.16 = ESPBOOT
    p.P0.outset.write(|w| w.pin16().set());
    p.P0.dirset.write(|w| w.pin16().set());

    // 0.24 = ESPEN
    p.P0.pin_cnf[24].write(|w| w.dir().output().drive().s0d1());

    // 0.07 = HOST_WK
    // ?

    // 1.04 = RXD
    p.UARTE0.psel.rxd.write(|w| unsafe { w.bits(0b100000 | 4) });
    // FTDI
    // p.UARTE0.psel.rxd.write(|w| unsafe { w.bits(0b100000 | 3) });

    // 1.05 = TXD
    p.P1.outset.write(|w| w.pin5().set());
    p.P1.dirset.write(|w| w.pin5().set());
    p.UARTE0.psel.txd.write(|w| unsafe { w.bits(0b100000 | 5) });
    // FTDI
    // p.P1.outset.write(|w| w.pin12().set());
    // p.P1.dirset.write(|w| w.pin12().set());
    // p.UARTE0
    //     .psel
    //     .txd
    //     .write(|w| unsafe { w.bits(0b100000 | 12) });

    // 1.06 = CTS
    p.UARTE0.psel.cts.write(|w| unsafe { w.bits(0b100000 | 6) });
    // FTDI
    // p.UARTE0
    //     .psel
    //     .cts
    //     .write(|w| unsafe { w.bits(0b100000 | 11) });

    // 1.07 = RTS
    p.P1.outset.write(|w| w.pin7().set());
    p.P1.dirset.write(|w| w.pin7().set());
    p.UARTE0.psel.rts.write(|w| unsafe { w.bits(0b100000 | 7) });
    // FTDI
    // p.P1.outset.write(|w| w.pin10().set());
    // p.P1.dirset.write(|w| w.pin10().set());
    // p.UARTE0
    //     .psel
    //     .rts
    //     .write(|w| unsafe { w.bits(0b100000 | 10) });

    p.UARTE0.config.write(|w| w.hwfc().enabled());
    p.UARTE0.baudrate.write(|w| w.baudrate().baud921600());

    p.UARTE0.enable.write(|w| w.enable().enabled());

    p.P0.outset.write(|w| w.pin16().set());
    wait100(&mut c.SYST);
    p.P0.outclr.write(|w| w.pin24().clear());
    wait100(&mut c.SYST);
    p.P0.outset.write(|w| w.pin24().set());
    wait1000(&mut c.SYST);

    let resp = [0; 1024];
    p.UARTE0.rxd.ptr.write(|w| {
        let addr = resp.as_ptr() as u32;
        unsafe { w.bits(addr) }
    });
    p.UARTE0
        .rxd
        .maxcnt
        .write(|w| unsafe { w.bits(resp.len() as u32) });
    p.UARTE0
        .tasks_startrx
        .write(|w| w.tasks_startrx().trigger());

    // p.UARTE0.errorsrc.reset();
    // p.UARTE0.events_error.reset();
    p.UARTE0.inten.write(|w| w.rxdrdy().enabled());
    p.UARTE0.inten.write(|w| w.txdrdy().enabled());
    // p.UARTE0.inten.write(|w| w.cts().enabled());
    unsafe {
        cortex_m::peripheral::NVIC::unmask(interrupt::UARTE0_UART0);
    }

    let at_cmd = b"AT+MVER\r\n";
    let at_cmd_dram = at_cmd.clone();
    // let sync_command = b"\x07\x07\x12 UUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUU";
    // let sync_command = b"\xC0\x00\x08\x24\x07\x07\x12 UUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUU";
    // let sync_final =
    //     b"\xC0\x00\x08\x24\x00\x00\x00\x00\x00\x07\x07\x12 UUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUU\xC0";
    p.UARTE0
        .txd
        .ptr
        .write(|w| unsafe { w.bits(at_cmd_dram.as_ptr() as u32) });
    p.UARTE0
        .txd
        .maxcnt
        .write(|w| unsafe { w.bits(at_cmd_dram.len() as u32) });

    rprintln!("tx");

    // p.UARTE0.events_endtx.reset();
    // p.UARTE0.events_txstopped.reset();

    p.UARTE0
        .tasks_starttx
        .write(|w| w.tasks_starttx().trigger());

    loop {

        // while p
        //     .UARTE0
        //     .events_endtx
        //     .read()
        //     .events_endtx()
        //     .is_not_generated()
        // {
        //     wait1000(&mut c.SYST);
        // }

        // p.UARTE0.tasks_stoptx.write(|w| w.tasks_stoptx().trigger());

        // while p
        //     .UARTE0
        //     .events_txstopped
        //     .read()
        //     .events_txstopped()
        //     .is_not_generated()
        // {
        //     wait1000(&mut c.SYST);
        // }
    }
}

const TICKS_PER_100_MS: u32 = 6_400_000;

fn wait100(syst: &mut cortex_m::peripheral::SYST) {
    syst.set_reload(TICKS_PER_100_MS - 1);
    syst.clear_current();
    syst.enable_counter();

    while !syst.has_wrapped() {}

    syst.disable_counter();
}

fn wait1000(syst: &mut cortex_m::peripheral::SYST) {
    for _ in 0..10 {
        wait100(syst)
    }
}

#[interrupt]
fn UARTE0_UART0() {
    static mut COUNT: u32 = 0;
    *COUNT += 1;

    rprintln!("UARTE0: {}", *COUNT);

    // rxdrdy
    let event = 0x40002108 as *mut u32;
    unsafe { event.write_volatile(0) };

    // txdrdy
    let event = 0x4000211C as *mut u32;
    unsafe { event.write_volatile(0) };
}

// #[exception]
// fn SysTick() {
//     rprintln!("SysTick");
// }
