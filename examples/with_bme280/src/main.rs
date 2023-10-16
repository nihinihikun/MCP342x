use anyhow::anyhow;
use log::*;
use esp_idf_hal::{delay::FreeRtos,delay::BLOCK, i2c, prelude::*};
use mcp342x::MCP342X;

const BME280_ADDRESS:u8 = 0x76;//change address to 0x77 if you need.

fn main() -> anyhow::Result<()> {
    esp_idf_sys::link_patches();
    esp_idf_svc::log::EspLogger::initialize_default();

    let peripherals = Peripherals::take().expect("never fail");

    //I2C init
    let i2c = peripherals.i2c0;
    let sda = peripherals.pins.gpio0;
    let scl = peripherals.pins.gpio1;
    let config = i2c::I2cConfig::new().baudrate(100.kHz().into());
    let mut i2c = i2c::I2cDriver::new(i2c, sda, scl, &config)?;

    //relation about bme280 ,read chip_id
    let init_addr=[0xF4,0b00100111];
    i2c.write(BME280_ADDRESS, &init_addr,BLOCK)?;
    FreeRtos::delay_ms(1000);

    //relation about bme280,read dig_T
    let dig_T=(i2c_read16_swab(&mut i2c,0x88)?,i2c_read16_swab(&mut i2c,0x8a)?,i2c_read16_swab(&mut i2c,0x8c)?);
    debug!("dig_T: {:?}",dig_T);
                                    
    //MCP342x address set
    let adc =MCP342X::new(0b1101000);
    loop{
        //relation about bme280,read temperature
        let temperature_addr = 0xFA;    
        let mut  data:[u8;3]=[0x00,0x00,0x00];
        i2c.write_read(BME280_ADDRESS, &[temperature_addr], &mut data, BLOCK)?;
        let data_msb = (data[0] as u32) << 12;//convert pressuredata
        let data_lsb = (data[1] as u32) << 4;
        let data_xlsb = (data[2] as u32) >> 4;
        let data_u32 = data_msb | data_lsb | data_xlsb;
        let temperature=temperature_converter(&data_u32,&dig_T)?;
        debug!(
            "BME280--data: {:?},data_u32: {:?},temperature: {:?}",
            data,
            data_u32,
            temperature,
        );
        //read mcp342x
        let voltage=adc.read_and_convert_mcp342x(&mut i2c,1,16,1)?;
        println!("bme temperature:{:?}C,voltage: {:?}",temperature,voltage);
        FreeRtos::delay_ms(1000);
    }
}


//relation about bme280
fn temperature_converter(adc_T:&u32,dig_T:&(u16,u16,u16))-> anyhow::Result<f32>{
    let (dig_T1,dig_T2,dig_T3)=dig_T;
    let var1 = ((((adc_T>>3) as f32) - (*dig_T1 as f32)) * (*dig_T2 as f32)) / 16384.0;
    let var2 = (((((adc_T>>4) as f32) - (*dig_T1 as f32)) * ((adc_T>>4) as f32) - (*dig_T2 as f32)) / 131072.0) * (*dig_T3 as f32);
    let t_fine = (var1 + var2) as i32;
    let T = (t_fine * 5 + 128) >> 8;
    let T = T as f32 / 100.0;
    Ok(T)
}

fn i2c_read16_swab(i2c: &mut i2c::I2cDriver,pointer:u8)-> anyhow::Result<u16>{
    let mut data_read16:[u8;2]=[0x00,0x00];
    i2c.write_read(BME280_ADDRESS, &[pointer], &mut data_read16, BLOCK)?;
    let data_u16 = ((data_read16[1] as u16) << 8) | (data_read16[0] as u16);
    FreeRtos::delay_ms(100);
    Ok(data_u16)
}

