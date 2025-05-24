use embedded_hal::spi::{ErrorType, Operation, SpiBus, SpiDevice};

/// Simple wrapper for SpiBus as SpiDevice, needed to compile the source, otherwise
/// compiler is confused with two versions of embedded-hal library used in the project
/// Hopefully this will not be needed with future versions of rp_2040
pub struct SpiWrapper<S: SpiBus> {
    pub bus: S,
}

impl<S: SpiBus> ErrorType for SpiWrapper<S> { type Error = S::Error; }

impl<S: SpiBus> SpiDevice for SpiWrapper<S> {
    fn transaction(&mut self, _operations: &mut [Operation<'_, u8>]) -> Result<(), Self::Error> {
        unimplemented!();
    }

    fn read(&mut self, buf: &mut [u8]) -> Result<(), Self::Error> {
        self.bus.read(buf)
    }

    fn write(&mut self, buf: &[u8]) -> Result<(), Self::Error> {
        self.bus.write(buf)
    }

    fn transfer(&mut self, read: &mut [u8], write: &[u8]) -> Result<(), Self::Error> {
        self.bus.transfer(read, write)
    }

    fn transfer_in_place(&mut self, buf: &mut [u8]) -> Result<(), Self::Error> {
        self.bus.transfer_in_place(buf)
    }
}