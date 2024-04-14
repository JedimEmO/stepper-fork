//! Compatibility code to help use Stepper on more platforms

use core::fmt;

use embedded_hal::digital::ErrorType;
use embedded_hal::digital::OutputPin;

/// Wrapper around a pin
///
/// Provides an implementation of [`embedded_hal::digital::OutputPin`]
pub struct Pin<T>(pub T);

impl<T> ErrorType for Pin<T>
where
    T: OutputPin,
    T::Error: fmt::Debug,
{
    type Error = T::Error;
}

impl<T> OutputPin for Pin<T>
where
    T: OutputPin,
    T::Error: fmt::Debug,
{
    fn set_low(&mut self) -> Result<(), Self::Error> {
        self.0.set_low()
    }

    fn set_high(&mut self) -> Result<(), Self::Error> {
        self.0.set_high()
    }
}
