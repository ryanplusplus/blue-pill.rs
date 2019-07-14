#![no_main]
#![no_std]

// set the panic handler
extern crate panic_semihosting;

use core::sync::atomic::{AtomicBool, Ordering};
use cortex_m::peripheral::syst::SystClkSource;
use cortex_m_rt::{entry, exception};
use stm32f1xx_hal::prelude::*;
use cortex_m_semihosting::hprintln;

use blue_pill::time;
use blue_pill::time::TimeSource;

static TOGGLE_LED: AtomicBool = AtomicBool::new(false);

#[entry]
fn main() -> ! {
  let mut core = cortex_m::Peripherals::take().unwrap();
  let device = stm32f1xx_hal::stm32::Peripherals::take().unwrap();
  let mut rcc = device.RCC.constrain();
  let mut flash = device.FLASH.constrain();

  let clocks = rcc
    .cfgr
    .use_hse(8.mhz())
    .sysclk(16.mhz())
    .freeze(&mut flash.acr);

  // configure the user led
  let mut gpioc = device.GPIOC.split(&mut rcc.apb2);
  let mut led = gpioc.pc13.into_push_pull_output(&mut gpioc.crh);

  // configure SysTick to generate an exception every second
  core.SYST.set_clock_source(SystClkSource::Core);
  core.SYST.set_reload(clocks.sysclk().0 / 1000);
  core.SYST.clear_current();
  core.SYST.enable_counter();
  core.SYST.enable_interrupt();

  let time_source = time::SystemTick::new();

  loop {
    // sleep
    cortex_m::asm::wfi();
    if TOGGLE_LED.swap(false, Ordering::AcqRel) {
      led.toggle();
      hprintln!("{}", time_source.get_ticks()).unwrap();
    }
  }
}

#[exception]
fn SysTick() {
  TOGGLE_LED.store(true, Ordering::Release);
}
