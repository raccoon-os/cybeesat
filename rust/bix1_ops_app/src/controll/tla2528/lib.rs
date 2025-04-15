
// pub mod channel;
// pub mod chip_definitions;
// mod chip_interface;
// pub mod error;

use crate::controll::tla2528::{
    channel::Channel,
    chip_definitions::{
        GeneralConfigFlags, Oversampling, SamplingRate, SequenceConfig, SystemStatusFlags,
    },
    chip_interface::ChipInterface,
    error::Error,
};
use i2cdev::linux::{LinuxI2CDevice, LinuxI2CError};

pub struct Tla2528 {
    chip: ChipInterface,
}

impl Tla2528 {
    pub fn new(i2c: LinuxI2CDevice) -> Self {
        Tla2528 {
            chip: ChipInterface::new(i2c),
        }
    }

    pub fn get_system_status(&mut self) -> Result<SystemStatusFlags, LinuxI2CError> {
        self.chip.read_system_status()
    }

    pub fn calibrate(&mut self) -> Result<(), LinuxI2CError> {
        self.chip
            .write_general_config(GeneralConfigFlags::CALIBRATE_ADC_OFFSET)?;

        while self
            .chip
            .read_general_config()?
            .contains(GeneralConfigFlags::CALIBRATE_ADC_OFFSET)
        {
            // Intentionally empty
        }
        Ok(())
    }

    pub fn set_oversampling_ratio(&mut self, ratio: Oversampling) -> Result<(), LinuxI2CError> {
        self.chip.configure_oversampling(ratio)
    }

    pub fn set_sampling_rate(&mut self, rate: SamplingRate) -> Result<(), LinuxI2CError> {
        self.chip.configure_sampling_rate(rate)
    }

    pub fn prepare_for_auto_sequence_mode(&mut self) -> Result<(), LinuxI2CError> {
        self.chip.configure_all_pins_as_analog_inputs()?;
        self.chip.configure_auto_sequence_mode()
    }

    pub fn prepare_for_manual_mode(&mut self) -> Result<(), LinuxI2CError> {
        self.chip.configure_all_pins_as_analog_inputs()?;
        self.chip.configure_manual_mode()
    }

    pub fn acquire_data(&mut self) -> Result<[u16; 8], LinuxI2CError> {
        self.chip
            .write_sequence_config(SequenceConfig::StartedAuto)?;

        let data = self.chip.data_read()?;

        self.chip
            .write_sequence_config(SequenceConfig::StoppedAuto)?;

        Ok(data)
    }

    pub fn acquire_channel_data(&mut self, channel: Channel) -> Result<u16, LinuxI2CError> {
        self.chip.set_channel(channel)?;
        match self.chip.data_channel_read(channel) {
            Ok((data, _)) => Ok(data),
            Err(err) => Err(err),
        }
    }
}
