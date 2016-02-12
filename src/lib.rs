#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]

extern crate datetime;
use datetime::{sys_timezone, TimeZone};
use datetime::zone::TimeZoneSource;

extern crate phf;

mod data;
pub use data::*;


pub trait ZoneinfoData {
    fn get(timezone_name: &str) -> Option<TimeZone>;

    fn system() -> Option<TimeZone> {
        sys_timezone().and_then(|tz| Self::get(&tz))
    }
}

impl ZoneinfoData for TimeZone {
    fn get(timezone_name: &str) -> Option<TimeZone> {
        lookup(timezone_name).map(|tz| TimeZone(TimeZoneSource::Static(tz)))
    }
}
