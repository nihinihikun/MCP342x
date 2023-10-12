# MCP342x
A Rust crate for interfacing with the MCP3424 ADC.

## dependences
- log
- esp_idf_hal::i2c
- esp_idf_hal::delay::BLOCK

## Usage
### 1. declare use mcp3424
```
use mcp3424::MCP3424;
```

### 2. decalre i2c
```
let i2c = peripherals.i2c0;
let sda = peripherals.pins.gpio0;//declare sda pin
let scl = peripherals.pins.gpio1;//declare scl pin
let config = i2c::I2cConfig::new().baudrate(100.kHz().into());
let mut i2c = i2c::I2cDriver::new(i2c, sda, scl, &config)?;
```
### 3. instantiate
Argument of new() is device address.
7bits device address is 0b110_1xxx. xxx is selectable. datasheet(P.21,Table5-3) or table below.
```
let adc =MCP3424::new(0b1101000);
```


<table>
	<tr>
		<td colspan="3">I²C Device Address Bits</td>
		<td colspan="2">Logic Status of Address Selection Pins</td>
	</tr>
	<tr>
		<td>A2</td>
		<td>A1</td>
		<td>A0</td>
		<td>Adr0 pin</td>
        <td>Adr1 pin</td>
	</tr>
	<tr>
		<td>0</td>
		<td>0</td>
		<td>0</td>
		<td>Low</td>
        <td>Low</td>
	</tr>
	<tr>
		<td>0</td>
		<td>0</td>
		<td>1</td>
		<td>Low</td>
        <td>Float</td>
	</tr>
	<tr>
		<td>0</td>
		<td>1</td>
		<td>0</td>
		<td>Low</td>
        <td>High</td>
	</tr>
	<tr>
		<td>1</td>
		<td>0</td>
		<td>0</td>
		<td>High</td>
        <td>Low</td>
	</tr>
	<tr>
		<td>1</td>
		<td>0</td>
		<td>1</td>
		<td>High</td>
        <td>Float</td>
	</tr>
	<tr>
		<td>1</td>
		<td>1</td>
		<td>0</td>
		<td>HIgh</td>
        <td>High</td>
	</tr>
	<tr>
		<td>0</td>
		<td>1</td>
		<td>1</td>
		<td>Float</td>
        <td>Low</td>
	</tr>
	<tr>
		<td>1</td>
		<td>1</td>
		<td>1</td>
		<td>Float</td>
        <td>High</td>
	</tr>
	<tr>
		<td>0</td>
		<td>0</td>
		<td>0</td>
		<td>Float</td>
        <td>Float</td>
	</tr>
</table>

### 4. Get voltage
After instantiate, you can get voltage data as f64 using read_and_convert_adc. 
You can select channel, samplerate, pga.

Argument-num:
1. I2C
1. channel
    - 1, 2, 3(MCP3424 only), 4(MCP3424 only)
1. sample rate
    - 12, 14, 16, 18 bit
1. pga
    - x1, x2, x4, x8

```
let voltage=adc.read_and_convert_adc(&mut i2c,1,16,1)?;  //chanel=1,16bitmode,pga=x1
```

### Advenced use
You can get raw data from mcp342x using read_mcp3424() and  convert raw data using read_and_convert_adc(). See script if you know details.


## Testing environment(2023/10/12)
### software
- WSL2 (Ubuntu 20.04)
- esp-idf-sys = { version = "=0.32", features = ["binstart"] }
- esp-idf-svc = { version="=0.45", features = ["experimental", "alloc"] }
- embedded-svc = "0.24"
- log = "0.4"
- anyhow = "1"
- embedded-hal = "=1.0.0-alpha.9"
- esp-idf-hal = "0.40.1"
### hardware
- M5Stamp c3
- mcp3424
