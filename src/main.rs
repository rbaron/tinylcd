#![no_std]
#![no_main]

use defmt::*;
use embassy_executor::Spawner;
use embassy_stm32::lcd::{Bias, Config, Duty, Lcd, LcdPin};
use embassy_time::Timer;
use {defmt_rtt as _, panic_probe as _};

mod segmap;

#[embassy_executor::main(executor = "embassy_stm32::Executor", entry = "cortex_m_rt::entry")]
async fn main(_spawner: Spawner) {
    let p = embassy_stm32::init(embassy_stm32::Config::default());

    let mut config = Config::default();
    config.bias = Bias::Third;
    config.duty = Duty::Quarter;
    config.use_voltage_output_buffer = true;
    config.voltage_source = embassy_stm32::lcd::VoltageSource::Internal;

    let mut lcd = Lcd::new(
        p.LCD,
        config,
        p.PB2,
        [
            // COM.
            LcdPin::new_com(p.PA8),
            LcdPin::new_com(p.PA9),
            LcdPin::new_com(p.PA10),
            LcdPin::new_com(p.PB9),
            // SEG.
            LcdPin::new_seg(p.PA1),
            LcdPin::new_seg(p.PA2),
            LcdPin::new_seg(p.PA3),
            LcdPin::new_seg(p.PA4),
            LcdPin::new_seg(p.PA6),
            LcdPin::new_seg(p.PA7),
            LcdPin::new_seg(p.PB3),
            LcdPin::new_seg(p.PB6),
        ],
    );

    let mut buff = segmap::Buff::new();

    let str = "  rbaron-net   ";
    let mut scroll_pos = 0;

    loop {
        let slice = &str[scroll_pos..scroll_pos + 3];

        info!("Displaying: \"{}\", len {}", slice, slice.len());

        for (i, c) in slice.chars().enumerate() {
            buff.set_char(i as u8, c);
        }

        for com in 0..4 {
            lcd.write_com_segments(com, buff.data[com as usize] as u64);
        }

        lcd.submit_frame();

        Timer::after_millis(500).await;

        scroll_pos = (scroll_pos + 1) % (str.len() - 3);
    }
}
