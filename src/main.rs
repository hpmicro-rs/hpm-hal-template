#![no_main]
#![no_std]
#![feature(type_alias_impl_trait)]

use defmt::info;
use hpm_hal::gpio::{Level, Output, Speed};
use {defmt_rtt as _, panic_halt as _, embassy_executor, hpm_hal};


#[embassy_executor::main]
async fn main(_spawner: embassy_executor::Spawner) {
    let p = hpm_hal::init(Default::default());

    // TODO: Change the LED output pin
    let mut led = Output::new(p.PA10, Level::Low, Speed::Fast);

    loop {
        led.toggle();
        embassy_time::Timer::after_millis(500).await;
        info!("tick");
    }
}
