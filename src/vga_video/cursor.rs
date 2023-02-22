use x86_64::instructions::port::Port;

pub trait Cursor {

}

pub struct VgaCursor {
    control_port: Port<u8>,
    data_port: Port<u8>,
}

impl VgaCursor {
    pub fn new() -> Self {
        Self {
            control_port: Port::new(0x3d4),
            data_port: Port::new(0x3d5),
        }
    }
}

impl Cursor for VgaCursor {

}