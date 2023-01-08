
pub trait InputStream {
    fn read(&self) -> Option<u8>;
}

pub trait OutputStream {
    fn write(&mut self, value: u8);
}
