use core::fmt::{Display, Formatter, Result};

use x86_64::instructions::port::{PortReadOnly, PortWriteOnly};

#[derive(Debug, Clone, Copy)]
#[allow(dead_code)]
#[repr(u8)]
enum BankIndex {
    Seconds = 0x00,
    Minutes = 0x02,
    Hours = 0x04,
    DayOfWeek = 0x06,
    DayOfMonth = 0x07,
    Month = 0x08,
    Year = 0x09,
    StatusRegisterA = 0x0A,
    StatusRegisterB = 0x0B,
    StatusRegisterC = 0x0C,
    StatusRegisterD = 0x0D,
}

bitflags! {
    struct StatusRegisterA: u8 {
        const INTERRUPT_INTERVAL = 0b0000_1111;
        const DIVIDER =            0b0111_0000;
        const UPDATING =           0b1000_0000;
    }

    struct StatusRegisterB: u8 {
        const DAYLIGHT_SAVINGS =       0b0000_0001;
        const MODE_24_HOUR =           0b0000_0010;
        const BINARY_DATA =            0b0000_0100;
        const SQUARE_WAVE_OUTPUT =     0b0000_1000;
        const UPDATE_ENDED_INTERRUPT = 0b0001_0000;
        const ALARM_INTERRUPT =        0b0010_0000;
        const PERIODIC_INTERRUPT =     0b0100_0000;
        const CYCLE_UPDATE =           0b1000_0000;
    }
}

pub struct RTC {
    address_port: PortWriteOnly<u8>,
    data_port: PortReadOnly<u8>,
}

impl RTC {
    pub fn new() -> Self {
        RTC {
            address_port: PortWriteOnly::<u8>::new(0x70),
            data_port: PortReadOnly::<u8>::new(0x71),
        }
    }
}

impl RTC {
    pub fn read_datetime(&mut self) -> RTCDateTime {
        while self.is_updating() {
            // Waiting to finish updating
        }

        let mut seconds = self.read_bank(BankIndex::Seconds);
        let mut minuts = self.read_bank(BankIndex::Minutes);
        let mut hours = self.read_bank(BankIndex::Hours);
        let mut day_of_month = self.read_bank(BankIndex::DayOfMonth);
        let mut month = self.read_bank(BankIndex::Month);
        let mut year = self.read_bank(BankIndex::Year);

        let status_b = self.read_status_b();

        // Convert BCD to binary if needed
        if !status_b.contains(StatusRegisterB::BINARY_DATA) {
            seconds = bdc_to_binary(seconds);
            minuts = bdc_to_binary(minuts);
            hours = bdc_to_binary(hours);
            day_of_month = bdc_to_binary(day_of_month);
            month = bdc_to_binary(month);
            year = bdc_to_binary(year);
        }

        // Convert 12 hour clock to 24 gour clock if neeed
        if !status_b.contains(StatusRegisterB::MODE_24_HOUR) {
            hours = ((hours & 0x7F) + 12) % 24;
        }

        // Convert to 4-digit year
        let year = (year as u16) + 2000;

        RTCDateTime {
            seconds,
            minuts,
            hours,
            day_of_month,
            month,
            year,
        }
    }

    fn is_updating(&mut self) -> bool {
        let mut status_a = self.read_status_a();
        status_a.contains(StatusRegisterA::UPDATING)
    }

    fn read_status_a(&mut self) -> StatusRegisterA {
        let value = self.read_bank(BankIndex::StatusRegisterA);
        StatusRegisterA::from_bits(value).unwrap()
    }

    fn read_status_b(&mut self) -> StatusRegisterB {
        let value = self.read_bank(BankIndex::StatusRegisterB);
        StatusRegisterB::from_bits(value).unwrap()
    }

    fn read_bank(&mut self, index: BankIndex) -> u8 {
        unsafe { self.address_port.write(index as u8) }
        unsafe { self.data_port.read() }
    }
}

pub struct RTCDateTime {
    seconds: u8,
    minuts: u8,
    hours: u8,
    day_of_month: u8,
    month: u8,
    year: u16,
}

impl Display for RTCDateTime {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> Result {
        write!(
            formatter,
            "{:04}-{:02}-{:02} {:02}:{:02}:{:02}",
            self.year, self.month, self.day_of_month, self.hours, self.minuts, self.seconds
        )
    }
}

fn bdc_to_binary(value: u8) -> u8 {
    (value & 0x0F) + (((value & 0x70) / 16) * 10) | (value & 0x80)
}
