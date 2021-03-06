//! API for the I2C peripherals
//!
//! The entry point to this API is the [`I2C`] struct.
//!
//! The I2C peripheral is described in the following user manuals:
//! - LPC82x user manual, chapter 15
//! - LPC84x user manual, chapter 19
//!
//! # Examples
//!
//! Write data to an I2C slave:
//!
//! ``` no_run
//! # let address = 0x0;
//! # let data    = [0; 8];
//! #
//! use lpc8xx_hal::{
//!     prelude::*,
//!     Peripherals,
//!     i2c,
//! };
//!
//! let mut p = Peripherals::take().unwrap();
//!
//! let mut swm    = p.SWM.split();
//! let mut syscon = p.SYSCON.split();
//!
//! #[cfg(feature = "82x")]
//! let mut swm_handle = swm.handle;
//! #[cfg(feature = "845")]
//! let mut swm_handle = swm.handle.enable(&mut syscon.handle);
//!
//! let (i2c0_sda, _) = swm.fixed_functions.i2c0_sda.assign(
//!     p.pins.pio0_11.into_swm_pin(),
//!     &mut swm_handle,
//! );
//! let (i2c0_scl, _) = swm.fixed_functions.i2c0_scl.assign(
//!     p.pins.pio0_10.into_swm_pin(),
//!     &mut swm_handle,
//! );
//!
//! #[cfg(feature = "82x")]
//! let clock = &(); // I2C is always powered by system clock on LPC82x
//! #[cfg(feature = "845")]
//! let clock = &syscon.iosc;
//!
//! let mut i2c = p.I2C0
//!     .enable(
//!         clock,
//!         i2c0_scl,
//!         i2c0_sda,
//!         &mut syscon.handle,
//!     )
//!     .enable_master_mode(
//!         &i2c::Clock::new_400khz(),
//!     );
//!
//! i2c.master.write(address, &data)
//!     .expect("Failed to write data");
//! ```
//!
//! Please refer to the [examples in the repository] for more example code.
//!
//! [`I2C`]: struct.I2C.html
//! [examples in the repository]: https://github.com/lpc-rs/lpc8xx-hal/tree/master/examples

mod clock;
mod error;
mod instances;
mod interrupts;
mod peripheral;

pub mod master;
pub mod slave;

pub use self::{
    clock::{Clock, ClockSource},
    error::Error,
    instances::Instance,
    interrupts::Interrupts,
    master::Master,
    peripheral::I2C,
    slave::Slave,
};
