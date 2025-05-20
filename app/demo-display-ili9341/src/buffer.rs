use embedded_graphics::pixelcolor::Rgb565;
use rp_pico::hal::dma::ReadTarget;

pub struct DataBuffer<const S: usize> {
    pub buffer: [Rgb565; S]
}

unsafe impl<const S: usize> ReadTarget for DataBuffer<S>{
    type ReceivedWord = u8;

    fn rx_treq() -> Option<u8> {
        None
    }

    fn rx_address_count(&self) -> (u32, u32) {
        let ptr = self.buffer.as_ptr();
        let len = self.buffer.len()*2;
        (ptr as u32, len as u32)
    }

    fn rx_increment(&self) -> bool {
        true
    }
}