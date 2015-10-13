
// ------
// This file is autogenerated!
// Any changes you make may be overwritten.
// ------


use datetime::zoned::zoneinfo::*;
use datetime::zoned::zoneinfo::Saving::*;

#[allow(unused_imports)]
use data::rulesets;

pub const ZONE: Zone<'static> = Zone {
    name: "HST",
    timespans: &[
        Timespan {
            offset: -36000,
            format: "HST",
            saving: Saving::NoSaving,
            start_time: None,
            end_time:   None,
        },
    ],
};


