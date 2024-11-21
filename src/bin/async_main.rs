#![no_std]
#![no_main]

use embassy_executor::{Spawner,task};
use embassy_time::{Duration, Timer};
use esp_backtrace as _;
use esp_hal::prelude::*;
use log::info;

extern crate alloc;


#[embassy_executor::task]
/// Task that ticks periodically
async fn tick_periodic() -> ! {
    loop {
        info!("I'm the periodic task!");
        Timer::after(Duration::from_millis(2000)).await;
    }
}

#[main]
async fn main(spawner: Spawner) {
    let peripherals = esp_hal::init({
        let mut config = esp_hal::Config::default();
        config.cpu_clock = CpuClock::max();
        config
    });

    esp_alloc::heap_allocator!(72 * 1024);

    esp_println::logger::init_logger_from_env();

    let timg0 = esp_hal::timer::timg::TimerGroup::new(peripherals.TIMG0);
    esp_hal_embassy::init(timg0.timer0);
    info!("Embassy initialized!");

    let timg1 = esp_hal::timer::timg::TimerGroup::new(peripherals.TIMG1);
    let _init = esp_wifi::init(
        esp_wifi::EspWifiInitFor::Wifi,
        timg1.timer0,
        esp_hal::rng::Rng::new(peripherals.RNG),
        peripherals.RADIO_CLK,
    )
    .unwrap();

    // TODO: Spawn some tasks
    //let _ = spawner;
    spawner.spawn(tick_periodic()).unwrap();

    loop {
        info!("Hello Raul Huertas!");
        Timer::after(Duration::from_secs(1)).await;
    }


}
