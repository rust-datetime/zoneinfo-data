#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]

extern crate datetime;
use datetime::{sys_timezone, TimeZone};

mod data;
pub use data::*;


pub trait ZoneinfoData {
    fn get(timezone_name: &str) -> Option<TimeZone<'static>>;

    fn system() -> Option<TimeZone<'static>> {
        sys_timezone().and_then(|tz| Self::get(&tz))
    }
}

impl<'a> ZoneinfoData for TimeZone<'a> {
    fn get(timezone_name: &str) -> Option<TimeZone<'static>> {
        lookup(timezone_name)
    }
}