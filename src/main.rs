#![no_std]
#![no_main]

use cortex_m_rt::entry;
use nrf52840_pac;
use panic_halt as _;

#[entry]
fn main() -> ! {
    let p = nrf52840_pac::Peripherals::take().unwrap();
    let gpio_p0 = p.P0;
    gpio_p0.dirset.write(|w| w.pin13().set());
    gpio_p0.outset.write(|w| w.pin13().set());
    gpio_p0.outclr.write(|w| w.pin13().clear());

    loop {}
}
