// Copyright (c) 2021 Intel Corporation
//
// SPDX-License-Identifier: BSD-2-Clause-Patent

#![no_std]
use chrono::NaiveDate;

pub mod rtc;

pub fn get_rtc_time() -> i64 {
    let year = rtc::read_year().unwrap_or(1970);
    let month = rtc::read_month().unwrap_or(1);
    let day = rtc::read_day().unwrap_or(1);
    let hour = rtc::read_hour().unwrap_or(0);
    let min = rtc::read_min().unwrap_or(0);
    let sec = rtc::read_sec().unwrap_or(0);
    NaiveDate::from_ymd(year, month, day).and_hms(hour, min, sec).timestamp()
}

#[cfg(test)]
mod tests {
    use super::get_rtc_time;
    #[test]
    fn it_works() {
        assert_ne!(get_rtc_time(), 0);
    }
}
