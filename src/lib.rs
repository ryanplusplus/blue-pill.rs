#![no_std]

pub mod time {
  use cortex_m_rt::exception;

  static mut SYSTEM_TICK: SystemTick = SystemTick { ticks: 0 };

  pub type Ticks = u32;

  pub trait TimeSource {
    fn get_ticks(&self) -> Ticks;
  }

  pub struct SystemTick {
    ticks: Ticks,
  }

  impl TimeSource for SystemTick {
    fn get_ticks(&self) -> Ticks {
      loop {
        unsafe {
          let a = core::ptr::read_volatile(&self.ticks);
          let b = core::ptr::read_volatile(&self.ticks);
          if a == b {
            return a;
          }
        }
      }
    }
  }

  impl SystemTick {
    pub fn get() -> &'static SystemTick {
      unsafe {
        return &SYSTEM_TICK;
      }
    }
  }

  #[exception]
  fn SysTick() {
    unsafe {
      SYSTEM_TICK.ticks += 1;
    }
  }
}
