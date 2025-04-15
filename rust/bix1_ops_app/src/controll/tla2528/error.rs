use i2cdev::linux::LinuxI2CError;

#[derive(Debug)]
#[non_exhaustive]
pub enum Error {
    I2cError(LinuxI2CError),
    _DataItemsMisOrdered,
    IncorrectChannelAddress,
    InvalidChannelAddress,
}

impl From<LinuxI2CError> for Error {
    fn from(other: LinuxI2CError) -> Self {
        Error::I2cError(other)
    }
}
