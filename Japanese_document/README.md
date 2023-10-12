# MCP342x
MCP342x シリーズのRustライブラリ
## 依存関係
- log
- esp_idf_hal::i2c
- esp_idf_hal::delay::BLOCK

## 使い方
### 1. mcp3424を使うことを宣言
```
use mcp3424::MCP3424;
```

### 2. i2c関係を宣言
```
let i2c = peripherals.i2c0;
let sda = peripherals.pins.gpio0;//declare sda pin
let scl = peripherals.pins.gpio1;//declare scl pin
let config = i2c::I2cConfig::new().baudrate(100.kHz().into());
let mut i2c = i2c::I2cDriver::new(i2c, sda, scl, &config)?;
```
### 3. インスタンス化
new()の引数はi2cの7bitのデバイスアドレスである．Adr0，Adr1のpinをデータシートのP.21のTable 5-3か以下の表のようにすることでアドレスを選択できる．
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

### 4. 電圧測定
インスタンス化がすんだら，read_and_convert_adc()でdouble型の電圧[V]が受け取れる．チャンネル，サンプリングレート，拡大率が選択できる．

引数:
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

### 高度な利用
read_mcp3424() でmcp342xからの生データ(2 または 3 Byte)を得られる．また，read_and_convert_adc()でそのデータをdoubleの電圧値[V]に変換できる．詳細はスクリプトをよんで欲しい．


## テストした環境(2023/10/12)
### ソフトウェア
- WSL2 (Ubuntu 20.04)
- esp-idf-sys = { version = "=0.32", features = ["binstart"] }
- esp-idf-svc = { version="=0.45", features = ["experimental", "alloc"] }
- embedded-svc = "0.24"
- log = "0.4"
- anyhow = "1"
- embedded-hal = "=1.0.0-alpha.9"
- esp-idf-hal = "0.40.1"
### ハードウェア
- M5Stamp c3
- mcp3424
