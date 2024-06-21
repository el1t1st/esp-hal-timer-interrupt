#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(dead_code)]
#![no_std]
#![no_main]

use core::cell::RefCell;
use critical_section::Mutex;
use esp_backtrace as _;
use esp_hal::{
    clock::ClockControl,
    delay::Delay,
    peripherals::{Interrupt, Peripherals},
    prelude::*,
    system::SystemControl,
    timer::systimer::{Alarm, Periodic, SystemTimer, Target},
    Blocking,
};
use esp_println::println;
use fugit::ExtU32;

static ALARM0: Mutex<RefCell<Option<Alarm<Periodic, Blocking, 0>>>> =
    Mutex::new(RefCell::new(None));
static ALARM1: Mutex<RefCell<Option<Alarm<Periodic, Blocking, 1>>>> =
    Mutex::new(RefCell::new(None));
static ALARM2: Mutex<RefCell<Option<Alarm<Periodic, Blocking, 2>>>> =
    Mutex::new(RefCell::new(None));

#[entry]
fn main() -> ! {
    let peripherals = Peripherals::take();
    let system = SystemControl::new(peripherals.SYSTEM);

    let clocks = ClockControl::max(system.clock_control).freeze();
    let delay = Delay::new(&clocks);

    // access the systemtimer peripheral on the soc
    let systimer = SystemTimer::new(peripherals.SYSTIMER);
    println!("SYSTIMER Current Value: {}", SystemTimer::now());

    critical_section::with(|cs| {
        let mut alarm0 = systimer.alarm0.into_periodic();
        alarm0.set_interrupt_handler(systimer_target0);
        alarm0.set_period(100u32.secs());
        alarm0.enable_interrupt(true);

        ALARM0.borrow_ref_mut(cs).replace(alarm0);
    });

    loop {
        // println!("Internal loop");
        // log::info!("Hello world!");
        // delay.delay(500.millis());
    }
}

#[handler(priority = esp_hal::interrupt::Priority::min())]
fn systimer_target0() {
    println!("Interrupt lvl1 (alarm0)");
    critical_section::with(|cs| {
        ALARM0
            .borrow_ref_mut(cs)
            .as_mut()
            .unwrap()
            .clear_interrupt();
    });
}
