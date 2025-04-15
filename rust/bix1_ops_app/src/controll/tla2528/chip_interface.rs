
use crate::controll::tla2528::{
    channel::{try_from_i2c_data, Channel},
    chip_definitions::{
        DataConfig, GeneralConfigFlags, OpCode, Oversampling, RegisterAddress, SamplingRate,
        SequenceConfig, SystemStatusFlags,
    },
    error::Error,
};
use i2cdev::linux::{LinuxI2CDevice, LinuxI2CError};
use std::io;
use i2cdev::core::{I2CDevice, I2CTransfer};

pub(crate) struct ChipInterface {
    i2c: LinuxI2CDevice,
}

impl ChipInterface {
    pub(crate) fn new(i2c: LinuxI2CDevice) -> Self {
        ChipInterface { i2c }
    }

    pub(crate) fn configure_all_pins_as_analog_inputs(&mut self) -> Result<(), LinuxI2CError> {
        self.write_sequence_config(SequenceConfig::Manual)?;

        self.register_write(RegisterAddress::GpioConfig, 0b_0000_0000)?;
        self.register_write(RegisterAddress::GpioDriveConfig, 0b_0000_0000)?;
        self.register_write(RegisterAddress::PinConfig, 0b_0000_0000)
    }

    pub(crate) fn configure_oversampling(&mut self, ratio: Oversampling) -> Result<(), LinuxI2CError> {
        self.register_write(RegisterAddress::OsrConfig, ratio.value())
    }

    pub(crate) fn configure_sampling_rate(&mut self, config: SamplingRate) -> Result<(), LinuxI2CError> {
        self.register_write(RegisterAddress::OpModeConfig, config.value())
    }

    pub(crate) fn set_channel(&mut self, channel: Channel) -> Result<(), LinuxI2CError> {
        self.register_write(RegisterAddress::ChannelSelect, channel as u8)
    }

    pub(crate) fn configure_auto_sequence_mode(&mut self) -> Result<(), LinuxI2CError> {
        self.write_data_config(DataConfig::NormalDataAddChannelID)?;
        self.write_sequence_config(SequenceConfig::StoppedAuto)?;
        self.register_write(RegisterAddress::AutoSequenceChannelSelect, 0b_1111_1111)
    }

    pub(crate) fn configure_manual_mode(&mut self) -> Result<(), LinuxI2CError> {
        self.write_data_config(DataConfig::NormalDataAddChannelID)?;
        self.write_sequence_config(SequenceConfig::Manual)
    }

    pub(crate) fn read_system_status(&mut self) -> Result<SystemStatusFlags, LinuxI2CError> {
        let bits = self.register_read(RegisterAddress::SystemStatus)?;
        Ok(SystemStatusFlags::from_bits_retain(bits))
    }

    pub(crate) fn read_general_config(&mut self) -> Result<GeneralConfigFlags, LinuxI2CError> {
        let bits = self.register_read(RegisterAddress::GeneralConfig)?;
        Ok(GeneralConfigFlags::from_bits_retain(bits))
    }

    pub(crate) fn write_general_config(&mut self, config: GeneralConfigFlags) -> Result<(), LinuxI2CError> {
        self.register_write(RegisterAddress::DataConfig, config.bits())
    }

    pub(crate) fn write_data_config(&mut self, config: DataConfig) -> Result<(), LinuxI2CError> {
        self.register_write(RegisterAddress::DataConfig, config.value())
    }

    pub(crate) fn write_sequence_config(&mut self, config: SequenceConfig) -> Result<(), LinuxI2CError> {
        self.register_write(RegisterAddress::SequenceConfig, config.value())
    }

    fn register_read(&mut self, r: RegisterAddress) -> Result<u8, LinuxI2CError> {
        let mut incoming = [0_u8; 1];
        self.i2c
            .write(&[OpCode::SingleRegisterRead.value(), r.value()])
            .map_err(Error::I2cError);

        self.i2c
            .read(&mut incoming)
            .map_err(Error::I2cError);

        Ok(incoming[0])
    }

    fn register_write(&mut self, r: RegisterAddress, val: u8) -> Result<(), LinuxI2CError> {
        self.i2c
            .write(&[OpCode::SingleRegisterWrite.value(), r.value(), val])
    }

    pub(crate) fn data_read(&mut self) -> Result<[u16; 8], LinuxI2CError> {
        let mut data_buffer = [0_u8; 24]; // 8 channels * 3 bytes each

        self.i2c.read(&mut data_buffer).map_err(Error::I2cError);

        let mut out = [0_u16; 8];
        data_buffer
            .chunks_exact(3)
            .zip(out.iter_mut())
            .for_each(|(chunk, destination)| {
                let mut buf = [0_u8; 2];
                buf.copy_from_slice(&chunk[..2]);
                *destination = u16::from_be_bytes(buf);
            });
        Ok(out)
    }

    pub(crate) fn data_channel_read(&mut self, desired_channel: Channel) -> Result<(u16, usize), LinuxI2CError> {
        const MAX_CHANNEL_READ_TRIES: usize = 32;
        for i in 0..MAX_CHANNEL_READ_TRIES {
            let mut data_buffer = [0_u8; 2];

            self.i2c.read(&mut data_buffer).unwrap();
            println!("BuffeR: {:?}", data_buffer);
            let read_channel = match try_from_i2c_data(data_buffer[1]){
                Ok(val) => val,
                Err(_) => return Err(LinuxI2CError::Errno(100))
            };

            if read_channel == desired_channel {
                let mut buf = [0_u8; 2];
                buf.copy_from_slice(&data_buffer[..2]);
                return Ok((u16::from_be_bytes(buf) >> 4, i + 1));
            }
        }
        Err(LinuxI2CError::Errno(99))
    }
}
