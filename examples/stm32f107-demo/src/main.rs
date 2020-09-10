//! VL53L1 test with the STM32F107.

#![no_main]
#![no_std]

use cortex_m_rt::entry;
use panic_rtt_target as _;
use rtt_target::{rprintln, rtt_init_print};
use stm32f1xx_hal::{
    self as hal,
    delay::Delay,
    gpio::{gpiob, Alternate, OpenDrain},
    pac,
    prelude::*,
    stm32,
};
use vl53l1::reg;

type I2C = stm32::I2C1;
type Scl = gpiob::PB8<Alternate<OpenDrain>>;
type Sda = gpiob::PB9<Alternate<OpenDrain>>;
type I2cPins = (Scl, Sda);
type BlockingI2c = hal::i2c::BlockingI2c<I2C, I2cPins>;

#[entry]
fn main() -> ! {
    rtt_init_print!();
    rprintln!("Begin!");

    let p = pac::Peripherals::take().unwrap();
    let cp = cortex_m::Peripherals::take().unwrap();

    let mut flash = p.FLASH.constrain();
    let mut rcc = p.RCC.constrain();
    let clocks = rcc.cfgr.freeze(&mut flash.acr);
    let mut afio = p.AFIO.constrain(&mut rcc.apb2);
    let mut gpiob = p.GPIOB.split(&mut rcc.apb2);

    let scl = gpiob.pb8.into_alternate_open_drain(&mut gpiob.crh);
    let sda = gpiob.pb9.into_alternate_open_drain(&mut gpiob.crh);

    // let i2c_mode = hal::i2c::Mode::Standard {
    //     frequency: 100_000.hz(),
    // };
    let i2c_mode = hal::i2c::Mode::Fast {
        frequency: 400_000.hz(),
        duty_cycle: hal::i2c::DutyCycle::Ratio2to1,
    };
    let start_timeout_us = 1_000;
    let start_retries = 10;
    let addr_timeout_us = 1_000;
    let data_timeout_us = 1_000;

    rprintln!("Initialising I2C");
    let mut i2c = BlockingI2c::i2c1(
        p.I2C1,
        (scl, sda),
        &mut afio.mapr,
        i2c_mode,
        clocks,
        &mut rcc.apb1,
        start_timeout_us,
        start_retries,
        addr_timeout_us,
        data_timeout_us,
    );

    let mut delay = Delay::new(cp.SYST, clocks);

    let mut vl53l1_dev = vl53l1::Device::default();

    rprintln!("Software reset...");
    vl53l1::software_reset(&mut vl53l1_dev, &mut i2c, &mut delay).unwrap();
    rprintln!("  Complete");

    rprintln!("Data init...");
    vl53l1::data_init(&mut vl53l1_dev, &mut i2c).unwrap();
    rprintln!("  Complete");

    rprintln!("Static init...");
    vl53l1::static_init(&mut vl53l1_dev).unwrap();
    rprintln!("  Complete");

    rprintln!("Setting region of interest...");
    let roi = vl53l1::UserRoi {
        bot_right_x: 10,
        bot_right_y: 6,
        top_left_x: 6,
        top_left_y: 10,
    };
    vl53l1::set_user_roi(&mut vl53l1_dev, roi).unwrap();
    rprintln!("  Complete");

    rprintln!("Setting timing budget and inter-measurement period...");
    vl53l1::set_measurement_timing_budget_micro_seconds(&mut vl53l1_dev, 50_000).unwrap();
    vl53l1::set_inter_measurement_period_milli_seconds(&mut vl53l1_dev, 60).unwrap();

    rprintln!("Start measurement...");
    vl53l1::start_measurement(&mut vl53l1_dev, &mut i2c).unwrap();
    rprintln!("  Complete");

    rprintln!("Wait measurement data ready...");
    vl53l1::wait_measurement_data_ready(&mut vl53l1_dev, &mut i2c, &mut delay).unwrap();
    rprintln!("  Ready");

    rprintln!("Get ranging measurement data...");
    let rmd = vl53l1::get_ranging_measurement_data(&mut vl53l1_dev, &mut i2c).unwrap();
    vl53l1::clear_interrupt_and_start_measurement(&mut vl53l1_dev, &mut i2c, &mut delay).unwrap();
    rprintln!("{:#?}", rmd);

    loop {
        rprintln!("Wait measurement data ready...");
        vl53l1::wait_measurement_data_ready(&mut vl53l1_dev, &mut i2c, &mut delay).unwrap();
        rprintln!("  Ready");

        rprintln!("Get ranging measurement data...");
        let rmd = vl53l1::get_ranging_measurement_data(&mut vl53l1_dev, &mut i2c).unwrap();
        vl53l1::clear_interrupt_and_start_measurement(&mut vl53l1_dev, &mut i2c, &mut delay)
            .unwrap();
        rprintln!("  {:#?} mm", rmd.range_milli_meter);

        //delay.delay_ms(1000_u32);
    }
}
