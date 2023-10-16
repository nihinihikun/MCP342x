use anyhow::anyhow;
use log::*;
use esp_idf_hal::{delay::FreeRtos,delay::BLOCK, i2c, prelude::*};
use mcp342x::MCP342X;

fn main() -> anyhow::Result<()> {
    esp_idf_sys::link_patches();
    esp_idf_svc::log::EspLogger::initialize_default();

    let peripherals = Peripherals::take().expect("never fail");
    
    let i2c = peripherals.i2c0;
    let sda = peripherals.pins.gpio0;
    let scl = peripherals.pins.gpio1;
    let config = i2c::I2cConfig::new().baudrate(100.kHz().into());
    let mut i2c = i2c::I2cDriver::new(i2c, sda, scl, &config)?;
    let adc =MCP342X::new(0b1101000);
    loop{
        let voltage=adc.read_and_convert_mcp342x(&mut i2c,1,16,1)?;
        println!("voltage: {}",voltage);
        FreeRtos::delay_ms(1500);
    }
}
