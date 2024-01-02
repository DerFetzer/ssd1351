pub mod spi;

pub trait DisplayInterface {
    fn send_command(&mut self, cmd: u8) -> Result<(), ()>;
    fn send_data(&mut self, buf: &[u8]) -> Result<(), ()>;
    async fn send_data_async(&mut self, buf: &[u8]) -> Result<(), ()>;
}

pub use self::spi::SpiInterface;
