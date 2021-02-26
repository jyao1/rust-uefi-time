// Copyright (c) 2021 Intel Corporation
//
// SPDX-License-Identifier: BSD-2-Clause-Patent
use x86::io;

//
// Dallas DS12C887 Real Time Clock
//
const RTC_ADDRESS_SECONDS:u8            =0;   // R/W  Range 0..59
// const RTC_ADDRESS_SECONDS_ALARM:u8      =1;   // R/W  Range 0..59
const RTC_ADDRESS_MINUTES:u8            =2;   // R/W  Range 0..59
// const RTC_ADDRESS_MINUTES_ALARM:u8      =3;   // R/W  Range 0..59
const RTC_ADDRESS_HOURS:u8              =4;   // R/W  Range 1..12 or 0..23 Bit 7 is AM/PM
// const RTC_ADDRESS_HOURS_ALARM:u8        =5;   // R/W  Range 1..12 or 0..23 Bit 7 is AM/PM
// const RTC_ADDRESS_DAY_OF_THE_WEEK:u8    =6;   // R/W  Range 1..7
const RTC_ADDRESS_DAY_OF_THE_MONTH:u8   =7;   // R/W  Range 1..31
const RTC_ADDRESS_MONTH:u8              =8;   // R/W  Range 1..12
const RTC_ADDRESS_YEAR:u8               =9;   // R/W  Range 0..99
// const RTC_ADDRESS_REGISTER_A:u8         =10;  // R/W[0..6]  R0[7]
// const RTC_ADDRESS_REGISTER_B:u8         =11;  // R/W
// const RTC_ADDRESS_REGISTER_C:u8         =12;  // RO
// const RTC_ADDRESS_REGISTER_D:u8         =13;  // RO

const RTC_INDEX_REGISTER:u16            =0x70; // RTC Index Register address in I/O space.
const RTC_TARGET_REGISTER:u16           =0x71; // RTC Target Register address in I/O space.

pub fn read_year() -> Option<i32> {
    let mut century = 19;
    let year = rtc_read(RTC_ADDRESS_YEAR)?;
    if year < 70 {
        century += 1;
    }
    Some(century * 100 + i32::from(year))

}

pub fn read_month() -> Option<u32> {
    Some(u32::from(rtc_read(RTC_ADDRESS_MONTH)?))
}

pub fn read_day() -> Option<u32> {
    Some(u32::from(rtc_read(RTC_ADDRESS_DAY_OF_THE_MONTH)?))
}

pub fn read_hour() -> Option<u32> {
    let hour = rtc_read(RTC_ADDRESS_HOURS)?;
    let is_pm = hour & 0x80u8 != 0;
    let hour = hour & 0x7f;
    let res = if is_pm && hour < 12 {
        u32::from(hour+12)
    } else if !is_pm && hour == 12 {
        0
    } else {
        u32::from(hour)
    };
    Some(res)
}

pub fn read_min() -> Option<u32> {
    Some(u32::from(rtc_read(RTC_ADDRESS_MINUTES)?))
}

pub fn read_sec() -> Option<u32> {
    Some(u32::from(rtc_read(RTC_ADDRESS_SECONDS)?))
}

// Converts an 8-bit BCD value to an 8-bit value.
fn bcd_to_decimal8(value: u8) -> u8 {
    (value >> 4) * 10 + (value & 0xf)
}

// Checks an 8-bit BCD value, and converts to an 8-bit value if valid.
pub fn check_and_convert_bcd8_to_decimal8 (value: u8) -> Option<u8> {
    if (value < 0xa0) && ((value & 0xf) < 0xa) {
        Some(bcd_to_decimal8 (value))
    } else {
        None
    }
}

fn rtc_read(address: u8) -> Option<u8> {
    unsafe {
        let addr = address | (io::inb(RTC_INDEX_REGISTER ) & 0x80);
            io::outb(RTC_INDEX_REGISTER, addr);
            check_and_convert_bcd8_to_decimal8(io::inb(RTC_TARGET_REGISTER))
    }
}

