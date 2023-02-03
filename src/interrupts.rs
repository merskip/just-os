use lazy_static::lazy_static;
use pic8259::ChainedPics;
use spin::Mutex;
use x86_64::{
    instructions::{self, port::Port},
    structures::idt::{InterruptDescriptorTable, InterruptStackFrame, PageFaultErrorCode},
};

use crate::{gdt, log_debug, log_error};

const PIC_1_OFFSET: u8 = 32;
const PIC_2_OFFSET: u8 = PIC_1_OFFSET + 8;

lazy_static! {
    static ref IDT: InterruptDescriptorTable = {
        let mut idt = InterruptDescriptorTable::new();
        idt.breakpoint.set_handler_fn(breakpoint_handler);
        unsafe {
            idt.double_fault
                .set_handler_fn(double_fault_handler)
                .set_stack_index(gdt::DOUBLE_FAULT_IST_INDEX);
        }
        idt.page_fault.set_handler_fn(page_fault_handler);
        idt[ExternalInterrupt::Timer.as_usize()].set_handler_fn(timer_interrupt_handler);
        idt[ExternalInterrupt::Keybaord.as_usize()].set_handler_fn(keybaord_handler);
        idt
    };
    static ref PICS: Mutex<ChainedPics> =
        Mutex::new(unsafe { ChainedPics::new(PIC_1_OFFSET, PIC_2_OFFSET) });
}

pub fn init() {
    IDT.load();
    unsafe { PICS.lock().initialize() };
}

pub fn enable() {
    instructions::interrupts::enable();
}

pub fn disable() {
    instructions::interrupts::disable();
}

pub fn halt_loop() -> ! {
    loop {
        instructions::hlt();
    }
}

#[derive(Debug, Clone, Copy)]
#[repr(u8)]
enum ExternalInterrupt {
    Timer = PIC_1_OFFSET,
    Keybaord = PIC_1_OFFSET + 1,
}

impl ExternalInterrupt {
    fn as_u8(self) -> u8 {
        self as u8
    }

    fn as_usize(self) -> usize {
        usize::from(self.as_u8())
    }
}

extern "x86-interrupt" fn breakpoint_handler(stack_frame: InterruptStackFrame) {
    log_debug!("EXCEPTION: BREAKPOINT\n{:#?}", stack_frame);
}

extern "x86-interrupt" fn double_fault_handler(
    stack_frame: InterruptStackFrame,
    _error_code: u64,
) -> ! {
    panic!("EXCEPTION: DOUBLE FAULT\n{:#?}", stack_frame);
}

extern "x86-interrupt" fn page_fault_handler(
    stack_frame: InterruptStackFrame,
    error_code: PageFaultErrorCode,
) {
    use x86_64::registers::control::Cr2;

    log_error!("EXCEPTION: PAGE FAULT");
    log_error!("Accessed Address: {:?}", Cr2::read());
    log_error!("Error Code: {:?}", error_code);
    log_error!("{:#?}", stack_frame);
    
    loop {}
}

extern "x86-interrupt" fn timer_interrupt_handler(_stack_frame: InterruptStackFrame) {
    // println!("timer_interrupt");
    unsafe {
        PICS.lock()
            .notify_end_of_interrupt(ExternalInterrupt::Timer.as_u8())
    }
}

extern "x86-interrupt" fn keybaord_handler(_stack_frame: InterruptStackFrame) {
    let mut port = Port::new(0x60);
    let scan_code: u8 = unsafe { port.read() };

    crate::task::keyboard::add_scan_code(scan_code);

    unsafe {
        PICS.lock()
            .notify_end_of_interrupt(ExternalInterrupt::Keybaord.as_u8())
    }
}
