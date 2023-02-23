use x86::cpuid::CpuId;
use crate::{println, serial_println};
use crate::command::command::Command;

pub fn cpuid_command(_command: Command) {
    let cpuid = CpuId::new();

    if let Some(processor_brand) = cpuid.get_processor_brand_string() {
        println!("CPU brand: {}", processor_brand.as_str());
    }
    if let Some(vendor_info) = cpuid.get_vendor_info() {
        println!("Vendor: {:?}", vendor_info.as_str());
    }

    serial_println!("{:#?}", cpuid);
}
