#![no_std]
#![no_main]

use cortex_m::peripheral::syst;
use cortex_m_rt::{entry, exception};
use nrf52840_pac::{aar::addrptr::R, interrupt};
use panic_halt as _;
use rtt_target::{rprintln, rtt_init_print};

#[entry]
fn main() -> ! {
    rtt_init_print!();
    rprintln!("hello world");

    let mut c = cortex_m::Peripherals::take().unwrap();
    c.SYST.set_clock_source(syst::SystClkSource::External);
    c.SYST.enable_interrupt();

    let p = nrf52840_pac::Peripherals::take().unwrap();

    // 0.16 = ESPBOOT
    p.P0.outset.write(|w| w.pin16().set());
    p.P0.dirset.write(|w| w.pin16().set());

    // 0.24 = ESPEN
    p.P0.pin_cnf[24].write(|w| w.dir().output().drive().s0d1());

    // 0.07 = HOST_WK
    // ?

    // 1.04 = RXD
    p.UARTE0.psel.rxd.write(|w| unsafe { w.bits(0b100000 | 4) });

    // 1.05 = TXD
    p.P1.outset.write(|w| w.pin5().set());
    p.P1.dirset.write(|w| w.pin5().set());
    p.UARTE0.psel.txd.write(|w| unsafe { w.bits(0b100000 | 5) });

    // 1.06 = CTS
    p.UARTE0.psel.cts.write(|w| unsafe { w.bits(0b100000 | 6) });

    // 1.07 = RTS
    p.P1.outset.write(|w| w.pin7().set());
    p.P1.dirset.write(|w| w.pin7().set());
    p.UARTE0.psel.rts.write(|w| unsafe { w.bits(0b100000 | 7) });

    p.UARTE0.config.write(|w| w.hwfc().enabled());
    p.UARTE0.baudrate.write(|w| w.baudrate().baud921600());

    p.UARTE0.enable.write(|w| w.enable().enabled());
    // p.UARTE0.inten.write(|w| w.cts().enabled());
    // p.UARTE0.inten.write(|w| unsafe { w.bits(0xffff_ffff) });
    // unsafe {
    //     cortex_m::peripheral::NVIC::unmask(interrupt::UARTE0_UART0);
    // }

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

    p.P0.outset.write(|w| w.pin16().set());
    wait(&mut c.SYST, 1000);
    p.P0.outclr.write(|w| w.pin24().clear());
    wait(&mut c.SYST, 1000);
    p.P0.outset.write(|w| w.pin24().set());
    wait(&mut c.SYST, 1000);

    let at_cmd = b"AT\r\n";
    p.UARTE0
        .txd
        .ptr
        .write(|w| unsafe { w.bits(at_cmd.as_ptr() as u32) });
    p.UARTE0
        .txd
        .maxcnt
        .write(|w| unsafe { w.bits(at_cmd.len() as u32) });

    loop {
        rprintln!("tx");

        p.UARTE0.events_endtx.reset();
        p.UARTE0.events_txstopped.reset();

        p.UARTE0
            .tasks_starttx
            .write(|w| w.tasks_starttx().trigger());

        while p
            .UARTE0
            .events_endtx
            .read()
            .events_endtx()
            .is_not_generated()
        {
            wait(&mut c.SYST, 1000);
        }

        p.UARTE0.tasks_stoptx.write(|w| w.tasks_stoptx().trigger());

        while p
            .UARTE0
            .events_txstopped
            .read()
            .events_txstopped()
            .is_not_generated()
        {
            wait(&mut c.SYST, 1000);
        }
    }
}

// const TICKS_PER_S: u32 = 64_000_000;
const TICKS_PER_MS: u32 = 64_000;

fn wait(syst: &mut cortex_m::peripheral::SYST, ms: u32) {
    syst.set_reload(ms * TICKS_PER_MS - 1);
    syst.clear_current();
    syst.enable_counter();

    while !syst.has_wrapped() {}

    syst.disable_counter();
}

#[interrupt]
fn UARTE0_UART0() {
    rprintln!("UARTE0");

    // let event_cts = 0x40002100 as *mut u32;
    // unsafe { event_cts.write_volatile(0) };
}

#[exception]
fn SysTick() {
    rprintln!("SysTick");
}
