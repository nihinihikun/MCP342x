# explanation

## new(address:u8)
This function sets a slave device address. Call this function at first.

## read_mcp342x(i2c: &mut i2c::I2cDriver,channel:u8,sample_rate_bit:u8,pga:u8)
read_mcp342x() sends a slave device to read voltage data, and catches data from the slave device. This returns Vec<u8>, 2 or 3(only 18bit mode) bytes.

## convert_readdata(read_rawdata: &Vec<u8>)
After geting data from slave device(call read_mcp342x()), convert_readdata() converts rawdata to voltage(double).


## read_and_convert_mcp342x(i2c: &mut i2c::I2cDriver,channel:u8,sample_rate_bit:u8,pga:u8)
read_and_convert_mcp342x() gets rawdata and converts it. This function just call read_mcp342x() and convert_readdata().