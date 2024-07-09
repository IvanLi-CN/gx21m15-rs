#![no_std]

#[cfg(not(feature = "async"))]
use embedded_hal::i2c::I2c;
#[cfg(feature = "async")]
use embedded_hal_async::i2c::I2c;

#[repr(u8)]
pub enum Register {
    Temperature = 0x00,
    Config = 0x01,
    TemperatureHysteresis = 0x02,
    TemperatureOverShutdown = 0x03,
}

#[repr(u8)]
pub enum OsFailQueueSize {
    One = 0,
    Two = 1,
    Four = 2,
    Eight = 3,
}

pub struct Gx21m15Config {
    value: u8,
}

impl Gx21m15Config {
    pub fn new() -> Self {
        Self { value: 0 }
    }

    pub fn set_os_fail_queue_size(&mut self, size: OsFailQueueSize) -> &mut Self {
        self.value |= ((size as u8) << 3) & 0b0001_1000;
        self
    }

    pub fn set_os_polarity(&mut self, positive: bool) -> &mut Self {
        self.value |= ((positive as u8) << 2) & 0b0000_0100;
        self
    }

    pub fn set_os_mode(&mut self, interrupt_mode: bool) -> &mut Self {
        self.value |= ((interrupt_mode as u8) << 1) & 0b0000_0010;
        self
    }

    pub fn set_shutdown(&mut self, shutdown: bool) -> &mut Self {
        self.value |= (shutdown as u8) & 0b0000_0001;
        self
    }

    pub fn build(&self) -> u8 {
        self.value
    }

    pub fn get_os_fail_queue_size(&self) -> OsFailQueueSize {
        match self.value & 0b0001_1000 {
            0b0000_0000 => OsFailQueueSize::One,
            0b0000_0100 => OsFailQueueSize::Two,
            0b0000_1000 => OsFailQueueSize::Four,
            0b0001_0000 => OsFailQueueSize::Eight,
            _ => OsFailQueueSize::One,
        }
    }

    pub fn get_os_polarity(&self) -> bool {
        (self.value & 0b0000_0100) != 0
    }

    pub fn get_os_mode(&self) -> bool {
        (self.value & 0b0000_0010) != 0
    }

    pub fn get_shutdown(&self) -> bool {
        (self.value & 0b0000_0001) != 0
    }
}

pub struct Gx21m15<I2C> {
    address: u8,
    i2c: I2C,
}

#[maybe_async_cfg::maybe(
    sync(cfg(not(feature = "async")), self = "Gx21m15",),
    async(feature = "async", keep_self)
)]
impl<I2C, E> Gx21m15<I2C>
where
    I2C: I2c<Error = E>,
{
    pub fn new(i2c: I2C, address: u8) -> Self {
        Self { address, i2c }
    }

    pub async fn set_config(&mut self, config: &Gx21m15Config) -> Result<(), E> {
        self.i2c
            .write(self.address, &[Register::Config as u8, config.build()])
            .await?;
        Ok(())
    }

    pub async fn get_config(&mut self) -> Result<Gx21m15Config, E> {
        let mut buf = [0u8; 1];
        self.i2c
            .write_read(self.address, &[Register::Config as u8], &mut buf)
            .await?;
        Ok(Gx21m15Config { value: buf[0] })
    }

    pub async fn get_temperature(&mut self) -> Result<f32, E> {
        let mut buf = [0u8; 2];
        self.i2c
            .write_read(self.address, &[Register::Temperature as u8], &mut buf)
            .await?;

        Ok(get_temperature_value_from_bytes(buf))
    }

    pub async fn get_temperature_hysteresis(&mut self) -> Result<f32, E> {
        let mut buf = [0u8; 2];
        self.i2c
            .write_read(
                self.address,
                &[Register::TemperatureHysteresis as u8],
                &mut buf,
            )
            .await?;

        Ok(get_temperature_value_from_bytes(buf))
    }

    pub async fn set_temperature_hysteresis(&mut self, value: f32) -> Result<(), E> {
        let buffer = get_temperature_hysteresis_bytes_from_value(value);
        self.i2c
            .write(
                self.address,
                &[Register::TemperatureHysteresis as u8, buffer[0], buffer[1]],
            )
            .await
    }

    pub async fn get_temperature_over_shutdown(&mut self) -> Result<f32, E> {
        let mut buf = [0u8; 2];
        self.i2c
            .write_read(
                self.address,
                &[Register::TemperatureOverShutdown as u8],
                &mut buf,
            )
            .await?;

        Ok(get_temperature_hysteresis_value_from_bytes(buf))
    }

    pub async fn set_temperature_over_shutdown(&mut self, value: f32) -> Result<(), E> {
        let buffer = get_temperature_hysteresis_bytes_from_value(value);
        self.i2c
            .write(
                self.address,
                &[
                    Register::TemperatureOverShutdown as u8,
                    buffer[0],
                    buffer[1],
                ],
            )
            .await
    }
}

pub fn get_temperature_value_from_bytes(bytes: [u8; 2]) -> f32 {
    let msb = bytes[0];
    let lsb = bytes[1];

    let is_negative = msb & 0x80 != 0;

    let integer_part;
    let fractional_part;

    if is_negative {
        let combined = (((msb as u16) << 3) | (lsb as u16) >> 5) - 1;
        let combined = !combined;
        integer_part = -(((combined & 0x7ff) >> 3) as i8);
        fractional_part = -((combined & 0x7) as f32 * 0.125);
    } else {
        integer_part = (msb & 0x7F) as i8;
        fractional_part = (lsb >> 5) as f32 * 0.125;
    }

    let value = integer_part as f32 + fractional_part;

    value
}

pub fn get_temperature_hysteresis_value_from_bytes(bytes: [u8; 2]) -> f32 {
    let msb = bytes[0];
    let lsb = bytes[1];

    let is_negative = msb & 0x80 != 0;

    let integer_part;
    let fractional_part;

    if is_negative {
        let combined = (((msb as u16) << 1) | (lsb as u16) >> 7) - 1;
        let combined = !combined;
        integer_part = -(((combined & 0xFF) >> 1) as i8);
        fractional_part = -((combined & 0x1) as f32 * 0.5);
    } else {
        integer_part = (msb & 0x7F) as i8;
        fractional_part = (lsb >> 7) as f32 * 0.5;
    }

    let value = integer_part as f32 + fractional_part;

    value
}

pub fn get_temperature_hysteresis_bytes_from_value(value: f32) -> [u8; 2] {
    let integer_part = value as i8;
    let fractional_part = value - integer_part as f32;

    let integer_part = integer_part.wrapping_abs() as u8;
    let fractional_part = if fractional_part == 0f32 { 0u8 } else { 1u8 };

    if value < 0.0 {
        let mut combined = (integer_part as u16) << 1 | fractional_part as u16;
        combined = !combined + 1;
        combined = (combined << 7) & 0x7F80 | 0x8000u16;
        combined.to_be_bytes()
    } else {
        let combined = (integer_part as u16) << 8 | (fractional_part as u16) << 7;
        combined.to_be_bytes()
    }
}
