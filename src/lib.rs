//MCP342x lib
use log::*;
use esp_idf_hal::{i2c,delay::BLOCK};//

//device address settings
// |Addressbit(LSB-2,1,0)||   Adr pins     |
// | A2 | A1 | A0 || Adr0pin    | Adr1pin  |
// -------------------------------------------
// | 0  | 0  | 0  || Addr_Low  | Addr_Low  |
// | 0  | 0  | 1  || Addr_Low  | Float     |
// | 0  | 1  | 0  || Addr_Low  | Addr_High |
// | 1  | 0  | 0  || Addr_High | Addr_Low  |
// | 1  | 0  | 1  || Addr_High | Float     |
// | 1  | 1  | 0  || Addr_High | Addr_High |
// | 0  | 1  | 1  || Float     | Addr_Low  |
// | 1  | 1  | 1  || Float     | Addr_High |
// | 0  | 0  | 0  || Float     | Float     |
//exapmle:if Adr0=HIGH,Adr1=LOW, address is 0b1101_"010"0

pub struct MCP342X {
    mcp342x_address: u8,
}

impl MCP342X {
    pub fn new(address: u8) -> Self {
        MCP342X {
            mcp342x_address: address,
        }
    }
    pub fn read_and_convert_mcp342x(&self,i2c: &mut i2c::I2cDriver,channel:u8,sample_rate_bit:u8,pga:u8)->Result<f64, anyhow::Error>{
        let buffer_vec=self.read_mcp342x(i2c,channel,sample_rate_bit,pga)?;
        let voltage=self.convert_readdata(&buffer_vec)?;
        Ok(voltage)
    }  

    pub fn read_mcp342x(&self,i2c: &mut i2c::I2cDriver,channel:u8,sample_rate_bit:u8,pga:u8) -> Result<Vec<u8>, anyhow::Error> {  
        let mut config_mcp342x = 0b1000_0000;
        // match readybit {
        //     1=>{config_mcp342x=config_mcp342x|0b1000_0000;}
        //     0=>{config_mcp342x=config_mcp342x|0b0000_0000;}
        // }
        match channel{
            1=>{config_mcp342x=config_mcp342x|0b0000_0000;}
            2=>{config_mcp342x=config_mcp342x|0b0010_0000;}
            3=>{config_mcp342x=config_mcp342x|0b0100_0000;}
            4=>{config_mcp342x=config_mcp342x|0b0110_0000;}
            _=>{debug!("user designed channel is out of range");}
        }
        // match conversation_mode {
        //     continuous => config_mcp342x = config_mcp342x | 0b0000_0000,
        //     one_shot => config_mcp342x = config_mcp342x | 0b0000_1000,
        // }
        match sample_rate_bit{
            12=>{config_mcp342x=config_mcp342x|0b0000_0000;}
            14=>{config_mcp342x=config_mcp342x|0b0000_0100;}
            16=>{config_mcp342x=config_mcp342x|0b0000_1000;}
            18=>{config_mcp342x=config_mcp342x|0b0000_1100;}
            _=>{debug!("user designed sample_rate_bit is out of range");}
        }
        match pga{
            1=>{config_mcp342x=config_mcp342x|0b0000_0000;}
            2=>{config_mcp342x=config_mcp342x|0b0000_0001;}
            4=>{config_mcp342x=config_mcp342x|0b0000_0010;}
            8=>{config_mcp342x=config_mcp342x|0b0000_0011;}
            _=>{debug!("user designed pga is out of range");}
        }
        debug!("senddata--config_mcp342x: {:08b}",config_mcp342x);
        //decide buffersize by sample_rate,only 18bitmode needs 4byte buffer,other mode needs 3byte buffer
        let buffer_length;
        if (config_mcp342x & 0b0000_1100) == 0b0000_1100 {
            buffer_length = 4;
        } else {
            buffer_length = 3;
        }
        let mut buffer = vec![0u8; buffer_length];
        i2c.write_read(self.mcp342x_address, &[config_mcp342x], &mut buffer, BLOCK)?;
        Ok(buffer)
    }

    pub fn convert_readdata(&self,read_rawdata: &Vec<u8>) -> Result<f64, anyhow::Error> {
        // convert configuration data
        let mut configration_byte_num = 0;
        let mut readdata_broken=false;
        match read_rawdata.len(){
            3=>{configration_byte_num=2;}//samplerate=18bit
            4=>{configration_byte_num=3;}//sample_rate=16,14,12bit
            _=>{readdata_broken=true;
                debug!("read_rawdata is too long");
            }
        }
        let configration_byte_data = read_rawdata[configration_byte_num];
        let channel = (((configration_byte_data & 0b0110_0000) >> 5) as u8)+ 1;
        let conversation_mode = (configration_byte_data & 0b0001_0000) >> 4;
        let samplerate_bit = (configration_byte_data & 0b0000_1100) >> 2;
        let mut samplerate_bitnum=0;
        match samplerate_bit {
            0b00 => {samplerate_bitnum=12}// 12bitmode, 240sps;
            0b01 => {samplerate_bitnum=14}// 14bitmode, 60sps;
            0b10 => {samplerate_bitnum=16}// 16bitmode, 15sps;
            0b11 => {samplerate_bitnum=18}// 18bitmode, 3sps;
            _ => {readdata_broken=true;}
        }
        let pga_bit = configration_byte_data & 0b0000_0011;
        let mut pga = 0.0;
        match pga_bit {
            0b00 => pga = 1.0,
            0b01 => pga = 2.0,
            0b10 => pga = 4.0,
            0b11 => pga = 8.0,
            _ => {readdata_broken=true;}
        }
        // convert rawadata to voltage
        let lsb=2.0*2.048/((2<<(samplerate_bitnum-1))as f64);//V_REF=2.048V
        let mut outputcode=0.0;
        let mut over_underflow=false;
    
        if((read_rawdata[0] & 0b1000_0000)>>7)==0{
        //positive
            match samplerate_bitnum{
                12=>{outputcode = (((read_rawdata[0] as u16 & 0b0000_0111) << 8) | (read_rawdata[1] as u16)) as f64;if outputcode>=2047 as f64{over_underflow=true;}}
                14=>{outputcode=(((read_rawdata[0] as u16 & 0b0001_1111) << 8) | (read_rawdata[1] as u16)) as f64;if outputcode>=8191 as f64{over_underflow=true;}}
                16=>{outputcode=(((read_rawdata[0] as u16 & 0b0111_1111) << 8) | (read_rawdata[1] as u16)) as f64;if outputcode>=32767 as f64{over_underflow=true;}}
                18=>{outputcode = (((read_rawdata[0] as u32) << 16) |((read_rawdata[1] as u32) << 8) |(read_rawdata[2] as u32)) as f64;if outputcode>=131071 as f64{over_underflow=true;}}
                _=>{outputcode=0.0;readdata_broken=true;}
            }
    
        }else{
        //negative  
            match samplerate_bitnum {
            12=>{outputcode = (((((!read_rawdata[0]) as u16 & 0b0000_0111)<<8)| (!read_rawdata[1]) as u16)+1) as f64;if outputcode<=-2048 as f64{over_underflow=true;}}
            14=>{outputcode = (((((!read_rawdata[0]) as u16 & 0b0001_1111)<<8)| (!read_rawdata[1]) as u16)+1) as f64;if outputcode<=-8192 as f64{over_underflow=true;}}
            16=>{outputcode = (((((!read_rawdata[0]) as u16 & 0b0111_1111)<<8)| (!read_rawdata[1]) as u16)+1) as f64;if outputcode<=-32768 as f64{over_underflow=true;}}
            18=>{outputcode = ((((((!read_rawdata[0]) & 0b0000_0001)as u32 )<<16 )| ((!read_rawdata[1]as u32)<<8 ) | (!read_rawdata[2] as u32))+1) as f64;if outputcode<=-131072 as f64{over_underflow=true;}}
            _=>{outputcode=0.0;readdata_broken=true;}
            } 
        outputcode = -outputcode;
        }
        let voltage=outputcode*lsb/pga;
    
        if readdata_broken {
            debug!("readdata is broken");
        }
    
        if over_underflow {
            debug!("adc says overflow or underflow");
        }
        debug!("received data ---buffer:{:?},channel: {}, conversation_mode: {}, {}bitmode, pga: {},outputcode: {}, lsb:{},voltage: {}",read_rawdata,channel, conversation_mode, samplerate_bitnum, pga,outputcode,lsb,voltage);
        Ok(voltage)
    }       
}


